use async_trait::async_trait;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::domain::{
    models::{
        user::{User, UserId},
        user_name::UserName,
        user_service::UserServiceError,
    },
    repositorys::user_repository::UserRepository,
};

#[derive(Debug, Clone, Default)]
pub struct InMemoryUserRepository {
    users: Arc<RwLock<HashMap<UserId, User>>>,
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn get_users(&self, skip: usize, limit: usize) -> Result<Vec<User>, UserServiceError> {
        let users = self.users.read().unwrap();
        Ok(users.values().skip(skip).take(limit).cloned().collect())
    }
    async fn get_user_by_id(&self, id: UserId) -> Result<User, UserServiceError> {
        let users = self.users.read().unwrap();
        users
            .get(&id)
            .cloned()
            .ok_or_else(|| UserServiceError::UserNotFound)
    }
    async fn get_user_by_name(&self, name: &str) -> Result<User, UserServiceError> {
        let users = self.users.read().unwrap();
        users
            .values()
            .find(|user| user.name.as_str() == name)
            .cloned()
            .ok_or_else(|| UserServiceError::UserNotFound)
    }
    async fn add_user(
        &self,
        name: String,
        display_name: String,
        intro: String,
        email: String,
        show_email: bool,
        pw_hash: Vec<u8>,
    ) -> Result<User, UserServiceError> {
        let mut users = self.users.write().unwrap();
        let user_name = validate_user_name(&users, name)?;
        let id = UserId::new();
        let user = User {
            id,
            name: user_name,
            display_name,
            intro,
            email,
            show_email,
            pw_hash,
            created_at: chrono::Utc::now(),
        };
        users.insert(id, user.clone());
        Ok(user)
    }
    async fn update_user(
        &self,
        id: UserId,
        name: Option<String>,
        display_name: Option<String>,
        intro: Option<String>,
        email: Option<String>,
        show_email: Option<bool>,
        pw_hash: Option<Vec<u8>>,
    ) -> Result<User, UserServiceError> {
        let mut users = self.users.write().unwrap();
        let validated_name = name
            .map(|name| validate_user_name(&users, name))
            .transpose()?;
        // ユーザーの更新
        let user = users
            .get_mut(&id)
            .ok_or_else(|| UserServiceError::UserNotFound)?;

        if let Some(valid_name) = validated_name {
            user.name = valid_name;
        }
        if let Some(new_display_name) = display_name {
            user.display_name = new_display_name;
        }
        if let Some(new_intro) = intro {
            user.intro = new_intro;
        }
        if let Some(new_email) = email {
            user.email = new_email;
        }
        if let Some(new_show_email) = show_email {
            user.show_email = new_show_email;
        }
        if let Some(new_password) = pw_hash {
            user.pw_hash = new_password;
        }
        Ok(user.clone())
    }
    async fn delete_user(&self, id: UserId) -> Result<(), UserServiceError> {
        let mut users = self.users.write().unwrap();
        if users.remove(&id).is_some() {
            Ok(())
        } else {
            Err(UserServiceError::UserNotFound)
        }
    }
    async fn validate_user_name(&self, name: &str) -> Result<UserName, UserServiceError> {
        let users = self.users.read().unwrap();
        validate_user_name(&users, name.to_string())
    }
}

fn validate_user_name(
    users: &HashMap<UserId, User>,
    name: String,
) -> Result<UserName, UserServiceError> {
    //ユーザー名が重複していた場合はエラー
    if users.values().any(|user| user.name.as_str() == name) {
        Err(UserServiceError::UserAlreadyExists)
    } else {
        Ok(UserName::new(name))
    }
}
