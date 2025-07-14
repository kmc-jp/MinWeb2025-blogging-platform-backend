pub mod db;
pub mod infrastructure;
pub mod domain;
pub mod usecase;
pub mod presentation;

use std::time::Duration;
use axum::{error_handling::HandleErrorLayer, http::StatusCode, routing::get, Router};
use dotenvy::dotenv;
use tokio::signal;
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    infrastructure::{inmemory_article_repository::InMemoryArticleRepository, inmemory_user_repository::InMemoryUserRepository},
    presentation::handlers::create_handler::create_handler,
    usecase::{article_usecase::ArticleUsecase, user_usecase::UserUsecase},
    domain::models::{article_service::ArticleService, user_service::UserService}
};

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

    let article_service = ArticleUsecase::new(InMemoryArticleRepository::default());
    let user_service = UserUsecase::new(InMemoryUserRepository::default());
    
    create_test_data(&article_service, &user_service).await;
        
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
        .nest("/api", create_handler(
            article_service,
            user_service,
        ));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).with_graceful_shutdown(async { signal::ctrl_c().await.unwrap() }).await.unwrap();
}

async fn root_handler() -> String {
    tracing::debug!("Root handler called");
    "Welcome to the Blogging Platform API!".to_string()
}

async fn create_test_data(article_service: &ArticleUsecase<InMemoryArticleRepository>, user_service: &UserUsecase<InMemoryUserRepository>) {
    let furakuta = user_service.create_user(
        "furakuta".to_string(),
        "ふらくた".to_string(),
        "Rustと機械学習を勉強中".to_string(),
        "otera65537@gmail.com".to_string(),
        true,
        "password123".to_string()
    ).await.expect("Failed to create user 'furakuta'");

    let hoge = user_service.create_user(
        "hoge".to_string(),
        "ほげ".to_string(),
        "プログラミング初心者".to_string(),
        "hogehogehoge@gmail.com".to_string(),
        false,
        "password456".to_string()
    ).await.expect("Failed to create user 'hoge'");

    article_service.create_article(
        "Pythonはくそ".to_string(),
        furakuta.name.clone(),
        "動的型付け言語でありあまりに自由な書き方ができてしまうPythonは、型安全性が低く、バグが発生しやすい。またパフォーマンスも悪く、特に大規模なプロジェクトでは問題が顕著になる。".to_string(),
    ).await.unwrap();
    article_service.create_article(
        "Rustは最高".to_string(),
        furakuta.name.clone(),
        "Rustは、メモリ安全性とパフォーマンスを両立させることができる素晴らしいプログラミング言語です。特に、所有権システムにより、コンパイル時に多くのバグを防ぐことができます。また比較的新しい言語であるため、最新のプログラミングパラダイムを取り入れやすい点も魅力です。".to_string(),
    ).await.unwrap();
    article_service.create_article(
        "ニューラルネットワークの基礎".to_string(),
        furakuta.name.clone(),
        "ニューラルネットワークは、人工知能の一分野であり、脳の神経細胞の働きを模倣したモデルです。基本的な構造は、入力層、中間層、出力層から成り立っています。各層のノードは、前の層からの入力を受け取り、重み付けされた合計を計算し、活性化関数を通じて次の層に出力します。".to_string(),
    ).await.unwrap();
    article_service.create_article(
        "機械学習のアルゴリズム".to_string(),
        hoge.name.clone(),
        "機械学習には、教師あり学習、教師なし学習、強化学習などのさまざまなアプローチがあります。教師あり学習では、ラベル付きデータを使用してモデルを訓練し、未知のデータに対する予測を行います。教師なし学習では、データのパターンや構造を見つけることに焦点を当てます。強化学習は、エージェントが環境と相互作用しながら最適な行動を学ぶ方法です。".to_string(),
    ).await.unwrap();
    article_service.create_article(
        "データサイエンスの重要性".to_string(),
        hoge.name.clone(),
        "データサイエンスは、データから価値を引き出すための学問であり、ビジネスや研究において非常に重要な役割を果たしています。データ分析、機械学習、統計学などの技術を駆使して、意思決定を支援し、新しい知見を発見することができます。".to_string(),
    ).await.unwrap();
    article_service.create_article(
        "「ほげ」って何だろうね".to_string(),
        hoge.name.clone(),
        "「ほげ」という言葉は、プログラミングの世界でよく使われる例え話やサンプルコードで見かけることがあります。特に日本のプログラマーの間では、何か具体的な意味を持たないプレースホルダーとして使われることが多いです。".to_string(),
    ).await.unwrap();
}