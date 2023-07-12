use std::collections::BTreeMap;

use actix_web::web::Data;
use chrono::Duration;
use enum_iterator::Sequence;

use crate::endpoints::user::User;

use super::{validation::{AuthConfig, AccessLevel}, error::AuthError};


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
            Claim::Subject => user.name.clone(),
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
        config: &Data<AuthConfig>,
    ) -> Result<(), AuthError> {
        let Some(claim) = claims.get(&self.key().to_string()) else {
        return Err(AuthError::MissingClaim(*self));
    };
        self.check(claim, config)
    }

    fn check(&self, value: &str, config: &Data<AuthConfig>) -> Result<(), AuthError> {
        match self {
            Claim::Subject => Ok(()),
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
                if level < config.level {
                    return Err(AuthError::InsufficientPermissions((level, config.level)));
                }
                Ok(())
            }
        }
    }
}