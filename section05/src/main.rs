use anyhow::Result;
use axum::Router;
use section05::app_state::{AppState, get_router};
use section05::config::{AppConfig, DEF_APP_YAML_PATRH};
use tokio::net::TcpListener;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{Layer as _, fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main] // cargo watch -x "run"
async fn main() -> Result<()> {
    println!("section05_chat_room!");
    println!("def_yaml_path = {}", DEF_APP_YAML_PATRH);

    // 日志收集(脚手架代码)
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    // 加载配置
    let config: AppConfig = AppConfig::load()?;
    let port: u16 = config.server.port;
    let addr: String = format!("0.0.0.0:{}", port); // 拼接url地址
    let state: AppState = AppState::new(config).await;

    // 创建tcp并绑定到addr地址(异步操作)
    let app: Router = get_router(state); // state用于with_state(state)
    let listener: TcpListener = TcpListener::bind(&addr).await?;
    info!("服务启动: {}", addr);
    // 根据tcp实例, 创建服务
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}
