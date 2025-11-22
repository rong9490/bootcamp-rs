use sea_orm::{ConnectOptions, DatabaseConnection};
use crate::config;

pub async fn init() -> anyhow::Result<DatabaseConnection> {
    let database_config = config::get().database();
    let database_url: String = format!("postgres://{}:{}@{}:{}/{}", database_config.user(), database_config.password(), database_config.host(), database_config.port(), database_config.schema());
    let mut options = ConnectOptions::new(
        database_url
    );

    // TODO 根据cpu个数确定线程数, num-cpus::get
    options.min_connections(10);

    todo!()
}