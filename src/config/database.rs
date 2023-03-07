use dotenv::dotenv;
use sqlx::any::AnyPoolOptions;

pub type DbPool = sqlx::AnyPool;


pub async fn establish_connection() -> sqlx::AnyPool {
    dotenv().ok();

    // Create connection pool
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL is missing");
    let pool = AnyPoolOptions::new()
        .max_connections(1)
        .connect(&connspec)
        .await
        .expect("ERROR Creating DB Pool");
    pool
}