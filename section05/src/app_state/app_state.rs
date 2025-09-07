use crate::{error::AppError, handlers::index_handler};
use anyhow::Result;
use axum::{Router, routing::get};
use crate::config::AppConfig;
use std::{ops::Deref, sync::Arc};

// clone很方便, 自带arc --> 但是 访问inner.config, 简化调用 --> Deref Trait --> 自动target指向inner
// TODO AppState需要保留一份, handler里需要读取 --> 要么实现clone-trait; 要么做成Arc更方便 --> 一个技巧, 做成inner间接引用
#[derive(Debug, Clone)]
pub struct AppState {
    inner: Arc<AppStateInner>,
}

#[allow(unused)]
#[derive(Debug)]
pub struct AppStateInner {
    config: AppConfig,
    // pub(crate) dk: DecodingKey,
    // pub(crate) ek: EncodingKey,
    // pub(crate) pool: PgPool,
}

// HINT 为 AppState 实现Detef Trait, 使其自动暴露内部属性 --> 因为基本都是访问inner, 不会使用外层壳(隐藏细节)
// state.config => state.inner.config / 对Arc进行deref, 自动完成*  自动deref的用法
impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AppState {
  // 实例化方法, 传入config
    pub async fn new(config: AppConfig) -> Self {
        Self {
            inner: Arc::new(AppStateInner { config }),
        }
    }

    // 尝试创建实例...可能异常
    pub async fn try_new(config: AppConfig) -> Result<Self, AppError> {
        todo!()
    }
}

// TODO 独立/隔离/维护handler, 否则耦合非常深
pub fn get_router(state: AppState) -> Router {
    // Ok(router)
    // let state = AppState::new(config);
    // 不断 get / post / patch / update / delete / message / ... 链式调用
    // let api = Router::new().router("/signin", post(signin_handler));
    // handler处理端需要单独维护(隔离独立) --> index.html页面
    // nest("/api", Router<AppState>) --> 转给另一个路由 Router
    // let app = Router::new().route("/", get(index_handler)).with_state(state);

    // let chat = Router::new()
    //     .route(
    //         "/:id",
    //         get(get_chat_handler).patch(update_chat_handler).delete(delete_chat_handler).post(send_message_handler),
    //     )
    //     .route("/:id/messages", get(list_message_handler))
    //     .layer(from_fn_with_state(state.clone(), verify_chat))
    //     .route("/", get(list_chat_handler).post(create_chat_handler));

    // let cors = CorsLayer::new()
    //     // allow `GET` and `POST` when accessing the resource
    //     .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE, Method::PUT])
    //     .allow_origin(cors::Any)
    //     .allow_headers(cors::Any);
    // let api = Router::new()
    //     .route("/users", get(list_chat_users_handler))
    //     .nest("/chats", chat)
    //     .route("/upload", post(upload_handler))
    //     .route("/files/:ws_id/*path", get(file_handler))
    //     .layer(from_fn_with_state(state.clone(), verify_token::<AppState>))
    //     // routes doesn't need token verification
    //     .route("/signin", post(signin_handler))
    //     .route("/signup", post(signup_handler))
    //     .layer(cors);

    // Ok(set_layer(app))

    // TODO 梳理路由与handler, 路由渲染端点, 链式调用, 逐步拓展
    let api = Router::new();
    Router::new()
        // .openapi()
        .route("/", get(index_handler))
        .route("/hello", get(|| async { "hello, chatRoom!" }))
        .nest("/api", api)
        .with_state(state)
}
