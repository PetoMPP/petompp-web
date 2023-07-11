use std::{env, io};

use actix_web::{
    dev::HttpServiceFactory,
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use endpoints::{endpoint::Endpoint, user::UserEndpoint};
mod auth;
mod endpoints;

pub struct Secrets {
    pub api_secret: String,
}

impl Default for Secrets {
    fn default() -> Self {
        let api_secret = env::var("API_SECRET").unwrap_or("shhhdonttellanyoneaboutit".to_string());
        Self { api_secret }
    }
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    // Initialize endpoints outside of the server creation closure
    // so every worker refers to the same data endpoints create.
    let endpoints = vec![UserEndpoint::default()];

    HttpServer::new(move || App::new().service(get_api_service(&endpoints.clone())))
        .bind("0.0.0.0:16969")?
        .run()
        .await
}

fn get_api_service(endpoints: &Vec<impl Endpoint>) -> impl HttpServiceFactory + 'static {
    let secrets = Data::new(Secrets::default());
    // Define api scope
    let mut api_scope = web::scope("/api/v1").app_data(secrets);
    // Register endpoints
    for endpoint in endpoints {
        api_scope = endpoint.register(api_scope);
    }
    // Wrap scope with logger
    api_scope.wrap(Logger::default())
}
