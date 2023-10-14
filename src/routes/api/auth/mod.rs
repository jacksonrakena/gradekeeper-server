use axum::http::uri::Scheme;
use axum::http::Uri;

pub(crate) mod callback;
pub(crate) mod login;

pub fn determine_callback_url(host: String) -> String {
    Uri::builder()
        .scheme(Scheme::HTTPS)
        .authority(host)
        .path_and_query("/api/auth/callback")
        .build()
        .unwrap()
        .to_string()
}
