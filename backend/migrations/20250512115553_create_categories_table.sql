-- Add migration script here
-- 创建 categories 表
CREATE TABLE categories
(
    id         UUID PRIMARY KEY,
    name       TEXT        NOT NULL UNIQUE,        -- 分类名称，唯一且不能为空
    slug       TEXT        NOT NULL UNIQUE,        -- URL友好型名称，唯一且不能为空
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(), -- 创建时间
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()  -- 更新时间
);

-- 为 updated_at 字段应用已有的时间戳更新触发器
CREATE TRIGGER set_categories_timestamp
    BEFORE UPDATE
    ON categories
    FOR EACH ROW
EXECUTE FUNCTION trigger_set_timestamp(); -- 假设 trigger_set_timestamp 函数已存在