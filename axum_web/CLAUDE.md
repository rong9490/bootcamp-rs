# CLAUDE.md

本文件为 Claude Code (claude.ai/code) 在此代码库中工作时提供指导。

## 项目概述

这是一个基于 Axum 的 Web 应用程序，作为 Rust Web 开发训练营的一部分。项目跟随教程系列（https://www.bilibili.com/video/BV1YpJWzWEjk）构建，实现了使用 SeaORM 集成数据库的 Web 服务器。

**核心技术栈：**
- **Web 框架**: Axum 0.7.4
- **数据库**: PostgreSQL + SeaORM 1.1.19
- **异步运行时**: Tokio
- **配置管理**: 基于 YAML 的配置，支持环境变量覆盖
- **日志系统**: tracing 和 tracing-subscriber

## 构建和开发命令

### 运行应用
```bash
# 从工作空间根目录运行服务器
cargo run --bin axum_web

# 开发模式（支持热重载，需要安装 cargo-watch）
cargo watch -x "run"

# 从 axum_web 目录运行
cd axum_web && cargo run
```

### 测试
```bash
# 运行所有测试
cargo test --package axum_web

# 运行特定模块测试并显示输出
cargo test --package axum_web --lib -- database::sea_orm::database_options::tests --nocapture

# 运行特定文件的测试
cargo test --package axum_web --lib -- database::sea_orm::database_connection::tests --nocapture
```

### 构建
```bash
# 开发构建
cargo build --package axum_web

# 发布构建
cargo build --release --package axum_web
```

### 数据库设置
项目使用 Docker 运行 PostgreSQL。提供了三个脚本：

```bash
# 构建并运行 PostgreSQL 容器（持久化）
./pg_build_docker.sh

# 运行 PostgreSQL 容器（临时，用于开发）
./pg_run_docker.sh

# 初始化数据库并导入种子数据
./pg_seed_init.sh
```

**数据库连接信息：**
- 主机: 127.0.0.1:5432
- 用户: hejj
- 密码: pass12345
- 数据库: postgres
- 模式: public

### SeaORM 实体生成
当数据库模式变更时，重新生成实体：

```bash
# 如果尚未安装 sea-orm-cli
cargo install sea-orm-cli@1.1.0

# 从数据库生成实体
DATABASE_URL="postgres://hejj:pass12345@127.0.0.1:5432/postgres" \
sea-orm-cli generate entity \
  --with-serde both \
  --model-extra-attributes 'serde(rename_all = "camelCase")' \
  --date-time-crate chrono \
  -o ./src/entity

# 从特定模式生成
sea-orm-cli generate entity \
  -s public \
  --with-serde both \
  --model-extra-attributes 'serde(rename_all = "camelCase")' \
  --date-time-crate chrono \
  -o ./src/entity
```

## 代码架构

### 工作空间结构
这是父工作空间中位于 `/axum_web` 的 Cargo 工作空间成员。工作空间具有：
- 解析器版本 3
- 在工作空间 `Cargo.toml` 中定义的共享依赖
- `axum_web` 是默认成员
- `axum_web/dummy` 包含参考实现（从构建中排除）

### 模块组织

代码库遵循分层架构：

```
src/
├── main.rs              # 入口点，初始化日志、配置、数据库和服务器
├── lib.rs               # 库导出（logger、config、database、server）
├── logger.rs            # Tracing 初始化，支持 env-filter
├── config/              # 配置管理
│   ├── mod.rs           # AppConfig，使用懒加载单例模式
│   ├── server.rs        # ServerConfig（端口）
│   └── database.rs      # DatabaseConfig（主机、端口、凭证、模式）
├── database/            # 数据库层
│   ├── mod.rs           # 数据库模块导出
│   ├── sea_orm/         # SeaORM 特定实现
│   │   ├── mod.rs       # 连接流程编排
│   │   ├── database_options.rs   # 连接池配置（基于 CPU 数量调整）
│   │   ├── database_connection.rs # 连接创建和 ping
│   │   └── database_version.rs    # PostgreSQL 版本查询
│   └── entity/          # SeaORM 生成的实体
│       ├── mod.rs
│       ├── prelude.rs
│       └── sys_user.rs  # 示例实体（支持 camelCase serde）
└── server/              # HTTP 服务器层
    ├── mod.rs           # 服务器模块导出
    ├── listener.rs      # Axum 路由设置和 TcpListener
    └── router_handle/   # 请求处理器
        └── mod.rs       # 带有 #[debug_handler] 的处理函数
```

### 关键设计模式

**1. 懒加载配置单例**
`config::get()` 使用 `LazyLock` 返回 `&'static AppConfig`。配置加载自：
- `application.yaml`（基础配置）
- 带有 `APP_` 前缀的环境变量（覆盖）

**2. 数据库连接池**
- 连接池大小根据 CPU 数量自动缩放：`min(cpu_count * 4, 10)` 到 `max(cpu_count * 4, 20)`
- 连接超时：3 秒连接，5 秒获取
- 启动时自动 ping
- 成功连接后进行版本检查以验证

**3. 异步处理器模式**
所有处理器使用 `#[debug_handler]` 宏以在开发期间获得更好的错误消息。处理器是返回 `impl IntoResponse` 的异步函数。

**4. 错误处理**
- 应用级错误使用 `anyhow::Result`
- 数据库操作返回 `anyhow::Result<T>` 包装 SeaORM 的 `DbErr`
- 配置使用 `Context` trait 进行错误链上下文

### 配置加载

配置遵循以下优先级：
1. `application.yaml`（必需）
2. 带有 `APP_` 前缀的环境变量（例如：`APP_SERVER_PORT=3000`）
3. 在配置结构体中定义的默认值

环境变量覆盖示例：
```bash
export APP_SERVER_PORT=8080
export APP_DATABASE_HOST=localhost
```

### 数据库连接流程

1. 在 `main()` 中调用 `database_connection_flow()`
2. `gen_database_options()` 创建带有 CPU 缩放连接池的 `ConnectOptions`
3. `gen_database_connect()` 建立连接并执行 ping
4. `get_database_version()` 查询并记录 PostgreSQL 版本
5. 返回连接供使用（当前存储在 main 中的 `_database_connection`）

### 测试方法

- 单元测试使用 `#[cfg(test)]` 与模块共存
- 集成测试使用 `tokio::test` 进行异步操作
- 数据库测试需要 PostgreSQL 运行
- 使用 `--nocapture` 标志查看测试中的 `println!` 输出
- 可以使用完整模块路径按模块运行测试

## 开发注意事项

- **Rust 版本**: 2024
- **处理器**: 始终添加 `#[debug_handler]` 以获得更好的错误消息
- **日志**: 使用 `tracing::info!()`、`tracing::error!()` 而不是 `println!()`
- **实体命名**: 生成的实体通过 `#[serde(rename_all = "camelCase")]` 对 JSON 序列化使用 camelCase
- **线程池**: 数据库连接使用 `actix-native-tls` 运行时
- **工作空间依赖**: 添加依赖时优先使用工作空间版本

## 参考实现

`dummy/` 目录包含完整的参考实现，具有额外的功能，包括身份验证、验证、中间件和更高级的模式。这从主构建中排除，但作为学习资源。
