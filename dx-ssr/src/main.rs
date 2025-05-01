use axum::{routing::get, Router};
use dx_ssr::handler;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:9527").await?;

    let app = Router::new()
        .route("/", get(handler::list))
        .route("/user/{id}", get(handler::detail));

    axum::serve(listener, app).await?;

    Ok(())
}
