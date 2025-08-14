use std::fmt::{Debug, Display};

use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::models::user_name::UserName;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Article {
    #[serde(rename = "_id")]
    pub id: ArticleId,
    pub author: UserName,
    pub content: String,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Article {
    pub fn new_article(title: String, author: UserName, content: String) -> Self {
        let now = Utc::now();
        Article {
            id: ArticleId::new(),
            title,
            author,
            content,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Hash, Serialize, Deserialize)]
pub struct ArticleId(ObjectId);

impl ArticleId {
    pub fn new() -> Self {
        Self(ObjectId::new())
    }
    pub fn parse_str(s: &str) -> Result<Self, bson::oid::Error> {
        ObjectId::parse_str(s).map(|inner| ArticleId(inner))
    }
}

impl Debug for ArticleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ArticleId")
            .field(&self.0.to_hex())
            .finish()
    }
}
impl Display for ArticleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_hex())
    }
}
