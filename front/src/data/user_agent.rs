use serde::{Deserialize, Serialize};
use std::fmt::Display;
use wasm_bindgen::prelude::*;
use yewdux::prelude::*;

use crate::components::atoms::flag::Country;

#[derive(PartialEq, Clone, Debug, Store, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct UserAgentStore {
    pub country: Country,
}

impl UserAgentStore {
    pub fn add_lang_change_event_listener(dispatch: Dispatch<Self>) {
        let closure = Closure::wrap(Box::new(move || {
            dispatch.reduce_mut(|state| state.country = Country::get_current())
        }) as Box<dyn FnMut()>);
        web_sys::window()
            .unwrap()
            .add_event_listener_with_callback("languagechange", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
}

impl Display for UserAgentStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl Default for UserAgentStore {
    fn default() -> Self {
        Self {
            country: Country::get_current(),
        }
    }
}
