use std::cmp::max;
use std::time::Duration;
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbBackend, QueryResult, Statement};
use crate::config;
use num_cpus;

// TODO 补充 postgres 数据库&表的创建

pub async fn init() -> anyhow::Result<DatabaseConnection> {
    let database_config = config::get().database();
    // "postgres://postgres:postgres123@127.0.0.1:5432/axum".to_string(); ✅
    let database_url: String = format!("postgres://{}:{}@{}:{}/{}", database_config.user(), database_config.password(), database_config.host(), database_config.port(), database_config.database());
    let mut options = ConnectOptions::new(
        database_url
    );

    // 根据cpu个数确定线程数, num-cpus::get
    let cpus: u32 = num_cpus::get() as u32;
    options.min_connections(max(cpus * 4, 10u32))
        .max_connections(max(cpus * 4, 20u32))
        .connect_timeout(Duration::from_secs(10))
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(3600 * 24))
        .sqlx_logging(false)
        .set_schema_search_path(database_config.schema());

    let db: DatabaseConnection = Database::connect(options).await?;
    db.ping().await?;
    tracing::info!("Database Connected Successfully!");

    log_database_version(&db).await?;
    Ok(db)
}

async fn log_database_version(db: &DatabaseConnection) -> anyhow::Result<()> {
    let version: QueryResult = db.query_one(
        Statement::from_string(DbBackend::Postgres, "SELECT version()")
    ).await?.ok_or_else(|| anyhow::anyhow!("Failed to get database Version!"))?; // 这里不能bail!()

    let version_str: String = version.try_get_by_index::<String>(0)?;
    tracing::info!("Database version: {}", version_str);
    Ok(())
}
