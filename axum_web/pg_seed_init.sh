#! /bin/bash

set -e  # 遇到错误立即退出

#=============================================================================================
# PostgreSQL 数据库初始化脚本
# 功能：在 Docker 启动数据库后注入种子数据
# 用途：方便测试调试，随时还原数据库状态
#=============================================================================================

# 数据库连接配置
DB_HOST="${DB_HOST:-127.0.0.1}"
DB_PORT="${DB_PORT:-5432}"
DB_USER="${DB_USER:-hejj}"
DB_PASSWORD="${DB_PASSWORD:-pass12345}"
DB_NAME="${DB_NAME:-postgres}"
DB_SCHEMA="${DB_SCHEMA:-public}"

# Docker 容器配置
CONTAINER_NAME="${CONTAINER_NAME:-pg_dev}"

# SQL 脚本路径
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SQL_FILE="${SQL_FILE:-$SCRIPT_DIR/schema.sql}"

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

#=============================================================================================
# 函数定义
#=============================================================================================

# 打印信息
info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

# 打印警告
warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

# 打印错误
error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

# 检查 Docker 容器是否运行
check_container() {
    info "检查 Docker 容器状态..."

    if docker ps | grep -q "${CONTAINER_NAME}"; then
        info "Docker 容器 '${CONTAINER_NAME}' 正在运行"
        return 0
    else
        error "Docker 容器 '${CONTAINER_NAME}' 未运行"
        return 1
    fi
}

# 检查数据库连接
check_connection() {
    info "检查数据库连接..."

    if docker exec "${CONTAINER_NAME}" pg_isready -U "${DB_USER}" -d "${DB_NAME}" > /dev/null 2>&1; then
        info "数据库连接正常"
        return 0
    else
        error "数据库连接失败"
        return 1
    fi
}

# 检查 SQL 文件是否存在
check_sql_file() {
    info "检查 SQL 文件: ${SQL_FILE}"

    if [ ! -f "${SQL_FILE}" ]; then
        error "SQL 文件不存在: ${SQL_FILE}"
        return 1
    else
        info "SQL 文件存在"
        return 0
    fi
}

# 执行数据库初始化
init_database() {
    info "开始初始化数据库..."
    info "  主机: ${DB_HOST}"
    info "  端口: ${DB_PORT}"
    info "  用户: ${DB_USER}"
    info "  数据库: ${DB_NAME}"
    info "  模式: ${DB_SCHEMA}"

    # 通过 Docker 执行 SQL 文件
    info "执行 SQL 脚本..."

    docker exec -i "${CONTAINER_NAME}" psql \
        -U "${DB_USER}" \
        -d "${DB_NAME}" \
        -v ON_ERROR_STOP=1 \
        < "${SQL_FILE}" || error "执行 SQL 脚本失败"

    info "数据库初始化完成！"
}

# 验证数据
verify_data() {
    info "验证数据..."

    # 验证 sys_user 表
    COUNT=$(docker exec "${CONTAINER_NAME}" psql \
        -U "${DB_USER}" \
        -d "${DB_NAME}" \
        -t -c "SELECT COUNT(*) FROM sys_user;" | tr -d ' ')

    if [ "${COUNT}" -eq "3" ]; then
        info "sys_user 表验证成功：已插入 ${COUNT} 条记录"
    else
        warn "警告：预期插入 3 条用户记录，实际插入 ${COUNT} 条"
    fi

    # 验证 sys_user_login_log 表
    COUNT=$(docker exec "${CONTAINER_NAME}" psql \
        -U "${DB_USER}" \
        -d "${DB_NAME}" \
        -t -c "SELECT COUNT(*) FROM sys_user_login_log;" | tr -d ' ')

    if [ "${COUNT}" -eq "8" ]; then
        info "sys_user_login_log 表验证成功：已插入 ${COUNT} 条登录记录"
    else
        warn "警告：预期插入 8 条登录记录，实际插入 ${COUNT} 条"
    fi
}

# 显示帮助信息
show_help() {
    cat << EOF
PostgreSQL 数据库初始化脚本

用法: $0 [选项]

选项:
    -h, --help              显示此帮助信息
    -c, --container NAME    指定 Docker 容器名称 (默认: pg_dev)
    -f, --file PATH         指定 SQL 文件路径 (默认: ./schema.sql)
    --skip-verify           跳过数据验证

环境变量:
    DB_HOST          数据库主机 (默认: 127.0.0.1)
    DB_PORT          数据库端口 (默认: 5432)
    DB_USER          数据库用户 (默认: hejj)
    DB_PASSWORD      数据库密码 (默认: pass12345)
    DB_NAME          数据库名称 (默认: postgres)
    DB_SCHEMA        数据库模式 (默认: public)
    CONTAINER_NAME   Docker 容器名称 (默认: pg_dev)

示例:
    # 使用默认配置
    $0

    # 指定容器名称
    $0 --container my_postgres

    # 指定 SQL 文件
    $0 --file /path/to/custom.sql

    # 通过环境变量配置
    DB_USER=admin DB_PASSWORD=secret $0

EOF
}

#=============================================================================================
# 主流程
#=============================================================================================

main() {
    # 解析命令行参数
    SKIP_VERIFY=false

    while [[ $# -gt 0 ]]; do
        case $1 in
            -h|--help)
                show_help
                exit 0
                ;;
            -c|--container)
                CONTAINER_NAME="$2"
                shift 2
                ;;
            -f|--file)
                SQL_FILE="$2"
                shift 2
                ;;
            --skip-verify)
                SKIP_VERIFY=true
                shift
                ;;
            *)
                error "未知参数: $1"
                ;;
        esac
    done

    # 打印标题
    echo "======================================"
    echo "PostgreSQL 数据库初始化"
    echo "======================================"
    echo ""

    # 执行检查和初始化
    check_container
    check_connection
    check_sql_file
    init_database

    # 验证数据（除非跳过）
    if [ "${SKIP_VERIFY}" = false ]; then
        verify_data
    fi

    echo ""
    info "✓ 所有操作完成！"
    echo "======================================"
}

# 执行主流程
main "$@"
