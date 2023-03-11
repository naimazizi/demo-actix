use actix_web::{
    get, post, web, HttpResponse, Responder,
};
use actix_web_grants::proc_macro::has_any_permission;
use actix_web_httpauth::middleware::HttpAuthentication;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde_json::json;

use crate::{
    service::jwt_auth::{Claims, create_jwt, validator},
    model::{user::{LoginUserSchema, RegisterUserSchema, User}, response::FilteredUser},
    dao::user::{check_existing_user, insert_new_user, get_user_by_email},
    AppState,
};


#[post("/register")]
async fn register_user_handler(
    body: web::Json<RegisterUserSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let exists: bool = check_existing_user(&body.email, &data.pool).await;

    if exists {
        return HttpResponse::Conflict().json(
            serde_json::json!({"status": "fail","message": "User with that email already exists"}),
        );
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .expect("Error while hashing password")
        .to_string();
    let query_result = insert_new_user(&body.name, &body.email, &hashed_password, &data.pool).await;

    match query_result {
        Ok(user) => {
            let user_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "user": filter_user_record(&user)
            })});

            return HttpResponse::Ok().json(user_response);
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    }
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
) -> impl Responder {

    let query_result = get_user_by_email(&body.email, &data.pool).await.unwrap();

    let is_valid = query_result.to_owned().map_or(false, |user| {
        let parsed_hash = PasswordHash::new(&user.password).unwrap();
        Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true)
    });

    if !is_valid {
        return HttpResponse::BadRequest()
            .json(json!({"status": "fail", "message": "Invalid email or password"}));
    }

    let user = query_result.unwrap();

    let claims = Claims::new(user.email.to_owned(), vec![user.role.to_owned()]);
    let token = create_jwt(claims, &data.env.jwt_secret);
    match token {
        Ok(token_str) => {
            HttpResponse::Ok()
                .json(json!({"status": "success", "token": token_str}))
        }
        Err(_) => {
            HttpResponse::InternalServerError()
                .json(json!({"status": "fail", "token": "failed to generate token"}))
        }
    }
    
}


#[get("")]
#[has_any_permission("ROLE_USER", "ROLE_ADMIN")]
async fn get_me_handler(
    opt_claims: Option<web::ReqData<Claims>>,
    data: web::Data<AppState>,
) -> impl Responder {

    let opt_user = match opt_claims {
        Some(claim) => {
            get_user_by_email(&claim.email, &data.pool).await.unwrap()
        }
        None => {
            None
        }
    };
    
    match opt_user {
        Some(user) => {
            let json_response = serde_json::json!({
                "status":  "success",
                "data": serde_json::json!({
                    "user": filter_user_record(&user)
                })
            });
        
            HttpResponse::Ok().json(json_response)
        },
        None => {
            let json_response = serde_json::json!({
                "status":  "fail",
                "data": format!("User is not found")
            });
        
            HttpResponse::InternalServerError().json(json_response)
        }
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
