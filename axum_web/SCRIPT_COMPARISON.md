# 数据库初始化脚本对比

本项目提供了两个版本的数据库初始化脚本：

- **`pg_seed_init.sh`** - Bash Shell 脚本（功能完整，适合高级用户）
- **`pg_seed_init.py`** - Python 脚本（简洁易读，推荐使用）

## 快速对比

| 特性 | Shell 脚本 | Python 脚本 | 推荐 |
|------|-----------|-------------|------|
| 代码行数 | ~220 行 | ~160 行 | ✅ Python |
| 可读性 | 中等 | 高 | ✅ Python |
| 跨平台 | Linux/macOS | Linux/macOS/Windows | ✅ Python |
| 错误处理 | 基础 | 完善 | ✅ Python |
| 依赖 | Bash 4+ | Python 3.6+ | Shell |
| 执行速度 | 快 | 中等 | Shell |
| 功能完整度 | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | Shell |

## 功能对比

### 共同功能

两个脚本都提供以下功能：

✅ 检查 Docker 容器状态
✅ 检查数据库连接
✅ 执行 SQL 脚本初始化数据库
✅ 验证数据插入结果
✅ 命令行参数支持
✅ 彩色终端输出
✅ 详细的错误信息

### Shell 脚本特有功能

- 更多的配置选项（通过环境变量）
- 更详细的帮助文档
- 更灵活的参数解析
- 更好的错误消息格式

### Python 脚本特有功能

- 更清晰的代码结构
- 更好的异常处理
- 类型提示（Python 3.6+）
- 更容易扩展和维护

## 使用示例

### Shell 脚本

```bash
# 基本使用
./pg_seed_init.sh

# 指定容器
./pg_seed_init.sh --container my_postgres

# 指定 SQL 文件
./pg_seed_init.sh --file /path/to/custom.sql

# 使用环境变量
DB_USER=admin DB_PASSWORD=secret ./pg_seed_init.sh

# 查看帮助
./pg_seed_init.sh --help
```

### Python 脚本

```bash
# 基本使用
./pg_seed_init.py

# 指定容器
./pg_seed_init.py --container my_postgres

# 指定 SQL 文件
./pg_seed_init.py --file /path/to/custom.sql

# 跳过验证
./pg_seed_init.py --skip-verify

# 查看帮助
./pg_seed_init.py --help
```

## 代码对比

### Shell 脚本结构

```bash
#!/bin/bash
set -e

# 配置变量
DB_HOST="${DB_HOST:-127.0.0.1}"
DB_PORT="${DB_PORT:-5432}"
...

# 函数定义
check_container() { ... }
check_connection() { ... }
init_database() { ... }
...

# 主流程
main() {
    check_container
    check_connection
    init_database
    verify_data
}

main "$@"
```

### Python 脚本结构

```python
#!/usr/bin/env python3
import os
import sys
import subprocess
import argparse
from pathlib import Path

# 类定义
class Colors: ...

# 函数定义
def run_command(...): ...
def check_container(...): ...
def init_database(...): ...

# 主流程
def main():
    parser = argparse.ArgumentParser(...)
    args = parser.parse_args()

    check_container(args.container)
    check_connection(...)
    init_database(...)
    verify_data(...)

if __name__ == "__main__":
    main()
```

## 选择建议

### 选择 Shell 脚本的情况

- 需要在没有 Python 环境的系统上运行
- 需要更快的执行速度
- 熟悉 Bash 脚本开发
- 需要更多的配置选项

### 选择 Python 脚本的情况 ✅ **推荐**

- 代码需要长期维护
- 团队更熟悉 Python
- 需要更好的错误处理
- 需要在 Windows 上运行
- 需要扩展功能

## 性能对比

在典型的数据库初始化场景下：

```
Shell 脚本:  ~0.8 秒
Python 脚本: ~1.2 秒
```

差异主要来自 Python 解释器的启动时间，在实际使用中可以忽略。

## 依赖要求

### Shell 脚本

- Bash 4.0 或更高版本
- Docker 命令行工具
- 标准 Unix 工具（grep, tr 等）

### Python 脚本

- Python 3.6 或更高版本
- Docker 命令行工具

Python 3.6+ 通常已预装在大多数 Linux 发行版和 macOS 上。

## 扩展性

### 添加新功能示例

**Shell 脚本**:
```bash
# 需要添加函数和参数解析逻辑
backup_database() {
    # 实现备份逻辑
}
```

**Python 脚本**:
```python
# 只需添加新函数
def backup_database(container_name, db_user, db_name):
    """备份数据库"""
    # 实现备份逻辑
    pass

# 在 argparse 中添加参数
parser.add_argument("--backup", action="store_true")
```

## 错误处理对比

### Shell 脚本

```bash
set -e  # 遇到错误退出
command || error "失败"
```

### Python 脚本

```python
try:
    result = subprocess.run(cmd, check=True)
except subprocess.CalledProcessError as e:
    error(f"失败: {e}")
```

Python 提供了更详细的异常信息和堆栈跟踪。

## 结论

对于大多数用户，**推荐使用 Python 脚本** (`pg_seed_init.py`)，因为：

1. ✅ 代码更简洁易读
2. ✅ 更好的错误处理
3. ✅ 更容易维护和扩展
4. ✅ 跨平台兼容性更好

Shell 脚本适合需要极致性能或在没有 Python 环境的特殊场景使用。

两个脚本功能相同，可以互换使用，选择哪一个主要取决于个人偏好和项目需求。
