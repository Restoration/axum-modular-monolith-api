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

    pub async fn list_users(&self) -> Result<Vec<User>, AppError> {
        self.repository.find_all().await
    }

    pub async fn get_user(&self, id: i64) -> Result<User, AppError> {
        self.repository.find_by_id(id).await
    }

    pub async fn create_user(&self, input: CreateUser) -> Result<User, AppError> {
        if input.name.is_empty() {
            return Err(AppError::BadRequest("name must not be empty".into()));
        }
        self.repository.create(input).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[derive(Clone)]
    struct MockUserRepository {
        users: Arc<Mutex<Vec<User>>>,
    }

    impl MockUserRepository {
        fn new(users: Vec<User>) -> Self {
            Self {
                users: Arc::new(Mutex::new(users)),
            }
        }
    }

    impl UserRepository for MockUserRepository {
        async fn find_all(&self) -> Result<Vec<User>, AppError> {
            Ok(self.users.lock().unwrap().clone())
        }

        async fn find_by_id(&self, id: i64) -> Result<User, AppError> {
            self.users
                .lock()
                .unwrap()
                .iter()
                .find(|u| u.id == id)
                .cloned()
                .ok_or_else(|| AppError::NotFound(format!("User with id {} not found", id)))
        }

        async fn create(&self, input: CreateUser) -> Result<User, AppError> {
            let mut users = self.users.lock().unwrap();
            let id = users.len() as i64 + 1;
            let user = User {
                id,
                name: input.name,
            };
            users.push(user.clone());
            Ok(user)
        }
    }

    fn setup(users: Vec<User>) -> UserUsecase<MockUserRepository> {
        UserUsecase::new(MockUserRepository::new(users))
    }

    fn sample_users() -> Vec<User> {
        vec![
            User { id: 1, name: "Alice".into() },
            User { id: 2, name: "Bob".into() },
        ]
    }

    #[tokio::test]
    async fn list_users_returns_all() {
        let usecase = setup(sample_users());
        let users = usecase.list_users().await.unwrap();
        assert_eq!(users.len(), 2);
        assert_eq!(users[0].name, "Alice");
        assert_eq!(users[1].name, "Bob");
    }

    #[tokio::test]
    async fn list_users_empty() {
        let usecase = setup(vec![]);
        let users = usecase.list_users().await.unwrap();
        assert!(users.is_empty());
    }

    #[tokio::test]
    async fn get_user_found() {
        let usecase = setup(sample_users());
        let user = usecase.get_user(1).await.unwrap();
        assert_eq!(user.name, "Alice");
    }

    #[tokio::test]
    async fn get_user_not_found() {
        let usecase = setup(sample_users());
        let err = usecase.get_user(999).await.unwrap_err();
        assert!(matches!(err, AppError::NotFound(_)));
    }

    #[tokio::test]
    async fn create_user_success() {
        let usecase = setup(vec![]);
        let user = usecase
            .create_user(CreateUser { name: "Charlie".into() })
            .await
            .unwrap();
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "Charlie");
    }

    #[tokio::test]
    async fn create_user_empty_name_rejected() {
        let usecase = setup(vec![]);
        let err = usecase
            .create_user(CreateUser { name: "".into() })
            .await
            .unwrap_err();
        assert!(matches!(err, AppError::BadRequest(_)));
    }
}
