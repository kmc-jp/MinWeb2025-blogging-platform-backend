use std::{collections::HashMap, sync::{Arc, RwLock}};
use chrono::{DateTime, Utc};
use axum::{extract::{path::Path, State}, response::{IntoResponse, Json}, routing::{get, post}, Router};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Article{
    id: Uuid,
    title: String,
    author: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    content: String,
}

impl Article {
    fn new(title: String, author: String, content: String) -> Self {
        let now = Utc::now();
        Article {
            id: Uuid::new_v4(),
            title,
            author,
            created_at: now,
            updated_at: now,
            content,
        }
    }
    
}

#[derive(Default, Clone)]
struct Articles {
    articles: Arc<RwLock<HashMap<Uuid, Article>>>,
}

impl Articles {
    fn add_article(&self, title: impl Into<String>, author: impl Into<String>, content: impl Into<String>) -> Uuid {
        let article = Article::new(title.into(), author.into(), content.into());
        let mut articles = self.articles.write().unwrap();
        articles.insert(article.id, article.clone());
        article.id
    }

    fn get_article(&self, id: Uuid) -> Option<Article> {
        let articles = self.articles.read().unwrap();
        articles.get(&id).cloned()
    }

    fn update_page(&self, id: Uuid, title: Option<String>, content: Option<String>) -> Option<Article> {
        let mut articles = self.articles.write().unwrap();
        if let Some(article) = articles.get_mut(&id) {
            if let Some(new_title) = title {
                article.title = new_title;
            }
            if let Some(new_content) = content {
                article.content = new_content;
            }
            article.updated_at = Utc::now();
            Some(article.clone())
        } else {
            None
        }
    }
}

#[tokio::main]
async fn main() {
    let articles = Articles::default();

    articles.add_article("Pythonはくそ", "furakuta", "Pythonはくそだ。なぜなら、Pythonは遅いからだ。");
    articles.add_article("Rustは最高", "furakuta", "Rustは最高だ。なぜなら、Rustは速いからだ。");
    articles.add_article("ニューラルネットワークの基礎", "furakuta", "ニューラルネットワークの基礎を学ぶことは、AIの世界を理解するための第一歩です。");
    articles.add_article("おいしいシチューの作り方", "akkey", "おいしいシチューを作るためには、まず新鮮な野菜と肉を用意します。次に、鍋に油を熱し、野菜と肉を炒めます。最後に、スープストックを加えて煮込みます。");
    articles.add_article("Rustの所有権システム", "akkey", "Rustの所有権システムは、メモリ安全性を保証するための重要な機能です。所有権は、データの所有者が一人だけであることを保証します。");
    
    let app = Router::new()
        .route("/", get(get_all_articles).post(post_query_articles))
        .route("/{user}/create-article", post(post_new_article))
        .route("/{user}/articles", get(ger_author_articles))
        .route("/{user}/articles/{id}", get(get_article))
        .route("/{user}/update-article/{id}", post(post_update_article))
        .with_state(articles);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_all_articles(
    State(articles): State<Articles>,
) -> Json<Vec<Article>> {
    let article_map = articles.articles.read().unwrap();
    let all_articles: Vec<Article> = article_map.values().cloned().collect();
    Json(all_articles)
}

#[derive(Deserialize, Debug, Clone)]
struct PostQueryArticles {
    title_query: String,
}

async fn post_query_articles(
    State(articles): State<Articles>,
    Json(query): Json<PostQueryArticles>,
) -> impl IntoResponse{
    let articles_map = articles.articles.read().unwrap();
    let filtered_articles: Vec<Article> = articles_map
        .values()
        .filter(|article| article.title.contains(&query.title_query))
        .cloned()
        .collect();
    Json(filtered_articles)
}

#[derive(Deserialize, Debug, Clone)]
struct PostNewArticle {
    title: String,
    content: String,
}

async fn post_new_article(
    State(articles): State<Articles>,
    Path(user): Path<String>,
    Json(new_article): Json<PostNewArticle>,
) -> impl IntoResponse {
    let article_id = articles.add_article(new_article.title, user, new_article.content);
    Json(article_id)
}

async fn ger_author_articles(
    State(articles): State<Articles>,
    Path(user): Path<String>,
) -> Json<Vec<Article>> {
    let article_map = articles.articles.read().unwrap();
    let user_articles: Vec<Article> = article_map
        .values()
        .filter(|article| article.author == user)
        .cloned()
        .collect();
    Json(user_articles)
}

async fn get_article(
    State(articles): State<Articles>,
    Path((_, id)): Path<(String, Uuid)>,
) -> impl IntoResponse {
    let article = articles.get_article(id);
    match article {
        Some(article) => Json(article),
        None => Json(Article::new("Not Found".to_string(), "Unknown".to_string(), "Article not found.".to_string())),
    }
}

#[derive(Deserialize, Debug, Clone)]
struct PostUpdateArticle {
    title: Option<String>,
    content: Option<String>,
}


async fn post_update_article(
    State(articles): State<Articles>,
    Path((_, id)): Path<(String, Uuid)>,
    Json(update_data): Json<PostUpdateArticle>,
) -> impl IntoResponse {
    let updated_article = articles.update_page(id, update_data.title, update_data.content);
    match updated_article {
        Some(article) => Json(article),
        None => Json(Article::new("Not Found".to_string(), "Unknown".to_string(), "Article not found.".to_string())),
    }
}

