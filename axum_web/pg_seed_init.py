#!/usr/bin/env python3
"""
PostgreSQL 数据库初始化脚本
功能：在 Docker 启动数据库后注入种子数据
"""

import os
import sys
import subprocess
import argparse
from pathlib import Path


class Colors:
    """终端颜色"""
    GREEN = '\033[0;32m'
    YELLOW = '\033[1;33m'
    RED = '\033[0;31m'
    NC = '\033[0m'  # No Color


def info(msg: str):
    """打印信息"""
    print(f"{Colors.GREEN}[INFO]{Colors.NC} {msg}")


def warn(msg: str):
    """打印警告"""
    print(f"{Colors.YELLOW}[WARN]{Colors.NC} {msg}")


def error(msg: str):
    """打印错误并退出"""
    print(f"{Colors.RED}[ERROR]{Colors.NC} {msg}")
    sys.exit(1)


def run_command(cmd: list, check: bool = True, capture: bool = True, input: bytes = None) -> subprocess.CompletedProcess:
    """运行命令"""
    try:
        result = subprocess.run(
            cmd,
            check=check,
            capture_output=capture,
            text=False,
            input=input
        )
        if capture and not input:
            result = subprocess.CompletedProcess(
                args=cmd,
                returncode=result.returncode,
                stdout=result.stdout.decode() if result.stdout else '',
                stderr=result.stderr.decode() if result.stderr else ''
            )
        return result
    except subprocess.CalledProcessError as e:
        if capture and e.stderr:
            stderr_text = e.stderr.decode() if isinstance(e.stderr, bytes) else e.stderr
            error(f"命令执行失败: {' '.join(cmd)}\n{stderr_text}")
        error(f"命令执行失败: {' '.join(cmd)}")


def check_container(container_name: str) -> bool:
    """检查 Docker 容器是否运行"""
    info("检查 Docker 容器状态...")

    result = run_command(["docker", "ps", "--filter", f"name={container_name}"])
    if container_name in result.stdout:
        info(f"Docker 容器 '{container_name}' 正在运行")
        return True
    else:
        error(f"Docker 容器 '{container_name}' 未运行")


def check_connection(container_name: str, db_user: str, db_name: str) -> bool:
    """检查数据库连接"""
    info("检查数据库连接...")

    result = run_command([
        "docker", "exec", container_name,
        "pg_isready", "-U", db_user, "-d", db_name
    ], check=False, capture=True)

    if result.returncode == 0:
        info("数据库连接正常")
        return True
    else:
        error("数据库连接失败")


def init_database(container_name: str, db_user: str, db_name: str, sql_file: Path):
    """执行数据库初始化"""
    info("开始初始化数据库...")

    # 读取 SQL 文件
    if not sql_file.exists():
        error(f"SQL 文件不存在: {sql_file}")

    info(f"执行 SQL 脚本: {sql_file}")

    # 通过 Docker 执行 SQL 文件
    with open(sql_file, 'r') as f:
        sql_content = f.read()

    run_command([
        "docker", "exec", "-i", container_name,
        "psql", "-U", db_user, "-d", db_name, "-v", "ON_ERROR_STOP=1"
    ], input=sql_content.encode())

    info("数据库初始化完成！")


def verify_data(container_name: str, db_user: str, db_name: str):
    """验证数据"""
    info("验证数据...")

    # 验证 sys_user 表
    result = run_command([
        "docker", "exec", container_name,
        "psql", "-U", db_user, "-d", db_name,
        "-t", "-c", "SELECT COUNT(*) FROM sys_user;"
    ])

    count = result.stdout.strip()
    expected = "3"

    if count == expected:
        info(f"sys_user 表验证成功：已插入 {count} 条记录")
    else:
        warn(f"警告：预期插入 {expected} 条用户记录，实际插入 {count} 条")

    # 验证 sys_user_login_log 表
    result = run_command([
        "docker", "exec", container_name,
        "psql", "-U", db_user, "-d", db_name,
        "-t", "-c", "SELECT COUNT(*) FROM sys_user_login_log;"
    ])

    count = result.stdout.strip()
    expected = "8"

    if count == expected:
        info(f"sys_user_login_log 表验证成功：已插入 {count} 条登录记录")
    else:
        warn(f"警告：预期插入 {expected} 条登录记录，实际插入 {count} 条")


def main():
    """主流程"""
    # 默认配置
    default_container = os.getenv("CONTAINER_NAME", "pg_dev")
    default_db_user = os.getenv("DB_USER", "hejj")
    default_db_name = os.getenv("DB_NAME", "postgres")
    script_dir = Path(__file__).parent
    default_sql_file = script_dir / "schema.sql"

    # 解析命令行参数
    parser = argparse.ArgumentParser(
        description="PostgreSQL 数据库初始化脚本",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
示例:
  # 使用默认配置
  %(prog)s

  # 指定容器名称
  %(prog)s --container my_postgres

  # 指定 SQL 文件
  %(prog)s --file /path/to/custom.sql

  # 跳过数据验证
  %(prog)s --skip-verify
        """
    )

    parser.add_argument(
        "-c", "--container",
        default=default_container,
        help="Docker 容器名称 (默认: %(default)s)"
    )
    parser.add_argument(
        "-f", "--file",
        type=Path,
        default=default_sql_file,
        help="SQL 文件路径 (默认: %(default)s)"
    )
    parser.add_argument(
        "--skip-verify",
        action="store_true",
        help="跳过数据验证"
    )

    args = parser.parse_args()

    # 打印标题
    print("=" * 40)
    print("PostgreSQL 数据库初始化")
    print("=" * 40)
    print()

    # 执行检查和初始化
    check_container(args.container)
    check_connection(args.container, default_db_user, default_db_name)
    init_database(args.container, default_db_user, default_db_name, args.file)

    # 验证数据
    if not args.skip_verify:
        verify_data(args.container, default_db_user, default_db_name)

    print()
    info("✓ 所有操作完成！")
    print("=" * 40)


if __name__ == "__main__":
    main()
