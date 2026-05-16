use axum::{routing::get, Router};
use shared::AppState;

async fn health_check() -> &'static str {
    "ok"
}

pub fn router() -> Router<AppState> {
    Router::new().route("/health", get(health_check))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
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
    async fn health_check_returns_ok() {
        let app = test_app();

        let response = app
            .oneshot(Request::get("/health").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), 200);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"ok");
    }
}
