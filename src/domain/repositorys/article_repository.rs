use async_trait::async_trait;
use bson::oid::ObjectId;
use mongodb::error::Error;

use crate::domain::{models::{article::Article, article_query::ArticleQuery, user_name::UserName}};

/// Articleのデータベースを管理する操作を抽象化したトレイト
#[async_trait]
pub trait ArticleRepository {
    /// 取得した記事のリストを返す
    /// skip: 取得開始位置, limit: 最大取得数
    async fn get_articles(&self, skip: usize, limit: usize) -> Result<Vec<Article>, Error>;

    /// IDを元に記事を取得する
    /// id: 記事のID
    /// 記事が存在しない場合はNoneを返す
    async fn get_article_by_id(&self, id: ObjectId) -> Result<Option<Article>, Error>;

    /// 新しい記事を追加する
    /// title: 記事のタイトル, author: 記事の著者, content: 記事の内容
    /// 追加された記事を返す
    async fn add_article(&self, title: String, author: UserName, content: String) -> Result<Article, Error>;

    /// 記事を更新する
    /// id: 記事のID, title: 新しいタイトル, content: 新しい内容
    /// 更新が成功した場合はOk(Article), 失敗した場合はErr(Error)
    /// titleとcontentのいずれかがNoneの場合は更新しない
    async fn update_article(&self, id: ObjectId, title: Option<String>, content: Option<String>) -> Result<Article, Error>;

    /// 記事を削除する
    /// id: 記事のID
    /// 削除が成功した場合はOk(()), 失敗した場合はErr(Error)
    async fn delete_article(&self, id: ObjectId) -> Result<(), Error>;

    /// クエリを元に記事を取得する
    /// skip: 取得開始位置, limit: 最大取得数, query: 記事のクエリ
    /// クエリにはタイトルや著者名などが含まれる
    async fn get_articles_with_query(&self, skip: usize, limit: usize, query: ArticleQuery) -> Result<Vec<Article>, Error>;
}