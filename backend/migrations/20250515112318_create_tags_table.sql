-- migrations/xxx_create_tags_table.sql

-- 创建 tags 表
CREATE TABLE tags
(
    id         UUID PRIMARY KEY,
    name       TEXT        NOT NULL UNIQUE,        -- 标签名称，唯一且不能为空
    slug       TEXT        NOT NULL UNIQUE,        -- URL友好型名称，唯一且不能为空
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(), -- 创建时间
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()  -- 更新时间
);

-- 为 updated_at 字段应用已有的时间戳更新触发器
-- 复用为 posts 和 categories 表创建的 trigger_set_timestamp() 函数
CREATE TRIGGER set_tags_timestamp
    BEFORE UPDATE
    ON tags
    FOR EACH ROW
    EXECUTE FUNCTION trigger_set_timestamp();

-- （可选）为 name 和 slug 创建索引，UNIQUE 约束通常会自动创建
-- CREATE INDEX idx_tags_name ON tags (name);
-- CREATE INDEX idx_tags_slug ON tags (slug);