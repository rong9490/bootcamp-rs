# 配置文件格式说明

本项目支持两种配置文件格式：**TOML** 和 **YAML**。

## 配置文件优先级

系统会按以下优先级加载配置文件：

1. **application.toml** - 如果存在，优先使用
2. **application.yaml** - 如果 TOML 不存在，则使用 YAML
3. **环境变量** - 始终可以覆盖配置文件中的值（使用 `APP_` 前缀）

## TOML 格式示例

**文件名**: `application.toml`

```toml
[server]
port = 4000

[database]
host = "127.0.0.1"
port = 5432
user = "hejj"
password = "pass12345"
database = "postgres"
schema = "public"
```

## YAML 格式示例

**文件名**: `application.yaml`

```yaml
server:
  port: 4000
database:
  host: 127.0.0.1
  port: 5432
  user: hejj
  password: pass12345
  database: postgres
  schema: public
```

## 使用方法

### 使用 TOML 配置
只需创建 `application.toml` 文件，系统会自动检测并使用它：

```bash
# 创建 TOML 配置文件
cat > application.toml << 'EOF'
[server]
port = 4000

[database]
host = "127.0.0.1"
port = 5432
user = "hejj"
password = "pass12345"
database = "postgres"
schema = "public"
EOF

# 运行应用程序
cargo run --bin axum_web
```

### 使用 YAML 配置
如果不存在 `application.toml`，系统会使用 `application.yaml`：

```bash
# 确保 TOML 文件不存在
rm -f application.toml

# 使用现有的 YAML 配置
cargo run --bin axum_web
```

### 环境变量覆盖
无论使用哪种配置文件格式，都可以使用环境变量覆盖配置值：

```bash
# 覆盖服务器端口
export APP_SERVER_PORT=8080

# 覆盖数据库主机
export APP_DATABASE_HOST=localhost

# 运行应用程序
cargo run --bin axum_web
```

## 配置切换

如果同时存在 `application.toml` 和 `application.yaml`，系统会优先使用 TOML 格式。

要从 TOML 切换到 YAML：
```bash
rm application.toml  # 删除或重命名 TOML 文件
```

要从 YAML 切换到 TOML：
```bash
# 创建 TOML 文件（会自动优先使用）
cat > application.toml << 'EOF'
[server]
port = 4000
...
EOF
```

## 实现细节

配置加载逻辑位于 `src/config/mod.rs` 中的 `AppConfig::load()` 方法：

```rust
let config_file = std::path::Path::new("application.toml");
let (format, file_name) = if config_file.exists() {
    (FileFormat::Toml, "application.toml")
} else {
    (FileFormat::Yaml, "application.yaml")
};
```

系统会记录正在加载的配置文件：
```
INFO Loading configuration from: application.toml
```
或
```
INFO Loading configuration from: application.yaml
```

## 测试

运行测试以验证配置加载：
```bash
cargo test --package axum_web --test test_config_formats -- --nocapture
```
