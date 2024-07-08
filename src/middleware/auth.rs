use axum::extract::Path;
use axum::http::header::AUTHORIZATION;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use axum::Extension;

use std::sync::Arc;

use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use google_oauth::AsyncClient;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Deserialize;

use crate::errors::{AppError, AppResult};
use crate::models::{Course, CourseComponent, CourseSubcomponent, StudyBlock};
use crate::routes::api::auth::callback::Session;
use crate::schema::course::block_id;
use crate::schema::course::dsl::course;
use crate::schema::course_component::course_id;
use crate::schema::course_component::dsl::course_component;
use crate::schema::course_subcomponent::component_id;
use crate::schema::course_subcomponent::dsl::course_subcomponent;

use crate::schema::study_block::dsl::study_block;
use crate::schema::study_block::{id, user_id};
use crate::ServerState;

#[derive(Deserialize)]
pub struct RouteAssetIdentifiers {
    block_id: Option<String>,
    course_id: Option<String>,
    component_id: Option<String>,
    subcomponent_id: Option<String>,
}
pub async fn validate_ownership_of_route_assets<B>(
    Path(route_asset_ids): Path<RouteAssetIdentifiers>,
    Extension(session): Extension<Arc<Session>>,
    Extension(state): Extension<Arc<ServerState>>,
    request: Request<B>,
    next: Next<B>
) -> Result<Response, AppError> {
    let con = &mut state.get_db_con()?;
    match &route_asset_ids.block_id {
        Some(_block_id) => {
            if !study_block
                .filter(id.eq(_block_id).and(user_id.eq(&session.id)))
                .select(StudyBlock::as_select())
                .first(con)
                .is_ok() {
                return Err(AppError::resource_access_denied())
            }
        },
        _ => {}
    }
    
    match &route_asset_ids.course_id {
        Some(_course_id) => {
            if !course
                .filter(crate::schema::course::id.eq(_course_id).and(block_id.eq(route_asset_ids.block_id.unwrap())))
                .select(Course::as_select())
                .first(con)
                .is_ok() {
                return Err(AppError::resource_access_denied())
            }
        },
        _ => {}
    }
    
    match &route_asset_ids.component_id {
        Some(_component_id) => {
            if !course_component
                .filter(crate::schema::course_component::id.eq(_component_id).and(course_id.eq(route_asset_ids.course_id.unwrap())))
                .select(CourseComponent::as_select())
                .first(con)
                .is_ok() {
                return Err(AppError::resource_access_denied())
            }
        },
        _ => {}
    }
    
    match &route_asset_ids.subcomponent_id {
        Some(_subcomponent_id) => {
            if !course_subcomponent
                .filter(crate::schema::course_subcomponent::id.eq(_subcomponent_id).and(component_id.eq(route_asset_ids.component_id.unwrap())))
                .select(CourseSubcomponent::as_select())
                .first(con)
                .is_ok() {
                return Err(AppError::resource_access_denied())
            }
        },
        _ => {}
    }
    Ok(next.run(request).await)
}

pub async fn check_authorization<B>(
    Extension(state): Extension<Arc<ServerState>>,
    Extension(google_client): Extension<Arc<AsyncClient>>,
    mut request: Request<B>,
    next: Next<B>,
) -> AppResult<Response> {
    let token = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer ") {
                Some(auth_value[7..].to_owned())
            } else {
                None
            }
        })
        .ok_or_else(|| AppError {
            status_code: StatusCode::UNAUTHORIZED,
            description: "No authorization header present.".to_string(),
        })?;

    let session = try_decode_session(token, &state, &google_client).await?;
    request.extensions_mut().insert(Arc::new(session));

    Ok(next.run(request).await)
}

pub async fn try_decode_session(token: String, state: &Arc<ServerState>, google_client: &Arc<AsyncClient>) -> AppResult<Session> {
    // Firstly, try and decode as a Gradekeeper proprietary JWT (signed in /api/auth/callback function handle_auth_callback)
    match decode::<Session>(
        &token,
        &DecodingKey::from_secret(state.config.jwt_secret.as_ref()),
        &Validation::default(),
    ) {
        Ok(session_data) => {
            Ok(session_data.claims)
        }
        // Otherwise, try and decode a Google signed ID token
        // This is for mobile native/RN app clients, which are experimental
        Err(_) => {
            match google_client.validate_id_token(&token).await {
                Ok(google_token) => {
                    Ok(Session {
                        id: google_token.email.unwrap(),
                        exp: google_token.exp as usize,
                        iat: google_token.iat as usize,
                        picture: google_token.picture.unwrap_or("".to_string()),
                        name: google_token.name.unwrap_or("".to_string())
                    })
                },
                Err(_) => {
                    Err(AppError {
                        status_code: StatusCode::FORBIDDEN,
                        description: "Invalid session token.".to_string(),
                    })
                }
            }
        }
    }
}