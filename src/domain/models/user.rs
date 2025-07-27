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

#[derive(PartialEq, Eq, Clone, Copy, Hash, Default)]
pub struct UserId {
    inner: ObjectId,
}

impl UserId {
    pub fn new() -> Self {
        Self {
            inner: ObjectId::new(),
        }
    }
}

impl Serialize for UserId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for UserId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        ObjectId::deserialize(deserializer).map(|inner| Self { inner })
    }
}

impl Debug for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("UserId").field(&self.inner.to_hex()).finish()
    }
}

impl Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.inner.to_hex())
    }
}
