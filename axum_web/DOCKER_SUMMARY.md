# Docker 配置总结

## 已创建的文件

### 1. **.dockerignore**
排除不需要打包到 Docker 镜像的文件：
- Git 相关文件
- 文档文件（*.md）
- 测试和脚本
- 构建产物（target/）
- IDE 和编辑器配置
- 临时文件和日志

### 2. **Dockerfile**
多阶段构建配置：
- **Stage 1 (builder)**: 使用 `rust:1.83-slim` 编译应用程序
- **Stage 2 (runtime)**: 使用 `debian:bookworm-slim` 运行应用程序

**特性**：
- ✅ 多阶段构建优化镜像大小
- ✅ 非 root 用户运行
- ✅ 健康检查支持
- ✅ 复制配置文件

### 3. **docker-compose.yml**
服务编排配置：
- `postgres` - PostgreSQL 16 数据库服务
- `axum_web` - Axum Web 应用服务

**特性**：
- ✅ 服务依赖管理
- ✅ 环境变量配置
- ✅ 数据卷持久化
- ✅ 健康检查
- ✅ 自定义网络

### 4. **DOCKER.md**
完整的 Docker 使用文档

## 快速开始

### 构建镜像

```bash
# 进入项目目录
cd /Users/hejj/Workspace/Zed25/rust-hejj/bootcamp-rs/axum_web

# 构建 Docker 镜像
docker build -t axum_web:latest .
```

### 使用 Docker Compose

```bash
# 启动所有服务（应用 + 数据库）
docker-compose up -d

# 查看日志
docker-compose logs -f

# 停止所有服务
docker-compose down
```

### 单独运行应用容器

```bash
# 运行容器
docker run -d \
  --name axum_web_app \
  -p 4000:4000 \
  -e RUST_LOG=info \
  axum_web:latest

# 查看日志
docker logs -f axum_web_app

# 停止容器
docker stop axum_web_app
```

## 架构说明

### 项目结构

```
bootcamp-rs/              (工作空间根)
├── Cargo.toml           (工作空间配置)
├── src/                 (共享模块 - 未在 axum_web 中使用)
└── axum_web/            (成员目录)
    ├── Cargo.toml       (成员配置)
    ├── src/             (不是真正的目录，模块文件在根目录)
    ├── config/          (配置模块)
    ├── database/        (数据库模块)
    ├── logger.rs        (日志模块)
    ├── server/          (服务器模块)
    ├── main.rs          (主入口)
    ├── lib.rs           (库导出)
    ├── Dockerfile       (Docker 构建文件)
    ├── docker-compose.yml (编排配置)
    └── application.toml (TOML 配置)
```

### Dockerfile 说明

由于项目使用 Cargo 工作空间，但源代码直接在成员根目录（不在 `src/` 子目录），Dockerfile 直接复制所需的文件：

```dockerfile
COPY Cargo.toml ./
COPY lib.rs ./
COPY main.rs ./
COPY config ./config
COPY database ./database
COPY logger.rs ./
COPY server ./server
COPY tests ./tests
```

然后创建一个虚拟的 `src` 目录作为工作空间结构的变通方案。

## 预期结果

### 镜像大小

- **Builder 阶段**: ~1.5 GB（包含 Rust 工具链）
- **Runtime 阶段**: ~80 MB（只包含二进制文件和运行时依赖）

### 服务访问

启动后可通过以下地址访问：

```
http://localhost:4000/          - 首页
http://localhost:4000/health    - 健康检查
http://localhost:4000/users     - 用户列表
```

### 数据库连接

```
主机: localhost
端口: 5432
用户: hejj
密码: pass12345
数据库: postgres
```

## 下一步

1. **测试构建** - 验证 Docker 镜像可以成功构建
2. **测试运行** - 验证容器可以正常运行
3. **测试端点** - 验证 HTTP 端点可访问
4. **配置 CI/CD** - 集成到持续集成流程

## 故障排查

### 构建失败

如果遇到构建问题：

1. 检查 Dockerfile 路径是否正确
2. 确保所有必需文件都已复制
3. 查看构建日志确定失败步骤

### 运行时错误

如果容器无法启动：

1. 检查日志：`docker logs axum_web_app`
2. 检查环境变量配置
3. 验证端口未被占用

### 数据库连接失败

1. 确保 PostgreSQL 容器正在运行
2. 检查网络配置
3. 验证连接字符串

## 注意事项

1. **构建时间**: 首次构建可能需要 5-10 分钟（下载依赖）
2. **工作空间结构**: 此项目使用 Cargo 工作空间，Dockerfile 需要特殊处理
3. **配置文件**: TOML 和 YAML 配置文件都会被复制到镜像中
4. **数据持久化**: 使用 docker-compose 卷可以持久化数据库数据

## 参考文档

- `DOCKER.md` - 完整的 Docker 使用指南
- `DATABASE_SETUP.md` - 数据库设置指南
- `CONFIG_FORMATS.md` - 配置文件格式说明


  1. Dockerfile - 简化并修复路径问题
    - 使用 COPY . . 复制整个工作空间
    - 使用 cargo build -p axum_web 只构建 axum_web 包
  2. .dockerignore - 移除了 Docker 相关文档的排除
  3. docker-compose.yml - 移除了过时的 version 属性

  项目结构说明

  这是一个 Cargo 工作空间项目：
  bootcamp-rs/              (工作空间根)
  ├── Cargo.toml           (工作空间配置)
  └── axum_web/            (成员目录)
      ├── Cargo.toml       (成员配置)
      ├── src/             (源代码)
      ├── Dockerfile       (Docker 构建)
      └── docker-compose.yml (编排配置)

  使用方法

  现在可以重新构建和启动：

  # 清理之前的构建
  docker-compose down

  # 重新构建和启动
  docker-compose up -d --build

  # 查看日志
  docker-compose logs -f axum_web

  由于这是一个工作空间项目，Docker 会从 bootcamp-rs/axum_web/ 目录构建，并复制整个工作空间来满足 Cargo 的依赖解析。这可能会导致构建上下文较大，但能确保正确解析工作空间依赖。