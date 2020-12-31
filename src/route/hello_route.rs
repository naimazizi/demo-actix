use actix_web::{get, HttpResponse, Responder, web};
use crate::config::database::DbPool;
use diesel::RunQueryDsl;
use diesel::sql_types::BigInt;


#[derive(QueryableByName, PartialEq, Debug)]
pub struct QueryResult {
    #[sql_type="BigInt"]
    pub col1: i64,
}


#[get("/")]
pub async fn hello(pool: web::Data<DbPool>) -> impl Responder {
    let conn = pool.get().unwrap();
    let test_query = web::block(move || diesel::sql_query("select 1 as col1").get_result::<QueryResult>(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        });
    
    HttpResponse::Ok().body(format!("Hello world! Succesfully connected to Database! Query Results: {}", &test_query.unwrap().col1))
}