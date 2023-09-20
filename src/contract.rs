use crate::event;
use crate::admin::{has_admin, read_admin, write_admin};
use soroban_sdk::{
    contract, contractimpl, contracttype, log, panic_with_error, Address, Env, String
};
use crate::storage_types::{BALANCE_BUMP_AMOUNT, DataKey, DatakeyMetadata, Error, INSTANCE_BUMP_AMOUNT, Seats, MAX_SEATS};

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

        // env.storage().instance().bump(10000);
        env.storage().instance().set(&DatakeyMetadata::Name, &name);
        env.storage()
            .instance()
            .set(&DatakeyMetadata::Symbol, &symbol);
    }

    // @return the total supply of the requested address
    pub fn balance_of(env: Env, owner: Address) -> i128 {
        env.storage().instance().bump(INSTANCE_BUMP_AMOUNT);

        let key = DataKey::Balance(owner);

        log!(&env, "balance_of - key: {}", key);
        
        if let Some(balance) = env.storage().persistent().get::<DataKey, i128>(&key) {
            env.storage().persistent().bump(&key, BALANCE_BUMP_AMOUNT);
            balance
        } else {
            0
        }
    }

    // I skipped uri; usually nft has a property that includes a link but I omitted
    // mint: enables admin to mint nfts
    pub fn mint(env: Env, to: Address, seat_num: u32) {
        let admin = read_admin(&env);
        admin.require_auth();

        // Check if the seat is taken
        if env.storage().persistent().has(&Seats::Token(seat_num)) {
            panic!("seat already taken");
        }
        
        env.storage().instance().bump(INSTANCE_BUMP_AMOUNT);

        let token_id: u32 = env.storage().instance().get(&DataKey::TokenId).unwrap_or(0); // If no value set, assume 0.

        let is_taken: bool = env.storage().persistent().has(&Seats::Token(seat_num));

        log!(&env, "mint - seat_num: {}", seat_num);
        log!(&env, "mint - token_id: {}", token_id);
        log!(&env, "mint - is_seat_num_taken: {}", is_taken);

        // Check if we reached the max supply
        if token_id > MAX_SEATS {
            panic_with_error!(&env, Error::OutOfBounds);
        }

        env.storage().instance().set(&DataKey::TokenId, &(token_id + 1));

        // Minting
        if !env.storage().persistent().has(&DataKey::TokenOwner(token_id)) {
            env.storage().persistent().set(&DataKey::TokenOwner(token_id), &to);
            env.storage().persistent().bump(&DataKey::TokenOwner(token_id), BALANCE_BUMP_AMOUNT);

            env.storage().persistent().set(&Seats::Token(seat_num), &token_id);
            env.storage().persistent().bump(&Seats::Token(seat_num), BALANCE_BUMP_AMOUNT);

            env.storage().persistent().set(&Seats::Seat(token_id), &seat_num);
            env.storage().persistent().bump(&Seats::Seat(token_id), BALANCE_BUMP_AMOUNT);

            let balance: i128 = Self::balance_of(env.clone(), to.clone());
            env.storage().persistent().set(&DataKey::Balance(to.clone()), &(balance + 1));
        }

        event::mint(&env, to, token_id);
    }

    pub fn owner_of(env: Env, seat_num: u32) -> Address{
        // get a 'token_id' from 'seat_num'
        let token_id = env.storage().persistent().get(&Seats::Token(seat_num)).unwrap_or_else(|| panic!("this seat is not taken by anyone"));

        // get an address of the seat owner from 'token_id'
        let owner = env.storage().persistent().get(&DataKey::TokenOwner(token_id)).unwrap_or_else(|| panic!("owner of this token does not exist"));

        log!(&env, "owner_of - token_id {}", token_id);
        log!(&env, "owner_of - owner {}", owner);

        owner
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
