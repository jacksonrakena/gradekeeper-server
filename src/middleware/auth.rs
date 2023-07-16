use std::sync::Arc;
use axum::Extension;
use axum::http::{Request, StatusCode};
use axum::http::header::AUTHORIZATION;
use axum::middleware::Next;
use axum::response::Response;
use axum_extra::extract::CookieJar;
use diesel::connection::TransactionManagerStatus::Valid;
use diesel::serialize::ToSql;
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::errors::{AppError, AppErrorType};
use crate::routes::api::auth::callback::Session;
use crate::ServerState;

pub(crate) const COOKIE_NAME: &'static str = "GK_COOKIE";

pub async fn check_authorization<B>(cookie_jar: CookieJar,
                                    Extension(state): Extension<Arc<ServerState>>,
                                    mut request: Request<B>,
                                    next: Next<B>)
    -> Result<Response, AppError> {
    let token = cookie_jar
        .get(COOKIE_NAME)
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            request.headers()
                .get(AUTHORIZATION)
                .and_then(|auth_header|auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        None
                    }
                })
        })
        .ok_or_else(|| {
            AppError {
                name: AppErrorType::NotAuthorized,
                status_code: StatusCode::UNAUTHORIZED,
                description: "No cookie or authorization header present.".to_string(),
            }
        })?;

    let session = decode::<Session>(&token,
                                    &DecodingKey::from_secret(state.config.jwt_secret.as_ref()),
                                    &Validation::default())
        .map_err(|_|{
           AppError {
               name: AppErrorType::NotAuthorized,
               status_code: StatusCode::UNAUTHORIZED,
               description: "Invalid session token.".to_string()
           }
        })?.claims;

    request.extensions_mut().insert(Arc::new(session));

    Ok(next.run(request).await)
}

