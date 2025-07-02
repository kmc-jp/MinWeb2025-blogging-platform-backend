use async_trait::async_trait;
use bson::oid::ObjectId;

use crate::domain::models::{article::Article, user::UserName};

/// Articleのデータベースを管理する操作を抽象化したトレイト
#[async_trait]
pub trait ArticleRepository {
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