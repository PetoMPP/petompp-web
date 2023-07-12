use std::{fmt::Display, collections::BTreeMap};

use actix_web::{web::Data, dev::ServiceRequest, Error};
use actix_web_httpauth::extractors::{bearer::{BearerAuth, self}, AuthenticationError};
use enum_iterator::{Sequence, all};
use hmac::{Hmac, digest::KeyInit};
use jwt::{VerifyWithKey, SignWithKey};
use serde_derive::{Serialize, Deserialize};
use sha2::Sha256;

use crate::{endpoints::user::User, app::Secrets};

use super::{error::AuthError, claim::Claim};


#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, PartialOrd, Sequence)]
pub enum AccessLevel {
    #[default]
    Registered,
    Admin,
}

impl Display for AccessLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

#[derive(Clone, Copy, Default)]
pub struct AuthConfig {
    pub level: AccessLevel,
}

impl AuthConfig {
    pub fn new(level: AccessLevel) -> Self {
        Self { level }
    }
}

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let secrets = req.app_data::<Data<Secrets>>().unwrap();
    let config = req.app_data::<Data<AuthConfig>>().unwrap();
    let Err(err) = validate_token(secrets, config, credentials.token()) else {
    return Ok(req);
};

    let config = req
        .app_data::<bearer::Config>()
        .cloned()
        .unwrap_or_default();

    Err((
        AuthenticationError::from(config)
            .with_error_description(err.to_string())
            .into(),
        req,
    ))
}

pub fn create_token(secrets: &Data<Secrets>, user: &User) -> Result<String, AuthError> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(secrets.api_secret.as_bytes()).unwrap();
    let mut claims: BTreeMap<&str, String> = BTreeMap::new();
    for claim in all::<Claim>() {
        claims.insert(claim.key(), claim.value(user));
    }

    Ok(claims.sign_with_key(&key)?)
}

pub fn validate_token(
    secrets: &Data<Secrets>,
    config: &Data<AuthConfig>,
    token: &str,
) -> Result<(), AuthError> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(secrets.api_secret.as_bytes()).unwrap();
    let token_data: BTreeMap<String, String> = token.verify_with_key(&key)?;

    for claim in all::<Claim>() {
        claim.validate(&token_data, config)?;
    }
    Ok(())
}
