mod handler;
pub mod model;
pub mod repository;
pub mod usecase;

use axum::Router;
use shared::AppState;

pub fn router() -> Router<AppState> {
    handler::router()
}
