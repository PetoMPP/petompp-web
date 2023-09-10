use crate::data::locales::{LocalesStore, TK};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, rc::Rc};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Error {
    AuthError(AuthError),
    DatabaseError(String),
    DatabaseConnectionError(String),
    UserNameTaken(String),
    UserNotFound(String),
    InvalidCredentials,
    UserNotConfirmed(String),
    ValidationError(ValidationError),
    Status(u16, String),
}

impl Error {
    pub fn into_localized(self, locales: Rc<LocalesStore>) -> String {
        match self {
            Error::AuthError(e) => match e {
                AuthError::MissingClaim(c) => locales.get(TK::E_Auth_MissingClaim(c)),
                AuthError::InvalidFormat(c) => locales.get(TK::E_Auth_InvalidFormat(c)),
                AuthError::TokenExpiredS(s) => locales.get(TK::E_Auth_TokenExpiredS(s as i32)),
                AuthError::JwtError(s) => locales.get(TK::E_Auth_JwtError(s)),
            },
            Error::DatabaseError(db) => locales.get(TK::E_Database(db)),
            Error::DatabaseConnectionError(dbc) => locales.get(TK::E_DatabaseConnection(dbc)),
            Error::UserNameTaken(u) => locales.get(TK::E_UserNameTaken(u)),
            Error::UserNotFound(u) => locales.get(TK::E_UserNotFound(u)),
            Error::InvalidCredentials => locales.get(TK::E_InvalidCredentials),
            Error::UserNotConfirmed(u) => locales.get(TK::E_UserNotConfirmed(u)),
            Error::ValidationError(ve) => match ve {
                ValidationError::Username(ue) => match ue {
                    UsernameValidationError::InvalidLength(min, max) => {
                        locales.get(TK::E_Validation_Username_InvalidLength(min, max))
                    }
                    UsernameValidationError::InvalidCharacters(chars) => {
                        locales.get(TK::E_Validation_Username_InvalidCharacters(
                            chars
                                .iter()
                                .map(|c| c.to_string())
                                .collect::<Vec<_>>()
                                .join(", "),
                        ))
                    }
                },
                ValidationError::Password(pr) => locales.get(TK::E_Validation_Password(pr)),
                ValidationError::Query(qe) => match qe {
                    QueryValidationError::InvalidColumn(c) => {
                        locales.get(TK::E_Validation_Query_InvalidColumn(c))
                    }
                },
                ValidationError::ResourceData(re) => match re {
                    ResourceDataValidationError::KeyMismatch(k1, k2) => {
                        locales.get(TK::E_Validation_ResourceData_KeyMismatch(k1, k2))
                    }
                    ResourceDataValidationError::KeyMissing => {
                        locales.get(TK::E_Validation_ResourceData_KeyMissing)
                    }
                    ResourceDataValidationError::ValueMissing => {
                        locales.get(TK::E_Validation_ResourceData_ValueMissing)
                    }
                },
            },
            Error::Status(_, m) => m,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ValidationError {
    Username(UsernameValidationError),
    Password(PasswordRequirements),
    Query(QueryValidationError),
    ResourceData(ResourceDataValidationError),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum UsernameValidationError {
    InvalidLength(i32, i32),
    InvalidCharacters(Vec<char>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum QueryValidationError {
    InvalidColumn(String),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ResourceDataValidationError {
    KeyMismatch(String, String),
    KeyMissing,
    ValueMissing,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl From<AuthError> for Error {
    fn from(value: AuthError) -> Self {
        Error::AuthError(value)
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum AuthError {
    MissingClaim(String),
    InvalidFormat(String),
    TokenExpiredS(i64),
    JwtError(String),
}

impl Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl std::error::Error for AuthError {}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct PasswordRequirements {
    pub min_length: i32,
    pub passes_required: i32,
    pub numbers: bool,
    pub uppercase: bool,
    pub lowercase: bool,
    pub special: bool,
}

impl PasswordRequirements {
    pub fn into_localized(self, locales: &LocalesStore) -> String {
        let mut requirements = vec![];
        if self.numbers {
            requirements.push(locales.get(TK::E_Validation_PasswordRequirement_ContainsNumber));
        }
        if self.uppercase {
            requirements.push(locales.get(TK::E_Validation_PasswordRequirement_ContainsUppercase));
        }
        if self.lowercase {
            requirements.push(locales.get(TK::E_Validation_PasswordRequirement_ContainsLowercase));
        }
        if self.special {
            requirements
                .push(locales.get(TK::E_Validation_PasswordRequirement_ContainsSpecialCharacter));
        }
        locales.get(TK::E_Validation_PasswordRequirement(
            self.min_length,
            self.passes_required,
            requirements.join(", "),
        ))
    }
}

impl Default for PasswordRequirements {
    fn default() -> Self {
        Self {
            min_length: 8,
            passes_required: Default::default(),
            numbers: Default::default(),
            uppercase: Default::default(),
            lowercase: Default::default(),
            special: Default::default(),
        }
    }
}
