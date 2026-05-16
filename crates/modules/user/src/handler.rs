use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use shared::{AppError, AppState};

use crate::model::CreateUser;
use crate::repository::PgUserRepository;
use crate::usecase::UserUsecase;

type Usecase = UserUsecase<PgUserRepository>;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/users", get(list_users).post(create_user))
        .route("/users/{id}", get(get_user))
}

async fn list_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<crate::model::User>>, AppError> {
    let usecase = build_usecase(&state);
    let users = usecase.list_users().await?;
    Ok(Json(users))
}

async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<crate::model::User>, AppError> {
    let usecase = build_usecase(&state);
    let user = usecase.get_user(id).await?;
    Ok(Json(user))
}

async fn create_user(
    State(state): State<AppState>,
    Json(input): Json<CreateUser>,
) -> Result<Json<crate::model::User>, AppError> {
    let usecase = build_usecase(&state);
    let user = usecase.create_user(input).await?;
    Ok(Json(user))
}

fn build_usecase(state: &AppState) -> Usecase {
    let repo = PgUserRepository::new(state.db.clone());
    UserUsecase::new(repo)
}
