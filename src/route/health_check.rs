use actix_multipart::Multipart;
use actix_web::{get, post, web, HttpResponse, Responder};
use log::{error, info};

use crate::{
    service::{ping_service, upload_image::upload_images},
    AppState,
};

#[get("/health_check")]
pub async fn ping(state: web::Data<AppState>) -> impl Responder {
    let pong = ping_service::ping(state).await.map_err(|_e| {
        error!("Error in pinging database");
        HttpResponse::InternalServerError().finish()
    });
    info!("Succes in pinging database");
    HttpResponse::Ok().body(format!(
        "Hello world! Succesfully connected to Database! Query Results: {}",
        &pong.unwrap().col1
    ))
}

#[post("/upload")]
pub async fn upload_image(payload: Multipart) -> impl Responder {
    let is_success = upload_images(payload).await;
    match is_success {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}
