use actix_multipart::Multipart;
use actix_web::{get, post, web, HttpResponse, Responder};
use log::{error, info};

use crate::{
    model::response::GeneralResponse,
    service::{errors::AppError, ping_service, upload_image::upload_images},
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
    _ = is_success.map_err(|e| AppError::InternalError {
        message: e.to_string(),
    });
    let resp: GeneralResponse<()> = GeneralResponse {
        status: "success".to_string(),
        message: "successufully upload image".to_string(),
        data: None,
    };
    HttpResponse::Ok().json(resp)
}
