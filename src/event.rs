use soroban_sdk::{Address, Env, symbol_short};

pub(crate) fn transfer(e: &Env, from: Address, to: Address, token_id: u32) {
    let topics = (symbol_short!("transfer"), from, to);
    e.events().publish(topics, token_id);
}

pub(crate) fn mint(e: &Env, to: Address, token_id: u32) {
    let topics = (symbol_short!("mint"), to);
    e.events().publish(topics, token_id);
}

pub(crate) fn set_admin(e: &Env, owner: Address, new_admin: Address) {
    let topics = (symbol_short!("set_admin"), owner);
    e.events().publish(topics, new_admin);
}