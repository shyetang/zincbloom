-- Add migration script here
-- backend/migrations/YYYYMMDDHHMMSS_add_author_id_to_posts.sql

-- 向 posts 表添加 author_id 列，用于关联帖子的作者
ALTER TABLE posts
    ADD COLUMN author_id UUID;
-- 初始允许 NULL，以便平稳迁移现有数据（如果存在）

-- 为 author_id 添加外键约束，关联到 users 表的 id
ALTER TABLE posts
    ADD CONSTRAINT fk_posts_author -- 外键约束的名称
        FOREIGN KEY (author_id) -- posts 表中的外键列
            REFERENCES users (id) -- 引用 users 表的 id 列
            ON DELETE SET NULL;
-- 当关联的 user被删除时，将 author_id 设置为 NULL
-- 另一种选择是 ON DELETE CASCADE (级联删除帖子)，根据业务需求选择