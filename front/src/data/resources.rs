use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use yewdux::prelude::*;

#[derive(Default, PartialEq, Clone, Debug, Serialize, Deserialize, Eq, Hash)]
pub struct Key {
    pub reskey: String,
    pub lang: String,
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}{}", self.reskey, self.lang))
    }
}

#[derive(Default, PartialEq, Clone, Debug, Store, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct ResourceStore {
    values: HashMap<String, String>,
}

impl ResourceStore {
    pub fn get_state<'a>(&'a self, key: &Key) -> Option<&'a String> {
        self.values.get(&key.to_string())
    }

    pub fn add_or_update_state(&mut self, key: &Key, state: String) {
        self.values.insert(key.to_string(), state);
    }
}
