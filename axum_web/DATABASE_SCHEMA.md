# 数据库表结构说明

本文档描述了项目中的数据库表结构。

## 表列表

1. **sys_user** - 用户表
2. **sys_user_login_log** - 用户登录记录表

## 1. sys_user (用户表)

存储用户的基本信息。

### 表结构

| 字段 | 类型 | 可空 | 说明 |
|------|------|------|------|
| id | varchar(32) | ❌ | 主键ID |
| name | varchar(16) | ❌ | 用户姓名 |
| gender | varchar(8) | ❌ | 性别（male/female） |
| account | varchar(16) | ❌ | 登录账号 |
| password | varchar(64) | ❌ | 密码（bcrypt 哈希） |
| mobile_phone | varchar(16) | ❌ | 手机号 |
| birthday | date | ❌ | 出生日期 |
| enabled | boolean | ❌ | 是否启用（默认 true） |
| created_at | timestamp | ❌ | 创建时间（自动） |
| updated_at | timestamp | ❌ | 更新时间（自动） |

### 索引
- 主键：id

### 种子数据
- 李四 (lisi) - 启用
- 张三 (admin) - 禁用
- 赵六 (zhaoliu) - 启用

---

## 2. sys_user_login_log (用户登录记录表)

记录用户的登录和登出历史，包括成功和失败的登录尝试。

### 表结构

| 字段 | 类型 | 可空 | 说明 |
|------|------|------|------|
| id | bigserial | ❌ | 主键ID（自增） |
| user_id | varchar(32) | ❌ | 用户ID，外键关联 sys_user.id |
| login_time | timestamp | ❌ | 登录时间 |
| logout_time | timestamp | ✅ | 登出时间（未登出时为空） |
| ip_address | varchar(45) | ✅ | 登录IP地址（支持 IPv6） |
| user_agent | varchar(512) | ✅ | 用户代理（浏览器/设备信息） |
| login_status | varchar(20) | ❌ | 登录状态（success/failed） |
| failure_reason | varchar(256) | ✅ | 登录失败原因 |
| created_at | timestamp | ❌ | 记录创建时间（自动） |

### 约束
- **主键**: id
- **外键**: user_id → sys_user.id (ON DELETE CASCADE)
- **检查约束**: login_status 必须是 'success' 或 'failed'

### 索引
- `idx_login_log_user_id` - 用户ID索引（用于查询某用户的登录记录）
- `idx_login_log_login_time` - 登录时间降序索引（用于查询最近登录）
- `idx_login_log_status` - 登录状态索引（用于统计成功/失败次数）

### 注释
```sql
COMMENT ON TABLE sys_user_login_log IS '用户登录记录表';
COMMENT ON COLUMN sys_user_login_log.id IS '主键ID';
COMMENT ON COLUMN sys_user_login_log.user_id IS '用户ID，关联sys_user表';
COMMENT ON COLUMN sys_user_login_log.login_time IS '登录时间';
COMMENT ON COLUMN sys_user_login_log.logout_time IS '登出时间（未登出时为空）';
COMMENT ON COLUMN sys_user_login_log.ip_address IS '登录IP地址（支持IPv6）';
COMMENT ON COLUMN sys_user_login_log.user_agent IS '用户代理（浏览器/设备信息）';
COMMENT ON COLUMN sys_user_login_log.login_status IS '登录状态：success-成功, failed-失败';
COMMENT ON COLUMN sys_user_login_log.failure_reason IS '登录失败原因';
```

### 种子数据
8 条登录记录，包括：
- **李四**: 3 次成功登录
- **张三**: 1 次成功登录 + 2 次失败登录（密码错误、账户已禁用）
- **赵六**: 2 次成功登录

### 业务逻辑

#### 登录成功
- `login_status = 'success'`
- `failure_reason = NULL`
- `logout_time` 初始为 NULL，用户登出时更新

#### 登录失败
- `login_status = 'failed'`
- `failure_reason` 记录失败原因（如：密码错误、账户不存在、账户已禁用等）
- `logout_time = NULL`

#### 当前会话识别
`logout_time IS NULL` 的记录表示用户当前仍处于登录状态（会话活跃）。

---

## 常用查询示例

### 查询用户的最近登录记录
```sql
SELECT
    l.id,
    l.login_time,
    l.logout_time,
    l.ip_address,
    l.login_status,
    l.failure_reason
FROM sys_user_login_log l
WHERE l.user_id = '6202954260741'
ORDER BY l.login_time DESC
LIMIT 10;
```

### 查询当前活跃的登录会话
```sql
SELECT
    u.name AS user_name,
    u.account,
    l.login_time,
    l.ip_address,
    l.user_agent
FROM sys_user_login_log l
JOIN sys_user u ON l.user_id = u.id
WHERE l.logout_time IS NULL
  AND l.login_status = 'success';
```

### 统计登录失败次数
```sql
SELECT
    u.name AS user_name,
    COUNT(*) AS failed_count
FROM sys_user_login_log l
JOIN sys_user u ON l.user_id = u.id
WHERE l.login_status = 'failed'
  AND l.login_time >= CURRENT_DATE - INTERVAL '7 days'
GROUP BY u.id, u.name
ORDER BY failed_count DESC;
```

### 查询可疑的登录活动（短时间内多次失败）
```sql
SELECT
    u.name AS user_name,
    l.ip_address,
    COUNT(*) AS failed_attempts,
    MAX(l.login_time) AS last_attempt
FROM sys_user_login_log l
JOIN sys_user u ON l.user_id = u.id
WHERE l.login_status = 'failed'
  AND l.login_time >= CURRENT_TIMESTAMP - INTERVAL '1 hour'
GROUP BY u.id, u.name, l.ip_address
HAVING COUNT(*) >= 3
ORDER BY failed_attempts DESC;
```

### 查询用户登录历史统计
```sql
SELECT
    u.name AS user_name,
    COUNT(*) FILTER (WHERE l.login_status = 'success') AS success_count,
    COUNT(*) FILTER (WHERE l.login_status = 'failed') AS failed_count,
    MAX(l.login_time) AS last_login
FROM sys_user_login_log l
JOIN sys_user u ON l.user_id = u.id
GROUP BY u.id, u.name
ORDER BY last_login DESC;
```

---

## 级联删除

当用户被删除时（`DELETE FROM sys_user WHERE id = '...'`），由于外键约束设置了 `ON DELETE CASCADE`，该用户的所有登录记录也会自动删除。

---

## 性能优化建议

1. **索引覆盖**: 已为常用查询字段创建索引（user_id, login_time, login_status）
2. **分区表**: 如果数据量很大，可以按时间范围对登录日志表进行分区
3. **归档策略**: 建议定期归档旧数据（如 1 年前的登录记录）到历史表
4. **清理策略**: 可以定期清理超过保留期限的日志记录

---

## 安全考虑

1. **敏感信息**: 登录日志中的 IP 地址和 User-Agent 可能包含用户隐私信息
2. **数据保留**: 建议制定日志保留策略，遵守相关隐私法规（如 GDPR）
3. **审计需求**: 登录日志是重要的审计数据，应妥善保护
4. **访问控制**: 应用层应控制谁能查看登录日志（管理员、用户本人）

---

## 下一步

如果需要使用 Rust + SeaORM 操作这些表，请重新生成实体：

```bash
DATABASE_URL="postgres://hejj:pass12345@127.0.0.1:5432/postgres" \
sea-orm-cli generate entity \
  --with-serde both \
  --model-extra-attributes 'serde(rename_all = "camelCase")' \
  --date-time-crate chrono \
  -o ./src/entity
```

这将生成 `SysUser` 和 `SysUserLoginLog` 两个实体。
