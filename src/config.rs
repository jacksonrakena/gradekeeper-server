use dotenvy::dotenv;
use hyper::Uri;
use std::env;
use std::str::FromStr;
use log::info;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_maxage: i64,
    pub google_client_id: String,
    pub google_client_secret: String,
    pub permitted_redirect_urls: Vec<Uri>,
}

impl Config {
    pub fn init_from_env() -> Config {
        if let Err(_) = dotenv() {
            info!("No .env file found: loading configuration from environment");
        }
        Config {
            database_url: Config::expect_var("DATABASE_URL"),
            jwt_secret: Config::expect_var("JWT_SECRET"),
            jwt_maxage: Config::expect_var("JWT_MAXAGE")
                .parse::<i64>()
                .expect("Cannot parse JWT_MAXAGE into i32"),
            google_client_id: Config::expect_var("GOOGLE_CLIENT_ID"),
            google_client_secret: Config::expect_var("GOOGLE_CLIENT_SECRET"),
            permitted_redirect_urls: Config::expect_array("PERMITTED_REDIRECT_URLS")
                .iter()
                .map(|d| {
                    Uri::from_str(d).expect(
                        format!(
                            "PERMITTED_REDIRECT_URLS: Could not parse '{}' as valid URL",
                            d
                        )
                        .as_str(),
                    )
                })
                .collect::<Vec<Uri>>(),
        }
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
