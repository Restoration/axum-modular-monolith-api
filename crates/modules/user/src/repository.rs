use crate::model::{CreateUser, User};
use shared::AppError;
use sqlx::PgPool;

pub trait UserRepository: Send + Sync + Clone {
    fn find_all(&self, limit: i64, offset: i64) -> impl std::future::Future<Output = Result<Vec<User>, AppError>> + Send;
    fn find_by_id(&self, id: i64) -> impl std::future::Future<Output = Result<User, AppError>> + Send;
    fn create(&self, input: CreateUser) -> impl std::future::Future<Output = Result<User, AppError>> + Send;
}

#[derive(Clone)]
pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl UserRepository for PgUserRepository {
    async fn find_all(&self, limit: i64, offset: i64) -> Result<Vec<User>, AppError> {
        sqlx::query_as::<_, User>("SELECT id, name, created_at FROM users ORDER BY id LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("DB error: {e}");
                AppError::Internal("internal server error".into())
            })
    }

    async fn find_by_id(&self, id: i64) -> Result<User, AppError> {
        sqlx::query_as::<_, User>("SELECT id, name, created_at FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("DB error: {e}");
                AppError::Internal("internal server error".into())
            })?
            .ok_or_else(|| AppError::NotFound(format!("User with id {} not found", id)))
    }

    async fn create(&self, input: CreateUser) -> Result<User, AppError> {
        sqlx::query_as::<_, User>("INSERT INTO users (name) VALUES ($1) RETURNING id, name, created_at")
            .bind(&input.name)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("DB error: {e}");
                AppError::Internal("internal server error".into())
            })
    }
}
