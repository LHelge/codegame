mod models;
mod prelude;
mod repositories;
mod routes;

use prelude::*;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::net::Ipv4Addr;
use std::str::FromStr;
use tower_http::trace::TraceLayer;
use tracing::{debug, error, info};
use tracing_subscriber::{EnvFilter, fmt};

use crate::routes::routes;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    fmt().with_env_filter(EnvFilter::from_default_env()).init();

    if let Err(e) = app().await {
        error!("Application error: {}", e);
        std::process::exit(1);
    }
}

async fn app() -> Result<()> {
    let config = Config::from_env()?;
    debug!("Loaded configuration from environment: {:#?}", config);

    // Create database if it doesn't exist and connect
    let options = SqliteConnectOptions::from_str(&config.database_url)?.create_if_missing(true);
    let db = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    // Run migrations
    info!("Running database migrations...");
    sqlx::migrate!().run(&db).await?;
    info!("Migrations completed successfully");

    let state = AppState::new(config.clone(), db);

    let app = routes().with_state(state).layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind((Ipv4Addr::UNSPECIFIED, config.server_port))
        .await
        .expect("failed to bind address");

    info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.expect("server error");

    Ok(())
}
