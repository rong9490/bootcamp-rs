use sea_orm::{DatabaseConnection, ConnectionTrait, DbBackend, Statement, QueryResult};

/** 获取数据库实例版本号 */
pub async fn get_database_version(db: &DatabaseConnection) -> anyhow::Result<String> {
  let version: QueryResult = db.query_one(
      Statement::from_string(DbBackend::Postgres, "SELECT version()")
  ).await?.ok_or_else(|| anyhow::anyhow!("Failed to get database Version!"))?;
  let version_str: String = version.try_get_by_index::<String>(0)?;
  Ok(version_str)
}

#[cfg(test)]
mod tests {
  use super::*;
  use sea_orm::{ConnectOptions};
  use super::super::{gen_database_options, gen_database_connect};

  #[tokio::test]
  async fn t1() {
    let database_options: ConnectOptions = gen_database_options(Option::None);
    let _database_connection = gen_database_connect(database_options).await;
    let database_connection: DatabaseConnection = _database_connection.unwrap();
    let _version_str = get_database_version(&database_connection).await;
    assert_eq!(_version_str.is_ok(), true);
    let version_str: String = _version_str.unwrap();
    assert_eq!(version_str.is_empty(), false);
    // PostgreSQL 18.1 (Debian 18.1-1.pgdg13+2) on aarch64-unknown-linux-gnu, compiled by gcc (Debian 14.2.0-19) 14.2.0, 64-bit
    println!("version_str: {}", version_str);
  }
}