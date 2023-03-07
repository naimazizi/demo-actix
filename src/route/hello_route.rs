use actix_web::{get, HttpResponse, Responder, web};
use crate::service::ping_service::ping;
use crate::AppState;
use log::{error, info};


#[get("/")]
pub async fn hello(state: web::Data<AppState>) -> impl Responder {
    let pong = ping(state).await.map_err(|_e| {
        error!("Error in pinging database");
        HttpResponse::InternalServerError().finish()
    });
    info!("Succes in pinging database");
    HttpResponse::Ok().body(format!("Hello world! Succesfully connected to Database! Query Results: {}", &pong.unwrap().col1))
}