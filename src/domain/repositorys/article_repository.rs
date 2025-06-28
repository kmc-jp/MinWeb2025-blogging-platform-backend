use async_trait::async_trait;

use crate::domain::models::{article::Article, user_name::UserName};

/// Articleのデータベースを管理する操作を抽象化したトレイト
#[async_trait]
pub trait ArticleRepositiory {
    /// 新しい記事を追加するメソッド
    async fn add_article(&self, article: Article) -> anyhow::Result<()>;
    /// titleとユーザー名を元に、記事を手に入れる関数
    /// 記事が存在しなかった場合は、Ok(None)を返す。
    async fn get_article(&self, author: UserName, title: &str) -> anyhow::Result<Option<Article>>;
}