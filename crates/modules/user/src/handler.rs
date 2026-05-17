use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use shared::{AppError, AppState};

use crate::model::{CreateUser, Pagination};
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
    Query(params): Query<Pagination>,
) -> Result<Json<Vec<crate::model::User>>, AppError> {
    let limit = params.limit.unwrap_or(20).min(100);
    let offset = params.offset.unwrap_or(0);
    let usecase = build_usecase(&state);
    let users = usecase.list_users(limit, offset).await?;
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
) -> Result<(StatusCode, Json<crate::model::User>), AppError> {
    let usecase = build_usecase(&state);
    let user = usecase.create_user(input).await?;
    Ok((StatusCode::CREATED, Json(user)))
}

fn build_usecase(state: &AppState) -> Usecase {
    let repo = PgUserRepository::new(state.db.clone());
    UserUsecase::new(repo)
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::Router;
    use http::Request;
    use http_body_util::BodyExt;
    use shared::AppState;
    use sqlx::postgres::PgPoolOptions;
    use tower::ServiceExt;

    fn test_app() -> Router {
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect_lazy("postgres://localhost/dummy")
            .unwrap();
        router().with_state(AppState::new(pool))
    }

    #[tokio::test]
    async fn route_not_found_returns_404() {
        let app = test_app();

        let response = app
            .oneshot(Request::get("/nonexistent").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), 404);
    }

    #[tokio::test]
    async fn create_user_invalid_content_type_returns_415() {
        let app = test_app();

        let response = app
            .oneshot(
                Request::post("/users")
                    .body(Body::from("not json"))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), 415);
    }

    #[tokio::test]
    async fn create_user_invalid_json_returns_422() {
        let app = test_app();

        let response = app
            .oneshot(
                Request::post("/users")
                    .header("content-type", "application/json")
                    .body(Body::from("{}"))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), 422);
    }
}
