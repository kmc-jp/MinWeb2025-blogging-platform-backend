use async_trait::async_trait;
use bson::oid::ObjectId;
use mongodb::error::Error;
use crate::domain::models::{user::User, user_name::UserName};

/// Userのデータベースを管理する操作を抽象化したトレイト
#[async_trait]
pub trait UserRepository {
    /// 取得したユーザーのリストを返す
    /// `skip`: 取得開始位置, `limit`: 最大取得数
    /// # Errors
    /// ユーザーの情報にアクセスできなかった場合は`Err(Error)`を返す
    async fn get_users(&self, skip: usize, limit: usize) -> Result<Vec<User>, Error>;

    /// IDを元にユーザーを取得する
    /// `id`: ユーザーのObjectId
    /// ユーザーが存在しない場合は`Ok(None)`を返す
    /// ユーザーが存在する場合は`Ok(Some(User))`を返す
    /// # Errors
    /// データベースへのアクセスに失敗した場合は`Err(Error)`を返す
    async fn get_user_by_id(&self, id: ObjectId) -> Result<Option<User>, Error>;

    /// ユーザー名を元にユーザー情報を取得する
    /// `user_name`: ユーザー名
    /// ユーザーが存在しない場合は`Ok(None)`を返す
    /// ユーザーが存在する場合は`Ok(Some(User))`を返す
    /// # Errors
    /// データベースへのアクセスに失敗した場合は`Err(Error)`を返す
    async fn get_user_by_name(&self, name: &str) -> Result<Option<User>, Error>;

    // 将来的にはここの入力を構造体にまとめるかも
    /// 新しいユーザーを追加する
    /// `name`: 追加するユーザー名, `display_name`: 表示名, `intro`: 自己紹介, `email`: メールアドレス, `show_email`: メールアドレスを公開するかどうか, `password`: パスワード
    /// このメソッドは、ユーザー名の重複チェックを行う必要があります。
    /// # Errors
    /// ユーザーが既に存在する場合や、データベースへのアクセスに失敗した場合は`Err(Error)`を返す
    async fn add_user(&self, name: String, display_name: String, intro: String, email: String, show_email: bool, pw_hash: Vec<u8>) -> Result<User, Error>;

    /// ユーザー情報を部分的に更新する
    /// `name`: 更新するユーザー名, `display_name`: 新しい表示名, `intro`: 新しい自己紹介, `email`: 新しいメールアドレス, `show_email`: メールアドレスを公開するかどうか, `password`: 新しいパスワード
    /// このメソッドは、ユーザー名の重複チェックを行う必要があります。
    /// `display_name`, `intro`, `email`, `show_email`, `password`のいずれかがNoneの場合は、そのフィールドは更新しません。
    /// # Errors
    /// ユーザーが存在しない場合や、データベースへのアクセスに失敗した場合は`Err(Error)`を返す
    async fn update_user(&self, id: ObjectId, name: Option<String>, display_name: Option<String>, intro: Option<String>, email: Option<String>, show_email: Option<bool>, pw_hash: Option<Vec<u8>>) -> Result<User, Error>;

    /// ユーザーを削除する
    /// `id`: ユーザーのObjectId
    /// 更新が成功した場合は`Ok(())`, 失敗した場合は`Err(Error)`
    /// # Errors
    /// ユーザーが存在しない場合や、データベースへのアクセスに失敗した場合は`Err(Error)`を返す
    async fn delete_user(&self, id: ObjectId) -> Result<(), Error>;

    /// ユーザー名が存在するかどうかをチェックし、存在しなかったときに`name`の型を`UserName`に変換して返す
    /// `name`: UserNameに変換するユーザー名
    /// # Errors
    /// その名前のユーザーが既に存在する場合や、データベースへのアクセスに失敗した場合は`Err(Error)`を返す
    async fn validate_user_name(&self, name: &str) -> Result<UserName, Error>;
}