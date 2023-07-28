use crate::{
    auth::{
        claims::{AdminClaims, Claims},
        token::create_token,
    },
    controllers::controller::Controller,
    data_sources::{azure::AzurePool, data_source::UserContext},
    models::{credentials::Credentials, user::User},
};
use rocket::{get, http::Status, post, response::status, routes, serde::json::Json, Build, State};

pub struct UsersController;

impl Controller for UsersController {
    fn path(&self) -> &'static str {
        "/users"
    }

    fn routes(&self) -> Vec<rocket::Route> {
        routes![create, login, get_self, activate]
    }

    fn add_managed(&self, rocket: rocket::Rocket<Build>) -> rocket::Rocket<Build> {
        rocket
    }
}

#[post("/", data = "<credentials>")]
async fn create(
    credentials: Json<Credentials>,
    pool: &State<AzurePool>,
) -> status::Custom<&'static str> {
    let mut client = pool.get().await.unwrap();
    return match client.get_user_by_name(credentials.name.clone()).await {
        Ok(_) => status::Custom(Status::BadRequest, "User already exists!"),
        Err(_) => match client.create_user(&credentials).await {
            Ok(_) => status::Custom(Status::Ok, "User created!"),
            Err(_) => status::Custom(Status::InternalServerError, "Error creating user!"),
        },
    };
}

#[post("/login", data = "<credentials>")]
async fn login(
    credentials: Json<Credentials>,
    pool: &State<AzurePool>,
    secrets: &State<crate::Secrets>,
) -> status::Custom<String> {
    let mut client = pool.get().await.unwrap();
    return match client.get_user_by_name(credentials.name.clone()).await {
        Ok(user) => {
            if !user.confirmed {
                return status::Custom(Status::BadRequest, "User not activated!".to_string());
            }
            if !user.password.verify(credentials.password.clone()) {
                return status::Custom(Status::BadRequest, "Wrong password!".to_string());
            }
            return match create_token(secrets, &user) {
                Ok(token) => status::Custom(Status::Ok, token),
                Err(e) => status::Custom(Status::InternalServerError, e.to_string()),
            };
        }
        Err(_) => status::Custom(Status::BadRequest, "User does not exist!".to_string()),
    };
}

#[get("/")]
async fn get_self(claims: Claims, pool: &State<AzurePool>) -> status::Custom<Option<Json<User>>> {
    let mut client = pool.get().await.unwrap();
    return match client.get_user_by_id(claims.sub).await {
        Ok(user) => status::Custom(Status::Ok, Some(Json(user.clone()))),
        Err(_) => status::Custom(Status::NotFound, None),
    };
}

#[post("/<id>/activate")]
async fn activate(
    _claims: AdminClaims,
    id: u32,
    pool: &State<AzurePool>,
) -> status::Custom<&'static str> {
    let mut client = pool.get().await.unwrap();
    return match client.get_user_by_id(id).await {
        Ok(user) => {
            let user = User {
                confirmed: true,
                ..user
            };
            match client.update_user(id, &user).await {
                Ok(_) => status::Custom(Status::Ok, "User activated!"),
                Err(_) => status::Custom(Status::InternalServerError, "Error activating user!"),
            }
        }
        Err(_) => status::Custom(Status::NotFound, "User not found!"),
    };
}
