use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tera::Tera;
use tower_http::services::ServeDir;
use tracing::info;

mod error;
mod handlers;

use crate::handlers::home::home;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            panic!("Parsing error(s): {}", e);
        }
    };
    let tera = Arc::new(tera);

    let app = Router::new()
        .route("/", get(home))
        .route("/education", get(handlers::contact::contact))
        .nest_service("/assets", ServeDir::new("assets"))
        .nest_service("/resume", ServeDir::new("resume"))
        .with_state(tera.clone())
        .fallback(handler_404);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler_404() -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::NOT_FOUND,
        "404 Not Found",
    )
}