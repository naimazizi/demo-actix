use actix_multipart::Multipart;
use actix_web::{get, post, web, HttpResponse, Responder};
use log::{error, info};
use tera::Context;

use crate::{
    model::response::GeneralResponse,
    service::{
        email,
        errors::{AppError, AppErrorType},
        ping_service,
        upload_image::upload_images,
    },
    AppState,
};

#[get("/health_check")]
pub async fn ping(state: web::Data<AppState>) -> impl Responder {
    let pong = ping_service::ping(&state).await.map_err(|_e| {
        error!("Error in pinging database");
        HttpResponse::InternalServerError().finish()
    });

    let email_body = &state
        .tera
        .render("email_registration_confirmation.html", &Context::new())
        .unwrap();

    let to_email = "cyber.virion@gmail.com";

    let _ = email::send_email(
        to_email,
        "Registration Confirmation",
        email_body,
        &state.mailer,
    )
    .await;

    HttpResponse::Ok().body(format!(
        "Hello world! Succesfully connected to Database! Query Results: {}",
        &pong.unwrap().col1
    ))
}

#[post("/upload")]
pub async fn upload_image(payload: Multipart) -> impl Responder {
    let is_success = upload_images(payload).await;
    _ = is_success.map_err(|e| {
        return AppError {
            cause: Some(e.to_string()),
            status: AppErrorType::InternalError,
            message: Some("Failed to upload image".to_string()),
        };
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
