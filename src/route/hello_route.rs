use actix_web::{get, HttpResponse, Responder, web};
use crate::config::database::DbPool;

#[get("/")]
pub async fn hello(pool: web::Data<DbPool>) -> impl Responder {
    let conn = pool.get();
    assert!(&conn.is_ok());
    HttpResponse::Ok().body("Hello world! Succesfully connected to Database!")
}