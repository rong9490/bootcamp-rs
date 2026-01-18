# Tracing 日志重构说明

本文档说明了对 tracing 日志系统的重构，集中化处理日志相关代码。

## 重构内容

### 1. 改进的 logger 模块 (`src/logger.rs`)

#### 新增功能

**日志配置类**
```rust
use axum_web::logger;

// 使用默认配置
logger::init();

// 使用自定义配置
logger::init_with_config(
    logger::Config::new()
        .with_level("debug")
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
);
```

**配置选项**
- `level` - 日志级别（trace, debug, info, warn, error）
- `with_file` - 显示文件名
- `with_line_number` - 显示行号
- `with_thread_ids` - 显示线程ID
- `with_thread_names` - 显示线程名称
- `with_target` - 显示目标（模块路径）
- `ansi` - 启用 ANSI 颜色

**文件日志支持**
```rust
// 将日志写入文件
let _guard = logger::init_with_file("logs/app.log").unwrap();
info!("这条日志会写入到文件");
// 当 guard 被丢弃时，所有日志都会被刷新
```

#### 重新导出的宏

所有常用的 tracing 宏现在可以直接从 logger 模块导入：

```rust
use axum_web::logger::{info, warn, error, debug, trace, instrument, span, Level};
```

### 2. 便捷的 log 模块 (`src/lib.rs`)

提供了更短的导入路径：

```rust
use axum_web::log::{info, warn, error, debug};
```

### 3. 统一的代码风格

所有模块现在使用统一的日志导入方式：

**之前（不一致）**
```rust
// src/config/mod.rs
tracing::info!("...");

// src/database/sea_orm/mod.rs
tracing::info!("...");

// src/main.rs
tracing::info!("...");
```

**现在（统一）**
```rust
// 任何模块
use crate::log::info;
info!("...");
```

或者直接：
```rust
use axum_web::log::info;
info!("...");
```

## 使用方式

### 方式 1: 使用 crate::log（推荐用于 crate 内部）

```rust
use crate::log::{info, warn, error};

pub fn some_function() {
    info!("函数开始执行");
    warn!("这是一个警告");
    error!("发生错误: {}", err);
}
```

### 方式 2: 使用 axum_web::log（推荐用于外部使用）

```rust
use axum_web::log::{info, warn, error};

#[tokio::main]
async fn main() {
    axum_web::logger::init();
    info!("应用启动");
}
```

### 方式 3: 使用 tracing（仍然支持）

```rust
use tracing::{info, instrument};

#[instrument]
pub async fn database_query() {
    info!("执行数据库查询");
}
```

## 代码示例

### 在不同模块中使用日志

**配置模块** (`src/config/mod.rs`)
```rust
use crate::log::info;

impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        info!("Loading configuration from: {}", file_name);
        // ...
    }
}
```

**数据库模块** (`src/database/sea_orm/mod.rs`)
```rust
use crate::log::info;

pub async fn database_connection_flow() -> anyhow::Result<DatabaseConnection> {
    database_connection.ping().await?;
    info!("Database Connected Successfully!");
    // ...
}
```

**主函数** (`src/main.rs`)
```rust
use axum_web::log::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init();
    // ...
    info!("Listening on http://0.0.0.0:{}", port);
}
```

## 环境变量配置

日志级别可以通过环境变量配置，例如：

```bash
# 设置所有日志为 debug 级别
export RUST_LOG=debug
cargo run

# 设置特定模块的日志级别
export RUST_LOG=axum_web=debug,sea_orm=warn
cargo run

# 只显示错误
export RUST_LOG=error
cargo run
```

## 重构的优势

### 1. 集中化管理
- 所有日志配置在 `logger.rs` 中统一管理
- 易于修改和扩展日志功能

### 2. 类型安全
- 使用 `Config` 构建器模式，编译时检查配置选项
- 防止运行时配置错误

### 3. 更好的文档
- 每个公共函数都有详细的文档注释和示例
- 模块级别的文档说明使用方式

### 4. 灵活性
- 支持多种初始化方式（默认、自定义、文件）
- 可以轻松切换不同的日志输出

### 5. 测试友好
- 包含单元测试，确保配置逻辑正确
- 易于在测试中模拟和验证

## 迁移指南

### 对于现有代码

现有代码无需修改，因为我们仍然重新导出了 `tracing` 宏。但建议逐步迁移到新的导入方式：

**旧代码**
```rust
use tracing::{info, warn, error};

info!("消息");
```

**新代码（推荐）**
```rust
use crate::log::info;

info!("消息");
```

### 对于新代码

所有新代码应使用统一的导入方式：

```rust
// 在模块内部
use crate::log::{info, warn, error};

// 在 main.rs 或外部
use axum_web::log::{info, warn, error};
```

## 依赖更新

添加了新依赖：

```toml
# Cargo.toml
tracing-appender = { workspace = true }
```

这提供了文件日志功能。

## 向后兼容性

✅ 完全向后兼容
- 所有旧的 `tracing::` 导入仍然有效
- 可以逐步迁移到新的导入方式
- 不影响现有代码的行为

## 测试

运行 logger 模块的测试：

```bash
cargo test --package axum_web --lib -- logger::tests
```

运行应用程序查看日志效果：

```bash
# 默认日志级别（info）
cargo run --bin axum_web

# Debug 级别
RUST_LOG=debug cargo run --bin axum_web

# 只显示错误
RUST_LOG=error cargo run --bin axum_web
```

## 未来改进

可能的功能扩展：

1. **日志轮转** - 按日期或大小自动轮转日志文件
2. **格式化输出** - 支持自定义日志格式
3. **异步日志** - 完全异步的日志写入
4. **日志过滤** - 基于内容的日志过滤
5. **结构化日志** - JSON 格式的结构化日志输出
6. **性能监控** - 记录每个操作的性能指标

## 相关文件

- `src/logger.rs` - 日志模块实现
- `src/lib.rs` - 重新导出日志宏
- `src/config/mod.rs` - 使用示例
- `src/database/sea_orm/mod.rs` - 使用示例
- `src/main.rs` - 使用示例

## 参考资料

- [tracing 官方文档](https://docs.rs/tracing/)
- [tracing-subscriber 文档](https://docs.rs/tracing-subscriber/)
- [tracing-appender 文档](https://docs.rs/tracing-appender/)
