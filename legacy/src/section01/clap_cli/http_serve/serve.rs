use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use clap::Parser;
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::{info, warn};

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(short, long, value_parser = verify_path, default_value = ".")]
    pub dir: PathBuf,
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}

fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    // if input is "-" or file exists
    let p = std::path::Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is not a directory")
    }
}

/* 处理http服务 */

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

// 服务入口: // cargo watch -x 'run -- http serve --dir ./ --port 8080'
pub async fn major_clap_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let state: HttpServeState = HttpServeState { path: path.clone() };

    // axum router(路由端点)
    let router: Router = Router::new()
        // index handler
        .route("/", get(index_handler))
        // tower_http 自带的文件服务 (路径自动)
        .nest_service("/tower", ServeDir::new(path.clone()))
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state)); // 复用原语, Arc::new() 创建并保护

    // 服务启动
    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], port));
    // 异步绑定 .await
    let listener: TcpListener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    info!("服务已启动: {:?} on {}", path, addr);
    Ok(())
}

/*
    本地的文件处理!
    tower-http::services
    ServeDir
    ServeFile
*/

async fn file_handler(
    State(state): State<Arc<HttpServeState>>, // './'
    Path(path): Path<String>, // 'Cargo.toml'
) -> (StatusCode, String) {
    info!("Path: {:?}", path.clone());
    println!("Path: {:?}", path.clone());

    // 拼接路径: './Cargo.toml'
    let p: PathBuf = std::path::Path::new(&state.path).join(path);

    // 文件不存在, 返回元组(404, "File {} note found")
    if !p.exists() {
        (
            StatusCode::NOT_FOUND,
            format!("File {} note found", p.display()),
        )
    } else {
        // TODO: test p is a directory
        // if it is a directory, list all files/subdirectories
        // as <li><a href="/path/to/file">file name</a></li>
        // <html><body><ul>...</ul></body></html>

        // 读取文件内容
        // 如果读取一个二进制文件, 无法读成String, 返回错误
        match tokio::fs::read_to_string(p).await {
            Ok(content) => {
                info!("Read {} bytes", content.len());
                (StatusCode::OK, content)
            }
            Err(e) => {
                warn!("Error reading file: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        }
    }
}

async fn index_handler(State(state): State<Arc<HttpServeState>>) -> (StatusCode, String) {
    let timestamp = chrono::Local::now();
    (
        StatusCode::OK,
        format!("Hello World! {:?} {}", state, timestamp),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("."),
        });
        let (status, content) = file_handler(State(state), Path("Cargo.toml".to_string())).await;
        assert_eq!(status, StatusCode::OK);
        assert!(content.trim().starts_with("[package]"));
    }
}

// 补充测试用例