use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize)]
pub struct Credentials {
    pub name: String,
    pub password: String,
}
