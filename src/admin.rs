use soroban_sdk::{Address, Env};
use crate::storage_types::DataKey;

// why should owner use 'instance'?
// persistent storage vs. instance; instance storage:
// - limited amount of storage available
// - suitable for "shared" contract state that cannot be Temporary (i.e. admin accounts, contract metadata, etc.)
// - one call to Env.storage().instance().bump() will extend the lifetime of ALL Instance entries

pub fn has_admin(e: &Env) -> bool {
    let key = DataKey::Admin;
    e.storage().instance().has(&key)
}

pub fn read_admin(e: &Env) -> Address {
    let key = DataKey::Admin;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_admin(e: &Env, id: &Address) {
    let key = DataKey::Admin;
    e.storage().instance().set(&key, id);
}
