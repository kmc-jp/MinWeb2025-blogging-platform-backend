use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use async_trait::async_trait;
use bson::oid::ObjectId;
use chrono::Utc;
use mongodb::error::Error;

use crate::domain::{
    models::{article::Article, article_query::ArticleQuery, user_name::UserName},
    repositorys::article_repository::ArticleRepository,
};

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

/// 上の実装が完成するまでの仮の実装
/// この実装は、記事をメモリ上に保存するだけの簡易的なものです。
/// サーバーが再起動すると、保存された記事は失われます。
#[derive(Clone, Default, Debug)]
pub struct InMemoryArticleRepository {
    articles: Arc<RwLock<HashMap<ObjectId, Article>>>,
}

#[async_trait]
impl ArticleRepository for InMemoryArticleRepository {
    async fn get_articles(&self, skip: usize, limit: usize) -> Response {
        let articles = self.articles.read().unwrap();
        let mut articles_vec: Vec<Article> = articles.values().cloned().collect();
        articles_vec.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        (
            StatusCode::OK,
            Json(
                articles_vec
                    .into_iter()
                    .skip(skip)
                    .take(limit)
                    .collect::<Vec<Article>>(),
            ),
        )
            .into_response()
    }

    async fn get_article_by_id(&self, id: ObjectId) -> Response {
        let articles = self.articles.read().unwrap();
        match articles.get(&id) {
            Some(article) => (StatusCode::OK, Json(article.clone())).into_response(),
            None => (StatusCode::NOT_FOUND, "Article not found").into_response(),
        }
    }

    async fn add_article(&self, title: String, author: UserName, content: String) -> Response {
        let mut articles = self.articles.write().unwrap();
        let id = ObjectId::new();
        let now = Utc::now();
        let article = Article {
            id,
            title,
            author,
            content,
            created_at: now,
            updated_at: now,
        };
        articles.insert(id, article.clone());
        (StatusCode::CREATED, Json(article)).into_response()
    }
    async fn update_article(
        &self,
        id: ObjectId,
        title: Option<String>,
        content: Option<String>,
    ) -> Response {
        let mut articles = self.articles.write().unwrap();
        if let Some(article) = articles.get_mut(&id) {
            if let Some(new_title) = title {
                article.title = new_title;
            }
            if let Some(new_content) = content {
                article.content = new_content;
            }
            article.updated_at = Utc::now();
            (StatusCode::OK, Json(article.clone())).into_response()
        } else {
            (StatusCode::NOT_FOUND, "Article not found").into_response()
        }
    }
    async fn delete_article(&self, id: ObjectId) -> Response {
        let mut articles = self.articles.write().unwrap();
        match articles.remove(&id) {
            Some(_) => (StatusCode::OK, "Article deleted").into_response(),
            None => (StatusCode::NOT_FOUND, "Article not found").into_response(),
        }
    }
    async fn get_articles_with_query(
        &self,
        skip: usize,
        limit: usize,
        query: ArticleQuery,
    ) -> Response {
        let articles = self.articles.read().unwrap();
        let mut filtered_articles: Vec<Article> = articles
            .values()
            .filter(|article| {
                query
                    .title
                    .as_ref()
                    .map_or(true, |title| article.title.contains(title))
                    && query
                        .author
                        .as_ref()
                        .map_or(true, |author| article.author.to_string() == *author)
            })
            .cloned()
            .collect();

        filtered_articles.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        (
            StatusCode::OK,
            Json(
                filtered_articles
                    .into_iter()
                    .skip(skip)
                    .take(limit)
                    .collect::<Vec<Article>>(),
            ),
        )
            .into_response()
    }
}
