use actix_web::{web};
use diesel::RunQueryDsl;
use diesel::sql_types::BigInt;
use crate::config::database::DbPool;

#[derive(QueryableByName, PartialEq, Debug)]
pub struct QueryResult {
    #[diesel(sql_type = BigInt)]
    pub col1: i64,
}

pub async fn ping(pool: &web::Data<DbPool>) -> Result<QueryResult, diesel::result::Error>{
    let mut conn = pool.get().unwrap();
    let ping = web::block(move || diesel::sql_query("select 1 as col1").get_result::<QueryResult>(&mut conn))
        .await;
    ping.unwrap()
}