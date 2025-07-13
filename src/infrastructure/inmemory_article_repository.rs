use std::{collections::HashMap, sync::{Arc, RwLock}};

use async_trait::async_trait;
use bson::oid::ObjectId;
use chrono::Utc;
use itertools::Itertools;
use mongodb::error::Error;

use crate::domain::{models::{article::Article, article_query::ArticleQuery, user_name::UserName}, repositorys::{article_repository::ArticleRepository}};


#[derive(Clone, Default, Debug)]
pub struct InMemoryArticleRepository {
    articles: Arc<RwLock<HashMap<ObjectId, Article>>>,
}


#[async_trait]
impl ArticleRepository for InMemoryArticleRepository {
    async fn get_articles(&self, skip: usize, limit: usize) -> Result<Vec<Article>, Error> {
        let articles = self.articles.read().unwrap();
        let mut articles_vec: Vec<Article> = articles.values().cloned().collect();
        articles_vec.sort_by_key(|article| article.created_at);
        Ok(articles_vec.into_iter().skip(skip).take(limit).collect())
    }
    async fn get_article_by_id(&self, id: ObjectId) -> Result<Option<Article>, Error> {
        let articles = self.articles.read().unwrap();
        Ok(articles.get(&id).cloned())
    }
    async fn add_article(&self ,title: String, author: UserName, content: String) -> Result<Article, Error> {
        let mut articles = self.articles.write().unwrap();
        let article = Article::new_article(title, author, content);
        articles.insert(article.id, article.clone());
        Ok(article)
    }
    async fn update_article(&self, id: ObjectId, title: Option<String>, content: Option<String>) -> Result<Article, Error> {
        let mut articles = self.articles.write().unwrap();
        let article = articles.get_mut(&id).ok_or_else(|| Error::custom("Article not found"))?;
        if let Some(new_title) = title {
            article.title = new_title;
        }
        if let Some(new_content) = content {
            article.content = new_content;
        }
        article.updated_at = Utc::now();
        Ok(article.clone())
    }
    async fn delete_article(&self, id: ObjectId) -> Result<(), Error> {
        let mut articles = self.articles.write().unwrap();
        if articles.remove(&id).is_some() {
            Ok(())
        } else {
            Err(Error::custom("Article not found"))
        }
    }
    async fn get_articles_with_query(&self, skip: usize, limit: usize, query: ArticleQuery) -> Result<Vec<Article>, Error> {
        let articles = self.articles.read().unwrap();
        let filtered_articles: Vec<Article> = articles.values()
            .filter(|article| {
                query.title.as_ref().map_or(true, |title| article.title.contains(title)) &&
                query.author.as_ref().map_or(true, |author| article.author.to_string() == *author)
            })
            .sorted_by_key(|article| article.created_at)
            .skip(skip)
            .take(limit)
            .cloned()
            .collect();
        Ok(filtered_articles)
    }
}

