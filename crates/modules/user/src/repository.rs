use crate::model::{CreateUser, User};
use shared::AppError;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};

pub trait UserRepository: Send + Sync {
    fn find_all(&self) -> Result<Vec<User>, AppError>;
    fn find_by_id(&self, id: u64) -> Result<User, AppError>;
    fn create(&self, input: CreateUser) -> Result<User, AppError>;
}

#[derive(Clone)]
pub struct InMemoryUserRepository {
    store: Arc<RwLock<Vec<User>>>,
    next_id: Arc<AtomicU64>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        let store = vec![
            User { id: 1, name: "Alice".into() },
            User { id: 2, name: "Bob".into() },
        ];
        Self {
            store: Arc::new(RwLock::new(store)),
            next_id: Arc::new(AtomicU64::new(3)),
        }
    }
}

impl UserRepository for InMemoryUserRepository {
    fn find_all(&self) -> Result<Vec<User>, AppError> {
        let store = self.store.read().map_err(|e| AppError::Internal(e.to_string()))?;
        Ok(store.clone())
    }

    fn find_by_id(&self, id: u64) -> Result<User, AppError> {
        let store = self.store.read().map_err(|e| AppError::Internal(e.to_string()))?;
        store
            .iter()
            .find(|u| u.id == id)
            .cloned()
            .ok_or_else(|| AppError::NotFound(format!("User with id {} not found", id)))
    }

    fn create(&self, input: CreateUser) -> Result<User, AppError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let user = User { id, name: input.name };
        let mut store = self.store.write().map_err(|e| AppError::Internal(e.to_string()))?;
        store.push(user.clone());
        Ok(user)
    }
}
