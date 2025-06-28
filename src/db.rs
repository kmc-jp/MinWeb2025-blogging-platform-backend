use mongodb::{Client, error::Result, Collection};
use bson::doc;

use crate::domain::models::{article::Article, user_name::UserName};

pub async fn get_blog_text(uri: &str, user_name: &UserName, title: &str) -> Result<Option<Article>> {
    let collection = get_blog_collection(uri, user_name).await?;
    let filter = doc! { "title": title };
    collection.find_one(filter).await
}
async fn get_blog_collection(uri: &str, user_name: &UserName) -> Result<Collection<Article>> {
    let client = Client::with_uri_str(uri).await?;
    let database = client.database("blog_data");
    Ok(database.collection::<Article>(&format!("user_name: {user_name}")))
}