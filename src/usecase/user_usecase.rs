use async_trait::async_trait;
use sha2::{Digest, Sha256};

use crate::domain::{
    models::{user::User, user_name::UserName, user_service::{UserService, UserServiceError}},
    repositorys::user_repository::UserRepository,
};

#[derive(Clone)]
pub struct UserUsecase<U: UserRepository + Clone> {
    repository: U,
}

impl<U: UserRepository + Clone> UserUsecase<U> {
    pub fn new(repository: U) -> Self {
        UserUsecase { repository }
    }
}

#[async_trait]
impl<U: UserRepository + Clone + Send + Sync> UserService for UserUsecase<U> {
    async fn get_users(
        &self,
        skip: usize,
        limit: usize,
    ) -> Result<Vec<User>, UserServiceError> {
        self.repository.get_users(skip, limit).await
    }

    async fn get_user_by_name(&self, name: &str) -> Result<User, UserServiceError> {
        self.repository.get_user_by_name(name).await
    }

    async fn create_user(
        &self,
        name: String,
        display_name: String,
        intro: String,
        email: String,
        show_email: bool,
        password: String,
    ) -> Result<User, UserServiceError> {
        self.repository
            .add_user(name, display_name, intro, email, show_email, Sha256::digest(password.as_bytes()).to_vec())
            .await
    }

    async fn update_user(
        &self,
        name: String,
        display_name: Option<String>,
        intro: Option<String>,
        email: Option<String>,
        show_email: Option<bool>,
        password: Option<String>,
    ) -> Result<User, UserServiceError> {
        let user = self.repository.get_user_by_name(&name).await?;
        self.repository
            .update_user(
                user.id,
                None, // user name is not updatable
                display_name,
                intro,
                email,
                show_email,
                password.and_then(|pw| Some(Sha256::digest(pw.as_bytes()).to_vec())),
            )
            .await
    }

    async fn delete_user(&self, name: &str) -> Result<(), UserServiceError> {
        let user = self.repository.get_user_by_name(name).await?;
        self.repository.delete_user(user.id).await
    }

    async fn validate_user_name(
        &self,
        name: &str,
    ) -> Result<UserName, UserServiceError> {
        self.repository.validate_user_name(name).await
    }
}
