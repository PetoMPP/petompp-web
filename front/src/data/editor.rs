use super::resources::Key;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use yewdux::prelude::*;

#[derive(Default, PartialEq, Clone, Debug, Store, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct EditorStore {
    values: HashMap<String, EditorState>,
}

impl EditorStore {
    pub fn get_state<'a>(&'a self, key: &Key) -> Option<&'a EditorState> {
        self.values.get(&key.to_string())
    }

    pub fn get_state_mut<'a>(&'a mut self, key: &Key) -> Option<&'a mut EditorState> {
        self.values.get_mut(&key.to_string())
    }

    pub fn remove_state(&mut self, key: &Key) {
        self.values.remove(&key.to_string());
    }

    pub fn add_state(&mut self, key: &Key, state: EditorState) {
        self.values.insert(key.to_string(), state);
    }
}

#[derive(Default, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct EditorState {
    pub value: String,
    pub footprint: i64,
}
