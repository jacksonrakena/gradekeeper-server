mod models;
mod schema;
mod routes;
mod middleware;
mod errors;
mod config;
use std::env;
use std::iter::{once};
use std::sync::Arc;
use axum::{Router, routing::{get, post},};
use axum::http::{Method};
use axum::http::header::AUTHORIZATION;
use diesel::{MysqlConnection};
use dotenvy::dotenv;
use log::{error, info, Level, LevelFilter};

use diesel::prelude::*;

use diesel::r2d2::{ConnectionManager, Pool};

use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tower_http::{add_extension::AddExtensionLayer};
use tower_http::sensitive_headers::SetSensitiveRequestHeadersLayer;
use tracing_subscriber::filter::Targets;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::config::Config;
use crate::middleware::auth::check_authorization;

pub struct ServerState {
    db_pool: Pool<ConnectionManager<MysqlConnection>>,
    config: Config
}

#[tokio::main]
async fn main() {
    let config =  Config::init_from_env();
    let filter = Targets::new()
        .with_target("tower_http::trace::on_response", tracing_subscriber::filter::LevelFilter::TRACE)
        .with_default(tracing_subscriber::filter::LevelFilter::INFO);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();
    info!("Gradekeeper Nova server starting on {} {} {}", env::consts::FAMILY, env::consts::OS,  env::consts::ARCH);

    let initial_state = ServerState {
        db_pool: Pool::builder()
            .test_on_check_out(true)
            .build(ConnectionManager::<MysqlConnection>::new(&config.database_url))
            .expect("Could not build connection pool"),
        config
    };

    let app = Router::new()
        // Users
        .route("/api/users/me", get(crate::routes::api::users::me::get_user))
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

        // Login
        .route("/api/auth/login", get(routes::api::auth::login::handle_login_request))
        .route("/api/auth/callback", get(routes::api::auth::callback::handle_auth_callback))
        // Final Layer - CORS
        .layer(SetSensitiveRequestHeadersLayer::new(once(AUTHORIZATION)))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods([Method::GET,Method::POST]))
        .layer(TraceLayer::new_for_http())
        .layer(AddExtensionLayer::new(Arc::new(initial_state)));

    let server = axum::Server::bind(&"0.0.0.0:3001".parse().unwrap())
        .serve(app.into_make_service());

    if let Err(err) = server.await {
        error!("server crashed: {}", err);
    }
}


