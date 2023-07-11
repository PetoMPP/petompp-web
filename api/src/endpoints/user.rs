use super::endpoint::Endpoint;
use crate::app::Secrets;
use crate::auth::*;
use actix_web::{
    web::{self, Data},
    HttpRequest, HttpResponse, Responder,
};
use actix_web_httpauth::middleware::HttpAuthentication;
use serde_derive::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDto {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct User {
    pub id: u32,
    pub name: String,
    #[serde(skip_serializing)]
    pub password: Password,
    pub access_level: AccessLevel,
    pub confirmed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Password {
    pub hash: String,
    pub salt: String,
}

impl Password {
    pub fn new(password: String) -> Self {
        let mut rng = urandom::csprng();
        let salt: [u8; 16] = rng.next();
        let salt = salt.iter().map(|x| format!("{:x}", x)).collect::<String>();
        let salty_password = password + &salt;
        let mut hasher = Sha256::new();
        hasher.update(&salty_password);
        let result = hasher.finalize();
        let hash = format!("{:x}", result);
        Self { hash, salt }
    }

    pub fn verify(&self, password: String) -> bool {
        let salty_password = password + &self.salt;
        let mut hasher = Sha256::new();
        hasher.update(&salty_password);
        let result = hasher.finalize();
        let hash = format!("{:x}", result);
        self.hash == hash
    }
}

#[derive(Clone)]
pub struct UserEndpoint {
    users: Data<Mutex<Vec<User>>>,
    user_passwords: Data<Mutex<Vec<(u32, String)>>>,
}

impl Default for UserEndpoint {
    fn default() -> Self {
        Self {
            users: Data::new(Mutex::new(Vec::new())),
            user_passwords: Data::new(Default::default()),
        }
    }
}

impl Endpoint for UserEndpoint {
    fn register(&self, scope: actix_web::Scope) -> actix_web::Scope {
        scope.service(
            web::scope("/users")
                .app_data(self.users.clone())
                .app_data(self.user_passwords.clone())
                // Not protected by authentication
                .route("", web::post().to(register))
                .route("/login", web::post().to(login))
                // Protected by authentication
                .service(
                    web::scope("")
                        .app_data(Data::new(AuthConfig::new(AccessLevel::Registered)))
                        .wrap(HttpAuthentication::bearer(validator))
                        .route("", web::get().to(all))
                        // Protected by authentication (admin access only)
                        .service(
                            web::scope("")
                                .wrap(HttpAuthentication::bearer(validator))
                                .app_data(Data::new(AuthConfig::new(AccessLevel::Admin)))
                                .route("/{id}/confirm", web::post().to(confirm)),
                        ),
                ),
        )
    }
}

async fn all(req: HttpRequest) -> impl Responder {
    let users = req.app_data::<Data<Mutex<Vec<User>>>>().unwrap().get_ref();
    HttpResponse::Ok().body(serde_json::to_string(&users).unwrap())
}

async fn register(body: web::Json<UserDto>, req: HttpRequest) -> impl Responder {
    let mut users = req
        .app_data::<Data<Mutex<Vec<User>>>>()
        .unwrap()
        .lock()
        .unwrap();

    let name = body.name.to_lowercase();
    if users.iter().any(|u| u.name.to_lowercase() == name) {
        return HttpResponse::BadRequest().body(format!("User with name {} already exists", name));
    };

    let password = Password::new(body.password.clone());

    let new_user = User {
        id: users
            .iter()
            .max_by(|x, y| x.id.cmp(&y.id))
            .map(|u| u.id)
            .unwrap_or(0)
            + 1,
        name: body.name.clone(),
        password,
        ..Default::default()
    };
    users.push(new_user.clone());
    HttpResponse::Ok().body(serde_json::to_string(&new_user).unwrap())
}

async fn login(body: web::Json<UserDto>, req: HttpRequest) -> impl Responder {
    let users = req
        .app_data::<Data<Mutex<Vec<User>>>>()
        .unwrap()
        .lock()
        .unwrap();
    let secrets = req.app_data::<Data<Secrets>>().unwrap();

    if let Some(existing) = users.iter().find(|u| u.name == body.name) {
        if !existing.confirmed {
            return HttpResponse::Unauthorized().body("User not confirmed");
        }
        if existing.password.verify(body.password.clone()) {
            let Ok(create_token) = create_token(secrets, existing) else {
                return HttpResponse::InternalServerError().body("Failed to create token");
            };
            return HttpResponse::Ok().body(create_token);
        }
        return HttpResponse::Unauthorized().body("Invalid password");
    };

    HttpResponse::BadRequest().body("User does not exists")
}

async fn confirm(id: web::Path<u32>, req: HttpRequest) -> impl Responder {
    let mut users = req
        .app_data::<Data<Mutex<Vec<User>>>>()
        .unwrap()
        .lock()
        .unwrap();

    if let Some(existing) = users.iter_mut().find(|u| u.id == *id) {
        if existing.confirmed {
            return HttpResponse::BadRequest().body("User already confirmed");
        }
        existing.confirmed = true;
        return HttpResponse::Ok().body("User confirmed");
    }

    HttpResponse::BadRequest().body("User does not exists")
}
