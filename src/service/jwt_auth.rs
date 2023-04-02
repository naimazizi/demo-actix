use actix_web::dev::ServiceRequest;
use actix_web::error::ErrorUnauthorized;
use actix_web::{web, Error, HttpMessage};
use actix_web_grants::permissions::AttachPermissions;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::AppState;

const JWT_EXPIRATION_HOURS: i64 = 24;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub email: String,
    pub permissions: Vec<String>,
    exp: i64,
}

impl Claims {
    pub fn new(email: String, permissions: Vec<String>) -> Self {
        Self {
            email,
            permissions,
            exp: (Utc::now() + Duration::hours(JWT_EXPIRATION_HOURS)).timestamp(),
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now().timestamp() > self.exp
    }
}

/// Create a json web token (JWT)
pub fn create_jwt(claims: Claims, jwt_secret_key: &String) -> Result<String, Error> {
    let encoding_key = EncodingKey::from_secret(jwt_secret_key.as_ref());
    jsonwebtoken::encode(&Header::default(), &claims, &encoding_key)
        .map_err(|e| ErrorUnauthorized(e.to_string()))
}

/// Decode a json web token (JWT)
pub fn decode_jwt(token: &str, jwt_secret_key: &String) -> Result<Claims, Error> {
    let decoding_key = DecodingKey::from_secret(jwt_secret_key.as_ref());
    jsonwebtoken::decode::<Claims>(token, &decoding_key, &Validation::default())
        .map(|data| data.claims)
        .map(|claims| {
            if claims.is_expired() {
                ErrorUnauthorized("Token is expired");
            }
            claims
        })
        .map_err(|e| ErrorUnauthorized(e.to_string()))
}

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    // We just get permissions from JWT
    let data = req.app_data::<web::Data<AppState>>().expect("AppState is not found");
    let result = decode_jwt(credentials.token(), &data.env.jwt_secret);
    match result {
        Ok(claims) => {
            req.attach(claims.permissions.to_owned());
            req.extensions_mut().insert(claims);
            Ok(req)
        }
        // required by `actix-web-httpauth` validator signature
        Err(e) => Err((e, req)),
    }
}
