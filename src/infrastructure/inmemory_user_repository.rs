use std::{collections::HashMap, sync::{Arc, RwLock}};
use mongodb::error::Error;
use bson::oid::ObjectId;
use async_trait::async_trait;

use crate::domain::{models::{user::User, user_name::UserName}, repositorys::user_repository::UserRepository};

#[derive(Debug, Clone, Default)]
pub struct InMemoryUserRepository {
    users: Arc<RwLock<HashMap<ObjectId, User>>>,
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn get_users(&self, skip: usize, limit: usize) -> Result<Vec<User>, Error> {
        let users = self.users.read().unwrap();
        let users_vec: Vec<User> = users.values().cloned().collect();
        Ok(users_vec.into_iter().skip(skip).take(limit).collect())
    }
    async fn get_user_by_id(&self, id: ObjectId) -> Result<Option<User>, Error> {
        let users = self.users.read().unwrap();
        Ok(users.get(&id).cloned())
    }
    async fn get_user_by_name(&self, name: &str) -> Result<Option<User>, Error> {
        let users = self.users.read().unwrap();
        Ok(users.values().find(|user| user.name.to_string() == name).cloned())
    }
    async fn add_user(&self, name: String, display_name: String, intro: String, email: String, show_email: bool, pw_hash: Vec<u8>) -> Result<User, Error> {
        // ユーザー名の重複チェック
        let mut users = self.users.write().unwrap();
        if users.values().any(|user| user.name.to_string() == name) {
            return Err(Error::custom("User name already exists"));
        }
        let user_name = UserName::new(name.to_string());
        let id = ObjectId::new();
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
    async fn update_user(&self, id: ObjectId, name: Option<String>, display_name: Option<String>, intro: Option<String>, email: Option<String>, show_email: Option<bool>, pw_hash: Option<Vec<u8>>) -> Result<User, Error> {
        // First, validate the new name if provided, before locking
        let validated_name = if let Some(ref new_name) = name {
            Some(self.validate_user_name(new_name).await.map_err(|e| Error::custom(e))?)
        } else {
            None
        };

        let mut users = self.users.write().unwrap();
        if let Some(user) = users.get_mut(&id) {
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
        } else {
            Err(Error::custom("User not found"))
        }
    }
    async fn delete_user(&self, id: ObjectId) -> Result<(), Error> {
        let mut users = self.users.write().unwrap();
        if users.remove(&id).is_some() {
            Ok(())
        } else {
            Err(Error::custom("User not found"))
        }
    }
    async fn validate_user_name(&self, name: &str) -> Result<UserName, Error> {
        let users = self.users.read().unwrap();
        if users.values().any(|user| user.name.to_string() == name) {
            Err(Error::custom("User name already exists"))
        } else {
            Ok(UserName::new(name.to_string()))
        }
    }
}