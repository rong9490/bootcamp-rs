use axum::{Router, routing};
use tokio::net::TcpListener;
use super::index_handler;

/** 创建http服务监听实例 */
pub async fn gen_server_listener(_port: Option<u16>) -> anyhow::Result<u16> {
    let port: u16 = _port.unwrap_or_else(|| 4000);
    let server_addr: String = format!("0.0.0.0:{port}");

    let router: Router = Router::new()
        .route("/", routing::get(index_handler))
        .route("/users", routing::get(async || "path: /users"));

    let listener: TcpListener = TcpListener::bind(server_addr).await?;
    axum::serve(listener, router).await?;
    Ok(port)
}
