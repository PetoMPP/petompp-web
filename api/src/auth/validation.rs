use super::{claim::Claim, error::AuthError};
use crate::{
    app::Secrets,
    endpoints::user::{CurrentUserId, User},
};
use actix_web::{
    dev::{Extensions, ServiceRequest},
    web::Data,
    Error,
};
use actix_web_httpauth::extractors::{
    bearer::{self, BearerAuth},
    AuthenticationError,
};
use enum_iterator::{all, Sequence};
use hmac::{digest::KeyInit, Hmac};
use jwt::{SignWithKey, VerifyWithKey};
use serde_derive::{Deserialize, Serialize};
use sha2::Sha256;
use std::{collections::BTreeMap, fmt::Display};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, PartialOrd, Sequence)]
pub enum AccessLevel {
    #[default]
    Registered,
    Vip,
    Admin,
}

impl Display for AccessLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

#[macro_export]
macro_rules! access_validator {
    ($access_level:expr) => {{
        use crate::auth::validation::validate;
        HttpAuthentication::bearer(|req, credentials| async {
            validate(req, credentials, $access_level).await
        })
    }};
}

pub async fn validate(
    mut req: ServiceRequest,
    credentials: BearerAuth,
    access_level: AccessLevel,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let secrets = req.app_data::<Data<Secrets>>().unwrap();
    match validate_token(secrets, access_level, credentials.token()) {
        Err(err) => {
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
        Ok(id) => {
            let mut ext = Extensions::new();
            ext.insert(CurrentUserId(id));
            req.add_data_container(ext.into());
            return Ok(req);
        }
    }
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
    access_level: AccessLevel,
    token: &str,
) -> Result<u32, AuthError> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(secrets.api_secret.as_bytes()).unwrap();
    let token_data: BTreeMap<String, String> = token.verify_with_key(&key)?;

    for claim in all::<Claim>() {
        claim.validate(&token_data, access_level)?;
    }
    Ok(token_data[Claim::Subject.key()].parse::<u32>().unwrap())
}
