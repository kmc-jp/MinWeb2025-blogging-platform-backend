use async_trait::async_trait;
use bson::oid::ObjectId;

use crate::domain::models::{article::Article, user_name::UserName};

/// Articleのデータベースを管理する操作を抽象化したトレイト
#[async_trait]
pub trait ArticleRepository {
    /// 新しい記事を追加するメソッド
    async fn add_article(&self, article: Article) -> anyhow::Result<ObjectId>;
    /// titleとユーザー名を元に、記事を手に入れるメソッド
    /// 記事が存在しなかった場合は、Ok(None)を返す。
    async fn get_article(&self, author: UserName, title: &str) -> anyhow::Result<Option<Article>>;
    /// idを元に記事を手に入れるメソッド
    /// 記事が存在しなかった場合は、Ok(None)を返す。
    async fn get_article_from_id(&self, id: ObjectId) -> anyhow::Result<Option<Article>>;
    /// 記事の更新をするメソッド
    async fn update_page(&self, id: ObjectId, title: Option<String>, content: Option<String>) -> anyhow::Result<()>;
}