/** 数据库配置 */
use std::cmp::max;
use std::time::Duration;
use num_cpus;
use sea_orm::ConnectOptions;

pub fn get_database_default_url() -> String {
    "postgres://hejj:pass12345@127.0.0.1:5432/postgres".to_string()
}

pub fn get_cpus() -> u32 {
    let cpus: u32 = num_cpus::get() as u32;
    cpus
}

pub fn gen_database_options(_database_schema: Option<String>) -> ConnectOptions {
    let database_schema: String = _database_schema.unwrap_or_else(|| "public".to_string()); // 给默认值
    let database_url: String = get_database_default_url();
    let cpus: u32 = get_cpus();
    let mut database_options: ConnectOptions = ConnectOptions::new(database_url);
    // 根据cpu个数确定线程数, num-cpus::get
    database_options
        .min_connections(max(cpus * 4, 10u32))
        .max_connections(max(cpus * 4, 20u32))
        .connect_timeout(Duration::from_secs(3))
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(3600 * 24))
        .sqlx_logging(false)
        .set_schema_search_path(database_schema);
    database_options
}

// cargo watch -x "test --package axum_web --lib -- database::sea_orm::database_options::tests --nocapture"
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let database_url: String = get_database_default_url();
        print!("数据库地址: {}", database_url);
    }
}
