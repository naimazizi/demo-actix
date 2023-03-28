use std::sync::Arc;

use actix_files as fs;
use actix_web::{
    middleware,
    web::{scope, Data},
    App, HttpServer,
};
use config::config::Config;
use dotenv::dotenv;
use env_logger::Env;
use lettre::{AsyncSmtpTransport, AsyncStd1Executor};

pub mod config;
pub mod constant;
pub mod dao;
pub mod model;
pub mod route;
pub mod service;

use crate::constant::ASSETS_PATH;

pub struct AppState {
    pool: sqlx::MySqlPool,
    env: Config,
    http_client: awc::Client,
    mailer: AsyncSmtpTransport<AsyncStd1Executor>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let config = Config::init();
    let pool = config::database::establish_connection(&config).await;
    let tls_client_config = Arc::new(config::http_client::rustls_config());
    let mailer = config::mailer::init(&config);
    let app_host = &config.app_host.to_owned();
    let app_port = &config.app_port.to_owned();
    let app_workers = config.app_workers.to_owned();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(Data::new(AppState {
                pool: pool.clone(),
                env: config.clone(),
                http_client: config::http_client::init(Arc::clone(&tls_client_config)),
                mailer: mailer.clone(),
            }))
            .service(
                scope("/api")
                    .service(route::public_data::get_public_data)
                    .configure(route::auth::config)
                    .configure(route::public_data::config_secured),
            )
            .service(route::health_check::ping)
            .service(route::health_check::upload_image)
            .service(route::health_check::get_image)
            .service(fs::Files::new("/assets", format!("./{}", ASSETS_PATH)).show_files_listing())
    })
    .bind(format!("{}:{}", app_host, app_port))?
    .workers(app_workers)
    .run()
    .await
}
