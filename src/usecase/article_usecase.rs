use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;

use crate::domain::{models::{article::Article, article_service::ArticleService, user_name::UserName, article_service::ArticleServiceError}, repositorys::article_repository::ArticleRepository};
use crate::domain::models::article_query::ArticleQuery;

#[derive(Clone)]
pub struct ArticleUsecase<A: ArticleRepository + Clone> {
    repository: A,
}

impl<A: ArticleRepository + Clone> ArticleUsecase<A> {
    pub fn new(repository: A) -> Self {
        ArticleUsecase { repository }
    }
}

#[async_trait]
impl<A: ArticleRepository + Clone + Send + Sync> ArticleService for ArticleUsecase<A> {
    async fn get_articles(
        &self,
        skip: usize,
        limit: usize,
    ) -> Result<Vec<Article>, ArticleServiceError> {
        self.repository.get_articles(skip, limit).await
    }

    async fn get_article_by_id(
        &self,
        id: ObjectId,
    ) -> Result<Option<Article>, ArticleServiceError> {
        self.repository.get_article_by_id(id).await
    }

    async fn create_article(
        &self,
        title: String,
        author: UserName,
        content: String,
    ) -> Result<Article, ArticleServiceError> {
        self.repository.add_article(title, author, content).await
    }

    async fn update_article(
        &self,
        id: ObjectId,
        title: Option<String>,
        content: Option<String>,
    ) -> Result<Article, ArticleServiceError> {
        self.repository.update_article(id, title, content).await
    }

    async fn delete_article(&self, id: ObjectId) -> Result<(), ArticleServiceError> {
        self.repository.delete_article(id).await
    }

    async fn search_articles(
        &self,
        skip: usize,
        limit: usize,
        query: ArticleQuery,
    ) -> Result<Vec<Article>, ArticleServiceError> {
        self.repository
            .get_articles_with_query(skip, limit, query)
            .await
    }
}