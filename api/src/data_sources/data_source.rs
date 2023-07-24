use crate::models::{credentials::Credentials, user::User};
use async_trait::async_trait;

#[async_trait]
pub trait DataSource: UserDataSource {
    async fn get_version(&mut self) -> Result<String, DataSourceError>;
}

#[derive(Debug)]
pub enum DataSourceError {
    NotConfigured,
    NotConnected,
    ConnectionError,
    NotFound,
}

#[async_trait]
pub trait UserDataSource {
    async fn get_user_by_id(&self, id: u32) -> Result<User, DataSourceError>;
    async fn get_user_by_name(&self, name: String) -> Result<User, DataSourceError>;
    async fn get_users(&self) -> Result<Vec<User>, DataSourceError>;
    async fn create_user(&self, credentials: &Credentials) -> Result<User, DataSourceError>;
}
