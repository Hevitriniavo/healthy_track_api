use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env;

lazy_static! {
     pub static ref CONFIG: Config = Config::load();
}

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_host: String,
    pub server_port: String,
    pub jwt_secret: String
}

impl Config {
    pub fn load() -> Self {
        dotenv().ok();

        Self {
            database_url: env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://default_url".to_string()),
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string()),
            jwt_secret: env::var("JWT_SECRET").unwrap_or_else(|_| "my_default_secret".to_string()),
        }
    }
}