use soroban_sdk::{contracttype, Address};

// @todo: find out the difference between 'pub const' and 'pub(crate) const

#[derive(Clone)]
#[contracttype]
// Contract State
pub enum DataKey {
    Admin,  // instance
    Name,   // instance
    Symbol, // instance
}

#[derive(Clone)]
#[contracttype]
pub enum UserDataKey {
    TokenOwner(u32), // takes in 'seat_num' and return a 'fan address'
    Seat(Address),   // takes in 'fan address' and return 'seat_numæ
}

// #[contracttype(storage=Instance)]
// pub enum ContractState {
//   Admin,
//   Metadata,
// }

// #[contracttype(storage=PERSISTENT)]
// pub enum UserData {
//   Balance,
//   Liabilities,
// }
