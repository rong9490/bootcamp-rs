mod user;

use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
		pub id: usize,
		pub fillname: String,
		pub email: String,
		pub created_at: DateTime<Utc>,

		// 密码: 保存加密前/还是加密后 --> 不加password字段
}
