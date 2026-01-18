# 服务器测试脚本对比

本项目提供了两个版本的测试脚本，用于验证 Axum Web 服务器的启动和运行。

## 脚本列表

1. **test_server_startup.sh** - Bash Shell 版本
2. **test_server_startup.py** - Python 版本（推荐）

## 快速对比

| 特性 | Shell 脚本 | Python 脚本 | 推荐 |
|------|-----------|-------------|------|
| 跨平台兼容性 | Linux/macOS | 所有平台 | ✅ Python |
| 输出格式控制 | 基础 | 彩色、结构化 | ✅ Python |
| 错误处理 | 基础 | 完善的异常处理 | ✅ Python |
| HTTP 测试 | curl | urllib (内置) | ✅ Python |
| 进程管理 | 基础 | 进程组管理 | ✅ Python |
| 日志高亮 | 无 | 彩色日志级别 | ✅ Python |
| 用户交互 | 无 | 交互式提示 | ✅ Python |
| 代码可读性 | 中等 | 高（类结构） | ✅ Python |
| 执行速度 | 快 | 快 | Shell |

## Python 脚本测试输出示例

```
╔══════════════════════════════════════════════════════════╗
║         Axum Web 服务器启动测试工具 (Python 版)            ║
╚══════════════════════════════════════════════════════════╝

============================================================
步骤 1: 启动服务器
============================================================

[INFO] 正在启动服务器...
[✓] 服务器已启动 (PID: 82246)

============================================================
步骤 2: 等待服务器就绪
============================================================

[INFO] 等待服务器启动 (最多 10 秒)...
[✓] 服务器启动成功 (耗时 4 秒)

============================================================
步骤 3: 启动日志
============================================================

[INFO] 显示最近 20 行日志...
------------------------------------------------------------
[INFO] HTTP server listening on http://0.0.0.0:4000
------------------------------------------------------------

============================================================
步骤 4: 测试 HTTP 端点
============================================================

[INFO] 测试 HTTP 端点...
  测试 健康检查: http://127.0.0.1:4000/health
[✓]   健康检查 - 状态码: 200
    响应: {"status": "ok"}
  测试 首页: http://127.0.0.1:4000/
[✓]   首页 - 状态码: 200
    响应: Hello, axum!
  测试 用户列表: http://127.0.0.1:4000/users
[✓]   用户列表 - 状态码: 200
    响应: path: /users
[✓] 所有端点测试通过

============================================================
步骤 5: 完整日志输出
============================================================

[INFO] 显示最近 50 行日志...
------------------------------------------------------------
[INFO] Database Connected Successfully!
[INFO] Database version: PostgreSQL 18.1 ...
[INFO] Loading configuration from: application.toml
[INFO] HTTP server listening on http://0.0.0.0:4000
------------------------------------------------------------

============================================================
步骤 6: 停止服务器
============================================================

[INFO] 正在停止服务器...
[✓] 服务器已停止

============================================================
测试完成
============================================================

[✓] ✓ 所有测试通过
```

## 功能对比详情

### 1. 启动服务器

**Shell 脚本**：
```bash
cargo run --bin axum_web > /tmp/axum_output.log 2>&1 &
SERVER_PID=$!
```

**Python 脚本**：
```python
self.process = subprocess.Popen(
    ['cargo', 'run', '--bin', 'axum_web'],
    cwd=self.project_dir,
    stdout=f,
    stderr=subprocess.STDOUT,
    preexec_fn=os.setsid if hasattr(os, 'setsid') else None
)
```

**优势**：
- Python 使用进程组，可以更好地管理子进程
- Python 提供更详细的启动信息（PID）

### 2. 等待服务器就绪

**Shell 脚本**：
```bash
sleep 2  # 简单等待固定时间
```

**Python 脚本**：
```python
for i in range(timeout):
    # 检查输出文件，查找关键日志
    if "HTTP server listening" in content or "Database Connected" in content:
        print_success(f"服务器启动成功 (耗时 {i + 1} 秒)")
        return True
    time.sleep(1)
```

**优势**：
- Python 智能检测服务器是否真正就绪
- 显示实际启动耗时

### 3. HTTP 端点测试

**Shell 脚本**：
```bash
curl -s http://127.0.0.1:4000/health
```

**Python 脚本**：
```python
import urllib.request

with urllib.request.urlopen(url, timeout=2) as response:
    data = response.read().decode('utf-8')
    json_data = json.loads(data)
    print(f"响应: {json.dumps(json_data, ensure_ascii=False)}")
```

**优势**：
- Python 无需外部依赖（curl）
- 自动解析和美化 JSON 响应
- 提供详细的错误信息
- 测试多个端点

### 4. 日志显示

**Shell 脚本**：
```bash
cat /tmp/axum_output.log | head -30
```

**Python 脚本**：
```python
# 高亮日志级别
line = line.replace("[INFO", Colors.GREEN + "[INFO" + Colors.NC)
line = line.replace("[WARN", Colors.YELLOW + "[WARN" + Colors.NC)
line = line.replace("[ERROR", Colors.RED + "[ERROR" + Colors.NC)
```

**优势**：
- Python 彩色高亮日志级别
- 只显示最后 N 行，避免输出过多
- 更好的格式化

### 5. 停止服务器

**Shell 脚本**：
```bash
kill $SERVER_PID 2>/dev/null
```

**Python 脚本**：
```python
# 优雅关闭：先 SIGTERM，超时后 SIGKILL
os.killpg(os.getpgid(self.process.pid), signal.SIGTERM)
try:
    self.process.wait(timeout=5)
except subprocess.TimeoutExpired:
    os.killpg(os.getpgid(self.process.pid), signal.SIGKILL)
```

**优势**：
- Python 支持优雅关闭（先 SIGTERM，后 SIGKILL）
- 更好的错误处理

### 6. 用户交互

**Shell 脚本**：
- 无用户交互

**Python 脚本**：
```python
response = input(f"\n是否保留日志文件? {self.output_file} [y/N]: ")
if response.lower() != 'y':
    self.cleanup()
```

**优势**：
- Python 提供交互式选项
- 更友好的用户体验

## 使用方式

### Shell 脚本

```bash
# 基本使用
./test_server_startup.sh

# 查看脚本内容
cat test_server_startup.sh
```

### Python 脚本

```bash
# 基本使用
python3 test_server_startup.py

# 指定日志输出文件
python3 test_server_startup.py --output /tmp/my_log.log

# 不清理日志文件
python3 test_server_startup.py --no-cleanup

# 查看帮助
python3 test_server_startup.py --help
```

## Python 脚本结构

```python
class ServerTester:
    """服务器测试类"""

    def __init__(self, output_file): ...
    def start_server(self) -> bool: ...       # 启动服务器
    def wait_for_server(self, timeout) -> bool: ...  # 等待就绪
    def test_http_endpoints(self) -> bool: ...        # 测试 HTTP
    def display_logs(self, lines): ...         # 显示日志
    def stop_server(self): ...                 # 停止服务器
    def cleanup(self): ...                     # 清理文件
    def run(self): ...                         # 运行测试
```

## 依赖要求

### Shell 脚本
- Bash 4.0+
- curl
- 标准 Unix 工具（sleep, kill, cat, tail）

### Python 脚本
- Python 3.6+
- 标准库（subprocess, urllib, json, signal, os）
- **无外部依赖**

## 选择建议

### 使用 Shell 脚本的情况
- 需要最快的执行速度
- 在纯 Unix/Linux 环境中运行
- 需要最小的依赖

### 使用 Python 脚本的情况 ✅ **推荐**
- 需要跨平台兼容性
- 需要更好的错误处理
- 需要彩色输出和格式化
- 需要交互式功能
- 需要详细的测试报告
- 团队更熟悉 Python

## 扩展性

### Python 脚本易于扩展

```python
# 添加性能测试
def test_performance(self):
    """测试服务器性能"""
    import time
    start = time.time()
    for i in range(100):
        urllib.request.urlopen("http://127.0.0.1:4000/health")
    elapsed = time.time() - start
    print(f"100 次请求耗时: {elapsed:.2f} 秒")

# 添加并发测试
def test_concurrent_requests(self):
    """测试并发请求"""
    import concurrent.futures
    with concurrent.futures.ThreadPoolExecutor(max_workers=10) as executor:
        futures = [
            executor.submit(self._make_request, url)
            for url in urls
        ]
```

## 结论

对于大多数场景，**Python 脚本是更好的选择**，因为：

1. ✅ 更强大的错误处理
2. ✅ 更好的跨平台兼容性
3. ✅ 更丰富的功能（彩色输出、交互式提示）
4. ✅ 更易于扩展和维护
5. ✅ 无需外部依赖（curl 等）
6. ✅ 面向对象的结构，代码更清晰

Shell 脚本适合需要极致性能或纯 Unix 环境的场景。

两个脚本功能相同，可以互换使用，选择哪一个主要取决于个人偏好和项目需求。
