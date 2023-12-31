use std::error::Error;
use std::sync::Arc;

use axum::{
    routing::get, Router,
};
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
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let app_cfg = AppConfig::init_from_env()?;
    
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "keyboard_gb_api=debug".into())
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_pool = establish_connection(app_cfg.database_url().to_string());
    tracing::info!("database connection established at \"{}\"", app_cfg.database_url());
    run_pending_migrations(db_pool.get()?);
    tracing::info!("Database migration run...");

    let redis_client = redis::Client::open(app_cfg.redis_url())?;

    // initialize state
    let app_state = Arc::new(Mutex::new(AppState::new(db_pool, redis_client)?));

    let app = routes()
        .with_state(app_state);

    let server_addr = format!("0.0.0.0:{}", app_cfg.app_port());
    let listener = TcpListener::bind(&server_addr).await?;
    
    tracing::debug!("listening on {}", server_addr);
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

async fn root() -> &'static str {
    tracing::info!("hey you got it here");

    "Welcome to THE API 😂"
}

fn routes() -> Router<Arc<Mutex<AppState>>> {
    Router::new()
        .merge(user_handler::routes())
        .route("/", get(root))
}