// use sea_orm::DatabaseConnection;


// pub async fn database_connection() -> DatabaseConnection {
//     let database_config = config::get().database();
//     let database_url: String = format!("postgres://{}:{}@{}:{}/{}", database_config.user(), database_config.password(), database_config.host(), database_config.port(), database_config.database());
//     let mut options = ConnectOptions::new(database_url);
//     options.min_connections(max(cpus * 4, 10u32))
//         .max_connections(max(cpus * 4, 20u32))
//         .connect_timeout(Duration::from_secs(10))
//         .acquire_timeout(Duration::from_secs(30))
//         .idle_timeout(Duration::from_secs(300))
//         .max_lifetime(Duration::from_secs(3600 * 24))
//         .sqlx_logging(false)
//         .set_schema_search_path(database_config.schema());
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::config;

//     #[tokio::test]
//     async fn test_database_connection() {
//         let db = database_connection().await;
//         assert!(db.is_ok());
//     }
// }