use super::resources::Key;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use yewdux::prelude::*;

#[derive(Default, PartialEq, Clone, Debug, Store, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct EditorStore {
    values: HashMap<String, String>,
}

impl EditorStore {
    pub fn get_state<'a>(&'a self, key: &Key) -> Option<&'a String> {
        self.values.get(&key.to_string())
    }

    pub fn add_or_update_state(&mut self, key: &Key, state: String) {
        self.values.insert(key.to_string(), state);
    }

    pub fn remove_state(&mut self, key: &Key) {
        self.values.remove(&key.to_string());
    }
}
