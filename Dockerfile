FROM rust:1.83-slim as builder

WORKDIR /app

COPY . .

RUN cargo install --path .
# 构建项目
RUN cargo build --release

CMD ["/app/target/release/toolbox"]