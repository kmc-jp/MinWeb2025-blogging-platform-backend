use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;

use crate::domain::models::article_query::ArticleQuery;
use crate::domain::{
    models::{article::Article, user_name::UserName},
    repositorys::article_repository::ArticleRepository,
};

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Clone)]
pub struct ArticleUsecase<T: ArticleRepository + Clone> {
    repository: T,
}

impl<T: ArticleRepository + Clone> ArticleUsecase<T> {
    pub fn new(repository: T) -> Self {
        ArticleUsecase { repository }
    }
}

#[async_trait]
pub trait ArticleService {
    async fn get_articles(&self, skip: usize, limit: usize) -> Response;
    async fn get_article_by_id(&self, id: ObjectId) -> Response;
    async fn create_article(&self, title: String, author: UserName, content: String) -> Response;
    async fn update_article(
        &self,
        id: ObjectId,
        title: Option<String>,
        content: Option<String>,
    ) -> Response;
    async fn delete_article(&self, id: ObjectId) -> Response;
    async fn search_articles(&self, skip: usize, limit: usize, query: ArticleQuery) -> Response;
}

#[async_trait]
impl<T: ArticleRepository + Clone + Send + Sync> ArticleService for ArticleUsecase<T> {
    async fn get_articles(&self, skip: usize, limit: usize) -> Response {
        self.repository.get_articles(skip, limit).await
    }

    async fn get_article_by_id(&self, id: ObjectId) -> Response {
        self.repository.get_article_by_id(id).await
    }

    async fn create_article(&self, title: String, author: UserName, content: String) -> Response {
        self.repository.add_article(title, author, content).await
    }

    async fn update_article(
        &self,
        id: ObjectId,
        title: Option<String>,
        content: Option<String>,
    ) -> Response {
        self.repository.update_article(id, title, content).await
    }

    async fn delete_article(&self, id: ObjectId) -> Response {
        self.repository.delete_article(id).await
    }

    async fn search_articles(&self, skip: usize, limit: usize, query: ArticleQuery) -> Response {
        self.repository
            .get_articles_with_query(skip, limit, query)
            .await
    }
}
