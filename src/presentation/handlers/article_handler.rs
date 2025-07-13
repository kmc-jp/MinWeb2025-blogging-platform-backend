use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use bson::oid::ObjectId;
use serde::Deserialize;

use crate::{
    domain::models::{article_query::ArticleQuery, article_service::ArticleService, user_service::UserService},
    presentation::handlers::{create_handler::AppState, util::*},
};

#[derive(Deserialize, Debug, Clone)]
pub struct GetArticlesParams {
    #[serde(default = "default_skip")]
    skip: usize,
    #[serde(default = "default_limit")]
    limit: usize,
}

pub async fn get_articles<T: ArticleService, U: UserService>(
    State(state): State<AppState<T, U>>,
    Query(params): Query<GetArticlesParams>,
) -> impl IntoResponse {
    match state
        .article_service
        .get_articles(params.skip, params.limit)
        .await
    {
        Ok(articles) => (StatusCode::OK, Json(articles)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateArticlePayload {
    author: String,
    title: String,
    content: String,
}

// この関数はUserAppStateに依存していることに注意してください
pub async fn create_article<T: ArticleService, U: UserService>(
    State(state): State<AppState<T, U>>,
    Json(payload): Json<CreateArticlePayload>,
) -> impl IntoResponse {
    let author_name = match state.user_service.get_user_by_name(&payload.author).await {
        Ok(Some(user)) => user.name,
        Ok(None) => {
            return (
                StatusCode::BAD_REQUEST,
                format!("User '{}' not found", payload.author),
            )
            .into_response()
        },
        Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    };

    match state
        .article_service
        .create_article(payload.title, author_name, payload.content)
        .await
    {
        Ok(article) => (StatusCode::CREATED, Json(article)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn get_article_by_id<T: ArticleService, U: UserService>(
    State(state): State<AppState<T, U>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let oid = match ObjectId::parse_str(&id) {
        Ok(oid) => oid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid ID format").into_response(),
    };

    match state.article_service.get_article_by_id(oid).await {
        Ok(Some(article)) => (StatusCode::OK, Json(article)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateArticlePayload {
    title: Option<String>,
    content: Option<String>,
}

pub async fn update_article<T: ArticleService, U: UserService>(
    State(state): State<AppState<T, U>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateArticlePayload>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return (StatusCode::BAD_REQUEST, "Invalid ID format").into_response()
    };

    match state
        .article_service
        .update_article(oid, payload.title, payload.content)
        .await
    {
        Ok(article) => (StatusCode::OK, Json(article)).into_response(),
        Err(e) => {
            // Assuming the service might return an error for not found cases
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

pub async fn delete_article<T: ArticleService, U: UserService>(
    State(state): State<AppState<T, U>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return (StatusCode::BAD_REQUEST, "Invalid ID format").into_response()
    };

    match state.article_service.delete_article(oid).await {
        Ok(_) => (StatusCode::NO_CONTENT).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct SearchParams {
    title_q: Option<String>,
    author: Option<String>,
    #[serde(default = "default_skip")]
    skip: usize,
    #[serde(default = "default_limit")]
    limit: usize,
}

pub async fn search_articles<T: ArticleService, U: UserService>(
    State(state): State<AppState<T, U>>,
    Query(params): Query<SearchParams>,
) -> impl IntoResponse {
    let query = ArticleQuery {
        title: params.title_q,
        author: params.author,
    };

    match state
        .article_service
        .search_articles(params.skip, params.limit, query)
        .await
    {
        Ok(articles) => (StatusCode::OK, Json(articles)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}