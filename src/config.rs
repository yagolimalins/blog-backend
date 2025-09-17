use axum::http::HeaderValue;
use dotenvy::dotenv;
use std::env;
use std::net::IpAddr;

pub struct Config {
    pub database: String,
    pub host: IpAddr,
    pub port: u16,
    pub origins: Vec<HeaderValue>,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        tracing_subscriber::fmt::init();

        let database = env::var("DATABASE").expect("DATABASE must be set in .env");

        let host = env::var("HOST")
            .unwrap_or_else(|_| String::from("0.0.0.0"))
            .parse()
            .expect("HOST must be a valid IP");

        let port = env::var("PORT")
            .unwrap_or_else(|_| String::from("3000"))
            .parse()
            .expect("PORT must be a valid u16");

        let origins = env::var("ORIGINS")
            .unwrap_or_else(|_| String::from("http://localhost:5500"))
            .split(",")
            .map(|s| HeaderValue::from_str(s.trim()).expect("Invalid origin format in ORIGINS"))
            .collect::<Vec<HeaderValue>>();

        Config {
            database: database,
            host: host,
            port: port,
            origins: origins,
        }
    }
}
