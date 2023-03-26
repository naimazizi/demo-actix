use actix_web::{web, get, HttpResponse, post};
use actix_web_grants::proc_macro::has_any_permission;
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::AppState;
use crate::dao::public_data::{get_by_key, upsert};
use crate::model::public_data::{PublicDataDto, PublicData};
use crate::model::response::GeneralResponse;
use crate::service::errors::AppError;
use crate::service::jwt_auth::validator;

#[get("/public_data/{key}")]
pub async fn get_public_data(path: web::Path<String>, state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let key = path.into_inner();
    let opt_pub_data = get_by_key(key.as_str(), &state.pool).await?;

    match opt_pub_data {
        Some(data) => {
            let json_response: GeneralResponse<String> = GeneralResponse {
                status: "success".to_string(),
                message: "succesfully get public data".to_string(),
                data: Some(data.value)
            };
            Ok(HttpResponse::Ok().json(json_response))
        }
        None => Err(AppError::BadRequest {
            message: format!("key {} is not found.", key),
        }),   
    }
}

#[post("/public_data")]
#[has_any_permission("ROLE_USER", "ROLE_ADMIN")]
pub async fn upsert_public_data(body: web::Json<PublicDataDto>, state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let pub_data = upsert(&body.key, &body.value, &state.pool).await;
    
    match pub_data {
        Ok(data) => {
            let json_response: GeneralResponse<PublicData> = GeneralResponse {
                status: "success".to_string(),
                message: "succesfully create public data".to_string(),
                data: Some(data)
            };
            Ok(HttpResponse::Ok().json(json_response))
        }
        Err(_) => Err(AppError::InternalError {
            message: format!("Error in creating public_data, key: {}", body.key),
        }),
    }
}

pub fn config_secured(conf: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(validator);
    let secured_scope = web::scope("")
        .wrap(auth)
        .service(upsert_public_data);
    conf.service(secured_scope);
}