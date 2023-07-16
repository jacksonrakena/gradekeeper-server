mod models;
mod schema;
mod routes;
mod middleware;

use std::collections::HashMap;
use std::env;
use std::iter::{Map, once};
use std::sync::Arc;
use axum::{Json, Router, routing::{get, post, delete}, response::IntoResponse,};
use axum::extract::{Path, Query};
use axum::http::{Method, StatusCode};
use axum::http::header::AUTHORIZATION;
use chrono::{DateTime, Utc};
use diesel::{Connection, ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};
use diesel::associations::{BelongsTo, HasTable};
use dotenvy::dotenv;
use log::{info, LevelFilter};
use serde::{Deserialize, Serialize};
use pretty_env_logger::env_logger::{Builder, Target};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use time::OffsetDateTime;
use diesel::prelude::*;
use diesel::query_dsl::methods::GroupByDsl;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tower_http::{add_extension::AddExtensionLayer};
use tower_http::sensitive_headers::SetSensitiveRequestHeadersLayer;
use crate::middleware::auth::check_authorization;

use crate::models::{Course, CourseComponent, CourseSubcomponent, StudyBlock, User};
use crate::schema::course::dsl::course;
use crate::schema::course::studyBlockId;
use crate::schema::course_component::dsl::course_component;
use crate::schema::course_component::subjectId;
use crate::schema::course_subcomponent::componentId;
use crate::schema::course_subcomponent::dsl::course_subcomponent;
use crate::schema::study_block::dsl::study_block;

pub struct ServerState {

}

#[tokio::main]
async fn main() {
    Builder::new()
        .filter_module(stringify!(gk-server), LevelFilter::Info)
        .target(Target::Stdout)
        .init();
    info!("Gradekeeper Nova server starting");

    let app = Router::new()
        // Users
        .route("/users/me", get(crate::routes::api::users::me::get_user))
        // Blocks
        .route("/block/create", post(routes::api::block::create::create_block))
        .route("/block/:block_id", get(routes::api::block::block_id::get_block))
        .route("/block/:block_id", axum::routing::delete(routes::api::block::block_id::delete_block))
        .route("/block/:block_id/import", post(routes::api::block::_block_id::import::import_block))

        // Courses
        .route("/block/:block_id/course/create", post(routes::api::block::course::create::create_course))
        .route("/block/:block_id/course/:course_id", get(routes::api::block::course::course_id::get_course))
        .route("/block/:block_id/course/:course_id", axum::routing::delete(routes::api::block::course::course_id::delete_course))
        .route("/block/:block_id/course/:course_id", post(routes::api::block::course::course_id::update_course))

        // Components
        .route("/block/:block_id/course/:course_id/component/:component_id",
               post(routes::api::block::course::_course_id::component::component_id::update_course_component)
        )
        .layer(axum::middleware::from_fn(check_authorization))
        // End authorised section


        // Final Layer - CORS
        .layer(SetSensitiveRequestHeadersLayer::new(once(AUTHORIZATION)))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods([Method::GET,Method::POST]))
        .layer(TraceLayer::new_for_http())
        .layer(AddExtensionLayer::new(Arc::new(ServerState{})));

    axum::Server::bind(&"0.0.0.0:3001".parse().unwrap()).serve(app.into_make_service()).await.unwrap();
    println!("online");
}


