use axum::{
    routing::{get},
    Router,
};

use crate::{
    domain::models::{article_service::ArticleService, user_service::UserService},
    presentation::handlers::{article_handler::*, user_handler::*},
};

#[derive(Clone)]
pub struct AppState<A: ArticleService, U: UserService> {
    pub article_service: A,
    pub user_service: U,
}

pub fn create_handler<A, U>(article_service: A, user_service: U) -> Router
where
    A: ArticleService + Clone + Send + Sync + 'static,
    U: UserService + Clone + Send + Sync + 'static,
{
    let app_state = AppState {
        article_service,
        user_service,
    };

    Router::new()
        .route(
            "/articles",
            get(get_articles::<A, U>).post(create_article::<A, U>),
        )
        .route(
            "/articles/{id}",
            get(get_article_by_id::<A, U>)
                .patch(update_article::<A, U>)
                .delete(delete_article::<A, U>),
        )
        .route("/articles/search", get(search_articles::<A, U>))
        .route("/users", get(list_users::<A, U>).post(create_user::<A, U>))
        .route(
            "/users/{user_name}",
            get(get_user::<A, U>)
                .patch(update_user::<A, U>)
                .delete(delete_user::<A, U>),
        )
        .with_state(app_state)
}