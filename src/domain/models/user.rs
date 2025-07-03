use std::fmt;

use axum_login::AuthUser;
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: UserName,
    pub display_name: String,
    pub introduction: String,
    pub email: String,
    pub show_email: bool,
    pub pw_hash: Vec<u8>, // ハッシュ化されたパスワード
    pub created_at: DateTime<Utc>
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserName {
    inner: String
}

impl fmt::Display for UserName{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.inner)
    }
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