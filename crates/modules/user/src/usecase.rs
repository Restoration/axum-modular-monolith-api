use crate::model::{CreateUser, User};
use crate::repository::UserRepository;
use shared::AppError;

#[derive(Clone)]
pub struct UserUsecase<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> UserUsecase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn list_users(&self) -> Result<Vec<User>, AppError> {
        self.repository.find_all()
    }

    pub fn get_user(&self, id: u64) -> Result<User, AppError> {
        self.repository.find_by_id(id)
    }

    pub fn create_user(&self, input: CreateUser) -> Result<User, AppError> {
        if input.name.is_empty() {
            return Err(AppError::BadRequest("name must not be empty".into()));
        }
        self.repository.create(input)
    }
}
