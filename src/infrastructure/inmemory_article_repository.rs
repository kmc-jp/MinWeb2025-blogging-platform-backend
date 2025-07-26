use std::{collections::HashMap, sync::{Arc, RwLock}};

use async_trait::async_trait;
use bson::oid::ObjectId;
use chrono::Utc;
use itertools::Itertools;

use crate::domain::{models::{article::Article, article_query::ArticleQuery, user_name::UserName, article_service::ArticleServiceError}, repositorys::{article_repository::ArticleRepository}};


#[derive(Clone, Default, Debug)]
pub struct InMemoryArticleRepository {
    articles: Arc<RwLock<HashMap<ObjectId, Article>>>,
}


#[async_trait]
impl ArticleRepository for InMemoryArticleRepository {
    async fn get_articles(&self, skip: usize, limit: usize) -> Result<Vec<Article>, ArticleServiceError> {
        let articles = self.articles.read().unwrap();
        let articles =
            articles.values()
            .k_smallest_by_key(skip + limit, |article| article.created_at)
            .skip(skip)
            .cloned()
            .collect();
        Ok(articles)
    }
    async fn get_article_by_id(&self, id: ObjectId) -> Result<Article, ArticleServiceError> {
        let articles = self.articles.read().unwrap();
        articles.get(&id).cloned().ok_or_else(|| ArticleServiceError::ArticleNotFound)
    }
    async fn add_article(&self ,title: String, author: UserName, content: String) -> Result<Article, ArticleServiceError> {
        let mut articles = self.articles.write().unwrap();
        let article = Article::new_article(title, author, content);
        articles.insert(article.id, article.clone());
        Ok(article)
    }
    async fn update_article(&self, id: ObjectId, title: Option<String>, content: Option<String>) -> Result<Article, ArticleServiceError> {
        let mut articles = self.articles.write().unwrap();
        let article = articles.get_mut(&id).ok_or_else(|| ArticleServiceError::ArticleNotFound)?;
        if let Some(new_title) = title {
            article.title = new_title;
        }
        if let Some(new_content) = content {
            article.content = new_content;
        }
        article.updated_at = Utc::now();
        Ok(article.clone())
    }
    async fn delete_article(&self, id: ObjectId) -> Result<(), ArticleServiceError> {
        let mut articles = self.articles.write().unwrap();
        if articles.remove(&id).is_some() {
            Ok(())
        } else {
            Err(ArticleServiceError::ArticleNotFound)
        }
    }
    async fn get_articles_with_query(&self, skip: usize, limit: usize, query: ArticleQuery) -> Result<Vec<Article>, ArticleServiceError> {
        let articles = self.articles.read().unwrap();
        let filtered_articles: Vec<Article> = articles.values()
            .filter(|article| {
                query.title.as_ref().is_none_or(|title| article.title.contains(title)) &&
                query.author.as_ref().is_none_or(|author| article.author.as_str() == author)
            })
            .k_smallest_by_key(skip + limit, |article| article.created_at)
            .skip(skip)
            .cloned()
            .collect();
        Ok(filtered_articles)
    }
}

