use crate::chat_server::app_state::AppState;
use axum::{extract::State, response::IntoResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, ToSchema, Serialize, Deserialize)]
pub struct AuthOutput {
    token: String,
}

// pub(crate) fn signin_handler(
//     // State(state): State<AppState>,
//     // Json(input): Json<CreateUser>,
// ) -> impl IntoResponse {
//     "signin@api!"
// }
// pub(crate) fn signup_handler() -> impl IntoResponse {
//     "signup@api!"
// }

pub async fn signin_handler(State(state): State<AppState>) -> impl IntoResponse {
    "Signed in (dummy response)"
}

pub async fn signup_handler(State(state): State<AppState>) -> impl IntoResponse {
    "Signed up (dummy response)"
}
