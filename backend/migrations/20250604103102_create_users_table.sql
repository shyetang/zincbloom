-- Add migration script here
-- backend/migrations/YYYYMMDDHHMMSS_create_users_table.sql

-- 创建 users 表，用于存储用户基本信息
CREATE TABLE users
(
    -- 用户ID，主键
    id              UUID PRIMARY KEY,
    -- 用户名，唯一且不能为空
    username        TEXT UNIQUE NOT NULL,
    -- 邮箱，唯一且不能为空
    email           TEXT UNIQUE NOT NULL,
    -- 哈希后的密码，不能为空
    hashed_password TEXT        NOT NULL,
    -- 记录创建时间，默认为当前时间
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    -- 记录最后更新时间，默认为当前时间
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 为 users 表的 updated_at 字段创建或复用时间戳更新触发器
CREATE TRIGGER set_users_timestamp
    BEFORE UPDATE -- 在更新操作之前触发
    ON users -- 作用于 users 表
    FOR EACH ROW -- 对每一行更新都执行
EXECUTE FUNCTION trigger_set_timestamp(); -- 调用已定义的函数