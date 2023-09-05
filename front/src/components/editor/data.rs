use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};
use yewdux::prelude::*;

#[derive(Default, PartialEq, Clone, Debug, Store, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct Store {
    values: HashMap<String, State>,
}

impl Store {
    pub fn get_state<'a>(&'a self, key: &Key) -> Option<&'a State> {
        self.values.get(&key.to_string())
    }

    pub fn get_state_mut<'a>(&'a mut self, key: &Key) -> Option<&'a mut State> {
        self.values.get_mut(&key.to_string())
    }

    pub fn remove_state(&mut self, key: &Key) {
        self.values.remove(&key.to_string());
    }

    pub fn add_state(&mut self, key: &Key, state: State) {
        self.values.insert(key.to_string(), state);
    }
}

#[derive(Default, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct State {
    pub value: String,
}

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
