# ==========================================
# Stage 1: 构建阶段
# ==========================================
FROM rust:1.83-slim AS builder

# 设置工作目录
WORKDIR /app

# 安装构建依赖
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# 复制整个工作空间
COPY . .

# 只构建 axum_web 成员
# RUN cargo build --release -p axum_web
RUN cargo build --release

# ==========================================
# Stage 2: 运行阶段
# ==========================================
FROM debian:bookworm-slim AS runtime

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# 创建非 root 用户
RUN useradd -m -u 1000 appuser

# 设置工作目录
WORKDIR /app

# 从构建阶段复制二进制文件
COPY --from=builder /app/target/release/axum_web /app/axum_web

# 复制配置文件
COPY application.toml application.yaml /app/

# 修改文件权限
RUN chown -R appuser:appuser /app

# 切换到非 root 用户
USER appuser

# 暴露端口
EXPOSE 4000

# 健康检查
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:4000/health || exit 1

# 设置环境变量
ENV RUST_LOG=info

# 运行应用程序
CMD ["/app/axum_web"]
