#![no_std]

use erc721::{DataKey, ERC721Metadata, Error, ERC721};
use soroban_sdk::{
    contract, contractimpl, panic_with_error, token, Address, Bytes, BytesN, Env, String, Vec
};

// pub trait ConcertNftTrait {
//     fn initialize(e: Env, owner: Address);
//     fn mint(e: Env, owner: Address, token: Address);
//     fn balance_of(e: Env, owner: Address) -> u32;
//     fn transfer(e: Env, from: Address, to: Address, token_id: u32);
//     fn burn(e: Env, from: Address, token_id: u32);
// }

#[contract]
pub struct ConcertNftContract;

#[contractimpl]
impl ConcertNftContract {
    pub fn initialize(env: Env, admin: Address) {
        let name = String::from_slice(&env, "Non-Fungible Token");
        let sym = String::from_slice(&env, "NFT");
        erc721::ERC721Contract::initialize(env, admin, name, sym);
    }

    // minting
    pub fn mint(env: Env, to: Address, uri: String) {
        // Check ownly the admin can mint
        erc721::get_admin(&env).require_auth();

        // Get and increment token id
        let token_id = env.storage().instance().get(&Id()).unwrap_or(0);
        env.storage().instance().set(&Id(), &(token_id + 1));

        // set the uri for the token id
        env.storage()
            .persistent()
            .set(&DatakeyMetadata::Uri(token_id), &uri);

        // Mint
        erc721::ERC721Contract::mint(env.clone(), to.clone(), token_id)
    }

    // @return owner's balance
    pub fn balance_of(env: Env, owner: Address) -> u32 {
        erc721::ERC721Contract::balance_of(env, owner)
    }

    // Transfer ownership of an NFT
    // @param from: The current owner of the NFT
    // @param to: The new owner
    // @param token_id: The NFT to transfer
    // @todo: use soroban's auth to verify the owner
    pub fn transfer(env: Env, from: Address, to: Address, token_id: u32) {
        // erc721::ERC721Contract::transfer_from(env, spender, from, to, token_id)
    }

    // @todo: use soroban's timelock to burn all the unclaimed tokens
    pub fn burn(env: Env, from: Address, token_id:u32) {

    }

}
