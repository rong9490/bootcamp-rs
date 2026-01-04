use axum::debug_handler;

// TODO 补充具体路由的处理句柄, 统一管理

#[debug_handler] // Generates better error messages when applied to handler functions.
pub async fn index_handler() -> &'static str {
    "Hello, axum!"
}
