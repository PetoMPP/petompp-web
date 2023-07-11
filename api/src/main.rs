use actix_web::{App, HttpServer};
use petompp_web_api::{app::get_api_service, endpoints::user::UserEndpoint};
use std::{env, io};

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
