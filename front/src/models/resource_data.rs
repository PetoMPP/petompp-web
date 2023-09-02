use serde::{Deserialize, Serialize};

use crate::api::client::Error;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct ResourceData {
    pub key: String,
    pub en: Option<String>,
    pub pl: Option<String>,
}

impl ResourceData {
    pub fn new_from_lang(key: impl Into<String>, lang: &str, value: impl Into<String>) -> Result<Self, Error> {
        match lang {
            "en" => Ok(Self {
                key: key.into(),
                en: Some(value.into()),
                pl: None,
            }),
            "pl" => Ok(Self {
                key: key.into(),
                en: None,
                pl: Some(value.into()),
            }),
            _ => Err(Error::Parse("Invalid language".into())),
        }
    }
}
