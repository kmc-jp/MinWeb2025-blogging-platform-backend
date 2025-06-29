use axum::{extract::{Path, State}, http::StatusCode, response::{IntoResponse, Response}, routing::{get, patch, post}, Json, Router};
use bson::oid::ObjectId;
use serde::Deserialize;

use crate::{domain::models::{article::Article, user_name::UserName}, usecase::article_usecase::ArticleService};

#[derive(Clone)]
pub struct AppState<T: ArticleService> {
    pub article_service: T,
} 

pub fn create_article_handler<T: ArticleService + Clone + Send + Sync + 'static>(todo_service: T) -> Router {
    let app_state = AppState {
        article_service: todo_service,
    };

    Router::new()
        .route("/", get(root_handler))
        .route("/articles", get(default_get_articles::<T>).post(post_query_by_title::<T>))
        .route("/articles/{user}/new", post(post_new_article::<T>))
        .route("/articles/{user}", get(get_articles_by_author::<T>))
        .route("/articles/{user}/{id}", get(get_article_by_id::<T>))
        .route("/articles/{user}/{id}/update", patch(update_article::<T>))
        .with_state(app_state)
} 

async fn root_handler() -> String {
    "Welcome to the Article API".to_string()
}

// #[derive(Deserialize, Debug, Clone)]
// struct GetArticlesParams {
//     from: usize,
//     max: usize,
// }

// async fn get_articles<T: ArticleService>(
//     State(state): State<AppState<T>>,
//     Json(params): Json<GetArticlesParams>,
// ) -> impl IntoResponse {
//     let articles = state.article_service.get_articles(params.from, params.max).await;
//     Json(articles)
// }

async fn default_get_articles<T: ArticleService>(
    State(state): State<AppState<T>>,
) -> impl IntoResponse {
    let articles = state.article_service.get_articles(0, 100).await;
    Json(articles)
}

#[derive(Deserialize, Debug, Clone)]
struct PostQueryArticles {
    title_query: String,
}

async fn post_query_by_title<T: ArticleService>(
    State(state): State<AppState<T>>,
    Json(query): Json<PostQueryArticles>,
) -> impl IntoResponse {
    let articles = state.article_service.query_by_title(query.title_query).await;
    Json(articles)
}

#[derive(Deserialize, Debug, Clone)]
struct PostNewArticle {
    title: String,
    content: String,
}


async fn post_new_article<T: ArticleService>(
    State(state): State<AppState<T>>,
    Path(user): Path<UserName>,
    Json(payload): Json<PostNewArticle>,
) -> impl IntoResponse {
    let article = state.article_service.add_article(payload.title, user, payload.content).await;
    Json(article)
}

async fn get_articles_by_author<T: ArticleService>(
    State(state): State<AppState<T>>,
    Path(user): Path<UserName>,
) -> impl IntoResponse {
    let articles = state.article_service.get_articles_by_author(user).await;
    Json(articles)
}

async fn get_article_by_id<T: ArticleService>(
    State(state): State<AppState<T>>,
    Path((_, id)): Path<(UserName, ObjectId)>,
) -> impl IntoResponse {
    let article = state.article_service.get_article_by_id(id).await;
    Json(article)
}

#[derive(Deserialize, Debug, Clone)]
struct UpdateArticle {
    title: Option<String>,
    content: Option<String>,
}

async fn update_article<T: ArticleService>(
    State(state): State<AppState<T>>,
    Path((_, id)): Path<(UserName, ObjectId)>,
    Json(payload): Json<UpdateArticle>,
) -> Result<(), String> {
    state.article_service.update_article(id, payload.title, payload.content).await
        .map_err(|e| e.to_string())
}