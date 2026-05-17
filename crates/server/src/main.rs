use axum::Router;
use sqlx::postgres::PgPoolOptions;
use shared::AppState;
use tokio::signal;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("../../migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let state = AppState::new(pool);

    let cors = match std::env::var("CORS_ORIGIN") {
        Ok(origin) => CorsLayer::new()
            .allow_origin(origin.parse::<axum::http::HeaderValue>().expect("Invalid CORS_ORIGIN"))
            .allow_methods(Any)
            .allow_headers(Any),
        Err(_) => CorsLayer::permissive(),
    };

    let app = Router::new()
        .merge(module_health::router())
        .merge(module_user::router())
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed to bind to port 8080");

    tracing::info!("Server listening on 0.0.0.0:8080");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Server error");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {}
        _ = terminate => {}
    }

    tracing::info!("Shutdown signal received, starting graceful shutdown");
}
