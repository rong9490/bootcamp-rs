mod auth;
mod chat;
mod messages;

pub(crate) use auth::*; // 重新简化导出
pub(crate) use chat::*;
pub(crate) use messages::*;

use axum::response::IntoResponse;

// 根路径响应
pub(crate) async fn index_handler() -> impl IntoResponse {
    "index@page"
}
