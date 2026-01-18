pub mod database_options;
pub use database_options::*;
pub mod database_connection;
pub use database_connection::*;
pub mod database_version;
pub use database_version::*;

use sea_orm::{ConnectOptions, DatabaseConnection};
use crate::log::info;

/** 数据库连接: 组合流程 */
pub async fn database_connection_flow() -> anyhow::Result<DatabaseConnection> {
  let database_options: ConnectOptions = gen_database_options(Option::None);
  let _database_connection = gen_database_connect(database_options).await;
  let database_connection: DatabaseConnection = _database_connection?;
  database_connection.ping().await?;
  info!("Database Connected Successfully!");
  let _version_str = get_database_version(&database_connection).await;
  let version_str: String = _version_str?;
  info!("Database version: {}", version_str);

  Ok(database_connection)
}