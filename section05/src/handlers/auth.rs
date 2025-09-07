// handlers - auth

use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Debug, ToSchema, Serialize, Deserialize)]
pub struct AuthOutput {
  token: String,
}