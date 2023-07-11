pub mod endpoints;

pub mod app {
    use crate::endpoints::endpoint::Endpoint;
    use actix_web::{
        dev::HttpServiceFactory,
        middleware::Logger,
        web::{self, Data},
    };
    use std::env;
    pub struct Secrets {
        pub api_secret: String,
    }

    impl Default for Secrets {
        fn default() -> Self {
            let api_secret =
                env::var("API_SECRET").unwrap_or("shhhdonttellanyoneaboutit".to_string());
            Self { api_secret }
        }
    }

    pub fn get_api_service(endpoints: &Vec<impl Endpoint>) -> impl HttpServiceFactory + 'static {
        let secrets = Data::new(Secrets::default());
        // Define api scope
        let mut api_scope = web::scope("/api/v1").app_data(secrets);
        // Register endpoints
        for endpoint in endpoints {
            api_scope = endpoint.register(api_scope);
        }
        // Wrap scope with logger
        api_scope.wrap(Logger::default())
    }
}

pub mod auth {
    use std::{collections::BTreeMap, fmt::Display};

    use actix_web::{dev::ServiceRequest, web::Data, Error};
    use actix_web_httpauth::extractors::{
        bearer::{self, BearerAuth},
        AuthenticationError,
    };
    use chrono::Duration;
    use enum_iterator::{all, Sequence};
    use hmac::{Hmac, Mac};
    use jwt::{SignWithKey, VerifyWithKey};
    use serde_derive::{Deserialize, Serialize};
    use sha2::Sha256;

    use crate::{app::Secrets, endpoints::user::User};

    #[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, PartialOrd)]
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

    #[derive(Debug)]
    pub enum AuthError {
        MissingClaim(Claim),
        InvalidFormat(Claim),
        TokenExpired(Duration),
        InsufficientPermissions((AccessLevel, AccessLevel)),
        JwtError(jwt::Error),
    }

    impl From<jwt::Error> for AuthError {
        fn from(value: jwt::Error) -> Self {
            Self::JwtError(value)
        }
    }

    impl Display for AuthError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            return match self {
                AuthError::MissingClaim(c) => {
                    f.write_fmt(format_args!("Claim {} is missing", c.key()))
                }
                AuthError::InvalidFormat(c) => {
                    f.write_fmt(format_args!("Claim {} has invalid format", c.key()))
                }
                AuthError::TokenExpired(d) => f.write_fmt(format_args!(
                    "Token is expired by {} seconds",
                    d.num_seconds()
                )),
                AuthError::InsufficientPermissions((a, r)) => f.write_fmt(format_args!(
                    "User has insufficient permissions: {} < {}",
                    a, r
                )),
                AuthError::JwtError(jwt) => jwt.fmt(f),
            };
        }
    }

    impl std::error::Error for AuthError {}

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
}
