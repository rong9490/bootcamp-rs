mod server;

use std::sync::LazyLock;
use anyhow::Context;
use config::Config;
use serde::Deserialize;
pub use server::*;
use config::FileFormat;

// 懒加载
static CONFIG: LazyLock<AppConfig> = LazyLock::new(|| AppConfig::load().expect("failed to initialize config"));

// 对外暴露获取(get_config)
pub fn get() -> &'static AppConfig {
    &CONFIG
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
}

/// 调用AppConfig::load()的时机: 1.全局静态变量(单例), 在加载时执行; 2.main主流程
impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        let config: Result<AppConfig, anyhow::Error> = Config::builder().add_source(
            config::File::with_name("application")
                .format(FileFormat::Yaml)
                .required(true)
        ).add_source(
            config::Environment::with_prefix("APP")
                .try_parsing(true)
                .separator("_")
                .list_separator(",")
        )
            .build().with_context(|| anyhow::anyhow!("Failed to load config"))?
            .try_deserialize::<AppConfig>().with_context(|| anyhow::anyhow!("Failed to load config"));
        config
    }
}