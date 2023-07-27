use petompp_web_api::{
    build_rocket,
    data_sources::{
        azure::{AzureConfig, AzureDataSource, AzurePool},
        data_source::DataSourceManager,
    },
};

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    let manager: DataSourceManager<AzureDataSource, _, _> =
        DataSourceManager::new(AzureConfig::default()).unwrap();
    let db_pool = AzurePool::builder(manager).build().unwrap();
    build_rocket(db_pool)
}
