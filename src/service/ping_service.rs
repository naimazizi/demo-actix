use actix_web::{web};
use actix_web::error::BlockingError;
use diesel::RunQueryDsl;
use diesel::sql_types::BigInt;
use diesel::result::Error;
use crate::config::database::DbPool;

#[derive(QueryableByName, PartialEq, Debug)]
pub struct QueryResult {
    #[sql_type="BigInt"]
    pub col1: i64,
}

pub async fn ping(pool: &web::Data<DbPool>) -> Result<QueryResult, BlockingError<Error>>{
    let conn = pool.get().unwrap();
    let ping = web::block(move || diesel::sql_query("select 1 as col1").get_result::<QueryResult>(&conn))
        .await;
    ping
}