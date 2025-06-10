-- Add migration script here
-- backend/migrations/YYYYMMDDHHMMSS_create_refresh_tokens_table.sql

-- 创建 refresh_tokens 表，用于存储刷新令牌的哈希值
-- 这个表是实现“记住我”和安全注销功能的核心
CREATE TABLE refresh_tokens
(
    -- 存储 Refresh Token 的 SHA256 哈希值作为主键，以便快速查找
    -- 使用 TEXT 类型，因为哈希值是十六进制字符串
    token_hash TEXT PRIMARY KEY,

    -- 关联的用户ID，外键约束
    user_id    UUID        NOT NULL,

    -- 此 Refresh Token 的过期时间
    expires_at TIMESTAMPTZ NOT NULL,

    -- 记录创建时间
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- 外键约束，确保 user_id 存在于 users 表中
    -- ON DELETE CASCADE 意味着如果用户被删除，他们所有的 Refresh Token 记录也会被自动删除，
    -- 这会自动撤销他们所有的会话，非常重要。
    CONSTRAINT fk_refresh_tokens_user
        FOREIGN KEY (user_id)
            REFERENCES users (id)
            ON DELETE CASCADE
);

-- 为 user_id 创建索引，可以提高查找某个用户所有 Refresh Token 的效率
CREATE INDEX idx_refresh_tokens_user_id ON refresh_tokens (user_id);