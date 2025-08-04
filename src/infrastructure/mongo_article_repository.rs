use bson::oid::ObjectId;
use futures::TryStreamExt;
use mongodb::{
    Client, Collection, Database,
    bson::{Document, doc},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use async_trait::async_trait;
use chrono::Utc;
use itertools::Itertools;

use crate::domain::{
    models::{
        article::{Article, ArticleId},
        article_query::ArticleQuery,
        article_service::ArticleServiceError,
        user_name::UserName,
    },
    repositorys::article_repository::ArticleRepository,
};

#[derive(Clone, Debug)]
pub struct MongodbArticleRepository {
    database: Database,
    collection: Collection<bson::Document>,
}

#[async_trait]
impl ArticleRepository for MongodbArticleRepository {
    async fn get_articles(
        &self,
        skip: usize,
        limit: usize,
    ) -> Result<Vec<Article>, ArticleServiceError> {
        let uri = "mongodb+srv://minweb:r8VasQm27aoDvphRRyhF@minweb-blog-backend.dt77xrg.mongodb.net/?retryWrites=true&w=majority&appName=minweb-blog-backend";
        // Create a new client and connect to the server
        let client = Client::with_uri_str(uri).await.unwrap();
        // Get a handle on the movies collection
        let database = client.database("blog_data");
        let collection: Collection<Document> = database.collection("articles");
        let mut cursor = collection
            .find(doc! {})
            .skip(skip as u64)
            .limit(limit as i64)
            .await
            .unwrap();

        let mut articles: Vec<Article> = Vec::new();

        println!("cursor: {:?}", cursor.try_next().await.unwrap());

        while let Some(doc) = cursor.try_next().await.unwrap() {
            if let Ok(article) = bson::from_document::<Article>(doc) {
                articles.push(article);
            }
        }
        Ok(articles)
    }

    async fn get_article_by_id(&self, id: ArticleId) -> Result<Article, ArticleServiceError> {
        return Err(ArticleServiceError::ArticleNotFound);
    }

    async fn add_article(
        &self,
        title: String,
        author: UserName,
        content: String,
    ) -> Result<Article, ArticleServiceError> {
        // let mut articles = self.articles.write().unwrap();
        // let article = Article::new_article(title, author, content);
        // articles.insert(article.id, article.clone());
        // Ok(article)
        return Err(ArticleServiceError::ArticleNotFound);
    }

    async fn update_article(
        &self,
        id: ArticleId,
        title: Option<String>,
        content: Option<String>,
    ) -> Result<Article, ArticleServiceError> {
        // let mut articles = self.articles.write().unwrap();
        // let article = articles
        //     .get_mut(&id)
        //     .ok_or_else(|| ArticleServiceError::ArticleNotFound)?;
        // if let Some(new_title) = title {
        //     article.title = new_title;
        // }
        // if let Some(new_content) = content {
        //     article.content = new_content;
        // }
        // article.updated_at = Utc::now();
        // Ok(article.clone())
        return Err(ArticleServiceError::ArticleNotFound);
    }
    async fn delete_article(&self, id: ArticleId) -> Result<(), ArticleServiceError> {
        // let mut articles = self.articles.write().unwrap();
        // if articles.remove(&id).is_some() {
        //     Ok(())
        // } else {
        //     Err(ArticleServiceError::ArticleNotFound)
        // }
        return Err(ArticleServiceError::ArticleNotFound);
    }
    async fn get_articles_with_query(
        &self,
        skip: usize,
        limit: usize,
        query: ArticleQuery,
    ) -> Result<Vec<Article>, ArticleServiceError> {
        // let articles = self.articles.read().unwrap();
        // let filtered_articles: Vec<Article> = articles
        //     .values()
        //     .filter(|article| {
        //         query
        //             .title
        //             .as_ref()
        //             .is_none_or(|title| article.title.contains(title))
        //             && query
        //                 .author
        //                 .as_ref()
        //                 .is_none_or(|author| article.author.as_str() == author)
        //     })
        //     .k_smallest_by_key(skip + limit, |article| article.created_at)
        //     .skip(skip)
        //     .cloned()
        //     .collect();
        // Ok(filtered_articles)
        return Err(ArticleServiceError::ArticleNotFound);
    }
}
