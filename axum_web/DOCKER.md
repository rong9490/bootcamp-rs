# Docker 部署指南

本文档说明如何使用 Docker 和 Docker Compose 部署 Axum Web 应用。

## 文件说明

- **Dockerfile** - 应用程序的 Docker 镜像定义
- **docker-compose.yml** - 多服务编排配置
- **.dockerignore** - 排除不需要打包到镜像的文件

## 快速开始

### 使用 Docker Compose（推荐）

一键启动应用和数据库：

```bash
# 构建并启动所有服务
docker-compose up -d

# 查看日志
docker-compose logs -f

# 查看服务状态
docker-compose ps

# 停止所有服务
docker-compose down

# 停止并删除数据卷
docker-compose down -v
```

### 使用 Docker 命令

手动构建和运行：

```bash
# 1. 构建镜像
docker build -t axum_web:latest .

# 2. 运行容器
docker run -d \
  --name axum_web_app \
  -p 4000:4000 \
  -e RUST_LOG=info \
  axum_web:latest

# 3. 查看日志
docker logs -f axum_web_app

# 4. 停止容器
docker stop axum_web_app

# 5. 删除容器
docker rm axum_web_app
```

## 多阶段构建

Dockerfile 使用了多阶段构建来优化镜像大小：

### Stage 1: 构建阶段 (builder)
- 基于 `rust:1.83-slim`
- 安装构建依赖
- 编译 Rust 代码
- 生成优化的二进制文件

### Stage 2: 运行阶段 (runtime)
- 基于 `debian:bookworm-slim`
- 只包含运行时依赖
- 复制编译好的二进制文件
- 使用非 root 用户运行

**镜像大小对比**：
- 完整开发环境：~1.5 GB
- 优化后的运行镜像：~80 MB

## 配置说明

### 环境变量

在 `docker-compose.yml` 中配置的环境变量：

| 变量名 | 默认值 | 说明 |
|--------|--------|------|
| `RUST_LOG` | info | 日志级别 |
| `APP_DATABASE_HOST` | postgres | 数据库主机 |
| `APP_DATABASE_PORT` | 5432 | 数据库端口 |
| `APP_DATABASE_USER` | hejj | 数据库用户 |
| `APP_DATABASE_PASSWORD` | pass12345 | 数据库密码 |
| `APP_DATABASE_NAME` | postgres | 数据库名称 |
| `APP_DATABASE_SCHEMA` | public | 数据库模式 |

### 端口映射

- `4000:4000` - Axum Web 应用 HTTP 端口
- `5432:5432` - PostgreSQL 数据库端口

### 数据卷

- `postgres_data` - PostgreSQL 数据持久化

## 服务管理

### 启动服务

```bash
# 前台启动（查看日志）
docker-compose up

# 后台启动
docker-compose up -d
```

### 查看状态

```bash
# 查看所有服务状态
docker-compose ps

# 查看资源使用
docker stats
```

### 查看日志

```bash
# 查看所有服务日志
docker-compose logs

# 跟踪日志输出
docker-compose logs -f

# 查看特定服务日志
docker-compose logs -f axum_web
docker-compose logs -f postgres
```

### 重启服务

```bash
# 重启所有服务
docker-compose restart

# 重启特定服务
docker-compose restart axum_web
```

### 停止服务

```bash
# 停止所有服务
docker-compose stop

# 停止并删除容器
docker-compose down

# 停止并删除容器和数据卷
docker-compose down -v
```

## 镜像管理

### 构建镜像

```bash
# 使用默认标签
docker build -t axum_web:latest .

# 使用特定标签
docker build -t axum_web:v1.0.0 .

# 不使用缓存构建
docker build --no-cache -t axum_web:latest .
```

### 查看镜像

```bash
# 列出本地镜像
docker images | grep axum_web

# 查看镜像详情
docker inspect axum_web:latest
```

### 删除镜像

```bash
# 删除镜像
docker rmi axum_web:latest

# 强制删除
docker rmi -f axum_web:latest
```

## 网络访问

### 应用端点

启动后，可以通过以下端点访问应用：

```
http://localhost:4000/          - 首页
http://localhost:4000/health    - 健康检查
http://localhost:4000/users     - 用户列表
http://localhost:4000/items/:id - 物品详情
http://localhost:4000/search    - 搜索
```

### 测试端点

```bash
# 健康检查
curl http://localhost:4000/health

# 首页
curl http://localhost:4000/

# 用户列表
curl http://localhost:4000/users
```

## 数据库连接

### 从主机连接

```bash
# 使用 psql 连接
docker exec -it axum_postgres psql -U hejj -d postgres

# 使用连接字符串
psql "postgres://hejj:pass12345@localhost:5432/postgres"
```

### 从应用容器连接

```bash
# 进入应用容器
docker exec -it axum_web_app bash

# 测试数据库连接
curl http://localhost:4000/health
```

## 故障排查

### 容器无法启动

```bash
# 查看容器日志
docker-compose logs axum_web

# 检查容器状态
docker-compose ps

# 检查网络
docker network ls
docker network inspect axum_axum_network
```

### 数据库连接失败

```bash
# 检查数据库容器状态
docker-compose ps postgres

# 检查数据库日志
docker-compose logs postgres

# 测试数据库连接
docker exec axum_postgres pg_isready -U hejj
```

### 重新构建镜像

```bash
# 停止服务
docker-compose down

# 重新构建（不使用缓存）
docker-compose build --no-cache

# 启动服务
docker-compose up -d
```

### 清理系统

```bash
# 停止并删除所有容器
docker-compose down -v

# 删除悬空镜像
docker image prune

# 删除未使用的数据卷
docker volume prune

# 清理所有未使用的资源
docker system prune -a
```

## 生产环境部署

### 安全建议

1. **修改默认密码**
   ```yaml
   environment:
     POSTGRES_PASSWORD: ${POSTGRES_PASSWORD}  # 使用环境变量
   ```

2. **使用 secrets 管理敏感信息**
   ```bash
   echo "your_secure_password" | docker secret create db_password -
   ```

3. **限制资源使用**
   ```yaml
   deploy:
     resources:
       limits:
         cpus: '0.5'
         memory: 512M
   ```

4. **使用健康检查**
   ```yaml
   healthcheck:
     test: ["CMD", "curl", "-f", "http://localhost:4000/health"]
     interval: 30s
     timeout: 3s
     retries: 3
   ```

### 性能优化

1. **调整工作进程数**
   ```yaml
   environment:
     - TOKIO_WORKER_THREADS=4
   ```

2. **启用日志持久化**
   ```yaml
   volumes:
     - ./logs:/app/logs
   ```

3. **使用反向代理**
   - 使用 Nginx/Caddy 作为反向代理
   - 启用 HTTPS
   - 配置缓存策略

### 监控

```bash
# 查看容器资源使用
docker stats axum_web_app

# 查看容器进程
docker top axum_web_app

# 导出容器日志
docker logs axum_web_app > app.log 2>&1
```

## CI/CD 集成

### GitHub Actions 示例

```yaml
name: Docker Build

on:
  push:
    branches: [main]

jobs:
  build:
    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v3

    - name: Build Docker image
      run: docker build -t axum_web:${{ github.sha }} .

    - name: Login to Docker Hub
      uses: docker/login-action@v2
      with:
        username: ${{ secrets.DOCKER_USERNAME }}
        password: ${{ secrets.DOCKER_PASSWORD }}

    - name: Push to Docker Hub
      run: docker push axum_web:${{ github.sha }}
```

## 常见问题

### Q: 如何修改应用端口？

A: 在 `docker-compose.yml` 中修改端口映射：
```yaml
ports:
  - "8080:4000"  # 主机端口:容器端口
```

### Q: 如何查看应用配置？

A: 进入容器查看：
```bash
docker exec -it axum_web_app cat /app/application.toml
```

### Q: 如何备份 PostgreSQL 数据？

A: 使用 pg_dump：
```bash
docker exec axum_postgres pg_dump -U hejj postgres > backup.sql
```

### Q: 如何恢复 PostgreSQL 数据？

A: 使用 psql：
```bash
docker exec -i axum_postgres psql -U hejj postgres < backup.sql
```

### Q: 镜像太大怎么办？

A: 使用 Alpine Linux 基础镜像：
```dockerfile
FROM alpine:latest
RUN apk add --no-cache ca-certificates
```

## 相关文件

- `Dockerfile` - Docker 镜像定义
- `docker-compose.yml` - 服务编排配置
- `.dockerignore` - 排除文件列表
- `DATABASE_SETUP.md` - 数据库设置指南

## 参考资料

- [Docker 官方文档](https://docs.docker.com/)
- [Docker Compose 文档](https://docs.docker.com/compose/)
- [Rust Docker 最佳实践](https://www.rust-lang.org/what/containers)
