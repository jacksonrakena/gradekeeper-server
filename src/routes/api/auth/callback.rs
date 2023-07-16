use std::sync::Arc;
use axum::Extension;
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};
use axum_extra::extract::cookie::{Cookie, SameSite};
use hyper::header::{AUTHORIZATION, CONTENT_TYPE};
use hyper::{Body, Client, header, Request};
use jsonwebtoken::{encode, EncodingKey, Header};
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::json;
use time::Duration;
use crate::errors::{AppError, AppErrorType};
use crate::middleware::auth::COOKIE_NAME;
use crate::ServerState;

#[derive(Debug, Deserialize)]
pub struct CallbackData {
    error: Option<String>,
    code: Option<String>
}
#[derive(Serialize)]
pub struct TokenRequest {
    client_id: String,
    client_secret: String,
    code: String,
    grant_type: String,
    redirect_uri: String
}
#[derive(Deserialize, Debug)]
pub struct TokenResponse {
    access_token: String,
    expires_in: i64,
    scope: String,
    token_type: String,
    id_token: String
}
#[derive(Deserialize, Debug)]
pub struct UserInfo {
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
    pub email: String,
    pub email_verified: bool
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub name: String,
    pub picture: String,
    pub id: String,
    pub exp: usize,
    pub iat: usize
}
pub async fn handle_auth_callback(Query(data): Query<CallbackData>, Extension(state): Extension<Arc<ServerState>>)
    -> Result<Response, AppError> {
    let Some(code) = data.code else { return Err(AppError {
        name: AppErrorType::NotAuthorized,
        status_code: StatusCode::UNAUTHORIZED,
        description: "Failed to authorize with Google.".to_string(),
    }) };

    let client = reqwest::Client::new();
    let request = client.post("https://oauth2.googleapis.com/token")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(Body::from(serde_urlencoded::to_string(&TokenRequest{
            client_id: state.config.google_client_id.clone(),
            client_secret: state.config.google_client_secret.clone(),
            code,
            grant_type: "authorization_code".to_string(),
            redirect_uri: "http://localhost:3001/api/auth/callback".to_string()
        }).unwrap()))
        .send().await.unwrap().json::<TokenResponse>().await.unwrap();

    let info_request = client
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .header(AUTHORIZATION, format!("{} {}", request.token_type, request.access_token))
        .body(Body::empty())
        .send().await.unwrap().json::<UserInfo>().await.unwrap();

    let now = chrono::Utc::now();
    if !info_request.email_verified { return Err(AppError {
        name: AppErrorType::InvalidRequest,
        status_code: StatusCode::BAD_REQUEST,
        description: "You have not verified your email with Google.".to_string(),
    }); };

    let session = Session {
        iat: now.timestamp() as usize,
        exp: (now + chrono::Duration::minutes(state.config.jwt_maxage)).timestamp() as usize,
        picture: info_request.picture,
        id: info_request.email,
        name: info_request.name
    };

    let token = encode(&Header::default(), &session, &EncodingKey::from_secret(state.config.jwt_secret.as_ref())).unwrap();

    let cookie = Cookie::build(COOKIE_NAME, token.to_owned())
        .path("/")
        .max_age(Duration::minutes(state.config.jwt_maxage))
        .same_site(SameSite::Lax)
        .http_only(false)
        .finish();

    let mut response = Redirect::to(&state.config.client_redirect_url).into_response();
    response.headers_mut().insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

    Ok(response)
}