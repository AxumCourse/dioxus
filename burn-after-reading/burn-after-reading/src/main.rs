use std::sync::Arc;

use axum::{routing::post, Router};
use burn_after_reading::{config::Config, frontend, AppState};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cfg = Config::from_env().unwrap();

    let listener = TcpListener::bind(&cfg.web_addr).await.unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(cfg.database_max_conns)
        .connect(&cfg.database_url)
        .await
        .unwrap();
    let state = Arc::new(AppState {
        pool,
        cfg: Arc::new(cfg),
    });

    let app = Router::new()
        .route("/message", post(frontend::create_message))
        .route("/message/view", post(frontend::read_message))
        .with_state(state);

    tracing::info!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
