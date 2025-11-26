```aiignore
    进军Web开发
    https://www.bilibili.com/video/BV1YpJWzWEjk
```

```aiignore
    01.[axum init]
    
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

```aiignore
    02.[logger]
    
     env_logger 老牌
     tracing ✅
     
     tracing 事件抽象
     tracing-subscriber 订阅事件
     
     unwrap_or_else + 闭包
     
    
```

```aiignore
    03.[集成配置功能]
    
    灵活配置端口号
    
    config模块单独声明
    go语言配置 viper
    rust语言配置 config
```

```aiignore
    04.[ORM集成]
    
    sea-orm
    
    线程池 / cpu个数 / 
    db.query_one / 
```

```aiignore
    05.[初体验sea_orm]
    
    命令行安装工具(生成数据实体): sea-orm-cli
    cargo install sea-orm-cli@1.1.0
    
    DATABASE_URL=protocol://username:password@localhost/database
    
    创建表sys_user, 
    
    sea-orm-cli --version
    sea-orm-cli generate entity
    
    sea-orm-cli generate entity --with-serde both --model-extra-attributes 'serde(rename_all = "camelCase")' --date-time-crate chrono -o ./src/entity
    
    sea-orm-cli generate entity -s demo --with-serde both --model-extra-attributes 'serde(rename_all = "camelCase")' --date-time-crate chrono -o ./src/entity
```