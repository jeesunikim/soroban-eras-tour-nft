use crate::contract::DataKey;
use soroban_sdk::{Address, Env};

// why should balance use 'persistent'?
// persistent storage vs. instance; persistent storage:
// - unlimited amount of storage
// - suitable for user data that cannot be Temporary (i.e. balances)
// - unlike instance storage, each entry has its OWN lifetime and must be bumped individually

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
