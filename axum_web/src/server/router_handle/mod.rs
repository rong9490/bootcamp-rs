use axum::{debug_handler, Json, extract::{Path, Query}};
use serde::Deserialize;

// TODO 补充具体路由的处理句柄, 统一管理

#[debug_handler] // Generates better error messages when applied to handler functions.
pub async fn index_handler() -> &'static str {
    "Hello, axum!"
}

/// 健康检查处理器 - 返回服务状态
#[debug_handler]
pub async fn health_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok"
    }))
}

/// 获取单个item - 演示路径参数提取
#[debug_handler]
pub async fn get_item_handler(Path(id): Path<String>) -> String {
    format!("Item ID: {}", id)
}

/// 搜索处理器 - 演示查询参数提取
#[derive(Deserialize, Debug)]
pub struct SearchParams {
    q: String,
    limit: Option<usize>,
}

#[debug_handler]
pub async fn search_handler(Query(params): Query<SearchParams>) -> String {
    let limit = params.limit.unwrap_or(10);
    format!("Searching for '{}' with limit {}", params.q, limit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_handler() {
        let response = health_handler().await;
        assert_eq!(response.0.get("status").unwrap(), "ok");
    }

    #[tokio::test]
    async fn test_get_item_handler() {
        let result = get_item_handler(Path("123".to_string())).await;
        assert_eq!(result, "Item ID: 123");
    }

    #[tokio::test]
    async fn test_search_handler_with_limit() {
        let params = SearchParams {
            q: "rust".to_string(),
            limit: Some(20),
        };
        let result = search_handler(Query(params)).await;
        assert_eq!(result, "Searching for 'rust' with limit 20");
    }

    #[tokio::test]
    async fn test_search_handler_without_limit() {
        let params = SearchParams {
            q: "axum".to_string(),
            limit: None,
        };
        let result = search_handler(Query(params)).await;
        assert_eq!(result, "Searching for 'axum' with limit 10");
    }
}
