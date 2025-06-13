-- Add migration script here
-- backend/migrations/YYYYMMDDHHMMSS_create_one_time_tokens_table.sql
CREATE TABLE one_time_tokens
(
    -- 存储令牌的哈希值作为主键
    token_hash TEXT PRIMARY KEY,
    -- 关联的用户ID
    user_id    UUID        NOT NULL,
    -- 令牌类型，用于区分是“邮箱验证”还是“密码重置”
    token_type TEXT        NOT NULL,
    -- 令牌的过期时间
    expires_at TIMESTAMPTZ NOT NULL,

    CONSTRAINT fk_one_time_tokens_user
        FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
        -- 当users表中的id记录被删除时，自动删除当前表中关联的user_id记录，确保数据一致性
);