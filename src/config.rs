use dotenvy::dotenv;
use hyper::Uri;
use log::info;
use std::env;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_user: String,
    pub database_password: String,
    pub database_host: String,
    pub database_name: String,
    pub jwt_secret: String,
    pub jwt_maxage: i64,
    pub google_client_id: String,
    pub google_client_secret: String,
    pub permitted_redirect_urls: Vec<Uri>,
}

impl Config {
    pub fn init_from_env() -> Config {
        if dotenv().is_err() {
            info!("No .env file found: loading configuration from environment");
        }
        Config {
            database_user: Config::expect_var("DATABASE_USERNAME"),
            database_name: Config::expect_var("DATABASE_NAME"),
            database_password: Config::expect_var("DATABASE_PASSWORD"),
            database_host: Config::expect_var("DATABASE_HOST"),
            jwt_secret: Config::expect_var("JWT_SECRET"),
            jwt_maxage: Config::expect_var("JWT_MAXAGE")
                .parse::<i64>()
                .expect("Cannot parse JWT_MAXAGE into i32"),
            google_client_id: Config::expect_var("GOOGLE_CLIENT_ID"),
            google_client_secret: Config::expect_var("GOOGLE_CLIENT_SECRET"),
            permitted_redirect_urls: Config::expect_array("PERMITTED_REDIRECT_URLS")
                .iter()
                .map(|d| {
                    Uri::from_str(d).unwrap_or_else(|_| {
                        panic!(
                            "PERMITTED_REDIRECT_URLS: Could not parse '{}' as valid URL",
                            d
                        )
                    })
                })
                .collect::<Vec<Uri>>(),
        }
    }

    pub fn build_database_url(&self) -> String {
        format!("postgresql://{}:{}@{}/{}", self.database_user, self.database_password, self.database_host, self.database_name)
    }
    fn expect_var(name: &'static str) -> String {
        match env::var(name) {
            Ok(v) => v,
            Err(e) => panic!("Expected environment variable '{}' to be set: {}", name, e),
        }
    }
    fn expect_array(name: &'static str) -> Vec<String> {
        Config::expect_var(name)
            .split(",")
            .map(str::to_string)
            .collect()
    }
}
