// must have functionalities
// 1. erc721's function: mint, transfer, owner_of
// 2. saving user data: Address and seat_num
#![cfg(test)]

// but the contract is a no_std crate
// we're using std::println to log
// and logs expect `std` to be in scope
extern crate std;

use crate::{contract::ErasNftContract, ErasNftContractClient};
use soroban_sdk::{
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
fn test_transfer() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::random(&env);
    let swift_fan_1 = Address::random(&env);
    let swift_fan_2 = Address::random(&env);

    let eras_token = create_token(&env, &admin);

    eras_token.mint(&swift_fan_1, &1);

    assert_eq!(eras_token.owner_of(&1), swift_fan_1);

    eras_token.transfer(&swift_fan_1, &swift_fan_2, &1);

    assert_eq!(eras_token.owner_of(&1), swift_fan_2);

    eras_token.transfer(&swift_fan_2, &swift_fan_1, &1);

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