use actix_web::{dev::ServiceResponse, http::header::AUTHORIZATION, test, web::Data, App};
use enum_iterator::all;
use petompp_web_api::{
    app::{get_api_service, Secrets},
    auth::validation::{create_token, AccessLevel},
    endpoints::user::{Password, User, UserDto, UserEndpoint},
};
use std::{str::from_utf8, sync::Mutex};

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

    test::call_service(&app, req).await
}

#[test]
async fn test_get_user() {
    let secrets = Data::new(Secrets::default());
    for acl in all::<AccessLevel>() {
        let expected_status = match acl {
            AccessLevel::Admin => 200,
            _ => 401,
        };
        let resp = send_get_req(acl, &secrets).await;
        assert_eq!(resp.status(), expected_status);
    }
}

async fn send_get_req(acl: AccessLevel, secrets: &Data<Secrets>) -> ServiceResponse {
    let user = User {
        id: 1,
        name: "registered".to_string(),
        password: Password::new("password".to_string()),
        access_level: acl,
        confirmed: true,
    };
    let user_endpoint = UserEndpoint {
        users: Data::new(Mutex::new(vec![user.clone()])),
    };
    let app = test::init_service(App::new().service(get_api_service(&vec![user_endpoint]))).await;
    let req = test::TestRequest::get()
        .uri("/api/v1/users/1")
        .insert_header((
            AUTHORIZATION,
            format!("Bearer {}", create_token(&secrets, &user).unwrap()),
        ))
        .to_request();
    test::call_service(&app, req).await
}

#[test]
async fn test_get_users() {
    let secrets = Data::new(Secrets::default());
    for acl in all::<AccessLevel>() {
        let expected_status = match acl {
            AccessLevel::Admin => 200,
            _ => 401,
        };
        let resp = send_users_req(acl, &secrets).await;
        assert_eq!(resp.status(), expected_status);
    }
}

async fn send_users_req(acl: AccessLevel, secrets: &Data<Secrets>) -> ServiceResponse {
    let user = User {
        id: 1,
        name: "registered".to_string(),
        password: Password::new("password".to_string()),
        access_level: acl,
        confirmed: true,
    };
    let user_endpoint = UserEndpoint {
        users: Data::new(Mutex::new(vec![user.clone()])),
    };
    let app = test::init_service(App::new().service(get_api_service(&vec![user_endpoint]))).await;
    let req = test::TestRequest::get()
        .uri("/api/v1/users/all")
        .insert_header((
            AUTHORIZATION,
            format!("Bearer {}", create_token(&secrets, &user).unwrap()),
        ))
        .to_request();
    test::call_service(&app, req).await
}

#[test]
async fn test_vip() {
    let secrets = Data::new(Secrets::default());
    for acl in all::<AccessLevel>() {
        let expected_status = match acl {
            AccessLevel::Vip => 200,
            AccessLevel::Admin => 200,
            _ => 401,
        };
        let resp = send_vip_req(acl, &secrets).await;
        assert_eq!(resp.status(), expected_status);
    }
}

async fn send_vip_req(acl: AccessLevel, secrets: &Data<Secrets>) -> ServiceResponse {
    let user = User {
        id: 1,
        name: "registered".to_string(),
        password: Password::new("password".to_string()),
        access_level: acl,
        confirmed: true,
    };
    let user_endpoint = UserEndpoint {
        users: Data::new(Mutex::new(vec![user.clone()])),
    };
    let app = test::init_service(App::new().service(get_api_service(&vec![user_endpoint]))).await;
    let req = test::TestRequest::get()
        .uri("/api/v1/users/vip")
        .insert_header((
            AUTHORIZATION,
            format!("Bearer {}", create_token(&secrets, &user).unwrap()),
        ))
        .to_request();
    test::call_service(&app, req).await
}

#[test]
async fn test_register() {
    let resp = send_register_req().await;
    assert_eq!(resp.status(), 200);
}

async fn send_register_req() -> ServiceResponse {
    let user_endpoint = UserEndpoint {
        users: Data::new(Mutex::new(vec![])),
    };
    let app = test::init_service(App::new().service(get_api_service(&vec![user_endpoint]))).await;
    let req = test::TestRequest::post()
        .uri("/api/v1/users")
        .set_json(UserDto {
            name: "registered".to_string(),
            password: "password".to_string(),
        })
        .to_request();
    test::call_service(&app, req).await
}

#[test]
async fn test_login() {
    let resp = send_login_req().await;
    assert_eq!(resp.status(), 200);
    let body = test::read_body(resp).await;
    let body = from_utf8(&*body).unwrap();
    assert_eq!(body.chars().into_iter().filter(|c| *c == '.').count(), 2);
}

async fn send_login_req() -> ServiceResponse {
    let user_endpoint = UserEndpoint {
        users: Data::new(Mutex::new(vec![User {
            id: 1,
            name: "registered".to_string(),
            password: Password::new("password".to_string()),
            access_level: AccessLevel::Registered,
            confirmed: true,
        }])),
    };
    let app = test::init_service(App::new().service(get_api_service(&vec![user_endpoint]))).await;
    let req = test::TestRequest::post()
        .uri("/api/v1/users/login")
        .set_json(UserDto {
            name: "registered".to_string(),
            password: "password".to_string(),
        })
        .to_request();
    test::call_service(&app, req).await
}

#[test]
async fn test_get_self() {
    let secrets = Data::new(Secrets::default());
    let resp = send_get_self_req(&secrets).await;
    assert_eq!(resp.status(), 200);
}

async fn send_get_self_req(secrets: &Data<Secrets>) -> ServiceResponse {
    let user = User {
        id: 1,
        name: "registered".to_string(),
        password: Password::new("password".to_string()),
        access_level: AccessLevel::Registered,
        confirmed: true,
    };
    
    let user_endpoint = UserEndpoint {
        users: Data::new(Mutex::new(vec![user.clone()])),
    };
    let app = test::init_service(App::new().service(get_api_service(&vec![user_endpoint]))).await;
    let req = test::TestRequest::get()
        .uri("/api/v1/users")
        .insert_header((
            AUTHORIZATION,
            format!("Bearer {}", create_token(&secrets, &user).unwrap()),
        ))
        .to_request();
    test::call_service(&app, req).await
}

