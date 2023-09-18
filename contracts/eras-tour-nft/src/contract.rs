use crate::event;
use crate::admin::{has_admin, read_admin, write_admin};
use soroban_sdk::{
    contract, contractimpl, contracttype, log, Address, Env, String
};
use crate::storage_types::{BALANCE_BUMP_AMOUNT, DataKey, DatakeyMetadata, INSTANCE_BUMP_AMOUNT, Error, Seats, MAX_SEATS};

#[contracttype]
pub struct Id();

#[contract]
pub struct ErasNftContract;

#[contractimpl]
impl ErasNftContract {
    // doc says: another requirement for complying with the token interface is to write the standard metadata (decimal, name, and symbol) for the token in a specific format
    // @todo: but I omitted decimals
    pub fn initialize(env: Env, admin: Address, name: String, symbol: String) {
        if has_admin(&env) {
            panic!("already initialized")
        }
        
        // set the admin of ErasNftContract
        write_admin(&env, &admin);

        // if decimal > u8::MAX.into() {
        //     panic!("Decimal must fit in a u8");
        // }

        env.storage().instance().set(&DatakeyMetadata::Name, &name);
        env.storage()
            .instance()
            .set(&DatakeyMetadata::Symbol, &symbol);
    }

    // @return the balance of the requested address
    pub fn balance_of(env: Env, owner: Address) -> i128 {
        env.storage().instance().bump(INSTANCE_BUMP_AMOUNT);

        let key = DataKey::Balance(owner);

        log!(&env, "get_banace: key: {}", key);
        
        if let Some(balance) = env.storage().persistent().get::<DataKey, i128>(&key) {
            env.storage().persistent().bump(&key, BALANCE_BUMP_AMOUNT);
            balance
        } else {
            0
        }
    }

    // I skipped uri; usually nft has a property that includes a link but I omitted
    // mint: enables admin to mint nfts
    pub fn mint(env: Env, to: Address, symbol: String, number: u32) -> Result<u32, Error>{
        to.require_auth();

         // Check if the seat is taken
         let key: Seats = Seats::Token(symbol.clone(), number);
         if env.storage().instance().has(&key) {
            panic!("This seat is taken");
        }
        env.storage().instance().set(&key, &(symbol, number));

        // let token_id: u32 = DataKey::TokenId.get(&env).unwrap_or(0);

        let token_id: u32 = env.storage().instance().get(&DataKey::TokenId).unwrap_or(0); // If no value set, assume 0.
        log!(&env, "count: {}", token_id);

        // Check if we reached the max supply
        if token_id > MAX_SEATS {
            //return Err(MillionError::Exhausted);
            panic!("Exhausted")
        }

        env.storage().instance().set(&DataKey::TokenId, &(token_id + 1));
        event::mint(&env, to, token_id);
        Ok(token_id)
    }

    // Transfer adminship of an NFT
    // @param from: The current admin of the NFT
    // @param to: The new admin
    // @param token_id: The NFT to transfer
    // @todo: use soroban's auth to verify the admin
    // fn transfer(env: &Env, from: Address, to: Address, token_id: u32) {
    //     from.require_auth();

    //     event::transfer(&env, from, to, token_id);
    // }
}
