use async_trait::async_trait;

use crate::domain::models::article::ArticleId;

use super::{article::Article, article_query::ArticleQuery, user_name::UserName};
#[async_trait]
pub trait ArticleService {
    async fn get_articles(
        &self,
        skip: usize,
        limit: usize,
    ) -> Result<Vec<Article>, ArticleServiceError>;
    async fn get_article_by_id(&self, id: ArticleId) -> Result<Article, ArticleServiceError>;
    async fn create_article(
        &self,
        title: String,
        author: UserName,
        content: String,
    ) -> Result<Article, ArticleServiceError>;
    async fn update_article(
        &self,
        id: ArticleId,
        title: Option<String>,
        content: Option<String>,
    ) -> Result<Article, ArticleServiceError>;
    async fn delete_article(&self, id: ArticleId) -> Result<(), ArticleServiceError>;
    async fn search_articles(
        &self,
        skip: usize,
        limit: usize,
        query: ArticleQuery,
    ) -> Result<Vec<Article>, ArticleServiceError>;
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ArticleServiceError {
    #[error("Article not found")]
    ArticleNotFound,
    #[error("Article already exists")]
    ArticleAlreadyExists,
    #[error("Database error")]
    DatabaseError(#[from] mongodb::error::Error),
}
