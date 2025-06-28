use std::{collections::HashMap, sync::{Arc, RwLock}};

use axum::{extract::{path::Path, State}, response::{IntoResponse, Json}, routing::{get, post}, Router};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BlogPage{
    id: Uuid,
    title: String,
    author: String,
    content: String,
}

impl BlogPage {
    fn new(title: String, author: String, content: String) -> Self {
        BlogPage {
            id: Uuid::new_v4(),
            title,
            author,
            content,
        }
    }
    
}

#[derive(Default, Clone)]
struct BlogPages {
    pages: Arc<RwLock<HashMap<Uuid, BlogPage>>>,
}

impl BlogPages {
    fn add_page(&self, title: String, author: String, content: String) -> Uuid {
        let page = BlogPage::new(title, author, content);
        let mut pages = self.pages.write().unwrap();
        pages.insert(page.id, page.clone());
        page.id
    }

    fn get_page(&self, id: Uuid) -> Option<BlogPage> {
        let pages = self.pages.read().unwrap();
        pages.get(&id).cloned()
    }
}

#[derive(Deserialize, Debug, Clone)]
struct PostQueryPages {
    title_query: String,
}

#[tokio::main]
async fn main() {
    let pages = BlogPages::default();

    pages.add_page(
        "First Post".to_string(),
        "Alice".to_string(),
        "This is the content of the first post.".to_string(),
    );
    pages.add_page(
        "Second Post".to_string(),
        "Bob".to_string(),
        "This is the content of the second post.".to_string(),
    );
    pages.add_page(
        "Third Post".to_string(),
        "Charlie".to_string(),
        "This is the content of the third post.".to_string(),
    );
    pages.add_page(
        "Fourth Post".to_string(),
        "Diana".to_string(),
        "This is the content of the fourth post.".to_string(),
    );
    pages.add_page(
        "Fifth Post".to_string(),
        "Eve".to_string(),
        "This is the content of the fifth post.".to_string(),
    );
    pages.add_page(
        "Sixth Post".to_string(),
        "Frank".to_string(),
        "This is the content of the sixth post.".to_string(),
    );
    
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/pages", post(post_query_pages))
        .route("/pages/{id}", get(get_page_handler))
        .with_state(pages);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root_handler() -> String {
    "Hello World".to_string()
}

async fn post_query_pages(
    State(pages): State<BlogPages>,
    Json(query): Json<PostQueryPages>,
) -> impl IntoResponse{
    let pages_map = pages.pages.read().unwrap();
    let filtered_pages: Vec<BlogPage> = pages_map
        .values()
        .filter(|page| page.title.contains(&query.title_query))
        .cloned()
        .collect();
    
    Json(filtered_pages)
}

async fn get_page_handler(
    State(pages): State<BlogPages>,
    Path(id): Path<Uuid>,
) -> Json<Option<BlogPage>> {
    let page = pages.get_page(id);
    Json(page)
}

