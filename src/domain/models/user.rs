use axum_login::AuthUser;
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::models::user_name::UserName;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: UserName, // ユーザー名
    pub display_name: String,
    pub intro: String,
    pub email: String,
    pub show_email: bool,
    pub pw_hash: Vec<u8>, // ハッシュ化されたパスワード
    pub created_at: DateTime<Utc>,
}

impl AuthUser for User {
    type Id = ObjectId;

    fn id(&self) -> Self::Id {
        self.id
    }
    fn session_auth_hash(&self) -> &[u8] {
        &self.pw_hash
    }
}
