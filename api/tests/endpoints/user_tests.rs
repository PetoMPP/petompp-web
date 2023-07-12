use actix_web::{dev::ServiceResponse, http::header::AUTHORIZATION, test, web::Data, App};
use enum_iterator::all;
use petompp_web_api::{
    app::{get_api_service, Secrets},
    auth::validation::{create_token, AccessLevel},
    endpoints::user::{Password, User, UserEndpoint},
};
use std::sync::Mutex;

#[test]
async fn test_confirm_user() {
    let secrets = Data::new(Secrets::default());
    for acl in all::<AccessLevel>() {
        let expected_status = match acl {
            AccessLevel::Admin => 200,
            _ => 401,
        };
        let resp = send_confirm_req(acl, &secrets).await;
        assert_eq!(resp.status(), expected_status);
    }
}

async fn send_confirm_req(acl: AccessLevel, secrets: &Data<Secrets>) -> ServiceResponse {
    let user = User {
        id: 1,
        name: "registered".to_string(),
        password: Password::new("password".to_string()),
        access_level: acl,
        confirmed: true,
    };
    let user_endpoint = UserEndpoint {
        users: Data::new(Mutex::new(vec![
            user.clone(),
            User {
                id: 2,
                name: "notconfirmed".to_string(),
                password: Password::new("password".to_string()),
                access_level: AccessLevel::Registered,
                confirmed: false,
            },
        ])),
    };
    let endpoints = vec![user_endpoint];
    let app = test::init_service(App::new().service(get_api_service(&endpoints.clone()))).await;
    let req = test::TestRequest::post()
        .uri("/api/v1/users/2/confirm")
        .insert_header((
            AUTHORIZATION,
            format!("Bearer {}", create_token(&secrets, &user).unwrap()),
        ))
        .to_request();
    println!("{:?}", req);

    test::call_service(&app, req).await
}
