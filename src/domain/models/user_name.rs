use serde::{Deserialize, Serialize};
use std::fmt;

use crate::domain::repositorys::user_repository::UserRepository;

/// UserNameは、ユーザー名を表す構造体です。
/// ユーザー名は一意であり、文字列として表現されます
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub struct UserName {
    inner: String
}

impl UserName {
    pub fn to_string(&self) -> String {
        self.inner.clone()
    }
    pub async fn validate(name: String, user_repository: &impl UserRepository) -> Result<UserName, String> {
        if name.is_empty() {
            return Err("ユーザー名は空にできません".to_string());
        }
        match user_repository.check_user_exists(&name).await {
            Ok(true) => Err("このユーザー名はすでに使用されています".to_string()),
            Ok(false) => Ok(UserName { inner: name }),
            Err(e) => Err(format!("ユーザー名のチェック中にエラーが発生しました: {}", e)),
        }
    }
}

impl fmt::Display for UserName{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.inner)
    }
}
