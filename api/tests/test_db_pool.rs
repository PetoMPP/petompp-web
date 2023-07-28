use async_trait::async_trait;
use petompp_web_api::{
    data_sources::data_source::{
        ConfigData, DataContext, DataSource, DataSourceError, DataSourceManager, ManagerConfig,
        UserContext,
    },
    models::{credentials::Credentials, user::User},
};

pub type TestPool =
    deadpool::managed::Pool<DataSourceManager<TestDataSource, TestConfig, TestConfig>>;

#[derive(Clone, Default)]
pub struct TestConfig {
    pub should_fail: bool,
    pub users: Vec<User>,
}

impl ManagerConfig<TestConfig> for TestConfig {
    fn data(&self) -> TestConfig {
        self.clone()
    }
}

impl ConfigData for TestConfig {}

pub struct TestDataSource {
    pub config: TestConfig,
}

#[async_trait]
impl DataSource<TestConfig> for TestDataSource {
    async fn from_config<C: ManagerConfig<TestConfig>>(
        config: &C,
    ) -> Result<Self, DataSourceError> {
        Ok(Self {
            config: config.data().clone(),
        })
    }
}

#[async_trait]
impl DataContext for TestDataSource {
    async fn get_version(&mut self) -> Result<String, DataSourceError> {
        Ok("test".to_string())
    }
}

#[async_trait]
impl UserContext for TestDataSource {
    async fn get_user_by_id(&mut self, _id: u32) -> Result<User, DataSourceError> {
        let user = User {
            id: 1,
            name: "Test User".to_string(),
            ..Default::default()
        };
        Ok(user)
    }
    async fn get_user_by_name(&mut self, _name: String) -> Result<User, DataSourceError> {
        let user = User {
            id: 1,
            name: "Test User".to_string(),
            ..Default::default()
        };
        Ok(user)
    }
    async fn get_users(&mut self) -> Result<Vec<User>, DataSourceError> {
        let user = User {
            id: 1,
            name: "Test User".to_string(),
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
