mod config;
mod errors;
mod middleware;
mod models;
mod routes;
mod schema;
use crate::config::Config;
use crate::errors::AppError;
use crate::middleware::auth::{
    check_authorization, validate_ownership_of_block, validate_ownership_of_block_and_course,
    validate_ownership_of_block_course_component,
};
use axum::http::header::AUTHORIZATION;
use axum::http::StatusCode;
use axum::{
    routing::{get, post},
    Router,
};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use log::{error, info};
use routes::api;
use std::env;
use std::iter::once;
use std::sync::Arc;
use tower_http::add_extension::AddExtensionLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::sensitive_headers::SetSensitiveRequestHeadersLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::filter::Targets;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub struct ServerState {
    db_pool: Pool<ConnectionManager<PgConnection>>,
    config: Config,
}

impl ServerState {
    fn get_db_con(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>, AppError> {
        self.db_pool.get().map_err(|_e| AppError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            description: "Unable to connect to the Gradekeeper database.".to_string(),
        })
    }
}

pub fn run_db_migrations(conn: &mut PooledConnection<ConnectionManager<PgConnection>>) {
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Could not run migrations");
}

#[tokio::main]
async fn main() {
    let config = Config::init_from_env();
    let filter = Targets::new()
        .with_target(
            "tower_http::trace::on_response",
            tracing_subscriber::filter::LevelFilter::TRACE,
        )
        .with_default(tracing_subscriber::filter::LevelFilter::INFO);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();
    info!(
        "Gradekeeper server v{} starting on {}/{}/{}",
        env!("CARGO_PKG_VERSION"),
        env::consts::FAMILY,
        env::consts::OS,
        env::consts::ARCH
    );
    info!(
        "Allowed redirect URLs: {}",
        config
            .permitted_redirect_urls
            .clone()
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );

    let initial_state = ServerState {
        db_pool: Pool::builder()
            .test_on_check_out(true)
            .build(ConnectionManager::<PgConnection>::new(&config.database_url))
            .expect("Could not build connection pool"),
        config,
    };

    run_db_migrations(
        &mut initial_state
            .get_db_con()
            .expect("Could not connect to database."),
    );

    let app = Router::new()
        // Users
        .route("/api/users/me", get(api::users::me::get_user))
        .route("/api/users/me", post(api::users::me::update_user))
        .route("/api/users/me", axum::routing::delete(api::users::me::delete_user))
        // Blocks
        .route("/api/block/create", post(api::block::create::create_block))
        .route("/api/block/:block_id", axum::routing::delete(api::block::block_id::delete_block))
        .route("/api/block/:block_id/import", post(api::block::_block_id::import::import_course)
            .layer(axum::middleware::from_fn(validate_ownership_of_block)))

        // Courses
        .route("/api/block/:block_id/course/create", post(api::block::_block_id::course::create::create_course)
            .layer(axum::middleware::from_fn(validate_ownership_of_block)))
        .route("/api/block/:block_id/course/:course_id", get(api::block::_block_id::course::course_id::get_course)
            .layer(axum::middleware::from_fn(validate_ownership_of_block_and_course)))
        .route("/api/block/:block_id/course/:course_id", axum::routing::delete(api::block::_block_id::course::course_id::delete_course)
            .layer(axum::middleware::from_fn(validate_ownership_of_block_and_course)))
        .route("/api/block/:block_id/course/:course_id", post(api::block::_block_id::course::course_id::update_course)
            .layer(axum::middleware::from_fn(validate_ownership_of_block_and_course)))

        // Components
        .route("/api/block/:block_id/course/:course_id/component/:component_id",
               post(api::block::_block_id::course::_course_id::component::component_id::update_course_component)
                   .layer(axum::middleware::from_fn(validate_ownership_of_block_course_component))
        )
        .layer(axum::middleware::from_fn(check_authorization))
        // End authorised section

        // Login
        .route("/api/auth/login", get(routes::api::auth::login::handle_login_request))
        .route("/api/auth/callback", get(routes::api::auth::callback::handle_auth_callback))
        // Final Layer - CORS
        .layer(SetSensitiveRequestHeadersLayer::new(once(AUTHORIZATION)))
        .layer(CorsLayer::new().allow_origin(Any).allow_headers([AUTHORIZATION]))
        .layer(TraceLayer::new_for_http())
        .layer(AddExtensionLayer::new(Arc::new(initial_state)));

    let server =
        axum::Server::bind(&"0.0.0.0:3000".parse().unwrap()).serve(app.into_make_service());

    if let Err(err) = server.await {
        error!("server crashed: {}", err);
    }
}
