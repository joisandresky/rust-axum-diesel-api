use std::sync::Arc;

use axum::{
    routing::get, Router,
};
use axum::routing::patch;
use envconfig::Envconfig;
use tokio::{net::TcpListener, sync::Mutex};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use dotenvy::dotenv;
use config::app_config::AppConfig;
use config::app_state::AppState;
use db_util::{establish_connection, run_pending_migrations};
use handler::user_handler;

mod config;
mod db_util;
mod schemas;
mod model;
mod custom_error;
mod dto;
mod mapper;
mod repository;
mod service;
mod handler;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app_cfg = AppConfig::init_from_env().unwrap();
    
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "keyboard_gb_api=debug".into())
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_pool = establish_connection(app_cfg.database_url().to_string());
    tracing::info!("database connection established at \"{}\"", app_cfg.database_url());
    run_pending_migrations(db_pool.get().unwrap());
    tracing::info!("Database migration run...");

    // initialize state
    let app_state = Arc::new(Mutex::new(AppState::new(db_pool)));

    let user_routes = Router::new()
        .route("/api/v1/users/set-verified/:id", patch(user_handler::set_verified_by_id))
        .route("/api/v1/users/:id", get(user_handler::find_by_id).delete(user_handler::delete_by_id))
        .route("/api/v1/users", get(user_handler::find_all).post(user_handler::create)
    );

    let app = Router::new()
        .merge(user_routes)
        .route("/", get(root))
        .with_state(app_state);

    let server_addr = format!("0.0.0.0:{}", app_cfg.app_port());
    let listener = TcpListener::bind(&server_addr).await.unwrap();
    
    tracing::debug!("listening on {}", server_addr);
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

async fn root() -> &'static str {
    tracing::info!("hey you got it here");

    "home woyy"
}