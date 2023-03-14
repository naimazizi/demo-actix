use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};

use crate::model::response::GeneralResponse;

#[derive(Debug, Display, Error)]
pub enum AppError {
    #[display(fmt = "An internal error occurred, caused by: .")]
    InternalError { message: String },
    #[display(fmt = "Bad request received: {}", message)]
    BadRequest { message: String },
}

impl error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let error_response: GeneralResponse<()> = GeneralResponse {
            status: "failed".to_string(),
            message: self.to_string(),
            data: None,
        };
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(error_response)
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::BadRequest { .. } => StatusCode::BAD_REQUEST,
            AppError::InternalError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
