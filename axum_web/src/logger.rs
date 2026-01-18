//! 日志模块 - 提供统一的 tracing 接口
//!
//! # 使用方式
//!
//! ## 方式 1: 使用预导入的宏（推荐）
//!
//! ```rust
//! use axum_web::log::{info, warn, error, debug};
//!
//! info!("应用启动成功");
//! warn!("配置文件未找到，使用默认配置");
//! error!("数据库连接失败: {}", err);
//! ```
//!
//! ## 方式 2: 直接使用 tracing 宏
//!
//! ```rust
//! use tracing::{info, warn, error, debug};
//!
//! info!("应用启动成功");
//! ```
//!
//! ## 方式 3: 使用初始化函数
//!
//! ```rust
//! use axum_web::logger;
//!
//! // 使用默认配置初始化
//! logger::init();
//!
//! // 或使用自定义配置
//! logger::init_with_config(logger::Config::default().with_level("debug"));
//! ```

use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};
use tracing_appender::non_blocking::WorkerGuard;

// ============================================================================
// 重新导出常用的 tracing 宏和类型
// ============================================================================

/// 重新导出 tracing 宏，方便使用
pub use tracing::{info, warn, error, debug, trace, instrument, span, Level};

// ============================================================================
// 日志配置
// ============================================================================

/// 日志配置
#[derive(Debug, Clone)]
pub struct Config {
    /// 日志级别 (trace, debug, info, warn, error)
    pub level: String,
    /// 是否显示文件名
    pub with_file: bool,
    /// 是否显示行号
    pub with_line_number: bool,
    /// 是否显示线程ID
    pub with_thread_ids: bool,
    /// 是否显示线程名称
    pub with_thread_names: bool,
    /// 是否显示目标（模块路径）
    pub with_target: bool,
    /// 是否启用 ANSI 颜色
    pub ansi: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            with_file: true,
            with_line_number: true,
            with_thread_ids: false,
            with_thread_names: false,
            with_target: false,
            ansi: true,
        }
    }
}

impl Config {
    /// 创建新的配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置日志级别
    pub fn with_level(mut self, level: impl Into<String>) -> Self {
        self.level = level.into();
        self
    }

    /// 设置是否显示文件名
    pub fn with_file(mut self, with_file: bool) -> Self {
        self.with_file = with_file;
        self
    }

    /// 设置是否显示行号
    pub fn with_line_number(mut self, with_line_number: bool) -> Self {
        self.with_line_number = with_line_number;
        self
    }

    /// 设置是否显示线程ID
    pub fn with_thread_ids(mut self, with_thread_ids: bool) -> Self {
        self.with_thread_ids = with_thread_ids;
        self
    }

    /// 设置是否显示线程名称
    pub fn with_thread_names(mut self, with_thread_names: bool) -> Self {
        self.with_thread_names = with_thread_names;
        self
    }

    /// 设置是否显示目标
    pub fn with_target(mut self, with_target: bool) -> Self {
        self.with_target = with_target;
        self
    }

    /// 设置是否启用 ANSI 颜色
    pub fn with_ansi(mut self, ansi: bool) -> Self {
        self.ansi = ansi;
        self
    }
}

// ============================================================================
// 初始化函数
// ============================================================================

/// 使用默认配置初始化日志系统
///
/// # 示例
///
/// ```rust
/// use axum_web::logger;
///
/// logger::init();
/// info!("日志已初始化");
/// ```
pub fn init() {
    init_with_config(Config::default())
}

/// 使用自定义配置初始化日志系统
///
/// # 示例
///
/// ```rust
/// use axum_web::logger;
///
/// logger::init_with_config(
///     logger::Config::new()
///         .with_level("debug")
///         .with_target(true)
/// );
/// ```
pub fn init_with_config(config: Config) {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(&config.level));

    let fmt_layer = fmt::layer()
        .with_file(config.with_file)
        .with_line_number(config.with_line_number)
        .with_thread_ids(config.with_thread_ids)
        .with_thread_names(config.with_thread_names)
        .with_target(config.with_target)
        .with_ansi(config.ansi);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .init();
}

/// 初始化日志系统并返回文件日志的 WorkerGuard
///
/// 当 guard 被丢弃时，所有待写入的日志都会被刷新。
/// 适合需要将日志写入文件的场景。
///
/// # 示例
///
/// ```rust
/// use axum_web::logger;
///
/// let _guard = logger::init_with_file("logs/app.log").unwrap();
/// info!("日志会写入到文件");
/// ```
pub fn init_with_file(
    file_path: impl AsRef<std::path::Path>,
) -> std::io::Result<WorkerGuard> {
    let path = file_path.as_ref();
    let parent = path.parent().unwrap_or_else(|| std::path::Path::new("."));
    let filename = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("app.log");

    let file_appender = tracing_appender::rolling::never(parent, filename);
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt::layer().with_writer(non_blocking))
        .init();

    Ok(guard)
}

// ============================================================================
// 工具函数
// ============================================================================

// tracing 已经提供了 span! 宏，无需额外包装

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.level, "info");
        assert_eq!(config.with_file, true);
        assert_eq!(config.with_line_number, true);
    }

    #[test]
    fn test_config_builder() {
        let config = Config::new()
            .with_level("debug")
            .with_file(false)
            .with_target(true);

        assert_eq!(config.level, "debug");
        assert_eq!(config.with_file, false);
        assert_eq!(config.with_target, true);
    }
}
