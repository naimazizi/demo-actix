use actix_web::{
    middleware,
    web::{scope, Data},
    App, HttpServer,
};
use actix_files as fs;
use config::config::Config;
use dotenv::dotenv;
use env_logger::Env;

pub mod config;
pub mod constant;
pub mod dao;
pub mod dto;
pub mod model;
pub mod route;
pub mod service;

use crate::constant::ASSETS_PATH;

pub struct AppState {
    pool: sqlx::MySqlPool,
    env: Config,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = Config::init();
    let pool = config::database::establish_connection(&config).await;

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(Data::new(AppState {
                pool: pool.clone(),
                env: config.clone(),
            }))
            .service(scope("/api").configure(route::auth::config))
            .service(route::health_check::ping)
            .service(route::health_check::upload_image)
            .service(fs::Files::new("/assets", format!("./{}", ASSETS_PATH )).show_files_listing())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
