use crate::admin::{has_admin, read_admin, write_admin};
use soroban_sdk::{
    contract, contractimpl, log, Address, Env, String
};
use crate::storage_types::{DataKey, UserDataKey};

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

        env.storage().instance().set(&DataKey::Name, &name);
        env.storage()
            .instance()
            .set(&DataKey::Symbol, &symbol);
    }

    // mint: enables admin to mint nfts
    pub fn mint(env: Env, to: Address, seat_num: u32) {
        let admin = read_admin(&env);
        admin.require_auth();

        // Check if the seat is taken
        if env.storage().persistent().has(&UserDataKey::TokenOwner(seat_num)) {
            panic!("seat already taken");
        }

        let is_taken: bool = env.storage().persistent().has(&UserDataKey::TokenOwner(seat_num));

        log!(&env, "mint - seat_num: {}", seat_num);
        log!(&env, "mint - is_seat_num_taken: {}", is_taken);

        // Minting
        if !env.storage().persistent().has(&UserDataKey::TokenOwner(seat_num)) {
            env.storage().persistent().set(&UserDataKey::TokenOwner(seat_num), &to);
            env.storage().persistent().set(&UserDataKey::Seat(to.clone()), &seat_num);
        }
    }

    pub fn owner_of(env: Env, seat_num: u32) -> Address{
        let owner: Address = env.storage().persistent().get(&UserDataKey::TokenOwner(seat_num)).unwrap_or_else(|| panic!("this seat is not taken by anyone"));

        log!(&env, "owner_of - owner {}", owner);

        owner
    }

    // Transfer ownership of an NFT
    // @param from: The current owner of the NFT
    // @param to: The new owner
    // @param seat_num: The NFT to transfer
    fn transfer(env: &Env, from: Address, to: Address, seat_num: u32) {
        from.require_auth();


    }
}
