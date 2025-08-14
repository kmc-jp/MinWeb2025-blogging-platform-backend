use serde::{Deserialize, Serialize};
use std::fmt;

/// UserNameは、ユーザー名を表す構造体です。
/// ユーザー名は一意であり、文字列として表現されます
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub struct UserName {
    inner: String,
}

impl UserName {
    pub fn as_str(&self) -> &str {
        &self.inner
    }
    /// UserNameを新しく作成する
    pub fn new(name: String) -> Self {
        UserName { inner: name }
    }
}

impl fmt::Display for UserName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.inner)
    }
}
