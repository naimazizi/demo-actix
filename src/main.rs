use actix_web::{
    middleware,
    web::{scope, Data},
    App, HttpServer,
};
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
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
