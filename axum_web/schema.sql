-- PostgreSQL 数据库初始化脚本
-- 创建 sys_user 表并插入种子数据

-- 删除已存在的表（如果存在）
DROP TABLE IF EXISTS sys_user_login_log CASCADE;
DROP TABLE IF EXISTS sys_user CASCADE;

-- 创建 sys_user 表
CREATE TABLE sys_user
(
    id           varchar(32)                         not null
        primary key,
    name         varchar(16)                         not null,
    gender       varchar(8)                          not null,
    account      varchar(16)                         not null,
    password     varchar(64)                         not null,
    mobile_phone varchar(16)                         not null,
    birthday     date                                not null,
    enabled      boolean   default true              not null,
    created_at   timestamp default CURRENT_TIMESTAMP not null,
    updated_at   timestamp default CURRENT_TIMESTAMP not null
);

ALTER TABLE sys_user
    OWNER TO hejj;

-- 创建用户登录记录表
CREATE TABLE sys_user_login_log
(
    id             bigserial    not null
        primary key,
    user_id        varchar(32)  not null
        references sys_user(id) on delete cascade,
    login_time     timestamp    not null,
    logout_time    timestamp,
    ip_address     varchar(45),
    user_agent     varchar(512),
    login_status   varchar(20)  not null, -- 'success' or 'failed'
    failure_reason varchar(256),
    created_at     timestamp default CURRENT_TIMESTAMP not null,
    constraint check_login_status check (login_status in ('success', 'failed'))
);

ALTER TABLE sys_user_login_log
    OWNER TO hejj;

-- 创建索引以提高查询性能
CREATE INDEX idx_login_log_user_id ON sys_user_login_log(user_id);
CREATE INDEX idx_login_log_login_time ON sys_user_login_log(login_time DESC);
CREATE INDEX idx_login_log_status ON sys_user_login_log(login_status);

-- 添加表注释
COMMENT ON TABLE sys_user_login_log IS '用户登录记录表';
COMMENT ON COLUMN sys_user_login_log.id IS '主键ID';
COMMENT ON COLUMN sys_user_login_log.user_id IS '用户ID，关联sys_user表';
COMMENT ON COLUMN sys_user_login_log.login_time IS '登录时间';
COMMENT ON COLUMN sys_user_login_log.logout_time IS '登出时间（未登出时为空）';
COMMENT ON COLUMN sys_user_login_log.ip_address IS '登录IP地址（支持IPv6）';
COMMENT ON COLUMN sys_user_login_log.user_agent IS '用户代理（浏览器/设备信息）';
COMMENT ON COLUMN sys_user_login_log.login_status IS '登录状态：success-成功, failed-失败';
COMMENT ON COLUMN sys_user_login_log.failure_reason IS '登录失败原因';

-- 插入种子数据
INSERT INTO sys_user (id, name, gender, account, password, mobile_phone, birthday, enabled, created_at, updated_at)
VALUES
    ('6202954260741', '李四', 'female', 'lisi', '$2b$12$PsumwxjxX/o1RNOKpkc.Kuxea0izqSuhaod4PCudXoRh3zet1TASK', '17361631996', '2025-05-13', true, '2025-05-18 12:39:53.133469', '2025-05-18 12:39:53.133469'),
    ('6161671639301', '张三', 'male', 'admin', '$2b$12$PsumwxjxX/o1RNOKpkc.Kuxea0izqSuhaod4PCudXoRh3zet1TASK', '19909407240', '2025-05-18', false, '2025-05-18 09:51:54.367501', '2025-05-18 09:51:54.367501'),
    ('11467064770821', '赵六', 'female', 'zhaoliu', '$2b$12$EJOKHLJLnfHrgrXbZl8uge3N4VEgR9FWHwq3a6pgTIM8O66Lf/9DW', '18361631783', '2025-06-11', true, '2025-06-02 09:39:36.366121', '2025-06-02 09:39:36.366121');

-- 插入登录记录种子数据
INSERT INTO sys_user_login_log (user_id, login_time, logout_time, ip_address, user_agent, login_status, failure_reason, created_at)
VALUES
    -- 李四的成功登录记录
    ('6202954260741', '2025-06-15 08:30:00', '2025-06-15 12:45:30', '192.168.1.100', 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36', 'success', NULL, '2025-06-15 08:30:00'),
    ('6202954260741', '2025-06-15 14:20:15', '2025-06-15 18:30:00', '192.168.1.100', 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36', 'success', NULL, '2025-06-15 14:20:15'),
    ('6202954260741', '2025-06-16 09:15:00', NULL, '192.168.1.101', 'Mozilla/5.0 (iPhone; CPU iPhone OS 15_0 like Mac OS X) AppleWebKit/605.1.15', 'success', NULL, '2025-06-16 09:15:00'),

    -- 张三的登录记录（包含成功和失败）
    ('6161671639301', '2025-06-15 10:00:00', NULL, '192.168.1.105', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36', 'success', NULL, '2025-06-15 10:00:00'),
    ('6161671639301', '2025-06-15 11:30:00', NULL, '192.168.1.105', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36', 'failed', '密码错误', '2025-06-15 11:30:00'),
    ('6161671639301', '2025-06-15 11:31:00', NULL, '192.168.1.105', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36', 'failed', '账户已禁用', '2025-06-15 11:31:00'),

    -- 赵六的成功登录记录
    ('11467064770821', '2025-06-14 16:45:00', '2025-06-14 17:30:00', '192.168.1.200', 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36', 'success', NULL, '2025-06-14 16:45:00'),
    ('11467064770821', '2025-06-15 20:00:00', NULL, '192.168.1.200', 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36', 'success', NULL, '2025-06-15 20:00:00');

-- 显示插入的数据
SELECT 'Database initialized successfully!' AS status;
SELECT COUNT(*) AS total_records FROM sys_user;
SELECT id, name, gender, account, enabled FROM sys_user;

SELECT 'Login log records:' AS info;
SELECT COUNT(*) AS total_login_records FROM sys_user_login_log;
SELECT
    l.id,
    u.name AS user_name,
    l.login_time,
    l.logout_time,
    l.ip_address,
    l.login_status,
    l.failure_reason
FROM sys_user_login_log l
LEFT JOIN sys_user u ON l.user_id = u.id
ORDER BY l.login_time DESC;
