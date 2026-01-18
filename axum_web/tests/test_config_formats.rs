/// 测试 TOML 和 YAML 配置文件加载
use axum_web::config;

#[test]
fn test_config_loading() {
    let app_config = config::get();

    // 验证 TOML 配置被正确加载
    assert_eq!(app_config.server().port(), 4000);
    assert_eq!(app_config.database().host(), "127.0.0.1");
    assert_eq!(app_config.database().port(), 5432);
    assert_eq!(app_config.database().user(), "hejj");
    assert_eq!(app_config.database().password(), "pass12345");
    assert_eq!(app_config.database().database(), "postgres");
    assert_eq!(app_config.database().schema(), "public");

    println!("✓ Configuration loaded successfully!");
    println!("  Server port: {}", app_config.server().port());
    println!("  Database host: {}", app_config.database().host());
}
