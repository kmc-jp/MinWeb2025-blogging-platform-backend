use serde::{Deserialize, Serialize};
use mongodb::{Client, error::Result, Collection};
use bson::{doc, oid::ObjectId};

pub async fn get_blog_text(uri: &str, user_name: &str, title: &str) -> Result<Option<BlogText>> {
    let collection = get_blog_collection(uri, user_name).await?;
    let filter = doc! { "title": title };
    collection.find_one(filter).await
}
async fn get_blog_collection(uri: &str, user_name: &str) -> Result<Collection<BlogText>> {
    let client = Client::with_uri_str(uri).await?;
    let database = client.database("blog_data");
    Ok(database.collection::<BlogText>(&format!("user_name: {user_name}")))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogText {
    #[serde(rename = "_id")]
    id: ObjectId,
    title: String,
    content: String
}