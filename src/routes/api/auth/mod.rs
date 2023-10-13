use crate::ServerState;
use axum::http::uri::{PathAndQuery, Scheme};
use axum::http::Uri;
use std::str::FromStr;
use std::sync::Arc;

pub(crate) mod callback;
pub(crate) mod login;

pub fn determine_callback_url(host: String, state: &Arc<ServerState>) -> String {
    Uri::builder()
        .scheme(Scheme::HTTPS)
        .authority(host)
        .path_and_query("/api/auth/callback")
        .build()
        .unwrap().to_string()
}
