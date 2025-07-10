-- Add migration script here

-- 修复缺失的 category:create 和 tag:create 权限
-- 这些权限在权限系统重构中被引用但没有被创建

-- 添加缺失的权限
INSERT INTO permissions (id, name, description)
VALUES 
    (gen_random_uuid(), 'category:create', '创建新分类'),
    (gen_random_uuid(), 'tag:create', '创建新标签')
ON CONFLICT (name) DO NOTHING;

-- 权限分配在单独的迁移文件中处理
