use std::fmt::Display;

use crate::models::{credentials::Credentials, user::User};
use async_trait::async_trait;

#[async_trait]
pub trait DataSource: UserDataSource {
    async fn get_version(&mut self) -> Result<String, DataSourceError>;
}

#[derive(Debug)]
pub enum DataSourceError {
    Tiberius(tiberius::error::Error),
    Io(std::io::Error),
    NotFound,
}

impl std::error::Error for DataSourceError {}

impl Display for DataSourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            DataSourceError::Tiberius(t) => t.fmt(f),
            DataSourceError::Io(s) => s.fmt(f),
            DataSourceError::NotFound => f.write_fmt(format_args!("{:?}", self)),
        };
    }
}

impl From<tiberius::error::Error> for DataSourceError {
    fn from(value: tiberius::error::Error) -> Self {
        Self::Tiberius(value)
    }
}

impl From<std::io::Error> for DataSourceError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

#[async_trait]
pub trait UserDataSource {
    async fn get_user_by_id(&self, id: u32) -> Result<User, DataSourceError>;
    async fn get_user_by_name(&self, name: String) -> Result<User, DataSourceError>;
    async fn get_users(&self) -> Result<Vec<User>, DataSourceError>;
    async fn create_user(&self, credentials: &Credentials) -> Result<User, DataSourceError>;
}
