use std::{collections::HashMap, sync::{Arc, RwLock}};

use async_trait::async_trait;
use bson::oid::ObjectId;
use chrono::Utc;

use crate::domain::{models::{article::Article, user::UserName}, repositorys::article_repository::ArticleRepository};

#[derive(Clone)]
pub struct ArticleRepositoryImpl{
    uri: String,
    user_name: String,
}

impl ArticleRepositoryImpl {
    pub fn new(uri: String, user_name: String) -> Self {
        ArticleRepositoryImpl { uri, user_name }
    }
}

/// データベースに詳しい人はここに実装を追加してください。
// #[async_trait]
// impl ArticleRepository for ArticleRepositoryImpl {
// }

/// 上の実装が完成するまでの仮の実装
/// この実装は、記事をメモリ上に保存するだけの簡易的なものです。
/// サーバーが再起動すると、保存された記事は失われます。
#[derive(Clone, Default)]
pub struct InMemoryArticleRepository {
    articles: Arc<RwLock<HashMap<ObjectId, Article>>>,
}

impl InMemoryArticleRepository {
    pub fn add_article(&self, title: impl Into<String>, author: impl Into<String>, content: impl Into<String>) -> Article {
        let article = Article::new_article(title.into(), UserName::try_from(author.into()).unwrap(), content.into());
        self.articles.write().unwrap().insert(article.id.clone(), article.clone());
        article
    }
}

#[async_trait]
impl ArticleRepository for InMemoryArticleRepository {
    async fn get_articles(&self, from: usize, max: usize) -> Vec<Article> {
        let articles = self.articles.read().unwrap();
        articles.values().skip(from).take(max).cloned().collect()
    }

    async fn query_by_title(&self, title_query: String) -> Vec<Article> {
        let articles = self.articles.read().unwrap();
        articles.values()
            .filter(|article| article.title.contains(&title_query))
            .cloned()
            .collect()
    }

    async fn add_article(&self, title: String, author: UserName, content: String) -> Article {
        let article = Article::new_article(title, author, content);
        self.articles.write().unwrap().insert(article.id.clone(), article.clone());
        article
    }

    async fn get_articles_by_author(&self, author: UserName) -> Vec<Article> {
        let articles = self.articles.read().unwrap();
        articles.values()
            .filter(|article| article.author == author)
            .cloned()
            .collect()
    }

    async fn get_article_by_id(&self, id: ObjectId) -> Option<Article> {
        let articles = self.articles.read().unwrap();
        articles.get(&id).cloned()
    }

    async fn update_article(&self, id: ObjectId, title: Option<String>, content: Option<String>) -> Result<(), String> {
        let mut articles = self.articles.write().unwrap();
        if let Some(article) = articles.get_mut(&id) {
            if let Some(new_title) = title {
                article.title = new_title;
            }
            if let Some(new_content) = content {
                article.content = new_content;
            }
            article.updated_at = Utc::now();
            Ok(())
        } else {
            Err("Article not found".to_string())
        }
    }
}

