use axum::{
    routing::{get},
    Router,
};

use crate::{
    usecase::{article_usecase::ArticleService, user_usecase::UserService},
    presentation::handlers::{article_handler::*, user_handler::*},
};

#[derive(Clone)]
pub struct AppState<T: ArticleService, U: UserService> {
    pub article_service: T,
    pub user_service: U,
}

pub fn create_handler<T, U>(article_service: T, user_service: U) -> Router
where
    T: ArticleService + Clone + Send + Sync + 'static,
    U: UserService + Clone + Send + Sync + 'static,
{
    let app_state = AppState {
        article_service,
        user_service,
    };

    Router::new()
        .route(
            "/articles",
            get(get_articles::<T, U>).post(create_article::<T, U>),
        )
        .route(
            "/articles/{id}",
            get(get_article_by_id::<T, U>)
                .patch(update_article::<T, U>)
                .delete(delete_article::<T, U>),
        )
        .route("/articles/search", get(search_articles::<T, U>))
        .route("/users", get(list_users::<T, U>).post(create_user::<T, U>))
        .route(
            "/users/{user_name}",
            get(get_user::<T, U>)
                .patch(update_user::<T, U>)
                .delete(delete_user::<T, U>),
        )
        .with_state(app_state)
}