use std::os::macos::raw::stat;
use std::sync::Arc;
use axum::Extension;
use axum::extract::Path;
use axum::http::{Request, StatusCode};
use axum::http::header::AUTHORIZATION;
use axum::middleware::Next;
use axum::response::Response;
use axum_extra::extract::{CookieJar, OptionalPath};
use diesel::connection::TransactionManagerStatus::Valid;
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::connection::LoadConnection;
use diesel::internal::derives::multiconnection::SelectStatementAccessor;
use diesel::r2d2::{ConnectionManager, ManageConnection, PooledConnection, R2D2Connection};
use diesel::serialize::ToSql;
use jsonwebtoken::{decode, DecodingKey, Validation};
use log::info;
use crate::errors::{AppError};
use crate::models::{Course, StudyBlock};
use crate::routes::api::auth::callback::Session;
use crate::schema::course::dsl::course;
use crate::schema::course::studyBlockId;
use crate::schema::study_block::dsl::study_block;
use crate::schema::study_block::{id, userId};
use crate::{schema, ServerState};

pub(crate) const COOKIE_NAME: &'static str = "GK_COOKIE";

async fn _validate_ownership_of_block(block_id: &String, session: Arc<Session>, state: Arc<ServerState>) -> Option<StudyBlock> {
    let con = &mut state.db_pool.get().unwrap();
    study_block
        .filter(id.eq(block_id.clone()).and(userId.eq(session.id.clone())))
        .select(StudyBlock::as_select()).first(con).ok()
}
pub async fn validate_ownership_of_block_and_course<B>(Path((block_id, course_id)): Path<(String, String)>,
                                                       Extension(session): Extension<Arc<Session>>,
                                                       Extension(state): Extension<Arc<ServerState>>,
                                                       mut request: Request<B>, next: Next<B>) -> Result<Response, AppError> {
    if _validate_ownership_of_block(&block_id, session, state).await.is_some() {
        return Ok(next.run(request).await)
    }
    AppError::resource_access_denied().into()
}

pub async fn validate_ownership_of_block<B>(Path(block_id): Path<String>,
                                            Extension(session): Extension<Arc<Session>>,
                                            Extension(state): Extension<Arc<ServerState>>,
                                            mut request: Request<B>, next: Next<B>) -> Result<Response, AppError> {
    if _validate_ownership_of_block(&block_id, session, state).await.is_some() {
        info!("validated ownership of {}",block_id);
        return Ok(next.run(request).await)
    }
    AppError::resource_access_denied().into()
}

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
                status_code: StatusCode::UNAUTHORIZED,
                description: "No cookie or authorization header present.".to_string(),
            }
        })?;

    let session = decode::<Session>(&token,
                                    &DecodingKey::from_secret(state.config.jwt_secret.as_ref()),
                                    &Validation::default())
        .map_err(|_|{
           AppError {
               status_code: StatusCode::UNAUTHORIZED,
               description: "Invalid session token.".to_string()
           }
        })?.claims;

    request.extensions_mut().insert(Arc::new(session));

    Ok(next.run(request).await)
}

