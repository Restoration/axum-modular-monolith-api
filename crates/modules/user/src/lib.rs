mod handler;
mod model;
mod repository;
mod usecase;

use axum::Router;
use shared::AppState;

pub fn router() -> Router<AppState> {
    handler::router()
}
