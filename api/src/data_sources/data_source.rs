use crate::{
    extensions::extension::Extension,
    models::{credentials::Credentials, user::User},
};
use async_trait::async_trait;
use deadpool::managed::{self, Manager};
use std::fmt::Display;

use super::{memory::MemoryPool, azure::AzurePool};

#[async_trait]
pub trait DataSource<D: ConfigData>: DataContext + Sized {
    async fn from_config<C: ManagerConfig<D>>(config: &C) -> Result<Self, DataSourceError>;
}

#[async_trait]
pub trait DataContext: UserContext + Send + Sync {
    async fn get_version(&mut self) -> Result<String, DataSourceError>;
}

pub enum DbConn {
    Memory(MemoryPool),
    Azure(AzurePool)
}

impl DbConn {
    async fn get(&mut self) -> impl DataContext {
        match self {
            DbConn::Memory(memory) => {
                memory.get().await.unwrap()
            },
            DbConn::Azure(_) => todo!(),
        }
    }
}

pub trait ConfigData: Clone + Send + Sync {}

pub trait ManagerConfig<D: ConfigData>: Send + Sync {
    fn data(&self) -> D;
}

pub type Pool<T, C, D> = deadpool::managed::Pool<DataSourceManager<T, C, D>>;

pub enum DataContextProvider {
    Memory,
    Azure,
}

impl<T, C, D> Extension<Pool<T, C, D>>
where
    D: ConfigData,
    T: DataSource<D>,
    C: ManagerConfig<D>,
{
}

#[async_trait]
impl<T, C, D> DataContext for Extension<Pool<T, C, D>>
where
    D: ConfigData,
    T: DataSource<D>,
    C: ManagerConfig<D>,
{
    async fn get_version(&mut self) -> Result<String, DataSourceError> {
        let mut client = self.0.get().await?;
        client.get_version().await
    }
}

#[async_trait]
impl<T, C, D> UserContext for Extension<Pool<T, C, D>>
where
    D: ConfigData,
    T: DataSource<D>,
    C: ManagerConfig<D>,
{
    async fn get_user_by_id(&mut self, id: u32) -> Result<User, DataSourceError> {
        let mut client = self.0.get().await?;
        client.get_user_by_id(id).await
    }
    async fn get_user_by_name(&mut self, name: String) -> Result<User, DataSourceError> {
        let mut client = self.0.get().await?;
        client.get_user_by_name(name).await
    }
    async fn get_users(&mut self) -> Result<Vec<User>, DataSourceError> {
        let mut client = self.0.get().await?;
        client.get_users().await
    }
    async fn update_user(&mut self, id: u32, user: &User) -> Result<User, DataSourceError> {
        let mut client = self.0.get().await?;
        client.update_user(id, user).await
    }
    async fn create_user(&mut self, credentials: &Credentials) -> Result<User, DataSourceError> {
        let mut client = self.0.get().await?;
        client.create_user(credentials).await
    }
}

pub struct DataSourceManager<T, C, D>
where
    D: ConfigData,
    T: DataSource<D>,
    C: ManagerConfig<D>,
{
    _source: std::marker::PhantomData<T>,
    config: C,
    _data: std::marker::PhantomData<D>,
}

impl<D: ConfigData, T: DataSource<D>, C: ManagerConfig<D>> DataSourceManager<T, C, D> {
    pub fn new(config: C) -> Result<Self, DataSourceError> {
        Ok(Self {
            _source: std::marker::PhantomData,
            config,
            _data: std::marker::PhantomData,
        })
    }

    pub async fn get_data_source(&self) -> Result<T, DataSourceError> {
        Ok(T::from_config(&self.config).await?)
    }
}

#[async_trait]
impl<D: ConfigData, T: DataSource<D>, C: ManagerConfig<D>> Manager for DataSourceManager<T, C, D> {
    type Type = T;
    type Error = DataSourceError;

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        self.get_data_source().await
    }

    async fn recycle(&self, item: &mut Self::Type) -> managed::RecycleResult<Self::Error> {
        let _test_query = item.get_version().await?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum DataSourceError {
    Tiberius(tiberius::error::Error),
    Io(std::io::Error),
    VarError(std::env::VarError),
    PoolError(String),
    NotFound,
}

impl std::error::Error for DataSourceError {}

impl Display for DataSourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            DataSourceError::Tiberius(t) => t.fmt(f),
            DataSourceError::Io(s) => s.fmt(f),
            DataSourceError::VarError(v) => v.fmt(f),
            DataSourceError::PoolError(e) => f.write_str(&e),
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

impl From<std::env::VarError> for DataSourceError {
    fn from(value: std::env::VarError) -> Self {
        Self::VarError(value)
    }
}

impl<E: Display> From<deadpool::managed::PoolError<E>> for DataSourceError {
    fn from(value: deadpool::managed::PoolError<E>) -> Self {
        Self::PoolError(value.to_string())
    }
}

#[async_trait]
pub trait UserContext {
    async fn get_user_by_id(&mut self, id: u32) -> Result<User, DataSourceError>;
    async fn get_user_by_name(&mut self, name: String) -> Result<User, DataSourceError>;
    async fn get_users(&mut self) -> Result<Vec<User>, DataSourceError>;
    async fn update_user(&mut self, id: u32, user: &User) -> Result<User, DataSourceError>;
    async fn create_user(&mut self, credentials: &Credentials) -> Result<User, DataSourceError>;
}
