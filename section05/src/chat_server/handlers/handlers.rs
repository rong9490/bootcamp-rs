use axum::{
    response::IntoResponse,
};

// 根路径响应 -> "impl IntoResponse"只要实现, 都可以转为响应对象
pub(crate) async fn index_handler() -> impl IntoResponse {
    "index@page!" // 后续应该是前端页面
}
