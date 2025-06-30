// use axum::{Router, routing::get};
// use MinWeb2025_blogging_platform_backend::db::{get_blog_text};
// #[tokio::main]
// async fn main() {
//     let uri = ""; //書き換えて
//     let user_name = "akkey";
//     let title = "おいしいシチューの作り方";
//     let blog_text = get_blog_text(uri, user_name, title).await;
//     match blog_text {
//         Ok(Some(blog_text)) => println!("{blog_text:?}"),
//         Ok(None) => println!("ブログは見つからなかった"),
//         Err(err) => println!("{err}"),
//     }


use std::time::Duration;
use axum::{error_handling::HandleErrorLayer, http::StatusCode, routing::get, Router};
use dotenvy::dotenv;
use tokio::signal;
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use MinWeb2025_blogging_platform_backend::{infrastructure::article_repository::InMemoryArticleRepository, presentation::handlers::article_handler::create_article_handler, usecase::article_usecase::ArticleUsecase};

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axum_sandbox=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let articles = InMemoryArticleRepository::default();

    articles.add_article(
        "Pythonはくそ",
        "furakuta",
        "Pythonはくそだ。なぜなら、Pythonは遅いからだ。",
    );
    articles.add_article(
        "Rustは最高",
        "furakuta",
        "Rustは最高だ。なぜなら、Rustは速いからだ。",
    );
    articles.add_article(
        "ニューラルネットワークの基礎",
        "furakuta",
        "ニューラルネットワークの基礎を学ぶことは、AIの世界を理解するための第一歩です。",
    );
    articles.add_article("おいしいシチューの作り方", "akkey", "おいしいシチューを作るためには、まず新鮮な野菜と肉を用意します。次に、鍋に油を熱し、野菜と肉を炒めます。最後に、スープストックを加えて煮込みます。");
    articles.add_article("Rustの所有権システム", "akkey", "Rustの所有権システムは、メモリ安全性を保証するための重要な機能です。所有権は、データの所有者が一人だけであることを保証します。");
    
    let app = Router::new()
        .route("/", get(root_handler))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {error}"),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        )
        .nest("/api/articles", create_article_handler(ArticleUsecase::new(articles)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).with_graceful_shutdown(async { signal::ctrl_c().await.unwrap() }).await.unwrap();
}

async fn root_handler() -> String {
    tracing::debug!("Root handler called");
    "Welcome to the Blogging Platform API!".to_string()
}
