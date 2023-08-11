use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub role: Role,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum Role {
    User,
    Admin,
}