use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;

use crate::domain::{models::{article::Article, user::UserName}, repositorys::article_repository::ArticleRepository};

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
    /// 取得した記事のリストを返す
    /// from: 取得開始位置, max: 最大取得数
    async fn get_articles(&self, from: usize, max: usize) -> Vec<Article>;
    /// タイトルを元に記事を検索する
    /// タイトルに部分一致する記事のリストを返す
    /// title_query: 検索するタイトルのクエリ
    async fn query_by_title(&self, title_query: String) -> Vec<Article>;
    /// 新しい記事を追加する
    /// title: 記事のタイトル, author: 記事の著者, content: 記事の内容
    /// 追加された記事を返す
    async fn add_article(&self, title: String, author: UserName, content: String) -> Article;
    /// 特定の著者が書いたすべての記事のリストを返す
    async fn get_articles_by_author(&self, author: UserName) -> Vec<Article>;
    /// IDを元に記事を取得する
    /// id: 記事のID
    async fn get_article_by_id(&self, id: ObjectId) -> Option<Article>;
    /// 記事を更新する
    /// id: 記事のID, title: 新しいタイトル, content: 新しい内容
    /// 更新が成功した場合はOk(()), 失敗した場合はErr(String)
    async fn update_article(&self, id: ObjectId, title: Option<String>, content: Option<String>) -> Result<(), String>;
}

#[async_trait]
impl<T: ArticleRepository + Clone + Send + Sync> ArticleService for ArticleUsecase<T> {
    async fn get_articles(&self, from: usize, max: usize) -> Vec<Article> {
        self.repository.get_articles(from, max).await
    }

    async fn query_by_title(&self, title_query: String) -> Vec<Article> {
        self.repository.query_by_title(title_query).await
    }

    async fn add_article(&self, title: String, author: UserName, content: String) -> Article {
        self.repository.add_article(title, author, content).await
    }

    async fn get_articles_by_author(&self, author: UserName) -> Vec<Article> {
        self.repository.get_articles_by_author(author).await
    }

    async fn get_article_by_id(&self, id: ObjectId) -> Option<Article> {
        self.repository.get_article_by_id(id).await
    }

    async fn update_article(&self, id: ObjectId, title: Option<String>, content: Option<String>) -> Result<(), String> {
        self.repository.update_article(id, title, content).await
    }
}