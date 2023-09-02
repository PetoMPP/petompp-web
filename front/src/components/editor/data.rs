use serde::{Deserialize, Serialize};
use std::{collections::HashMap, rc::Rc, fmt::Display};
use yewdux::prelude::*;

#[derive(Default, PartialEq, Clone, Debug, Store, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct Store {
    pub values: HashMap<String, State>,
}

#[derive(Default, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct State {
    pub value: String,
    pub preview: bool,
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

pub fn get_or_create_state(
    key: &Key,
    editor_store: &Rc<Store>,
    editor_dispatch: Dispatch<Store>,
) -> State {
    gloo::console::log!(editor_store.values.len());
    match editor_store.values.get(&key.to_string()) {
        Some(s) => {
            gloo::console::log!("found state");
            s.clone()
        }
        None => {
            gloo::console::log!("creating state");
            let state = State::default();
            editor_dispatch.reduce_mut(|s| {
                s.values.insert(key.to_string(), state.clone());
            });
            state
        }
    }
}
