use axum::{debug_handler, routing::get, Router};
use tokio::net::TcpListener;
use axum_web::logger;
use axum_web::config;
use axum_web::config::AppConfig;

// cargo watch -x "run"
#[tokio::main] // 展开原理: tokio::runtime::Builder::new_multi_thread
async fn main() {
    println!("Hello, axum!");

    logger::init();
    let app_config: &AppConfig = config::get();
    println!("config: {:?}", app_config);
    let port: u16 = app_config.server().port();

    let router: Router = Router::new().route("/", get(index_handler));
    let listener: TcpListener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
    tracing::info!("Listening on http://0.0.0.0:{port}"); // 替代println打印日志

    // 底层基于tower, 封装一层: tower -> tokio -> axum
    // 异步方法 / 可能失败
    axum::serve(listener, router).await.unwrap();
}

#[debug_handler] // Generates better error messages when applied to handler functions.
async fn index_handler() -> &'static str {
    "Hello, axum!"
}