use crate::contract::DataKey;
use soroban_sdk::{Address, Env};

// @todo:
// why should balance use 'persistent'?
// because balance should be permanent
// you do not want to mess with people's balance

pub fn read_balance(e: &Env, addr: Address) -> i128 {
    let key = DataKey::ErasNFTs(addr);
    if let Some(balance) = e.storage().persistent().get::<DataKey, i128>(&key) {
        balance.len()
    } else {
        0
    }
}

fn write_balance(e: &Env, addr: Address, amount: i128) {
    let key = DataKey::Balance(addr);
    e.storage().persistent().set(&key, &amount);
}

pub fn receive_balance(e: &Env, addr: Address, amount: i128) {
    let balance = read_balance(e, addr.clone());
    if !is_authorized(e, addr.clone()) {
        panic!("can't receive when deauthorized");
    }
    write_balance(e, addr, balance + amount);
}

pub fn spend_balance(e: &Env, addr: Address, amount: i128) {
    let balance = read_balance(e, addr.clone());
    if !is_authorized(e, addr.clone()) {
        panic!("can't spend when deauthorized");
    }
    if balance < amount {
        panic!("insufficient balance");
    }
    write_balance(e, addr, balance - amount);
}

pub fn is_authorized(e: &Env, addr: Address) -> bool {
    let key = DataKey::State(addr);
    if let Some(state) = e.storage().persistent().get::<DataKey, bool>(&key) {
        state
    } else {
        true
    }
}

pub fn write_authorization(e: &Env, addr: Address, is_authorized: bool) {
    let key = DataKey::State(addr);
    e.storage().persistent().set(&key, &is_authorized);
}