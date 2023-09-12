use soroban_sdk::{contracterror, contracttype, Address, String};

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

// #[derive(Clone)]
// #[contracttype]
// pub struct SeatDataKey {
//     pub symbol: String,
//     pub number: Number,
// }

// #[contracttype]
// pub struct SeatValue {
//     pub amount: i128,
//     pub expiration_ledger: u32,
// }

#[derive(Clone)]
#[contracttype]
pub enum Seats {
    Token(String, u32),
    Seat(u32),
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    TokenId,
    Balance(Address),
    Nonce(Address),
    State(Address),
    Admin,
}

#[contracterror]
#[derive(Copy, Clone, Debug)]
pub enum Error {
    NotOwner = 0,
    NotNFT = 1,
    NotAuthorized = 2,
    OutOfBounds = 4,
}