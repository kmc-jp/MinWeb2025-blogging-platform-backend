use async_trait::async_trait;

use super::{user::User, user_name::UserName};

#[async_trait]
pub trait UserService {
    async fn get_users(&self, skip: usize, limit: usize) -> Result<Vec<User>, UserServiceError>;
    async fn get_user_by_name(&self, name: &str) -> Result<User, UserServiceError>;
    async fn create_user(
        &self,
        name: String,
        display_name: String,
        intro: String,
        email: String,
        show_email: bool,
        password: String,
    ) -> Result<User, UserServiceError>;
    async fn update_user(
        &self,
        name: String,
        display_name: Option<String>,
        intro: Option<String>,
        email: Option<String>,
        show_email: Option<bool>,
        password: Option<String>,
    ) -> Result<User, UserServiceError>;
    async fn delete_user(&self, name: &str) -> Result<(), UserServiceError>;
    async fn validate_user_name(&self, name: &str) -> Result<UserName, UserServiceError>;
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum UserServiceError {
    #[error("User not found")]
    UserNotFound,
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Database error: {0}")]
    DatabaseError(mongodb::error::Error),
}
