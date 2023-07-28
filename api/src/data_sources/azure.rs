use super::data_source::{
    ConfigData, DataSource, DataSourceError, DataSourceManager, ManagerConfig, UserContext, DataContext,
};
use crate::{
    extensions::extension::ExtensionCl,
    models::{credentials::Credentials, user::User},
};
use async_std::net::TcpStream;
use async_trait::async_trait;
use tiberius::{Client, Config, QueryStream};

pub type AzurePool =
    deadpool::managed::Pool<DataSourceManager<AzureDataSource, AzureConfig, ExtensionCl<Config>>>;

#[derive(Clone)]
pub struct AzureConfig {
    pub config: ExtensionCl<Config>,
}

impl Default for AzureConfig {
    fn default() -> Self {
        let server = std::env::var("DB_HOST").unwrap();
        let server = match server.contains(":") {
            true => {
                let tcp: Vec<&str> = server.split(":").take(2).collect();
                format!("tcp:{}, {}", tcp[0], tcp[1])
            }
            false => server,
        };
        let database = option_env!("DB_NAME").unwrap();
        let user: Vec<&str> = option_env!("DB_USER").unwrap().split(":").take(2).collect();
        let (user, password) = (user[0], user[1]);
        let connection_string = format!(
                "Server={server};Initial Catalog={database};Persist Security Info=False;User ID={user};Password={password};MultipleActiveResultSets=False;Encrypt=True;TrustServerCertificate=False;Connection Timeout=30;",
                server = server,
                database = database,
                user = user,
                password = password,
            );

        Self {
            config: Config::from_ado_string(&connection_string).unwrap().into(),
        }
    }
}

impl ManagerConfig<ExtensionCl<Config>> for AzureConfig {
    fn data(&self) -> ExtensionCl<Config> {
        self.config.clone()
    }
}

impl ConfigData for ExtensionCl<Config> {}

pub struct AzureDataSource {
    client: Client<TcpStream>,
}

impl From<Client<TcpStream>> for AzureDataSource {
    fn from(value: Client<TcpStream>) -> Self {
        Self { client: value }
    }
}

#[async_trait]
impl DataSource<ExtensionCl<Config>> for AzureDataSource {
    async fn from_config<C: ManagerConfig<ExtensionCl<Config>>>(
        config: &C,
    ) -> Result<Self, DataSourceError> {
        let config = config.data().clone().into();
        let tcp_stream = TcpStream::connect(config.get_addr()).await?;
        let client = Client::connect(config.clone(), tcp_stream).await?;

        Ok(Self { client })
    }
}

#[async_trait]
impl DataContext for AzureDataSource {
    async fn get_version(&mut self) -> Result<String, DataSourceError> {
        let res: QueryStream = self.client.simple_query("SELECT @@version").await?;
    
        res.into_row()
            .await?
            .ok_or(DataSourceError::NotFound)?
            .get(0)
            .map(|sql: &str| sql.to_string())
            .ok_or(DataSourceError::NotFound)
    }
}

#[async_trait]
impl UserContext for AzureDataSource {
    async fn get_user_by_id(&mut self, _id: u32) -> Result<User, DataSourceError> {
        let user = User {
            id: 1,
            name: "Azure User".to_string(),
            ..Default::default()
        };
        Ok(user)
    }
    async fn get_user_by_name(&mut self, _name: String) -> Result<User, DataSourceError> {
        let user = User {
            id: 1,
            name: "Azure User".to_string(),
            ..Default::default()
        };
        Ok(user)
    }
    async fn get_users(&mut self) -> Result<Vec<User>, DataSourceError> {
        let user = User {
            id: 1,
            name: "Azure User".to_string(),
            ..Default::default()
        };
        Ok(vec![user])
    }
    async fn create_user(&mut self, _credentials: &Credentials) -> Result<User, DataSourceError> {
        let user = User {
            id: 1,
            name: "Azure User".to_string(),
            ..Default::default()
        };
        Ok(user)
    }
    async fn update_user(&mut self, _id: u32, _user: &User) -> Result<User, DataSourceError> {
        let user = User {
            id: 1,
            name: "Azure User".to_string(),
            ..Default::default()
        };
        Ok(user)
    }
}
