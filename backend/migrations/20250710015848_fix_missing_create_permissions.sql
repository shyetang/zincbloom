-- Add migration script here

-- 修复缺失的 category:create 和 tag:create 权限
-- 这些权限在权限系统重构中被引用但没有被创建

-- 添加缺失的权限
INSERT INTO permissions (id, name, description)
VALUES 
    (gen_random_uuid(), 'category:create', '创建新分类'),
    (gen_random_uuid(), 'tag:create', '创建新标签')
ON CONFLICT (name) DO NOTHING;

-- 确保这些权限已经分配给相应的角色
-- (权限分配在之前的迁移中已经定义，这里只是确保权限存在)
