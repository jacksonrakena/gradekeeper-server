mod models;
mod schema;
mod routes;
mod middleware;


use std::env;
use std::iter::{once};
use std::sync::Arc;
use axum::{Router, routing::{get, post},};

use axum::http::{Method};
use axum::http::header::AUTHORIZATION;

use diesel::{MysqlConnection};

use dotenvy::dotenv;
use log::{info, LevelFilter};

use pretty_env_logger::env_logger::{Builder, Target};



use diesel::prelude::*;

use diesel::r2d2::{ConnectionManager, Pool};

use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tower_http::{add_extension::AddExtensionLayer};
use tower_http::sensitive_headers::SetSensitiveRequestHeadersLayer;
use crate::middleware::auth::check_authorization;










pub struct ServerState {
    db_pool: Pool<ConnectionManager<MysqlConnection>>
}

#[tokio::main]
async fn main() {
    dotenv().unwrap();
    Builder::new()
        .filter_module(stringify!(gk_server), LevelFilter::Info)
        .target(Target::Stdout)
        .init();
    info!("Gradekeeper Nova server starting");

    let initial_state = ServerState {
        db_pool: Pool::builder()
            .test_on_check_out(true)
            .build(ConnectionManager::<MysqlConnection>::new(env::var("DATABASE_URL").expect("DATABASE_URL must be set")))
            .expect("Could not build connection pool")
    };

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
        .layer(AddExtensionLayer::new(Arc::new(initial_state)));

    axum::Server::bind(&"0.0.0.0:3001".parse().unwrap()).serve(app.into_make_service()).await.unwrap();
    println!("online");
}


