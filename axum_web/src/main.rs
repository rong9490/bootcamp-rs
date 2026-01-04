use axum_web::logger;
use axum_web::config;
use axum_web::config::AppConfig;
use axum_web::database::sea_orm::{database_connection_flow};
use sea_orm::DatabaseConnection;
use anyhow;
use axum_web::server::gen_server_listener;

// cargo watch -x "run"
// 展开原理: tokio::runtime::Builder::new_multi_thread
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init();
    // 创建数据库实例先
    let _database_connection: DatabaseConnection = database_connection_flow().await?;
    let app_config: &AppConfig = config::get();
    println!("config: {:#?}", app_config);
    let port: u16 = app_config.server().port();

    gen_server_listener(Option::Some(port)).await?;
    tracing::info!("Listening on http://0.0.0.0:{port}"); // 替代println打印日志
    Ok(())
}