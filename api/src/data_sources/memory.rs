use super::data_source::{
    ConfigData, DataSource, DataSourceError, DataSourceManager, ManagerConfig, UserContext, DataContext,
};
use crate::models::{credentials::Credentials, user::User};
use async_trait::async_trait;

pub type MemoryPool =
    deadpool::managed::Pool<DataSourceManager<MemoryDataSource, MemoryConfig, MemoryConfig>>;

#[derive(Clone, Default)]
pub struct MemoryConfig {
    pub should_fail: bool,
    pub users: Vec<User>,
}

impl ManagerConfig<MemoryConfig> for MemoryConfig {
    fn data(&self) -> MemoryConfig {
        self.clone()
    }
}

impl ConfigData for MemoryConfig {}

pub struct MemoryDataSource {
    pub config: MemoryConfig,
}

#[async_trait]
impl DataSource<MemoryConfig> for MemoryDataSource {
    async fn from_config<C: ManagerConfig<MemoryConfig>>(
        config: &C,
    ) -> Result<Self, DataSourceError> {
        Ok(Self {
            config: config.data().clone(),
        })
    }
}

#[async_trait]
impl DataContext for MemoryDataSource {
    async fn get_version(&mut self) -> Result<String, DataSourceError> {
        Ok("Memory".to_string())
    }
}

#[async_trait]
impl UserContext for MemoryDataSource {
    async fn get_user_by_id(&mut self, _id: u32) -> Result<User, DataSourceError> {
        let user = User {
            id: 1,
            name: "Memory User".to_string(),
            ..Default::default()
        };
        Ok(user)
    }
    async fn get_user_by_name(&mut self, _name: String) -> Result<User, DataSourceError> {
        let user = User {
            id: 1,
            name: "Memory User".to_string(),
            ..Default::default()
        };
        Ok(user)
    }
    async fn get_users(&mut self) -> Result<Vec<User>, DataSourceError> {
        let user = User {
            id: 1,
            name: "Memory User".to_string(),
            ..Default::default()
        };
        Ok(vec![user])
    }
    async fn update_user(&mut self, id: u32, user: &User) -> Result<User, DataSourceError> {
        let user = User {
            id,
            name: user.name.clone(),
            ..Default::default()
        };
        Ok(user)
    }
    async fn create_user(&mut self, credentials: &Credentials) -> Result<User, DataSourceError> {
        let user = User {
            id: 1,
            name: credentials.name.clone(),
            ..Default::default()
        };
        Ok(user)
    }
}
