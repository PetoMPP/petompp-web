use crate::endpoints::endpoint::Endpoint;
use actix_web::{
    dev::HttpServiceFactory,
    middleware::Logger,
    web::{self, Data},
};
use std::env;

pub struct Secrets {
    pub api_secret: String,
}

impl Default for Secrets {
    fn default() -> Self {
        let api_secret = env::var("API_SECRET").unwrap_or("shhhdonttellanyoneaboutit".to_string());
        Self { api_secret }
    }
}

pub fn get_api_service(endpoints: &Vec<impl Endpoint>) -> impl HttpServiceFactory + 'static {
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
