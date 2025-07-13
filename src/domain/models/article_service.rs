use async_trait::async_trait;
use super::{article::Article, user_name::UserName, article_query::ArticleQuery};
use mongodb::bson::oid::ObjectId;

#[async_trait]
pub trait ArticleService {
    async fn get_articles(
        &self,
        skip: usize,
        limit: usize,
    ) -> Result<Vec<Article>, mongodb::error::Error>;
    async fn get_article_by_id(
        &self,
        id: ObjectId,
    ) -> Result<Option<Article>, mongodb::error::Error>;
    async fn create_article(
        &self,
        title: String,
        author: UserName,
        content: String,
    ) -> Result<Article, mongodb::error::Error>;
    async fn update_article(
        &self,
        id: ObjectId,
        title: Option<String>,
        content: Option<String>,
    ) -> Result<Article, mongodb::error::Error>;
    async fn delete_article(&self, id: ObjectId) -> Result<(), mongodb::error::Error>;
    async fn search_articles(
        &self,
        skip: usize,
        limit: usize,
        query: ArticleQuery,
    ) -> Result<Vec<Article>, mongodb::error::Error>;
}