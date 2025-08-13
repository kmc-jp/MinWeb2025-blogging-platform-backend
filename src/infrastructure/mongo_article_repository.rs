use chrono::Utc;
use futures::TryStreamExt;
use mongodb::{
    Collection, Database,
    bson::doc,
};

use async_trait::async_trait;

use crate::domain::{
    models::{
        article::{Article, ArticleId},
        article_query::ArticleQuery,
        article_service::ArticleServiceError,
        user_name::UserName
    },
    repositorys::article_repository::ArticleRepository,
};

#[derive(Clone, Debug)]
pub struct MongodbArticleRepository {
    database: Database,
    collection: Collection<Article>,
}

impl MongodbArticleRepository {
    pub fn new(database: Database) -> Self {
        let collection = database.collection("articles");
        Self {
            database,
            collection,
        }
    }
}

#[async_trait]
impl ArticleRepository for MongodbArticleRepository {
    async fn get_articles(
        &self,
        skip: usize,
        limit: usize,
    ) -> Result<Vec<Article>, ArticleServiceError> {
        let mut cursor = self
            .collection
            .find(doc! {})
            .skip(skip as u64)
            .limit(limit as i64)
            .await?;

        let mut articles: Vec<Article> = Vec::new();
        while let Some(article) = cursor.try_next().await? {
            articles.push(article);
        }
        Ok(articles)
    }

    async fn get_article_by_id(&self, id: ArticleId) -> Result<Article, ArticleServiceError> {
        let filter = doc! {
            "_id": bson::to_bson(&id).unwrap()
        };

        self
            .collection
            .find_one(filter)
            .await?
            .ok_or(ArticleServiceError::ArticleNotFound)
    }

    async fn add_article(
        &self,
        title: String,
        author: UserName,
        content: String,
    ) -> Result<Article, ArticleServiceError> {
        let article = Article::new_article(title, author, content);
        self.collection.insert_one(article.clone()).await?;
        Ok(article)
    }

    async fn update_article(
        &self,
        id: ArticleId,
        title: Option<String>,
        content: Option<String>,
    ) -> Result<Article, ArticleServiceError> {
        let filter = doc! {
            "_id": bson::to_bson(&id).unwrap(),
            "update_at": bson::to_bson(&Utc::now()).unwrap()
        };

        let mut set_doc = doc! {};
        if let Some(new_title) = title {
            set_doc.insert("title", new_title);
        }
        if let Some(new_content) = content {
            set_doc.insert("content", new_content);
        }

        let mut update_doc = doc! {};
        if !set_doc.is_empty() {
            update_doc.insert("$set", set_doc);
        }
        self.collection
            .update_one(filter.clone(), update_doc)
            .await?;

        self
            .collection
            .find_one(filter)
            .await?
            .ok_or(ArticleServiceError::ArticleNotFound)
    }

    async fn delete_article(&self, id: ArticleId) -> Result<(), ArticleServiceError> {
        let filter = doc! { "_id": bson::to_bson(&id).unwrap() };
        let result = self.collection.delete_one(filter).await?;
        if result.deleted_count == 1 {
            Ok(())
        } else {
            Err(ArticleServiceError::ArticleNotFound)
        }
    }

    async fn get_articles_with_query(
        &self,
        skip: usize,
        limit: usize,
        query: ArticleQuery,
    ) -> Result<Vec<Article>, ArticleServiceError> {
        let mut filter = doc! {};
        if let Some(title_query) = query.title {
            filter.insert("title", doc! {"$regex": title_query, "$options": "i"});
        }
        if let Some(author_query) = query.author {
            filter.insert("author.inner", author_query);
        }

        let mut cursor = self
            .collection
            .find(filter)
            .skip(skip as u64)
            .limit(limit as i64)
            .await?;

        let mut articles: Vec<Article> = Vec::new();
        while let Some(article) = cursor.try_next().await? {
            articles.push(article);
        }
        Ok(articles)
    }
}
