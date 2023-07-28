use petompp_web_api::{
    auth::token::create_token,
    data_sources::data_source::DataSourceManager,
    models::user::{Role, User},
    Secrets,
};
use rocket::{http::Header, local::blocking::Client};
use strum::IntoEnumIterator;

use crate::test_db_pool::{TestConfig, TestDataSource, TestPool};

#[test]
fn activate_test() {
    let config = TestConfig {
        should_fail: false,
        users: vec![User {
            id: 1,
            ..Default::default()
        }],
    };
    let manager: DataSourceManager<TestDataSource, _, _> = DataSourceManager::new(config).unwrap();
    let db_pool = TestPool::builder(manager).build().unwrap();
    let rocket = petompp_web_api::build_rocket(db_pool);
    let client = Client::untracked(rocket).unwrap();
    for role in Role::iter() {
        let user = User {
            role,
            ..Default::default()
        };
        let mut req = client.post("/api/v1/users/1/activate");
        req.add_header(Header::new(
            "Authorization",
            format!(
                "Bearer {}",
                create_token(&Secrets::default(), &user).unwrap()
            ),
        ));
        let expected = match role {
            Role::Admin => rocket::http::Status::Ok,
            _ => rocket::http::Status::Unauthorized,
        };

        let response = req.dispatch();

        assert_eq!(response.status(), expected);
    }
}

#[test]
fn activate_test_no_auth() {
    let config = TestConfig {
        should_fail: false,
        users: vec![User {
            id: 1,
            ..Default::default()
        }],
    };
    let manager: DataSourceManager<TestDataSource, _, _> = DataSourceManager::new(config).unwrap();
    let db_pool = TestPool::builder(manager).build().unwrap();
    let rocket = petompp_web_api::build_rocket(db_pool);
    let client = Client::untracked(rocket).unwrap();
    let req = client.post("/api/v1/users/1/activate");

    let response = req.dispatch();

    assert_eq!(response.status(), rocket::http::Status::Unauthorized);
}
