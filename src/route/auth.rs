use actix_web::{get, post, web, HttpResponse};
use actix_web_grants::proc_macro::has_any_permission;
use actix_web_httpauth::middleware::HttpAuthentication;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde_json::json;

use crate::{
    dao::user::{check_existing_user, get_user_by_email, insert_new_user},
    model::{
        response::{FilteredUser, GeneralResponse},
        user::{LoginUserSchema, RegisterUserSchema, User},
    },
    service::{
        errors::AppError,
        jwt_auth::{create_jwt, validator, Claims},
    },
    AppState,
};

#[post("/register")]
async fn register_user_handler(
    body: web::Json<RegisterUserSchema>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let exists: bool = check_existing_user(&body.email, &data.pool).await;

    if exists {
        return Err(AppError::BadRequest {
            message: "User with that email already exists".to_string(),
        });
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .expect("Error while hashing password")
        .to_string();
    let user = insert_new_user(&body.name, &body.email, &hashed_password, &data.pool).await?;
    let user_response = GeneralResponse {
        status: "success".to_string(),
        message: "succesfully get current user".to_string(),
        data: Some(filter_user_record(&user)),
    };

    Ok(HttpResponse::Ok().json(user_response))
}

fn filter_user_record(user: &User) -> FilteredUser {
    FilteredUser {
        id: user.id.to_string(),
        email: user.email.to_owned(),
        name: user.name.to_owned(),
        photo: user.photo.to_owned(),
        role: user.role.to_owned(),
        verified: user.verified,
        createdAt: user.created_at.unwrap(),
        updatedAt: user.updated_at.unwrap(),
    }
}

#[post("/login")]
async fn login_user_handler(
    body: web::Json<LoginUserSchema>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let query_result = get_user_by_email(&body.email, &data.pool).await?;

    let is_valid = query_result.to_owned().map_or(false, |user| {
        let parsed_hash = PasswordHash::new(&user.password).unwrap();
        Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true)
    });

    if !is_valid {
        return Err(AppError::BadRequest {
            message: "Invalid email or password".to_string(),
        });
    }

    let user = query_result.unwrap();

    let claims = Claims::new(user.email.to_owned(), vec![user.role.to_owned()]);
    let token = create_jwt(claims, &data.env.jwt_secret);
    match token {
        Ok(token_str) => {
            Ok(HttpResponse::Ok().json(json!({"status": "success", "token": token_str})))
        }
        Err(_) => Err(AppError::InternalError {
            message: "failed to generate token".to_string(),
        }),
    }
}

#[get("")]
#[has_any_permission("ROLE_USER", "ROLE_ADMIN")]
async fn get_me_handler(
    opt_claims: Option<web::ReqData<Claims>>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let opt_user = match opt_claims {
        Some(claim) => get_user_by_email(&claim.email, &data.pool).await?,
        None => None,
    };

    let user = opt_user.ok_or_else(|| AppError::BadRequest {
        message: "email is not found".to_string(),
    });

    match user {
        Ok(u) => {
            let json_response: GeneralResponse<FilteredUser> = GeneralResponse {
                status: "success".to_string(),
                message: "succesfully get current user".to_string(),
                data: Some(filter_user_record(&u)),
            };
            Ok(HttpResponse::Ok().json(json_response))
        }
        Err(e) => Err(AppError::InternalError { message: (e.to_string()) }),
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(validator);
    let secured_scope = web::scope("/auth/whoami")
        .wrap(auth)
        .service(get_me_handler);

    let unsecured_scope = web::scope("/auth")
        .service(register_user_handler)
        .service(login_user_handler);

    conf.service(secured_scope);
    conf.service(unsecured_scope);
}
