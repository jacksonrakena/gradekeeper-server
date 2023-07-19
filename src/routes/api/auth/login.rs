use crate::errors::AppError;
use crate::ServerState;
use axum::extract::Host;
use axum::response::Redirect;
use axum::Extension;
use std::sync::Arc;

use crate::routes::api::auth::determine_callback_url;

pub async fn handle_login_request(
    Extension(state): Extension<Arc<ServerState>>,
    Host(host): Host,
) -> Result<Redirect, AppError> {
    let redirection_url = format!("https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type={}&scope=openid+email+profile",
        &state.config.google_client_id,
        determine_callback_url(host, &state),
                                  "code"
    );

    Ok(Redirect::to(&*redirection_url))
}
