#[derive(Debug, Clone)]
pub struct Config {
    pub app_name: String,
    pub app_host: String,
    pub app_port: String,
    pub app_workers: usize,
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_maxage: i32,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_url: String,
}

impl Config {
    pub fn init() -> Config {
        let app_name = std::env::var("APP_NAME").expect("APP_NAME must be set");
        let app_host = std::env::var("APP_HOST").expect("APP_HOST must be set");
        let app_port = std::env::var("APP_PORT").expect("APP_PORT must be set");
        let app_workers = std::env::var("APP_WORKERS").expect("APP_WORKERS must be set");
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_expires_in = std::env::var("JWT_EXPIRED_IN").expect("JWT_EXPIRED_IN must be set");
        let jwt_maxage = std::env::var("JWT_MAXAGE").expect("JWT_MAXAGE must be set");
        let smtp_username = std::env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");
        let smtp_password = std::env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");
        let smtp_url = std::env::var("SMTP_URL").expect("SMTP_URL must be set");
        Config {
            app_name,
            app_host,
            app_port,
            app_workers: app_workers.parse::<usize>().unwrap(),
            database_url,
            jwt_secret,
            jwt_expires_in,
            jwt_maxage: jwt_maxage.parse::<i32>().unwrap(),
            smtp_username,
            smtp_password,
            smtp_url,
        }
    }
}
