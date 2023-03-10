use sqlx::mysql::MySqlPoolOptions;

use super::config::Config;

pub async fn establish_connection(config: &Config) -> sqlx::MySqlPool {
    let pool = MySqlPoolOptions::new()
        .max_connections(1)
        .connect(&config.database_url)
        .await
        .expect("ERROR Creating DB Pool");
    pool
}