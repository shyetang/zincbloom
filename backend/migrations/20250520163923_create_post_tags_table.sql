-- backend/migrations/YYYYMMDDHHMMSS_create_post_tags_table.sql

-- 创建 post_tags 表，用于帖子和标签之间的多对多关系
CREATE TABLE post_tags
(
    post_id     UUID        NOT NULL,               -- 帖子ID，外键关联 posts 表的 id
    tag_id      UUID        NOT NULL,               -- 标签ID，外键关联 tags 表的 id
    assigned_at TIMESTAMPTZ NOT NULL DEFAULT NOW(), -- 关联创建时间，默认为当前时间

    -- 复合主键，确保一个帖子不会多次关联同一个标签
    PRIMARY KEY (post_id, tag_id),

    -- 外键约束：关联到 posts 表的 post_id
    CONSTRAINT fk_post_tags_post
        FOREIGN KEY (post_id)
            REFERENCES posts (id)
            ON DELETE CASCADE,                      -- 如果帖子被删除，则自动删除此关联记录

    -- 外键约束：关联到 tags 表的 tag_id
    CONSTRAINT fk_post_tags_tag
        FOREIGN KEY (tag_id)
            REFERENCES tags (id)
            ON DELETE CASCADE                       -- 如果标签被删除，则自动删除此关联记录
);

-- 可选索引 (原因同 post_categories 表)
-- CREATE INDEX IF NOT EXISTS idx_post_tags_post_id ON post_tags(post_id);
-- CREATE INDEX IF NOT EXISTS idx_post_tags_tag_id ON post_tags(tag_id);
