// swift_fan related data: persistent because swift_fan must pay for himself
// authorizations: temporary because it should not be restored
// metadata: instance because the contract "admin" should pay

// must have functionalities
// 1. nft token admin minting 10 tickets as a token
// 2. transferring the token to a ticket purchaser
// 2.1 metadata to reflect the concert seating
// 3. burning the unsold tokens

#![cfg(test)]
extern crate std;

use crate::{contract::ErasNftContract, ErasNftContractClient};
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, Logs},
    Address, Env, IntoVal,
};

fn create_token<'a>(env: &Env, admin: &Address) -> ErasNftContractClient<'a> {
    let token = ErasNftContractClient::new(env, &env.register_contract(None, ErasNftContract {}));
    token.initialize(admin, &"Eras Tour".into_val(env), &"Eras".into_val(env));
    
    token
}

#[test]
fn test_mint() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::random(&env);
    let swift_fan_1 = Address::random(&env);
    let swift_fan_2 = Address::random(&env);

    let eras_token = create_token(&env, &admin);

    eras_token.mint(&swift_fan_1, &1);
    eras_token.mint(&swift_fan_2, &2);
    eras_token.mint(&swift_fan_1, &3);
    // assert_eq!(eras_token.balance_of(&swift_fan_1), 2);

    std::println!("{}", env.logs().all().join("\n"));
}

#[test]
fn test_owner_of() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::random(&env);
    let swift_fan_1 = Address::random(&env);

    let eras_token = create_token(&env, &admin);

    eras_token.mint(&swift_fan_1, &1);

    assert_eq!(eras_token.owner_of(&1), swift_fan_1);

    std::println!("{}", env.logs().all().join("\n"));
}

#[test]
#[should_panic(expected = "seat already taken")]
fn seat_already_taken() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::random(&env);
    let swift_fan_1 = Address::random(&env);
    let swift_fan_2 = Address::random(&env);

    let eras_token = create_token(&env, &admin);

    eras_token.mint(&swift_fan_1, &1);
    // assert_eq!(eras_token.balance_of(&swift_fan_1), 1);

    eras_token.mint(&swift_fan_2, &1);

    std::println!("{}", env.logs().all().join("\n"));
}