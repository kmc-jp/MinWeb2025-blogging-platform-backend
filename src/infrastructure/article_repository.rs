use async_trait::async_trait;

use crate::domain::repositorys::article_repository::ArticleRepository;

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

#[async_trait]
impl ArticleRepository for ArticleRepositoryImpl {
    async fn add_article(&self, article: crate::domain::models::article::Article) -> anyhow::Result<bson::oid::ObjectId> {
        // 実装をここに追加
        unimplemented!()
    }
    async fn get_article(&self, author: crate::domain::models::user_name::UserName, title: &str) -> anyhow::Result<Option<crate::domain::models::article::Article>> {
        // 実装をここに追加
        unimplemented!()
    }
    async fn get_article_from_id(&self, id: bson::oid::ObjectId) -> anyhow::Result<Option<crate::domain::models::article::Article>> {
        // 実装をここに追加
        unimplemented!()
    }
    async fn update_page(&self, id: bson::oid::ObjectId, title: Option<String>, content: Option<String>) -> anyhow::Result<()> {
        // 実装をここに追加
        unimplemented!()
    }
}