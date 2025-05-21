-- backend/migrations/YYYYMMDDHHMMSS_create_post_categories_table.sql

-- 创建 post_categories 表，用于帖子和分类之间的多对多关系
CREATE TABLE post_categories
(
    post_id     UUID        NOT NULL,               -- 帖子ID，外键关联 posts 表的 id
    category_id UUID        NOT NULL,               -- 分类ID，外键关联 categories 表的 id
    assigned_at TIMESTAMPTZ NOT NULL DEFAULT NOW(), -- 关联创建时间，默认为当前时间

    -- 复合主键，确保一个帖子不会多次关联同一个分类
    PRIMARY KEY (post_id, category_id),

    -- 外键约束：关联到 posts 表的 post_id
    CONSTRAINT fk_post_categories_post
        FOREIGN KEY (post_id)
            REFERENCES posts (id)
            ON DELETE CASCADE,                      -- 如果帖子被删除，则自动删除此关联记录

    -- 外键约束：关联到 categories 表的 category_id
    CONSTRAINT fk_post_categories_category
        FOREIGN KEY (category_id)
            REFERENCES categories (id)
            ON DELETE CASCADE                       -- 如果分类被删除，则自动删除此关联记录
);

-- 可选索引：可以提高通过此表进行连接查询的性能
-- 主键 (post_id, category_id) 已经创建了一个覆盖这两个列的索引。
-- 如果经常单独通过 post_id 或 category_id 在此表中查询，可以考虑创建单独索引。
-- 初期可以不加，根据后续性能分析决定是否添加。
-- CREATE INDEX IF NOT EXISTS idx_post_categories_post_id ON post_categories(post_id);
-- CREATE INDEX IF NOT EXISTS idx_post_categories_category_id ON post_categories(category_id);
