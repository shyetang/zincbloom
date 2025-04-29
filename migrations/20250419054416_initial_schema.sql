-- 创建 posts 表
CREATE TABLE posts
(
    -- ID: UUID 类型，作为主键
    id           UUID PRIMARY KEY,

    -- Slug: 文本类型，用于 URL，必须唯一且不能为空
    slug         TEXT        NOT NULL UNIQUE,

    -- Title: 文本类型，文章标题，不能为空
    title        TEXT        NOT NULL,

    -- Content: 文本类型，文章内容，不能为空
    content      TEXT        NOT NULL,

    -- Created At: 带时区的时间戳，记录创建时间，不能为空，默认为当前时间
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Updated At: 带时区的时间戳，记录最后更新时间，不能为空，默认为当前时间
    -- (稍后会添加触发器自动更新此字段)
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Published At: 带时区的时间戳，记录发布时间，可以为空 (表示草稿)
    published_at TIMESTAMPTZ NULL
);

-- 可选但推荐: 为常用查询字段添加索引以提高性能
-- UNIQUE 约束通常会自动创建 slug 上的索引，但显式声明有时更清晰
-- CREATE UNIQUE INDEX posts_slug_idx ON posts (slug);

-- 为按发布时间排序和筛选（例如获取已发布文章列表）创建索引
-- DESC NULLS LAST 意味着 NULL 值（草稿）会排在最后
CREATE INDEX posts_published_at_idx ON posts (published_at DESC NULLS LAST);

-- 为按创建时间排序创建索引（如果需要）
CREATE INDEX posts_created_at_idx ON posts (created_at DESC);


-- === 用于自动更新 updated_at 字段的触发器 ===

-- 1. 创建一个可重用的函数，用于将 updated_at 设置为当前时间
CREATE OR REPLACE FUNCTION trigger_set_timestamp()
    RETURNS TRIGGER AS
$$
BEGIN
    -- 将 NEW (正在被插入或更新的行) 的 updated_at 字段设置为当前时间
    NEW.updated_at = NOW();
    RETURN NEW; -- 返回修改后的行
END;
$$ LANGUAGE plpgsql; -- 指定语言为 plpgsql

-- 2. 创建一个触发器，在 posts 表每次更新行之前 (BEFORE UPDATE) 调用上述函数
--    FOR EACH ROW 表示对每一行更新都执行
CREATE TRIGGER set_timestamp
    BEFORE UPDATE
    ON posts
    FOR EACH ROW
EXECUTE FUNCTION trigger_set_timestamp();