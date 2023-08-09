use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
}
