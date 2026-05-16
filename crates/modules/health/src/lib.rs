use axum::{routing::get, Router};
use shared::AppState;

async fn health_check() -> &'static str {
    "ok"
}

pub fn router() -> Router<AppState> {
    Router::new().route("/health", get(health_check))
}
