#![no_std]

pub use crate::erc721traits::burnable::ERC721Burnable;
pub use crate::erc721traits::erc721::ERC721;
pub use crate::erc721traits::metadata::ERC721Metadata;
pub use crate::types::*;
use storage::Storage;

use soroban_sdk::{panic_with_error, Address, BytesN, Env, IntoVal, Map, String, Val, Vec};

mod erc721traits;
mod types;

#[cfg(test)]
mod tests;

#[cfg_attr(test, soroban_sdk::contract)]
pub struct ERC721Contract;

// https://eips.ethereum.org/EIPS/eip-721
#[cfg_attr(test, soroban_sdk::contractimpl)]
impl ERC721 for ERC721Contract {
    // @param env: smart contract environment
    // @param owner: an address for whom to query the balance
    // @return the number of all NFTs owned by owner
    fn balance_of(env: Env, owner: Address) -> u32 {
        DataKey::Balance(owner)
            .bump(&env, 1000)
            .get(&env)
            .unwrap_or(0)
    }

    // Transfer ownership of an NFT
    // @param from: The current owner of the NFT
    // @param to: The new owner
    // @param token_id: The NFT to transfer
    fn transfer_from(env: Env, from: Address, to: Address, token_id: u32) {
        from.require_auth();

        if let Some(addr) = DataKey::TokenOwner(token_id).get::<Address>(&env) {
            if addr == from {
                DataKey::TokenOwner(token_id).set(&env, &to);
            } else {
                panic_with_error!(&env, Error::NotOwner);
            }
        } else {
            panic_with_error!(&env, Error::NotNFT);
        }
    }
    fn approve(
        env: Env,
        caller: Address,
        operator: Option<Address>,
        token_id: u32,
        expiration_ledger: u32,
    ) {
        if let Some(owner) = DataKey::TokenOwner(token_id).get::<Address>(&env) {
            if owner == caller {
                owner.require_auth();
            } else if DataKey::Operator(owner, caller.clone())
                .get::<bool>(&env)
                .unwrap_or(false)
            {
                caller.require_auth();
            }
        } else {
            panic_with_error!(&env, Error::NotNFT);
        }
        if let Some(to_approve) = operator {
            DataKey::Approved(token_id).set(&env, &to_approve);
            DataKey::Approved(token_id).bump_until(&env, expiration_ledger);
        } else {
            DataKey::Approved(token_id).remove(&env);
        }
    }
    
    fn set_approval_for_all(
        env: Env,
        caller: Address,
        owner: Address,
        operator: Address,
        approved: bool,
        expiration_ledger: u32,
    ) {
        if owner == caller {
            owner.require_auth();
        } else if DataKey::Operator(owner.clone(), caller.clone())
            .get::<bool>(&env)
            .unwrap_or(false)
        {
            caller.require_auth();
        } else {
            panic_with_error!(&env, Error::NotAuthorized);
        }
        let key = DataKey::Operator(owner, operator);
        if approved {
            key.set(&env, &true);
            key.bump_until(&env, expiration_ledger);
        } else {
            key.remove(&env);
        }
    }
    // Get the approved address for a single NFT
    // @param token_id: The NFT to find the approved address for
    // @return The approved address for this NFT, or "None"
    fn get_approved(env: Env, token_id: u32) -> Option<Address> {
        DataKey::Approved(token_id).get(&env).unwrap_or(None)
    }
    // Query if an address is an authorized operator for another address
    // @param owner: The address that owns the NFTs
    // @param operator: The address that acts on behalf of the owner
    // @return True if `operator` is an approved operator for `owner`, false otherwise
    fn is_approval_for_all(env: Env, owner: Address, operator: Address) -> bool {
        DataKey::Operator(owner, operator)
            .get(&env)
            .unwrap_or(false)
    }
}

#[cfg(feature = "metadata")]
#[cfg_attr(test, soroban_sdk::contractimpl)]
impl ERC721Metadata for ERC721Contract {
    fn name(env: Env) -> String {
        DatakeyMetadata::Name.get(&env).unwrap()
    }
    fn symbol(env: Env) -> String {
        DatakeyMetadata::Symbol.get(&env).unwrap()
    }
    fn token_uri(env: Env, token_id: u32) -> String {
        DatakeyMetadata::Uri(token_id)
            .get(&env)
            .unwrap_or_else(|| String::from_slice(&env, "no uri"))
    }
}

#[cfg(feature = "burnable")]
#[cfg_attr(test, soroban_sdk::contractimpl)]
impl ERC721Burnable for ERC721Contract {
    fn burn(env: Env, caller: Address, token_id: u32) {
        let owner: Address = DataKey::TokenOwner(token_id)
            .get(&env)
            .unwrap_or_else(|| panic_with_error!(&env, Error::NotNFT));
        if owner == caller {
            owner.require_auth();
        } else if DataKey::Operator(owner.clone(), caller.clone())
            .get::<bool>(&env)
            .unwrap_or(false)
        {
            caller.require_auth();
        } else {
            panic_with_error!(&env, Error::NotAuthorized);
        }

        DataKey::Approved(token_id).remove(&env);
        DataKey::TokenOwner(token_id).remove(&env);

        let balance_key = DataKey::Balance(owner.clone());
        let balance = balance_key.get(&env).unwrap_or(0);
        balance_key.set(&env, &(balance - 1));

        let v: Val = token_id.into();
        Event::Burn.publish(&env, v);
    }
}

#[cfg_attr(test, soroban_sdk::contractimpl)]
impl ERC721Contract {
    pub fn initialize(
        env: Env,
        admin: Address,
        #[cfg(feature = "metadata")] name: String,
        #[cfg(feature = "metadata")] symbol: String,
    ) {
        if Admin::User.has(&env) {
            panic!("Already initialized")
        }
        Admin::User.set(&env, &admin);

        env.storage().instance().bump(10000);
        if cfg!(feature = "metadata") {
            env.storage().instance().set(&DatakeyMetadata::Name, &name);
            env.storage()
                .instance()
                .set(&DatakeyMetadata::Symbol, &symbol);
        }
    }

    pub fn upgrade(env: Env, hash: BytesN<32>) {
        get_admin(&env).require_auth();
        env.deployer().update_current_contract_wasm(hash);
    }

    pub fn mint(env: Env, to: Address, token_id: u32) {
        // Authorization should be handled by the caller of the actual implementation
        // get_admin(&env).require_auth();

        if !env.storage().instance().has(&DataKey::TokenOwner(token_id)) {
            DataKey::TokenOwner(token_id).set(&env, &to);

            let key = DataKey::Balance(to.clone());
            let balance: u32 = key.get(&env).unwrap_or(0);
            key.set(&env, &(balance + 1));
        }
        let mut v: Vec<Val> = Vec::new(&env);
        v.push_back(to.into_val(&env));
        v.push_back(token_id.into());
        Event::Mint.publish(&env, v);
    }
}

pub fn get_admin(env: &Env) -> Address {
    if let Some(addr) = Admin::User.get(env) {
        addr
    } else {
        panic_with_error!(env, Error::NotAuthorized)
    }
}
