use crate::{
    presentation::handlers::create_handler::AppState, usecase::article_usecase::ArticleService,
    usecase::user_usecase::UserService,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::util::{default_limit, default_skip};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub display_name: String,
    pub intro: String,
    pub email: String,
    pub show_email: bool,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub display_name: Option<String>,
    pub intro: Option<String>,
    pub email: Option<String>,
    pub show_email: Option<bool>,
    pub password: Option<String>,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: ObjectId,
    pub name: String,
    pub display_name: String,
    pub intro: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Deserialize)]
pub struct GetUsersParams {
    #[serde(default = "default_skip")]
    skip: usize,
    #[serde(default = "default_limit")]
    limit: usize,
}

pub async fn create_user<T: ArticleService, U: UserService>(
    State(state): State<AppState<T, U>>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), StatusCode> {
    let user =  state
        .user_service
        .create_user(
            payload.name,
            payload.display_name,
            payload.intro,
            payload.email,
            payload.show_email,
            payload.password,
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let user_response = UserResponse {
        id: user.id,
        name: user.name.to_string(),
        display_name: user.display_name,
        intro: user.intro,
        email: if user.show_email { Some(user.email)} else { None },
    };
    Ok((StatusCode::CREATED, Json(user_response)))
}

pub async fn get_user<T: ArticleService, U: UserService>(
    State(state): State<AppState<T, U>>,
    Path(user_name): Path<String>,
) -> Result<Json<UserResponse>, StatusCode> {
    let user =
        state
        .user_service
        .get_user_by_name(&user_name)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or_else(|| StatusCode::NOT_FOUND)?;
    let user_response = UserResponse {
        id: user.id,
        name: user.name.to_string(),
        display_name: user.display_name,
        intro: user.intro,
        email: if user.show_email { Some(user.email) } else { None },
    };
    Ok(Json(user_response))
}

pub async fn list_users<T: ArticleService, U: UserService>(
    State(state): State<AppState<T, U>>,
    Query(params): Query<GetUsersParams>,
) -> Result<Json<Vec<UserResponse>>, StatusCode> {
    let users = state
        .user_service
        .get_users(params.skip, params.limit)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let user_responses = users
        .into_iter()
        .map(|user| UserResponse {
            id: user.id,
            name: user.name.to_string(),
            display_name: user.display_name,
            intro: user.intro,
            email: if user.show_email { Some(user.email) } else { None },
        })
        .collect();
    Ok(Json(user_responses))
}

pub async fn update_user<T: ArticleService, U: UserService>(
    State(state): State<AppState<T, U>>,
    Path(user_name): Path<String>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
    let user = state
        .user_service
        .update_user(
            user_name,
            payload.display_name,
            payload.intro,
            payload.email,
            payload.show_email,
            payload.password,
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let user_response = UserResponse {
        id: user.id,
        name: user.name.to_string(),
        display_name: user.display_name,
        intro: user.intro,
        email: if user.show_email { Some(user.email) } else { None },
    };
    Ok(Json(user_response))
}

pub async fn delete_user<T: ArticleService, U: UserService>(
    State(state): State<AppState<T, U>>,
    Path(user_name): Path<String>,
) -> Result<StatusCode, StatusCode> {
    match state.user_service.delete_user(&user_name).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

