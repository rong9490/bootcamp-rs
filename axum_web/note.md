```aiignore
    进军Web开发
    https://www.bilibili.com/video/BV1YpJWzWEjk
```

```aiignore
    actix / Rocket [宏]/ axum[✅]
    
    Router / Listener / Serve
    
    axum::response::Response;
    axum::response::IntoResponse;
    
    处理句柄方法(async 闭包 / async 函数)
    
    适当的前缀命名空间, 语义更清晰
    scope::method::param;
    
    axum::Json;
    Json<Vec<String>>
    &'static str
    
    get() 方法参数有泛型约束
    H: Handler<T, S>,
    T: 'static,
    S: Clone + Send + Sync + 'static
    
    方便调试, 使用宏: #[debug_handler]
    无脑加上就行, 方便
```