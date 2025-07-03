#[derive(Debug, Clone)]
pub struct ArticleQuery {
    pub title: Option<String>,
    pub author: Option<String>,
}