use actix_web::web;

use crate::AppState;

#[derive(sqlx::FromRow, Clone, Copy)]
pub struct QueryResult {
    pub col1: i32,
}

pub async fn ping(state: web::Data<AppState>) -> Result<QueryResult, sqlx::Error> {
    let result = sqlx::query_as::<_, QueryResult>("SELECT 1 as col1")
        .fetch_all(&state.pool)
        .await?;
    Ok(result[0])
}
