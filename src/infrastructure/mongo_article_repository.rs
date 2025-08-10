use futures::TryStreamExt;
use mongodb::{
    Client, Collection, Database,
    bson::{Document, doc},
};
// use serde::{Deserialize, Serialize};

use async_trait::async_trait;

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
        let filter = bson::doc! {
            "_id": bson::to_bson(&id).unwrap()
        };

        if let Ok(Some(doc)) = self.collection.find_one(filter).await {
            if let Ok(article) = bson::from_document::<Article>(doc) {
                return Ok(article);
            }
        }

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
        let article = Article::new_article(title, author, content);

        let article_doc = bson::to_document(&article).unwrap();

        self.collection.insert_one(article_doc).await.unwrap();

        Ok(article)
    }

    async fn update_article(
        &self,
        id: ArticleId,
        title: Option<String>,
        content: Option<String>,
    ) -> Result<Article, ArticleServiceError> {
        let filter = doc! { "_id": bson::to_bson(&id).unwrap() };

        let mut set_doc = doc! {};
        if let Some(new_title) = title {
            set_doc.insert("title", new_title);
        }
        if let Some(new_content) = content {
            set_doc.insert("content", new_content);
        }

        let mut update_doc = doc! {"$currentDate": {"updated_at": true}};
        if !set_doc.is_empty() {
            update_doc.insert("$set", set_doc);
        }
        self.collection
            .update_one(filter.clone(), update_doc)
            .await?;

        if let Some(doc) = self.collection.find_one(filter).await? {
            if let Ok(article) = bson::from_document::<Article>(doc) {
                return Ok(article);
            }
        }
        Err(ArticleServiceError::ArticleNotFound)
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
            // UserNameはデフォルトでは { inner: String } としてシリアライズされるため、ネストしたフィールドで検索
            filter.insert("author.inner", author_query);
        }

        let mut cursor = self
            .collection
            .find(filter)
            .skip(skip as u64)
            .limit(limit as i64)
            .await?;

        let mut articles: Vec<Article> = Vec::new();
        while let Some(doc) = cursor.try_next().await? {
            if let Ok(article) = bson::from_document::<Article>(doc) {
                articles.push(article);
            }
        }
        Ok(articles)
    }
}
