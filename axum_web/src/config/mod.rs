mod server;
mod database;

use std::sync::LazyLock;
use anyhow::Context;
use config::Config;
use serde::Deserialize;
pub use server::*;
use config::FileFormat;
use crate::config::database::DatabaseConfig;

// 懒加载
static CONFIG: LazyLock<AppConfig> = LazyLock::new(|| AppConfig::load().expect("failed to initialize config"));

// 对外暴露获取(get_config)
pub fn get() -> &'static AppConfig {
    &CONFIG
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    server: ServerConfig,
    database: DatabaseConfig,
}

/// 调用AppConfig::load()的时机: 1.全局静态变量(单例), 在加载时执行; 2.main主流程
impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        // 尝试按优先级加载配置文件：TOML > YAML
        // 只使用找到的第一个配置文件，环境变量始终可以覆盖
        let config_file = std::path::Path::new("application.toml");
        let (format, file_name) = if config_file.exists() {
            (FileFormat::Toml, "application.toml")
        } else {
            (FileFormat::Yaml, "application.yaml")
        };

        tracing::info!("Loading configuration from: {}", file_name);

        let config: Result<AppConfig, anyhow::Error> = Config::builder()
            .add_source(
                config::File::with_name("application")
                    .format(format)
                    .required(true)
            )
            .add_source(
                config::Environment::with_prefix("APP")
                    .try_parsing(true)
                    .separator("_")
                    .list_separator(",")
            )
            .build().with_context(|| anyhow::anyhow!("Failed to load config"))?
            .try_deserialize::<AppConfig>().with_context(|| anyhow::anyhow!("Failed to load config"));
        config
    }

    // 避免移动所有权, 而是返回引用
    pub fn server(&self) -> &ServerConfig {
        &self.server
    }

    pub fn database(&self) -> &DatabaseConfig {
        &self.database
    }
}