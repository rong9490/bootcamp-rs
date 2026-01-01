/** 数据库连接 */
use anyhow::Result;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

// 考虑错误+异步
pub async fn gen_database_connect(database_options: ConnectOptions) -> anyhow::Result<DatabaseConnection> {
    let _database_connection: Result<DatabaseConnection, DbErr> = Database::connect(database_options).await;
    let database_connection: DatabaseConnection = _database_connection?;
    database_connection.ping().await?;
    Ok(database_connection)
}

// 如果无法连接数据库, 报错!
// Caused by:
//     0: pool timed out while waiting for an open connection
//     1: pool timed out while waiting for an open connection

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::database_options::*;

    #[tokio::test]
    async fn t1() {
        let database_options: ConnectOptions = gen_database_options(Option::None);
        // print!("{:#?}", database_options);
        let _database_connection = gen_database_connect(database_options).await;
        // assert_eq!(_database_connection.is_ok(), true);
        let database_connection: DatabaseConnection = _database_connection.unwrap();
        print!("End!");
    }
}