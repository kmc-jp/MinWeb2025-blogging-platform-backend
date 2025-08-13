use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::{
    bson::doc,
    Collection, Database,
};

use crate::domain::{
    models::{
        user::{User, UserId},
        user_name::UserName,
        user_service::UserServiceError,
    },
    repositorys::user_repository::UserRepository,
};

#[derive(Debug, Clone)]
pub struct MongodbUserRepository {
    database: Database,
    collection: Collection<User>,
}

impl MongodbUserRepository {
    pub fn new(database: Database) -> Self {
        let collection: Collection<User> = database.collection("users");
        Self { database, collection }
    }
}

#[async_trait]
impl UserRepository for MongodbUserRepository {
    async fn get_users(&self, skip: usize, limit: usize) -> Result<Vec<User>, UserServiceError> {
        let mut cursor = self
            .collection
            .find(doc! {})
            .skip(skip as u64)
            .limit(limit as i64)
            .await
            .map_err(UserServiceError::DatabaseError)?;

        let mut users: Vec<User> = Vec::new();
        while let Some(user) = cursor
            .try_next()
            .await
            .map_err(UserServiceError::DatabaseError)?
        {
            users.push(user);
        }
        Ok(users)
    }

    async fn get_user_by_id(&self, id: UserId) -> Result<User, UserServiceError> {
        let filter = doc! {"_id": bson::to_bson(&id).unwrap() };
        
        self
            .collection
            .find_one(filter)
            .await
            .map_err(UserServiceError::DatabaseError)?
            .ok_or(UserServiceError::UserNotFound)
    }

    async fn get_user_by_name(&self, name: &str) -> Result<User, UserServiceError> {
        let filter = doc! {"name.inner": name };

        self
            .collection
            .find_one(filter)
            .await
            .map_err(UserServiceError::DatabaseError)?
            .ok_or(UserServiceError::UserNotFound)
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
        // 重複チェック
        if self
            .collection
            .find_one(doc! {"name.inner": &name })
            .await
            .map_err(UserServiceError::DatabaseError)?
            .is_some()
        {
            return Err(UserServiceError::UserAlreadyExists);
        }

        let user = User {
            id: UserId::new(),
            name: UserName::new(name),
            display_name,
            intro,
            email,
            show_email,
            pw_hash,
            created_at: chrono::Utc::now(),
        };
        self.collection
            .insert_one(user.clone())
            .await
            .map_err(UserServiceError::DatabaseError)?;
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
        // 名前を変更する場合は重複チェック
        if let Some(ref new_name) = name {
            if self
                .collection
                .find_one(doc! {"name.inner": new_name })
                .await
                .map_err(UserServiceError::DatabaseError)?
                .is_some()
            {
                return Err(UserServiceError::UserAlreadyExists);
            }
        }

        let mut set_doc = doc! {};
        if let Some(new_name) = name {
            set_doc.insert("name", bson::to_bson(&UserName::new(new_name)).unwrap());
        }
        if let Some(v) = display_name { set_doc.insert("display_name", v); }
        if let Some(v) = intro { set_doc.insert("intro", v); }
        if let Some(v) = email { set_doc.insert("email", v); }
        if let Some(v) = show_email { set_doc.insert("show_email", v); }
        if let Some(v) = pw_hash { set_doc.insert("pw_hash", bson::to_bson(&v).unwrap()); }

        let mut update_doc = doc! {};
        if !set_doc.is_empty() {
            update_doc.insert("$set", set_doc);
        }

        let filter = doc! {"_id": bson::to_bson(&id).unwrap() };
        self.collection
            .update_one(filter.clone(), update_doc)
            .await
            .map_err(UserServiceError::DatabaseError)?;

        self
            .collection
            .find_one(filter)
            .await
            .map_err(UserServiceError::DatabaseError)?
            .ok_or(UserServiceError::UserNotFound)
    }

    async fn delete_user(&self, id: UserId) -> Result<(), UserServiceError> {
        let filter = doc! {"_id": bson::to_bson(&id).unwrap() };
        let result = self
            .collection
            .delete_one(filter)
            .await
            .map_err(UserServiceError::DatabaseError)?;
        if result.deleted_count == 1 { Ok(()) } else { Err(UserServiceError::UserNotFound) }
    }

    async fn validate_user_name(&self, name: &str) -> Result<UserName, UserServiceError> {
        if self
            .collection
            .find_one(doc! {"name.inner": name })
            .await
            .map_err(UserServiceError::DatabaseError)?
            .is_some()
        {
            Err(UserServiceError::UserAlreadyExists)
        } else {
            Ok(UserName::new(name.to_string()))
        }
    }
}
