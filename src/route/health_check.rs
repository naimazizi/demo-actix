use actix_multipart::Multipart;
use actix_web::{
    get, post,
    web::{self},
    HttpResponse, Responder,
};
use log::{error, info};

use crate::{
    model::response::GeneralResponse,
    service::{errors::AppError, ping_service, upload_image::upload_images},
    AppState,
};

#[get("/health_check")]
pub async fn ping(state: web::Data<AppState>) -> impl Responder {
    let pong = ping_service::ping(&state).await.map_err(|_e| {
        error!("Error in pinging database");
        HttpResponse::InternalServerError().finish()
    });
    let pong_mail = &state.mailer.test_connection().await.unwrap();
    info!(
        "Succes in pinging database, Success in pinging mail: {:?}",
        pong_mail
    );
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

#[get("/image")]
pub async fn get_image(state: web::Data<AppState>) -> impl Responder {
    let map_url =
        "https://upload.wikimedia.org/wikipedia/commons/f/ff/Pizigani_1367_Chart_10MB.jpg";
    let mut res = state.http_client.get(map_url).send().await.unwrap();

    if !res.status().is_success() {
        log::error!("Wikipedia did not return expected image");
        return HttpResponse::InternalServerError().finish();
    }

    let payload = res
        .body()
        // expected image is larger than default body limit
        .limit(20_000_000) // 20MB
        .await
        .unwrap();

    HttpResponse::Ok()
        .content_type(mime::IMAGE_JPEG)
        .body(payload)
}
