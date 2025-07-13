use async_trait::async_trait;
use sha2::{Digest, Sha256};

use crate::domain::{
    models::{user::User, user_name::UserName},
    repositorys::user_repository::UserRepository,
};

#[derive(Clone)]
pub struct UserUsecase<T: UserRepository + Clone> {
    repository: T,
}

impl<T: UserRepository + Clone> UserUsecase<T> {
    pub fn new(repository: T) -> Self {
        UserUsecase { repository }
    }
}

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

#[async_trait]
impl<T: UserRepository + Clone + Send + Sync> UserService for UserUsecase<T> {
    async fn get_users(
        &self,
        skip: usize,
        limit: usize,
    ) -> Result<Vec<User>, mongodb::error::Error> {
        self.repository.get_users(skip, limit).await
    }

    async fn get_user_by_name(&self, name: &str) -> Result<Option<User>, mongodb::error::Error> {
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
    ) -> Result<User, mongodb::error::Error> {
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
    ) -> Result<User, mongodb::error::Error> {
        let user = self.repository.get_user_by_name(&name).await?.ok_or_else(|| mongodb::error::Error::from(
            std::io::Error::new(std::io::ErrorKind::NotFound, "User not found"),
        ))?;
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

    async fn delete_user(&self, name: &str) -> Result<(), mongodb::error::Error> {
        let user = self.repository.get_user_by_name(name).await?.ok_or_else(|| mongodb::error::Error::from(
            std::io::Error::new(std::io::ErrorKind::NotFound, "User not found"),
        ))?;
        self.repository.delete_user(user.id).await
    }

    async fn validate_user_name(
        &self,
        name: &str,
    ) -> Result<UserName, mongodb::error::Error> {
        self.repository.validate_user_name(name).await
    }
}
