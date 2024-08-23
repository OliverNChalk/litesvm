use std::collections::HashMap;

use solana_sdk::{account::AccountSharedData, pubkey::Pubkey};

pub trait AccountLoader {
    fn get(&self, key: &Pubkey) -> Option<AccountSharedData>;
    fn get_mut(&mut self, key: &Pubkey) -> Option<&mut AccountSharedData>;
    fn insert(&mut self, key: Pubkey, account: AccountSharedData);
}

impl AccountLoader for HashMap<Pubkey, AccountSharedData> {
    fn get(&self, key: &Pubkey) -> Option<AccountSharedData> {
        self.get(key).cloned()
    }

    fn get_mut(&mut self, key: &Pubkey) -> Option<&mut AccountSharedData> {
        self.get_mut(key)
    }

    fn insert(&mut self, key: Pubkey, account: AccountSharedData) {
        self.insert(key, account);
    }
}
