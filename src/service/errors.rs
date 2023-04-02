use std::fmt;

use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};
use lettre::address::AddressError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, Serialize, Deserialize, Clone)]
pub enum AppErrorType {
    #[display(fmt = "An internal error occurred")]
    InternalError,
    #[display(fmt = "Bad request received")]
    BadRequest,
}

#[derive(Debug, Error, Serialize, Deserialize)]
pub struct AppError {
    pub cause: Option<String>,
    pub message: Option<String>,
    pub status: AppErrorType,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(&self)
    }
    fn status_code(&self) -> StatusCode {
        match self.status {
            AppErrorType::BadRequest => StatusCode::BAD_REQUEST,
            AppErrorType::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
impl From<sqlx::Error> for AppError {
    fn from(_error: sqlx::Error) -> AppError {
        AppError {
            cause: Some(_error.to_string()),
            message: Some("Error in querying to database".to_string()),
            status: AppErrorType::InternalError,
        }
    }
}
impl From<AddressError> for AppError {
    fn from(_error: AddressError) -> AppError {
        AppError {
            cause: Some(_error.to_string()),
            status: AppErrorType::InternalError,
            message: Some("Failed to parse email address".to_string()),
        }
    }
}