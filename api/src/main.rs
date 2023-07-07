use std::{io, env};

use actix_web::{HttpServer, middleware::Logger, App, get, Responder, HttpResponse};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(Logger::default())
            // register HTTP requests handlers
            .service(index)
    })
    .bind("0.0.0.0:16969")?
    .run()
    .await
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}