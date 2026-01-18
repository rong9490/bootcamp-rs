# 服务监听日志优化说明

## 问题描述

原代码中，`gen_server_listener` 调用后的 `info!` 日志永远不会执行，因为 `axum::serve()` 是一个阻塞调用。

### 原代码问题

**src/main.rs**
```rust
gen_server_listener(Option::Some(port)).await?;
info!("Listening on http://0.0.0.0:{port}");  // ❌ 永远不会执行
Ok(())
```

**src/server/listener.rs**
```rust
pub async fn gen_server_listener(_port: Option<u16>) -> anyhow::Result<u16> {
    // ...
    axum::serve(listener, router).await?;  // 阻塞在这里
    Ok(port)
}
```

## 优化方案

将日志输出移到 `gen_server_listener` 函数内部，在绑定端口后、启动服务前输出。

### 优化后的代码

**src/server/listener.rs**
```rust
use crate::log::info;

pub async fn gen_server_listener(_port: Option<u16>) -> anyhow::Result<u16> {
    let port: u16 = _port.unwrap_or_else(|| 4000);
    let server_addr: String = format!("0.0.0.0:{port}");

    let router: Router = Router::new()
        .route("/", routing::get(index_handler))
        .route("/users", routing::get(async || "path: /users"))
        .route("/health", routing::get(health_handler))
        .route("/items/:id", routing::get(get_item_handler))
        .route("/search", routing::get(search_handler));

    let listener: TcpListener = TcpListener::bind(&server_addr).await?;

    // ✅ 在启动服务前输出日志
    info!("HTTP server listening on http://0.0.0.0:{}", port);

    axum::serve(listener, router).await?;
    Ok(port)
}
```

**src/main.rs**
```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init();

    // 创建数据库实例
    let _database_connection: DatabaseConnection = database_connection_flow().await?;

    // 加载配置
    let app_config: &AppConfig = config::get();
    println!("config: {:#?}", app_config);
    let port: u16 = app_config.server().port();

    // 启动 HTTP 服务（内部会输出监听日志）
    gen_server_listener(Option::Some(port)).await?;

    Ok(())
}
```

## 优化效果

### 日志输出顺序

现在启动应用时，会看到以下顺序的日志输出：

```
1. [INFO] Database Connected Successfully!
2. [INFO] Database version: PostgreSQL 18.1 ...
3. [INFO] Loading configuration from: application.toml
4. config: AppConfig { ... }  (println! 输出)
5. [INFO] HTTP server listening on http://0.0.0.0:4000  ← 新增
```

### 优势

1. ✅ **日志可见** - 服务监听日志能够正常输出
2. ✅ **职责清晰** - 日志输出在函数内部，符合单一职责原则
3. ✅ **调试友好** - 可以清楚地看到服务启动的每个步骤
4. ✅ **代码简洁** - main 函数更简洁，不包含日志输出逻辑

## 进一步优化建议

### 1. 添加启动日志级别

可以根据需要添加不同级别的日志：

```rust
pub async fn gen_server_listener(_port: Option<u16>) -> anyhow::Result<u16> {
    let port: u16 = _port.unwrap_or_else(|| 4000);
    let server_addr: String = format!("0.0.0.0:{port}");

    debug!("Building HTTP router with 5 routes");
    let router: Router = Router::new()
        .route("/", routing::get(index_handler))
        // ... 其他路由
        ;

    debug!("Binding TCP listener on {}", server_addr);
    let listener: TcpListener = TcpListener::bind(&server_addr).await?;

    info!("HTTP server listening on http://0.0.0.0:{}", port);

    axum::serve(listener, router).await?;
    Ok(port)
}
```

### 2. 添加优雅关闭信号

可以添加 Ctrl+C 信号处理：

```rust
use tokio::signal;
use axum::extract::State;

pub async fn gen_server_listener(_port: Option<u16>) -> anyhow::Result<u16> {
    // ... 路由和监听器设置

    info!("HTTP server listening on http://0.0.0.0:{}", port);

    // 添加优雅关闭
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    info!("HTTP server shutdown complete");
    Ok(port)
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {
            info!("Received Ctrl+C, shutting down...");
        },
        _ = terminate => {
            info!("Received terminate signal, shutting down...");
        },
    }
}
```

### 3. 移除 println! 调试输出

将 `println!("config: {:#?}", app_config)` 改为使用 tracing 日志：

**src/main.rs**
```rust
use axum_web::log::debug;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init();

    let _database_connection: DatabaseConnection = database_connection_flow().await?;
    let app_config: &AppConfig = config::get();

    // 使用 debug 级别而不是 println!
    debug!("Loaded configuration: {:#?}", app_config);

    let port: u16 = app_config.server().port();
    gen_server_listener(Option::Some(port)).await?;

    Ok(())
}
```

这样可以通过 `RUST_LOG=debug` 环境变量控制是否显示配置详情。

## 相关文件

- `src/server/listener.rs` - HTTP 服务器监听逻辑
- `src/main.rs` - 应用入口点

## 测试验证

运行应用查看完整的启动日志：

```bash
# 默认日志级别（info）
cargo run --bin axum_web

# Debug 级别（可以看到更多细节）
RUST_LOG=debug cargo run --bin axum_web

# 只显示错误
RUST_LOG=error cargo run --bin axum_web
```

访问健康检查端点验证服务器运行：

```bash
curl http://127.0.0.1:4000/health
# 输出: {"status":"ok"}
```
