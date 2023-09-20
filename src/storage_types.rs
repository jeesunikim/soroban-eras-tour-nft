use soroban_sdk::{contracterror, contracttype, Address};

// @todo: find out the difference between 'pub const' and 'pub(crate) const
pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = 34560; // 2 days
pub(crate) const BALANCE_BUMP_AMOUNT: u32 = 518400; // 30 days
pub(crate) const MAX_SEATS: u32 = 28; 

#[derive(Clone)]
#[contracttype]
pub enum DatakeyMetadata {
    Name,     // instance
    Symbol,   // instance
    // Uri, // instance
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Balance(Address),
    TokenId,
    TokenOwner(u32),
}

#[derive(Clone)]
#[contracttype]
pub enum Seats {
    Token(u32), // takes in seat_num and return token_id
    Seat(u32), // takes in token_id and return seat_num
}

#[contracterror]
#[derive(Copy, Clone, Debug)]
pub enum Error {
    NotOwner = 0,
    NotNFT = 1,
    NotAuthorized = 2,
    SeatTaken = 3,
    OutOfBounds = 4,
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