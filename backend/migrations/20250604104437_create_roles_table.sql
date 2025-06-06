-- Add migration script here
-- backend/migrations/YYYYMMDDHHMMSS_create_roles_table.sql

-- 创建 roles 表，用于定义用户角色
CREATE TABLE roles
(
    -- 角色ID，主键
    id          UUID PRIMARY KEY,
    -- 角色名称 (例如: 'admin', 'editor', 'user')，唯一且不能为空
    name        TEXT UNIQUE NOT NULL,
    -- 对角色的可选描述
    description TEXT,
    -- 记录创建时间
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    -- 记录最后更新时间
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 为 roles 表的 updated_at 字段应用时间戳更新触发器
CREATE TRIGGER set_roles_timestamp
    BEFORE UPDATE
    ON roles
    FOR EACH ROW
EXECUTE FUNCTION trigger_set_timestamp();

-- 插入一些默认的角色定义
-- 使用 gen_random_uuid() (PostgreSQL 13+功能) 或 uuid_generate_v4() (需开启 uuid-ossp 扩展) 来生成UUID
-- 这里使用 gen_random_uuid()
INSERT INTO roles (id, name, description)
VALUES (gen_random_uuid(), 'admin', '系统管理员，拥有所有权限'),
       (gen_random_uuid(), 'editor', '编辑人员，可以管理所有内容和分类标签'),
       (gen_random_uuid(), 'user', '注册用户，可以创建和管理自己的内容');