use crate::controllers::controller::ControllerRegisterer;
use crate::controllers::users::UsersController;
use crate::extensions::extension::Extension;
use data_sources::data_source::{DataSource, ManagerConfig, Pool, ConfigData};
use rocket::{Build, Config, Rocket};
use std::env;

pub mod auth;
pub mod controllers;
pub mod data_sources;
pub mod extensions;
pub mod models;

pub struct Secrets {
    pub api_secret: String,
}

impl Default for Secrets {
    fn default() -> Self {
        let api_secret = env::var("API_SECRET").unwrap_or("shhhdonttellanyoneaboutit".to_string());
        Self { api_secret }
    }
}

pub fn build_rocket<D: ConfigData + 'static, T: DataSource<D> + 'static, C: ManagerConfig<D> + 'static>(
    db_pool: Pool<T, C, D>,
) -> Rocket<Build> {
    Extension(rocket::build())
        .add(UsersController)
        .into()
        .manage(Secrets::default())
        .manage(db_pool)
        .configure(Config {
            port: 16969,
            address: "0.0.0.0".parse().unwrap(),
            ..Default::default()
        })
}
