mod models;
mod prelude;
mod routes;

use prelude::*;
use sqlx::SqlitePool;
use std::net::Ipv4Addr;
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

    let db = SqlitePool::connect(&config.database_url).await?;

    let state = AppState::new(config.clone(), db);

    let app = routes().with_state(state);

    let listener = tokio::net::TcpListener::bind((Ipv4Addr::UNSPECIFIED, config.server_port))
        .await
        .expect("failed to bind address");

    info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.expect("server error");

    Ok(())
}
