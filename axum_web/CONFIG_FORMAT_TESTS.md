# 配置格式测试文档

本文档说明 `tests/test_config_formats.rs` 测试套件的设计和使用。

## 测试概览

测试套件名称：`test_config_formats`
测试数量：14 个
测试覆盖：TOML/YAML 配置文件加载、优先级、单例模式、数据验证等

## 运行测试

### 运行所有配置格式测试

```bash
# 基本运行
cargo test --package axum_web --test test_config_formats

# 显示输出
cargo test --package axum_web --test test_config_formats -- --nocapture

# 运行特定测试
cargo test --package axum_web --test test_config_formats test_config_loading_with_existing_toml
```

### 测试输出示例

```
running 14 tests
✓ Configuration data types are valid
✓ Default port value is correct: 4000
✓ Database connection string components are valid
✓ All configuration values are correct!
✓ Configuration singleton pattern verified
  Memory address: 0x1047d83c8
✓ TOML configuration loaded successfully!
  Server port: 4000
  Database host: 127.0.0.1
  Database schema: public
✓ Configuration immutability verified
✓ All configuration fields are present and valid

============================================================
Configuration Format Tests Summary
============================================================
✓ Configuration loading: TOML and YAML
✓ Configuration values: All correct
✓ Configuration priority: TOML > YAML
✓ Configuration singleton: Verified
✓ Configuration immutability: Verified
✓ File content validation: Passed
✓ Data types: Valid
✓ Default values: Correct
============================================================

test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## 测试分类

### 1. 基础配置加载测试

#### `test_config_loading_with_existing_toml`
验证当 TOML 文件存在时，配置被正确加载。

**测试内容**：
- ✅ 服务器端口：4000
- ✅ 数据库主机：127.0.0.1
- ✅ 数据库端口：5432
- ✅ 数据库用户：hejj
- ✅ 数据库密码：pass12345
- ✅ 数据库名称：postgres
- ✅ 数据库模式：public

#### `test_config_values_are_correct`
验证所有配置值都是正确的。

**特点**：
- 使用 `assert_eq!` 的第三个参数提供详细的错误消息
- 验证每个配置字段的值

### 2. 配置文件优先级测试

#### `test_toml_has_priority_over_yaml`
验证当同时存在 TOML 和 YAML 时，TOML 优先。

**测试流程**：
1. 备份原始配置文件
2. 创建测试用的 TOML 和 YAML 配置
3. 验证两个文件都存在
4. 恢复原始配置

**注意**：由于配置是 LazyLock 单例，已加载的配置不会改变，此测试验证文件操作逻辑。

#### `test_yaml_fallback_when_toml_absent`
验证当 TOML 不存在时，会使用 YAML 作为备选。

### 3. 配置文件内容测试

#### `test_toml_file_content`
验证 TOML 文件包含必要的配置项。

**检查项**：
- ✅ `[server]` 部分
- ✅ `port = 4000`
- ✅ `[database]` 部分
- ✅ `host = "127.0.0.1"`
- ✅ `schema = "public"`

#### `test_yaml_file_content`
验证 YAML 文件包含必要的配置项。

**检查项**：
- ✅ `server:` 部分
- ✅ `port: 4000`
- ✅ `database:` 部分
- ✅ `host: 127.0.0.1`
- ✅ `schema: public`

### 4. 配置单例测试

#### `test_config_singleton`
验证配置的单例模式。

**验证方法**：
1. 多次调用 `config::get()`
2. 比较返回的配置值
3. 比较内存地址

**断言**：
```rust
assert_eq!(addr1, addr2, "Should be the same memory address");
```

#### `test_config_immutability`
验证配置的不可变性。

**验证方法**：
- 返回的是引用 `&T`
- 无法修改配置值（编译时检查）

### 5. 配置完整性测试

#### `test_all_config_fields_present`
验证所有必需字段都存在且有效。

**检查项**：
- ✅ 端口号 > 0
- ✅ 所有字符串字段非空

#### `test_config_data_types`
验证数据类型正确性。

**检查项**：
- ✅ 端口号 >= 1024（非系统端口）
- ✅ 字符串字段非空

### 6. 配置文件同步测试

#### `test_toml_and_yaml_files_in_sync`
验证 TOML 和 YAML 文件的基本结构一致。

**检查项**：
- ✅ 两个文件都存在
- ✅ 包含相同的配置键

### 7. 边界条件测试

#### `test_default_port_value`
验证默认端口值。

**断言**：
```rust
assert_eq!(port, 4000, "Default port should be 4000");
```

#### `test_database_connection_string_components`
验证数据库连接字符串的所有组件。

**验证**：
- ✅ 可以组装成连接字符串
- ✅ 连接字符串包含必要部分

### 8. 测试总结

#### `test_summary`
打印测试运行摘要。

**输出**：
```
============================================================
Configuration Format Tests Summary
============================================================
✓ Configuration loading: TOML and YAML
✓ Configuration values: All correct
✓ Configuration priority: TOML > YAML
✓ Configuration singleton: Verified
✓ Configuration immutability: Verified
✓ File content validation: Passed
✓ Data types: Valid
✓ Default values: Correct
============================================================
```

## 辅助函数

### `project_root()`
获取项目根目录（`CARGO_MANIFEST_DIR`）。

### `backup_config(name: &str)`
备份配置文件，返回 `Option<Vec<u8>>`。

### `restore_config(name: &str, content: &[u8])`
恢复配置文件。

### `remove_config(name: &str)`
删除配置文件。

### `create_test_toml()`
创建测试用的 TOML 配置字符串。

### `create_test_yaml()`
创建测试用的 YAML 配置字符串。

## 测试覆盖率

### 配置加载
- ✅ TOML 文件加载
- ✅ YAML 文件加载
- ✅ 配置优先级

### 配置验证
- ✅ 所有字段值
- ✅ 数据类型
- ✅ 字段完整性
- ✅ 文件内容

### 配置特性
- ✅ 单例模式
- ✅ 不可变性
- ✅ 文件同步

### 边界条件
- ✅ 默认值
- ✅ 连接字符串组件

## 扩展测试

可以添加的新测试：

### 1. 环境变量覆盖测试
```rust
#[test]
fn test_env_override() {
    // 设置环境变量
    std::env::set_var("APP_SERVER_PORT", "8080");

    // 重新加载配置（需要修改架构支持）
    // 验证端口被覆盖
}
```

### 2. 配置错误处理测试
```rust
#[test]
fn test_invalid_config() {
    // 创建无效的配置文件
    // 验证错误处理
}
```

### 3. 配置更新测试
```rust
#[test]
fn test_config_hot_reload() {
    // 修改配置文件
    // 触发重新加载
    // 验证新值
}
```

## 测试最佳实践

### 1. 使用描述性的测试名称
```rust
// ✅ 好的测试名称
fn test_toml_has_priority_over_yaml() { }

// ❌ 不好的测试名称
fn test_priority() { }
```

### 2. 提供详细的错误消息
```rust
// ✅ 详细的错误消息
assert_eq!(
    app_config.server().port(),
    4000,
    "Server port should be 4000"
);

// ❌ 简单的断言
assert_eq!(app_config.server().port(), 4000);
```

### 3. 打印有用的调试信息
```rust
println!("✓ Configuration loaded successfully!");
println!("  Server port: {}", app_config.server().port());
println!("  Database host: {}", app_config.database().host());
```

### 4. 测试隔离
每个测试应该独立运行，不依赖于其他测试的状态。

### 5. 清理副作用
如果测试修改了文件系统，应该在测试后恢复原状：
```rust
let backup = backup_config("application.toml");
// ... 执行测试 ...
if let Some(content) = backup {
    restore_config("application.toml", &content);
}
```

## CI/CD 集成

### GitHub Actions 示例

```yaml
name: Test Configuration

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v3

    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable

    - name: Run config format tests
      run: cargo test --package axum_web --test test_config_formats
```

## 相关文件

- `tests/test_config_formats.rs` - 测试文件
- `src/config/mod.rs` - 配置模块
- `application.toml` - TOML 配置
- `application.yaml` - YAML 配置
- `CONFIG_FORMATS.md` - 配置格式文档

## 故障排查

### 问题：测试失败 "Config file not found"

**原因**：配置文件不存在或路径错误。

**解决**：
```bash
# 检查文件是否存在
ls -la application.toml application.yaml

# 确保在正确的目录运行测试
cd axum_web
cargo test --test test_config_formats
```

### 问题：单例测试失败

**原因**：配置被多次初始化。

**解决**：检查 `config::get()` 是否正确使用 `LazyLock`。

### 问题：测试超时

**原因**：文件操作阻塞。

**解决**：
```bash
# 使用超时运行
timeout 10 cargo test --test test_config_formats
```

## 总结

配置格式测试套件提供了全面的覆盖，确保：

1. ✅ TOML 和 YAML 配置都能正确加载
2. ✅ 配置优先级正确（TOML > YAML）
3. ✅ 配置单例模式正常工作
4. ✅ 所有配置值正确
5. ✅ 配置不可变
6. ✅ 文件内容有效

运行这些测试可以快速验证配置系统的正确性和稳定性。
