-- Add migration script here
-- backend/migrations/YYYYMMDDHHMMSS_create_permissions_table.sql

-- 创建 permissions 表，用于定义具体的操作权限
CREATE TABLE permissions
(
    -- 权限ID，主键
    id          UUID PRIMARY KEY,
    -- 权限名称 (例如: 'post:create', 'post:edit_any', 'user:manage')，唯一且不能为空
    name        TEXT UNIQUE NOT NULL,
    -- 建议命名规范: resource:action 或 resource:action:scope
    -- 对权限的可选描述
    description TEXT,
    -- 记录创建时间
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    -- 记录最后更新时间
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 为 permissions 表的 updated_at 字段应用时间戳更新触发器
CREATE TRIGGER set_permissions_timestamp
    BEFORE UPDATE
    ON permissions
    FOR EACH ROW
EXECUTE FUNCTION trigger_set_timestamp();

-- 插入一些基础的权限定义
INSERT INTO permissions (id, name, description)
VALUES (gen_random_uuid(), 'post:create', '允许创建新帖子'),
       (gen_random_uuid(), 'post:read_any', '允许阅读所有帖子（包括未发布的，如果业务需要）'),
       (gen_random_uuid(), 'post:edit_own', '允许编辑自己的帖子'),
       (gen_random_uuid(), 'post:edit_any', '允许编辑任意用户的帖子'),
       (gen_random_uuid(), 'post:delete_own', '允许删除自己的帖子'),
       (gen_random_uuid(), 'post:delete_any', '允许删除任意用户的帖子'),
       (gen_random_uuid(), 'category:manage', '允许管理（增删改查）分类'),
       (gen_random_uuid(), 'tag:manage', '允许管理（增删改查）标签'),
       (gen_random_uuid(), 'user:list', '允许查看用户列表'),
       (gen_random_uuid(), 'user:manage_roles', '允许分配和撤销用户角色'),
       (gen_random_uuid(), 'role:manage_permissions', '允许管理角色及其权限分配');