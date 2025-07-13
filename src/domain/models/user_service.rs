use async_trait::async_trait;
use super::{user::User, user_name::UserName};

#[async_trait]
pub trait UserService {
    async fn get_users(
        &self,
        skip: usize,
        limit: usize,
    ) -> Result<Vec<User>, mongodb::error::Error>;
    async fn get_user_by_name(&self, name: &str) -> Result<Option<User>, mongodb::error::Error>;
    async fn create_user(
        &self,
        name: String,
        display_name: String,
        intro: String,
        email: String,
        show_email: bool,
        password: String,
    ) -> Result<User, mongodb::error::Error>;
    async fn update_user(
        &self,
        name: String,
        display_name: Option<String>,
        intro: Option<String>,
        email: Option<String>,
        show_email: Option<bool>,
        password: Option<String>,
    ) -> Result<User, mongodb::error::Error>;
    async fn delete_user(&self, name: &str) -> Result<(), mongodb::error::Error>;
    async fn validate_user_name(&self, name: &str) -> Result<UserName, mongodb::error::Error>;
}