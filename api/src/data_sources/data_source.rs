use crate::models::{credentials::Credentials, user::User};
use async_trait::async_trait;
use deadpool::managed::{self, Manager};
use std::fmt::Display;

#[async_trait]
pub trait DataSource<D: ConfigData>: UserDataSource + Send + Sync + Sized {
    async fn from_config<C: ManagerConfig<D>>(config: &C) -> Result<Self, DataSourceError>;
    async fn get_version(&mut self) -> Result<String, DataSourceError>;
}

pub trait ConfigData: Clone + Send + Sync {}

pub trait ManagerConfig<D: ConfigData>: Send + Sync {
    fn data(&self) -> D;
}

pub type Pool<T, C, D> = deadpool::managed::Pool<DataSourceManager<T, C, D>>;

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
    NotFound,
}

impl std::error::Error for DataSourceError {}

impl Display for DataSourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            DataSourceError::Tiberius(t) => t.fmt(f),
            DataSourceError::Io(s) => s.fmt(f),
            DataSourceError::VarError(v) => v.fmt(f),
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

#[async_trait]
pub trait UserDataSource {
    async fn get_user_by_id(&self, id: u32) -> Result<User, DataSourceError>;
    async fn get_user_by_name(&self, name: String) -> Result<User, DataSourceError>;
    async fn get_users(&self) -> Result<Vec<User>, DataSourceError>;
    async fn update_user(&self, id: u32, user: &User) -> Result<User, DataSourceError>;
    async fn create_user(&self, credentials: &Credentials) -> Result<User, DataSourceError>;
}
