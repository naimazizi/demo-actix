use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, Pool};
use dotenv::dotenv;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn establish_connection() -> Pool<ConnectionManager<SqliteConnection>>{
    dotenv().ok();

    // Create connection pool
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    pool
}