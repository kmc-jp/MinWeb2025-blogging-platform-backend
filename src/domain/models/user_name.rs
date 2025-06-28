use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserName {
    inner: String
}
impl TryFrom<String> for UserName {
    type Error = anyhow::Error;
    fn try_from(inner: String) -> anyhow::Result<Self> {
        Ok(UserName { inner }) //とりあえず素通し
    }
}