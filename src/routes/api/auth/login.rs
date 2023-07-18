use crate::errors::AppError;
use crate::ServerState;
use axum::response::Redirect;
use axum::Extension;
use std::sync::Arc;

pub async fn handle_login_request(
    Extension(state): Extension<Arc<ServerState>>,
) -> Result<Redirect, AppError> {
    let redirection_url = format!("https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type={}&scope=openid+email+profile",
        &state.config.google_client_id, "http://localhost:3000/api/auth/callback", "code"
    );

    Ok(Redirect::to(&*redirection_url))
}
