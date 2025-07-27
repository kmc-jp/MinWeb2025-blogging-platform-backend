use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::models::user_name::UserName;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Article {
    #[serde(rename = "_id")]
    pub id: ObjectId,
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
            id: ObjectId::new(),
            title,
            author,
            content,
            created_at: now,
            updated_at: now,
        }
    }
}
