use std::fmt::{Debug, Display};

use axum_login::AuthUser;
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::models::user_name::UserName;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: UserId,
    pub name: UserName, // ユーザー名
    pub display_name: String,
    pub intro: String,
    pub email: String,
    pub show_email: bool,
    pub pw_hash: Vec<u8>, // ハッシュ化されたパスワード
    pub created_at: DateTime<Utc>,
}

impl AuthUser for User {
    type Id = UserId;

    fn id(&self) -> Self::Id {
        self.id
    }
    fn session_auth_hash(&self) -> &[u8] {
        &self.pw_hash
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Default, Serialize, Deserialize)]
pub struct UserId(ObjectId);

impl UserId {
    pub fn new() -> Self {
        Self(ObjectId::new())
    }
}

impl Debug for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("UserId").field(&self.0.to_hex()).finish()
    }
}

impl Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_hex())
    }
}
