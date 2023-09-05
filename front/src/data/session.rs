use crate::models::user::User;
use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Default, PartialEq, Clone, Debug, Store, Serialize, Deserialize)]
#[store(storage = "session", storage_tab_sync)]
pub struct SessionStore {
    pub user: Option<User>,
    pub token: Option<String>,
}
