use async_trait::async_trait;
use bson::oid::ObjectId;
use crate::domain::{models::{article::Article, article_query::ArticleQuery, user_name::UserName, article_service::ArticleServiceError}};

/// Articleのデータベースを管理する操作を抽象化したトレイト
#[async_trait]
pub trait ArticleRepository {
    /// 取得した記事のリストを返す
    /// `skip`: 取得開始位置, `limit`: 最大取得数
    /// 
    /// # Errors
    /// 記事の情報にアクセスできなかった場合は`Err(Error)`を返す
    async fn get_articles(&self, skip: usize, limit: usize) -> Result<Vec<Article>, ArticleServiceError>;

    /// IDを元に記事を取得する
    /// `id`: 記事のID
    /// 記事が存在しない場合は`Ok(None)`を返す
    /// 記事が存在する場合は`Ok(Some(Article))`を返す
    /// 
    /// # Errors
    /// データベースへのアクセスに失敗した場合は`Err(Error)`を返す
    async fn get_article_by_id(&self, id: ObjectId) -> Result<Option<Article>, ArticleServiceError>;

    /// 新しい記事を追加する
    /// `title`: 記事のタイトル, `author`: 記事の著者, `content`: 記事の内容
    /// 追加された記事を返す
    /// 
    /// # Errors
    /// 記事の追加に失敗した場合は`Err(Error)`を返す
    async fn add_article(&self, title: String, author: UserName, content: String) -> Result<Article, ArticleServiceError>;

    /// 記事を更新する
    /// `id`: 記事のID, `title`: 新しいタイトル, `content`: 新しい内容
    /// 更新が成功した場合は`Ok(Article)`, 失敗した場合は`Err(Error)`
    /// `title`と`content`のいずれかが`None`の場合は更新しない
    /// 
    /// # Errors
    /// 記事が存在しない場合や、データベースへのアクセスに失敗した場合は`Err(Error)`を返す
    async fn update_article(&self, id: ObjectId, title: Option<String>, content: Option<String>) -> Result<Article, ArticleServiceError>;

    /// 記事を削除する
    /// `id`: 記事のID
    /// 削除が成功した場合は`Ok(())`, 失敗した場合は`Err(Error)`
    /// 
    /// # Errors
    /// 記事が存在しない場合や、データベースへのアクセスに失敗した場合は`Err(Error)`を返す
    async fn delete_article(&self, id: ObjectId) -> Result<(), ArticleServiceError>;

    /// クエリを元に記事を取得する
    /// `skip`: 取得開始位置, `limit`: 最大取得数, `query`: 記事のクエリ
    /// クエリにはタイトルや著者名などが含まれる
    /// 
    /// # Errors
    /// データベースへのアクセスに失敗した場合は`Err(Error)`を返す
    async fn get_articles_with_query(&self, skip: usize, limit: usize, query: ArticleQuery) -> Result<Vec<Article>, ArticleServiceError>;
}