use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use shared::{AppError, AppState};

use crate::model::CreateUser;
use crate::repository::InMemoryUserRepository;
use crate::usecase::UserUsecase;

type Usecase = UserUsecase<InMemoryUserRepository>;

pub fn router() -> Router<AppState> {
    let usecase = UserUsecase::new(InMemoryUserRepository::new());

    Router::new()
        .route("/users", get(list_users).post(create_user))
        .route("/users/{id}", get(get_user))
        .with_state(usecase)
}

async fn list_users(
    State(usecase): State<Usecase>,
) -> Result<Json<Vec<crate::model::User>>, AppError> {
    let users = usecase.list_users()?;
    Ok(Json(users))
}

async fn get_user(
    State(usecase): State<Usecase>,
    Path(id): Path<u64>,
) -> Result<Json<crate::model::User>, AppError> {
    let user = usecase.get_user(id)?;
    Ok(Json(user))
}

async fn create_user(
    State(usecase): State<Usecase>,
    Json(input): Json<CreateUser>,
) -> Result<Json<crate::model::User>, AppError> {
    let user = usecase.create_user(input)?;
    Ok(Json(user))
}
