# Docker 快速开始指南

## 重要说明

这是一个 **Cargo 工作空间**项目。Docker 构建必须从**工作空间根目录**（`bootcamp-rs/`）进行，而不是从成员子目录（`axum_web/`）。

## 项目结构

```
bootcamp-rs/              (工作空间根) ← 从这里运行 Docker！
├── Cargo.toml           (工作空间配置)
├── Cargo.lock           (依赖锁定)
├── Dockerfile.axum_web  (Docker 构建)
├── docker-compose.axum.yml (编排配置)
└── axum_web/            (成员目录)
    ├── Cargo.toml       (成员配置)
    ├── src/             (源代码)
    ├── application.toml (TOML 配置)
    └── application.yaml (YAML 配置)
```

## 快速开始

### 从工作空间根目录构建和运行

```bash
# 1. 进入工作空间根目录
cd /Users/hejj/Workspace/Zed25/rust-hejj/bootcamp-rs

# 2. 构建并启动服务
docker-compose -f docker-compose.axum.yml up -d --build

# 3. 查看日志
docker-compose -f docker-compose.axum.yml logs -f axum_web

# 4. 测试服务
curl http://localhost:4000/health

# 5. 停止服务
docker-compose -f docker-compose.axum.yml down
```

## 文件说明

### Dockerfile.axum_web
- 多阶段构建配置
- 构建阶段：编译 Rust 代码
- 运行阶段：只包含必要的运行时依赖
- 镜像大小：约 80 MB

### docker-compose.axum.yml
- `postgres` - PostgreSQL 16 数据库
- `axum_web` - Axum Web 应用
- 服务依赖、健康检查、数据卷配置

## 服务访问

### 应用端点
- http://localhost:4000/ - 首页
- http://localhost:4000/health - 健康检查
- http://localhost:4000/users - 用户列表

### 数据库连接
```
主机: localhost
端口: 5432
用户: hejj
密码: pass12345
数据库: postgres
```

## 故障排查

### 构建失败

如果看到 "failed to find a workspace root" 错误：

1. **确保从正确的目录运行**：
   ```bash
   pwd  # 应该显示 .../bootcamp-rs
   ```

2. **清理并重新构建**：
   ```bash
   docker-compose -f docker-compose.axum.yml down
   docker-compose -f docker-compose.axum.yml build --no-cache
   docker-compose -f docker-compose.axum.yml up -d
   ```

### 容器无法启动

1. 检查容器状态：
   ```bash
   docker-compose -f docker-compose.axum.yml ps
   ```

2. 查看日志：
   ```bash
   docker-compose -f docker-compose.axum.yml logs axum_web
   docker-compose -f docker-compose.axum.yml logs postgres
   ```

3. 检查端口占用：
   ```bash
   lsof -ti:4000 | xargs kill -9
   lsof -ti:5432 | xargs kill -9
   ```

## 清理

```bash
# 停止并删除容器
docker-compose -f docker-compose.axum.yml down

# 停止并删除容器和数据卷
docker-compose -f docker-compose.axum.yml down -v

# 清理未使用的镜像
docker image prune
```

## 注意事项

1. **构建位置** - 必须从 `bootcamp-rs/` 目录运行
2. **文件命名** - 使用 `.axum_web` 后缀避免与项目成员冲突
3. **配置文件** - 同时支持 TOML 和 YAML 格式
4. **数据库持久化** - 使用 Docker 卷持久化数据

## 验证部署

```bash
# 1. 检查服务状态
docker-compose -f docker-compose.axum.yml ps

# 2. 测试健康检查
curl http://localhost:4000/health

# 3. 测试数据库连接
docker exec -it axum_postgres psql -U hejj -c "SELECT version();"

# 4. 查看应用日志
docker-compose -f docker-compose.axum.yml logs -f --tail=50 axum_web
```

## 成功标志

如果一切正常，你应该看到：

```
✅ PostgreSQL 容器运行并健康
✅ Axum Web 容器运行并响应
✅ 日志显示 "HTTP server listening on http://0.0.0.0:4000"
✅ curl 返回 {"status":"ok"}
```

## 相关文件

- `Dockerfile.axum_web` - Docker 镜像定义
- `docker-compose.axum.yml` - 服务编排配置
- `.dockerignore` - Docker 构建排除文件
- `axum_web/` - 应用程序源代码
