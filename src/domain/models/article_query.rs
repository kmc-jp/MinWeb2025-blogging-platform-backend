use crate::domain::models::user::UserName;

pub struct ArticleQuery {
    pub title_query: Option<String>,
    pub author_query: Option<UserName>,
}