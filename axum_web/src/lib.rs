//! Axum Web 应用程序库
//!
//! 提供配置管理、数据库连接、日志记录等功能。

pub mod logger;
pub mod config;
pub mod database;
pub mod server;

// 重新导出日志宏，提供更短的导入路径
pub mod log {
    //! 日志宏的便捷导出
    //!
    //! # 使用方式
    //!
    //! ```rust
    //! use axum_web::log::{info, warn, error, debug};
    //!
    //! info!("应用启动成功");
    //! warn!("配置文件未找到");
    //! error!("操作失败: {}", err);
    //! ```
    pub use crate::logger::*;
}