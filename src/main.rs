use axum::{Router, routing::get};
use MinWeb2025_blogging_platform_backend::db::{get_blog_text};
#[tokio::main]
async fn main() {
    let uri = ""; //書き換えて
    let user_name = "akkey";
    let title = "おいしいシチューの作り方";
    let blog_text = get_blog_text(uri, user_name, title).await;
    match blog_text {
        Ok(Some(blog_text)) => println!("{blog_text:?}"),
        Ok(None) => println!("ブログは見つからなかった"),
        Err(err) => println!("{err}"),
    }
}
