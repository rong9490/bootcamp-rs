use axum::response::IntoResponse;

pub(crate) async fn send_message_handler() -> impl IntoResponse {
  "send message"
}

pub(crate) async fn list_message_handler() -> impl IntoResponse {
  "list message"
}

pub(crate) async fn file_handler() -> impl IntoResponse {
  "file"
}

pub(crate) async fn upload_handler() -> impl IntoResponse {
  "upload"
}