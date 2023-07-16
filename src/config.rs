use std::env;
use dotenvy::dotenv;

#[derive(Debug,Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_maxage: i64,
    pub google_client_id: String,
    pub google_client_secret: String,
    pub client_redirect_url: String
}

impl Config {
    pub fn init_from_env() -> Config {
        dotenv().unwrap();
        Config {
            database_url: Config::expect_var("DATABASE_URL"),
            jwt_secret: Config::expect_var("JWT_SECRET"),
            jwt_maxage: Config::expect_var("JWT_MAXAGE").parse::<i64>()
                .expect("Cannot parse JWT_MAXAGE into i32"),
            google_client_id: Config::expect_var("GOOGLE_CLIENT_ID"),
            google_client_secret: Config::expect_var("GOOGLE_CLIENT_SECRET"),
            client_redirect_url: Config::expect_var("CLIENT_REDIRECT_URL")
        }
    }

    fn expect_var(name: &'static str) -> String {
        match env::var(name) {
            Ok(v) => v,
            Err(e) => panic!("Expected environment variable '{}' to be set: {}", name, e)
        }
    }

    fn expect_var_array(name: &'static str) -> Vec<String> {
        Config::expect_var(name).split(",").map(|s|s.to_string()).collect::<Vec<String>>()
    }
}