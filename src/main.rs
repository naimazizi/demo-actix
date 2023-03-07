#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer, middleware, web::Data};
use env_logger::Env;

pub mod route;
pub mod config;
pub mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let pool = config::database::establish_connection();
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(Data::new(pool.clone()))
            .service(route::hello_route::hello)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}