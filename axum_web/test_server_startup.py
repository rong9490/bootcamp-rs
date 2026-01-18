#!/usr/bin/env python3
"""
服务器启动测试脚本 - Python 版本

功能：
1. 启动 Rust 服务器
2. 捕获日志输出
3. 测试 HTTP 端点
4. 优雅地停止服务器
5. 显示完整的启动日志
"""


# 查看占用端口 4000 的进程
#lsof -ti:4000
# 清理端口
# lsof -ti:4000 | xargs kill -9

import subprocess
import time
import sys
import os
import signal
from pathlib import Path
from typing import Optional


class Colors:
    """终端颜色"""
    GREEN = '\033[0;32m'
    BLUE = '\033[0;34m'
    YELLOW = '\033[1;33m'
    RED = '\033[0;31m'
    CYAN = '\033[0;36m'
    NC = '\033[0m'  # No Color


def print_header(title: str):
    """打印标题"""
    print(f"\n{Colors.CYAN}{'=' * 60}{Colors.NC}")
    print(f"{Colors.CYAN}{title}{Colors.NC}")
    print(f"{Colors.CYAN}{'=' * 60}{Colors.NC}\n")


def print_info(msg: str):
    """打印信息"""
    print(f"{Colors.BLUE}[INFO]{Colors.NC} {msg}")


def print_success(msg: str):
    """打印成功消息"""
    print(f"{Colors.GREEN}[✓]{Colors.NC} {msg}")


def print_warning(msg: str):
    """打印警告"""
    print(f"{Colors.YELLOW}[!]{Colors.NC} {msg}")


def print_error(msg: str):
    """打印错误"""
    print(f"{Colors.RED}[✗]{Colors.NC} {msg}")


class ServerTester:
    """服务器测试类"""

    def __init__(self, output_file: str = "/tmp/axum_output.log"):
        self.output_file = output_file
        self.process: Optional[subprocess.Popen] = None
        self.project_dir = Path(__file__).parent

    def start_server(self) -> bool:
        """启动服务器"""
        print_info("正在启动服务器...")

        try:
            # 打开输出文件
            with open(self.output_file, 'w') as f:
                # 启动 cargo run，并将输出重定向到文件
                self.process = subprocess.Popen(
                    ['cargo', 'run', '--bin', 'axum_web'],
                    cwd=self.project_dir,
                    stdout=f,
                    stderr=subprocess.STDOUT,
                    preexec_fn=os.setsid if hasattr(os, 'setsid') else None
                )

            print_success(f"服务器已启动 (PID: {self.process.pid})")
            return True

        except Exception as e:
            print_error(f"启动服务器失败: {e}")
            return False

    def wait_for_server(self, timeout: int = 10) -> bool:
        """等待服务器启动"""
        print_info(f"等待服务器启动 (最多 {timeout} 秒)...")

        for i in range(timeout):
            try:
                # 检查进程是否还在运行
                if self.process.poll() is not None:
                    print_error("服务器进程意外退出")
                    return False

                # 检查输出文件是否有内容
                if os.path.exists(self.output_file):
                    with open(self.output_file, 'r') as f:
                        content = f.read()
                        if "HTTP server listening" in content or "Database Connected" in content:
                            print_success(f"服务器启动成功 (耗时 {i + 1} 秒)")
                            return True

            except Exception as e:
                print_warning(f"检查服务器状态时出错: {e}")

            time.sleep(1)

        print_warning("服务器启动超时，但继续测试...")
        return True

    def test_http_endpoints(self) -> bool:
        """测试 HTTP 端点"""
        print_info("测试 HTTP 端点...")

        import urllib.request
        import urllib.error
        import json

        endpoints = [
            ("健康检查", "http://127.0.0.1:4000/health"),
            ("首页", "http://127.0.0.1:4000/"),
            ("用户列表", "http://127.0.0.1:4000/users"),
        ]

        all_passed = True

        for name, url in endpoints:
            try:
                print(f"  测试 {name}: {url}")
                with urllib.request.urlopen(url, timeout=2) as response:
                    data = response.read().decode('utf-8')
                    print_success(f"  {name} - 状态码: {response.status}")

                    # 尝试解析 JSON
                    try:
                        json_data = json.loads(data)
                        print(f"    响应: {json.dumps(json_data, ensure_ascii=False)}")
                    except:
                        if len(data) < 100:
                            print(f"    响应: {data}")
                        else:
                            print(f"    响应长度: {len(data)} 字节")

            except urllib.error.HTTPError as e:
                print_error(f"  {name} - HTTP 错误: {e.code}")
                all_passed = False
            except urllib.error.URLError as e:
                print_error(f"  {name} - 连接失败: {e.reason}")
                all_passed = False
            except Exception as e:
                print_error(f"  {name} - 错误: {e}")
                all_passed = False

        return all_passed

    def display_logs(self, lines: int = 30):
        """显示日志输出"""
        print_info(f"显示最近 {lines} 行日志...")

        if not os.path.exists(self.output_file):
            print_warning("日志文件不存在")
            return

        with open(self.output_file, 'r') as f:
            content = f.read()

        if not content:
            print_warning("日志文件为空")
            return

        # 分割成行
        lines_list = content.split('\n')

        # 显示最后 N 行
        recent_lines = lines_list[-lines:] if len(lines_list) > lines else lines_list

        print("\n" + Colors.CYAN + "-" * 60 + Colors.NC)
        for line in recent_lines:
            if line.strip():
                # 高亮日志级别
                line = line.replace("[INFO", Colors.GREEN + "[INFO" + Colors.NC)
                line = line.replace("[WARN", Colors.YELLOW + "[WARN" + Colors.NC)
                line = line.replace("[ERROR", Colors.RED + "[ERROR" + Colors.NC)
                print(line)
        print(Colors.CYAN + "-" * 60 + Colors.NC)

    def stop_server(self):
        """停止服务器"""
        print_info("正在停止服务器...")

        if self.process is None:
            print_warning("服务器进程未启动")
            return

        try:
            # 发送 SIGTERM 信号
            if hasattr(os, 'killpg'):
                os.killpg(os.getpgid(self.process.pid), signal.SIGTERM)
            else:
                self.process.terminate()

            # 等待进程结束
            try:
                self.process.wait(timeout=5)
                print_success("服务器已停止")
            except subprocess.TimeoutExpired:
                print_warning("服务器未响应 SIGTERM，使用 SIGKILL")
                if hasattr(os, 'killpg'):
                    os.killpg(os.getpgid(self.process.pid), signal.SIGKILL)
                else:
                    self.process.kill()
                self.process.wait()
                print_success("服务器已强制停止")

        except Exception as e:
            print_error(f"停止服务器时出错: {e}")

    def cleanup(self):
        """清理临时文件"""
        if os.path.exists(self.output_file):
            try:
                os.remove(self.output_file)
                print_info("已清理临时日志文件")
            except Exception as e:
                print_warning(f"清理日志文件失败: {e}")

    def run(self):
        """运行完整的测试流程"""
        try:
            # 1. 启动服务器
            print_header("步骤 1: 启动服务器")
            if not self.start_server():
                return False

            # 2. 等待服务器启动
            print_header("步骤 2: 等待服务器就绪")
            if not self.wait_for_server():
                return False

            # 3. 显示启动日志
            print_header("步骤 3: 启动日志")
            self.display_logs(lines=20)

            # 4. 测试 HTTP 端点
            print_header("步骤 4: 测试 HTTP 端点")
            all_passed = self.test_http_endpoints()

            if all_passed:
                print_success("所有端点测试通过")
            else:
                print_warning("部分端点测试失败")

            # 5. 显示完整日志
            print_header("步骤 5: 完整日志输出")
            self.display_logs(lines=50)

            # 6. 停止服务器
            print_header("步骤 6: 停止服务器")
            self.stop_server()

            print_header("测试完成")
            if all_passed:
                print_success("✓ 所有测试通过")
                return True
            else:
                print_warning("⚠ 部分测试失败")
                return False

        except KeyboardInterrupt:
            print_warning("\n收到中断信号，正在停止...")
            self.stop_server()
            return False
        finally:
            # 询问是否清理日志文件
            try:
                if os.path.exists(self.output_file):
                    response = input(f"\n是否保留日志文件? {self.output_file} [y/N]: ")
                    if response.lower() != 'y':
                        self.cleanup()
                    else:
                        print_info(f"日志文件保存在: {self.output_file}")
            except:
                pass


def main():
    """主函数"""
    print(f"""
{Colors.CYAN}
╔══════════════════════════════════════════════════════════╗
║         Axum Web 服务器启动测试工具 (Python 版)            ║
╚══════════════════════════════════════════════════════════╝
{Colors.NC}
    """)

    # 解析命令行参数
    import argparse
    parser = argparse.ArgumentParser(description="测试 Axum Web 服务器启动")
    parser.add_argument(
        '--output',
        default='/tmp/axum_output.log',
        help='日志输出文件路径'
    )
    parser.add_argument(
        '--no-cleanup',
        action='store_true',
        help='不清理日志文件'
    )

    args = parser.parse_args()

    # 运行测试
    tester = ServerTester(output_file=args.output)
    success = tester.run()

    sys.exit(0 if success else 1)


if __name__ == "__main__":
    main()
