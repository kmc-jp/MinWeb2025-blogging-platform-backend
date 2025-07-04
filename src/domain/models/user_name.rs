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
    /// UserNameを新しく作成する
    /// 必ず一意の名前を指定する必要があります
    /// UserRepositoryのvalidate_user_nameメソッドのみで使用されます
    pub fn new(name: String) -> Self {
        UserName { inner: name }
    }
}

impl fmt::Display for UserName{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.inner)
    }
}
