# PostgreSQL 数据库设置指南

本项目使用 Docker 运行 PostgreSQL 数据库，提供了自动化脚本来启动容器和初始化种子数据。

## 快速开始

### 1. 启动 PostgreSQL 容器

```bash
./pg_launch.docker.sh
```

这将创建并启动一个名为 `pg_dev` 的 PostgreSQL 容器，配置如下：

- **端口**: 5432
- **用户**: hejj
- **密码**: pass12345
- **数据库**: postgres
- **模式**: public

### 2. 初始化数据库（创建表和种子数据）

```bash
./pg_seed_init.sh
```

这将执行以下操作：
- 创建 `sys_user` 表
- 插入 3 条测试用户数据
- 验证数据是否正确插入

## 脚本说明

### `pg_launch.docker.sh` - 启动数据库容器

创建并启动 PostgreSQL Docker 容器的脚本。

**使用方法**:
```bash
./pg_launch.docker.sh
```

**容器配置**:
- 容器名称: `pg_dev`
- 使用 `--rm` 标志（容器停止后自动删除）
- 端口映射: `5432:5432`

### `pg_seed_init.sh` - 初始化数据库

在 Docker 容器中执行 SQL 脚本，创建表结构并注入种子数据。

**使用方法**:
```bash
# 使用默认配置
./pg_seed_init.sh

# 查看帮助信息
./pg_seed_init.sh --help

# 指定容器名称
./pg_seed_init.sh --container my_postgres

# 使用自定义 SQL 文件
./pg_seed_init.sh --file /path/to/custom.sql

# 跳过数据验证
./pg_seed_init.sh --skip-verify

# 通过环境变量配置
DB_USER=admin DB_PASSWORD=secret ./pg_seed_init.sh
```

**功能**:
- ✅ 检查 Docker 容器是否运行
- ✅ 检查数据库连接是否正常
- ✅ 检查 SQL 文件是否存在
- ✅ 执行 SQL 脚本创建表和数据
- ✅ 验证数据是否正确插入
- ✅ 彩色输出和详细的错误信息

### `schema.sql` - 数据库结构和种子数据

包含完整的数据库初始化 SQL 脚本。

**内容**:
- `sys_user` 表结构定义
- 3 条测试用户数据（李四、张三、赵六）

**表结构**:
```sql
CREATE TABLE sys_user (
    id           varchar(32)    not null primary key,
    name         varchar(16)    not null,
    gender       varchar(8)     not null,
    account      varchar(16)    not null,
    password     varchar(64)    not null,
    mobile_phone varchar(16)    not null,
    birthday     date           not null,
    enabled      boolean        not null default true,
    created_at   timestamp      not null default CURRENT_TIMESTAMP,
    updated_at   timestamp      not null default CURRENT_TIMESTAMP
);
```

## 环境变量

两个脚本都支持通过环境变量进行配置：

| 环境变量 | 说明 | 默认值 |
|---------|------|--------|
| `DB_HOST` | 数据库主机 | 127.0.0.1 |
| `DB_PORT` | 数据库端口 | 5432 |
| `DB_USER` | 数据库用户 | hejj |
| `DB_PASSWORD` | 数据库密码 | pass12345 |
| `DB_NAME` | 数据库名称 | postgres |
| `DB_SCHEMA` | 数据库模式 | public |
| `CONTAINER_NAME` | Docker 容器名称 | pg_dev |

## 常见使用场景

### 场景 1: 重置数据库

在开发过程中，如果需要重置数据库到初始状态：

```bash
# 1. 停止并删除容器（使用 --rm 会自动删除，或者手动删除）
docker stop pg_dev
docker rm pg_dev

# 2. 重新启动容器
./pg_launch.docker.sh

# 3. 重新初始化数据
./pg_seed_init.sh
```

### 场景 2: 自定义配置

如果需要使用不同的配置：

```bash
# 使用自定义用户名和密码
DB_USER=myuser DB_PASSWORD=mypass ./pg_launch.docker.sh
DB_USER=myuser DB_PASSWORD=mypass ./pg_seed_init.sh
```

### 场景 3: 连接到数据库

使用 `psql` 客户端连接到容器中的数据库：

```bash
# 使用 docker exec 连接
docker exec -it pg_dev psql -U hejj -d postgres

# 查询数据
SELECT * FROM sys_user;

# 退出
\q
```

### 场景 4: 重新生成 SeaORM 实体

当数据库表结构变更后，重新生成 Rust 实体：

```bash
# 从数据库生成实体
DATABASE_URL="postgres://hejj:pass12345@127.0.0.1:5432/postgres" \
sea-orm-cli generate entity \
  --with-serde both \
  --model-extra-attributes 'serde(rename_all = "camelCase")' \
  --date-time-crate chrono \
  -o ./src/entity
```

## 数据库管理

### 查看容器日志

```bash
docker logs pg_dev
```

### 停止容器

```bash
docker stop pg_dev
```

### 进入容器

```bash
docker exec -it pg_dev bash
```

### 备份数据库

```bash
docker exec pg_dev pg_dump -U hejj postgres > backup.sql
```

### 恢复数据库

```bash
docker exec -i pg_dev psql -U hejj postgres < backup.sql
```

## 故障排查

### 问题: 容器名称已存在

**错误信息**:
```
Error: The container name "/pg_dev" is already in use
```

**解决方案**:
```bash
# 停止并删除现有容器
docker stop pg_dev
docker rm pg_dev

# 或者删除所有停止的容器
docker container prune
```

### 问题: 端口已被占用

**错误信息**:
```
Error: bind: address already in use
```

**解决方案**:
```bash
# 查看占用端口的进程
lsof -i :5432

# 停止其他 PostgreSQL 实例或修改端口映射
# 在 pg_launch.docker.sh 中修改 --publish 5433:5432
```

### 问题: 数据库连接失败

**检查清单**:
1. 容器是否正在运行: `docker ps | grep pg_dev`
2. 数据库是否就绪: `docker exec pg_dev pg_isready -U hejj`
3. 查看容器日志: `docker logs pg_dev`

## 测试用户数据

初始化后，数据库包含以下测试用户：

| ID | 姓名 | 性别 | 账号 | 状态 |
|----|------|------|------|------|
| 6202954260741 | 李四 | female | lisi | 启用 |
| 6161671639301 | 张三 | male | admin | 禁用 |
| 11467064770821 | 赵六 | female | zhaoliu | 启用 |

**注意**: 密码是使用 bcrypt 加密的哈希值，不是明文密码。

## 相关文档

- [SeaORM 实体生成](CLAUDE.md#seaorm-实体生成)
- [配置文件格式](CONFIG_FORMATS.md)
- [项目架构](CLAUDE.md#代码架构)
