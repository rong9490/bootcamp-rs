use anyhow::Result;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{Layer as _, fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // 日志收集
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    println!("Hello, world!");
}
