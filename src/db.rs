use serde::{Deserialize, Serialize};
use mongodb::{bson::Uuid, Client, error::Result};
use bson::doc;

pub async fn get_blog_text(uri: &str, user_name: &str, title: &str) -> Result<Option<BlogText>> {
    let client = Client::with_uri_str(uri).await?;
    let database = client.database("blog_data");
    let collection = database.collection::<BlogText>(user_name);
    let filter = doc! { "title": title };
    collection.find_one(filter).await
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogText {
    #[serde(rename = "_id")]
    id: Uuid,
    title: String,
    content: String
}