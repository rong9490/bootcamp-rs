use axum::{debug_handler, routing::get, Router};
use tokio::net::TcpListener;

// cargo watch -x "run"
#[tokio::main] // 展开原理: tokio::runtime::Builder::new_multi_thread
async fn main() {
    println!("Hello, axum!");

    let router: Router = Router::new().route("/", get(index_handler));
    let listener: TcpListener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();

    println!("Listening on http://0.0.0.0:4000");

    // 底层基于tower, 封装一层: tower -> tokio -> axum
    // 异步方法 / 可能失败
    axum::serve(listener, router).await.unwrap();
}

#[debug_handler] // Generates better error messages when applied to handler functions.
async fn index_handler() -> &'static str {
    "Hello, axum!"
}