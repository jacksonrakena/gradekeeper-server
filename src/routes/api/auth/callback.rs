use axum::extract::{Host, Query};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};
use axum::Extension;
use axum_extra::extract::cookie::{Cookie, SameSite};
use hyper::header::{AUTHORIZATION, CONTENT_TYPE};
use hyper::{header, Body};
use jsonwebtoken::{encode, EncodingKey, Header};
use std::sync::Arc;
use base64::Engine;
use base64::engine::general_purpose;

use serde::{Deserialize, Serialize};

use crate::errors::AppError;
use crate::middleware::auth::COOKIE_NAME;
use crate::routes::api::auth::{determine_callback_url};
use crate::ServerState;
use time::Duration;
use crate::routes::api::auth::login::LoginRequestInfo;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct CallbackData {
    error: Option<String>,
    code: Option<String>,
    state: String
}
#[derive(Serialize)]
pub struct TokenRequest {
    client_id: String,
    client_secret: String,
    code: String,
    grant_type: String,
    redirect_uri: String,
}
#[derive(Deserialize, Debug)]
pub struct TokenResponse {
    access_token: String,
    token_type: String,
}
#[derive(Deserialize, Debug)]
pub struct UserInfo {
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
    pub email: String,
    pub email_verified: bool,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub name: String,
    pub picture: String,
    pub id: String,
    pub exp: usize,
    pub iat: usize,
}
pub async fn handle_auth_callback(
    Query(data): Query<CallbackData>,
    Extension(state): Extension<Arc<ServerState>>,
    Host(host): Host,
) -> Result<Response, AppError> {
    let Some(code) = data.code else { return Err(AppError {
        status_code: StatusCode::UNAUTHORIZED,
        description: "Failed to authorize with Google.".to_string(),
    }) };

    let Ok(decoded_info_bytes) =
        general_purpose::STANDARD_NO_PAD
            .decode(&*data.state)else { return AppError::bad_request("Unable to decode state.").into() };

    let Ok(lri) = serde_json::from_slice::<LoginRequestInfo>(&*decoded_info_bytes)
        else { return AppError::bad_request("Unable to decode state.").into() };

    let client = reqwest::Client::new();
    let request = client
        .post("https://oauth2.googleapis.com/token")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(Body::from(
            serde_urlencoded::to_string(&TokenRequest {
                client_id: state.config.google_client_id.clone(),
                client_secret: state.config.google_client_secret.clone(),
                code,
                grant_type: "authorization_code".to_string(),
                redirect_uri: determine_callback_url(host, &state),
            })
            .unwrap(),
        ))
        .send()
        .await
        .unwrap()
        .json::<TokenResponse>()
        .await
        .unwrap();

    let info_request = client
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .header(
            AUTHORIZATION,
            format!("{} {}", request.token_type, request.access_token),
        )
        .body(Body::empty())
        .send()
        .await
        .unwrap()
        .json::<UserInfo>()
        .await
        .unwrap();

    let now = chrono::Utc::now();
    if !info_request.email_verified {
        return Err(AppError {
            status_code: StatusCode::BAD_REQUEST,
            description: "You have not verified your email with Google.".to_string(),
        });
    };

    let session = Session {
        iat: now.timestamp() as usize,
        exp: (now + chrono::Duration::minutes(state.config.jwt_maxage)).timestamp() as usize,
        picture: info_request.picture,
        id: info_request.email,
        name: info_request.name,
    };

    let token = encode(
        &Header::default(),
        &session,
        &EncodingKey::from_secret(state.config.jwt_secret.as_ref()),
    )
    .unwrap();

    let mut response = Redirect::to(format!("{}/?token={}", lri.redirect_url, token.to_owned()).as_str()).into_response();

    Ok(response)
}
