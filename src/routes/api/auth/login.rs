use crate::errors::AppError;
use crate::ServerState;
use axum::response::Redirect;
use axum::Extension;
use std::sync::Arc;
use axum::extract::Host;
use axum::http::Uri;
use axum::http::uri::Scheme;

pub async fn handle_login_request(
    Extension(state): Extension<Arc<ServerState>>,
    Host(host): Host
) -> Result<Redirect, AppError> {
    let redirection_url = format!("https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type={}&scope=openid+email+profile",
        &state.config.google_client_id, Uri::builder().scheme(Scheme::HTTPS).authority(host).path_and_query("/api/auth/callback").build().unwrap().to_string(), "code"
    );

    Ok(Redirect::to(&*redirection_url))
}
