-- Add migration script here
-- backend/migrations/YYYYMMDDHHMMSS_create_login_attempts_table.sql

-- 创建 login_attempts 表，用于追踪登录失败次数以防止暴力破解
CREATE TABLE login_attempts
(
    -- 以用户名作为追踪的关键，因为登录时用户输入的是用户名
    -- 设为 TEXT 类型并作为主键
    username           TEXT PRIMARY KEY,

    -- 失败尝试的次数
    failure_count      INTEGER     NOT NULL DEFAULT 1,

    -- 账户被锁定的截止时间。如果为 NULL，表示未被锁定。
    lockout_expires_at TIMESTAMPTZ,

    -- 最后一次尝试的时间，用于判断是否要重置失败次数
    last_attempt_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 为 lockout_expires_at 创建索引，可能有助于快速查询被锁定的用户
CREATE INDEX idx_login_attempts_lockout_expires_at ON login_attempts (lockout_expires_at);