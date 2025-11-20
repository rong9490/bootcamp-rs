// use super::auth::signin_handler;
use super::index_handler;
use crate::chat_server::{
    app_state::AppState,
    handlers::{signin_handler, signup_handler},
};
use axum::{
    Router,
    routing::{get, post},
};

/** 创建生成服务路由 */
pub fn gen_router(state: AppState) -> Router {
    let chat: Router<AppState> = Router::new();
    // let cors = CorsLayer::new();

    let api: Router<AppState> = Router::new()
        .route("/signin", post(signin_handler))
        .route("/signup", post(signup_handler))
        .nest("/chats", chat);
        // .layer(cors);

    // [dummy]
    // .route("/users", get(list_chat_users_handler)) // TODO 下次此处接着
    // .route("/upload", post(upload_handler))
    // .route("/files/:ws_id/*path", get(file_handler))
    // .layer(from_fn_with_state(state.clone(), verify_token::<AppState>))
    // // routes doesn't need token verification

    let router: Router = Router::new()
        // .openapi()
        .route("/", get(index_handler)) // 返回处理句柄
        .route("/hello", get(|| async { "hello, chatRoom!" })) // 直接执行异步函数
        .nest("/api", api) // 路由嵌套nest, 转发给另一个Router实例
        .with_state(state);
    router
}

#[cfg(test)]
mod tests {
    use super::*;
}
