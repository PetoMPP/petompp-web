use std::collections::BTreeMap;

use chrono::Duration;
use enum_iterator::Sequence;

use crate::endpoints::user::User;

use super::{error::AuthError, validation::AccessLevel};

#[derive(Debug, Sequence, Clone, Copy)]
pub enum Claim {
    Subject,
    Expiration,
    AccessLevel,
}

impl Claim {
    pub fn key(&self) -> &'static str {
        match self {
            Claim::Subject => "sub",
            Claim::Expiration => "exp",
            Claim::AccessLevel => "acl",
        }
    }

    pub fn value(&self, user: &User) -> String {
        match self {
            Claim::Subject => user.id.to_string(),
            Claim::Expiration => chrono::Utc::now()
                .checked_add_signed(Duration::minutes(30))
                .unwrap()
                .timestamp()
                .to_string(),
            Claim::AccessLevel => serde_json::to_string(&user.access_level).unwrap(),
        }
    }

    pub fn validate(
        &self,
        claims: &BTreeMap<String, String>,
        access_level: AccessLevel,
    ) -> Result<(), AuthError> {
        let Some(claim) = claims.get(&self.key().to_string()) else {
        return Err(AuthError::MissingClaim(*self));
    };
        self.check(claim, access_level)
    }

    fn check(&self, value: &str, access_level: AccessLevel) -> Result<(), AuthError> {
        match self {
            Claim::Subject => {
                let Ok(_) = value.parse::<u32>() else {
                return Err(AuthError::InvalidFormat(*self));
            };
                Ok(())
            }
            Claim::Expiration => {
                let Ok(exp) = value.parse::<i64>() else {
                return Err(AuthError::InvalidFormat(*self));
            };
                let remaining = exp - chrono::Utc::now().timestamp();
                if remaining < 0 {
                    return Err(AuthError::TokenExpired(Duration::seconds(remaining.abs())));
                }
                Ok(())
            }
            Claim::AccessLevel => {
                let Ok(level) = serde_json::from_str::<AccessLevel>(value) else {
                return Err(AuthError::InvalidFormat(*self));
            };
                if level < access_level {
                    return Err(AuthError::InsufficientPermissions((level, access_level)));
                }
                Ok(())
            }
        }
    }
}
