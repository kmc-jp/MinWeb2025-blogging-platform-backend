use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::domain::models::user_name::UserName;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Article {
    #[serde(rename = "_id")]
    id: ObjectId,
    author: UserName,
    content: String,
    title: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}