use super::data_source::{DataSource, DataSourceError, UserDataSource};
use crate::models::{credentials::Credentials, user::User};
use async_std::net::TcpStream;
use async_trait::async_trait;
use deadpool::managed::{self, Manager};
use tiberius::{Client, Config, QueryStream};

pub struct AzureDataSource {
    client: Client<TcpStream>,
}

impl From<Client<TcpStream>> for AzureDataSource {
    fn from(value: Client<TcpStream>) -> Self {
        Self { client: value }
    }
}

#[async_trait]
impl DataSource for AzureDataSource {
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
impl UserDataSource for AzureDataSource {
    async fn get_user_by_id(&self, id: u32) -> Result<User, DataSourceError> {
        let user = User {
            id: 1,
            name: "Azure User".to_string(),
            ..Default::default()
        };
        Ok(user)
    }
    async fn get_user_by_name(&self, name: String) -> Result<User, DataSourceError> {
        let user = User {
            id: 1,
            name: "Azure User".to_string(),
            ..Default::default()
        };
        Ok(user)
    }
    async fn get_users(&self) -> Result<Vec<User>, DataSourceError> {
        let user = User {
            id: 1,
            name: "Azure User".to_string(),
            ..Default::default()
        };
        Ok(vec![user])
    }
    async fn create_user(&self, credentials: &Credentials) -> Result<User, DataSourceError> {
        let user = User {
            id: 1,
            name: "Azure User".to_string(),
            ..Default::default()
        };
        Ok(user)
    }
}

pub type AzurePool = deadpool::managed::Pool<AzureDataSourceManager>;

pub struct AzureDataSourceManager {
    config: Config,
}

impl AzureDataSourceManager {
    pub fn new() -> Result<Self, DataSourceError> {
        let host: Vec<&str> = option_env!("DB_HOST").unwrap().split(":").take(2).collect();
        let (host, port) = (host[0], host[1]);
        let database = option_env!("DB_NAME").unwrap();
        let user: Vec<&str> = option_env!("DB_USER").unwrap().split(":").take(2).collect();
        let (user, password) = (user[0], user[1]);
        let config: Config = Config::from_ado_string(
            format!(
                "Server=tcp:{host},{port};Initial Catalog={database};Persist Security Info=False;User ID={user};Password={password};MultipleActiveResultSets=False;Encrypt=True;TrustServerCertificate=False;Connection Timeout=30;",
                host = host,
                port = port,
                database = database,
                user = user,
                password = password,
            )
            .as_str(),
        )?
        .into();

        Ok(Self { config })
    }

    pub async fn get_data_source(&self) -> Result<AzureDataSource, DataSourceError> {
        let tcp_stream = TcpStream::connect(self.config.get_addr()).await?;

        let client = Client::connect(self.config.clone(), tcp_stream).await?;

        Ok(AzureDataSource { client })
    }
}

#[async_trait]
impl Manager for AzureDataSourceManager {
    type Type = AzureDataSource;
    type Error = DataSourceError;

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        self.get_data_source().await
    }

    async fn recycle(&self, item: &mut Self::Type) -> managed::RecycleResult<Self::Error> {
        let _test_query = item.get_version();
        Ok(())
    }
}
