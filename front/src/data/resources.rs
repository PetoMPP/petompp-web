use serde::{Deserialize, Serialize};
use std::fmt::Display;

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
