-- Add migration script here

-- 修复文章权限设计：区分管理界面和展示界面
-- 管理界面：用户只能管理自己的文章，管理员可以管理所有文章
-- 展示界面：所有人（包括游客）都可以查看已发布的文章

-- 1. 移除 user 角色的 post:read_any 权限（这个权限仅用于管理界面）
DELETE FROM role_permissions 
WHERE role_id = (SELECT id FROM roles WHERE name = 'user')
  AND permission_id = (SELECT id FROM permissions WHERE name = 'post:read_any');

-- 2. 添加新的权限用于区分公开阅读和管理阅读
INSERT INTO permissions (id, name, description)
VALUES 
    (gen_random_uuid(), 'post:read_published', '允许阅读所有已发布的文章（公开权限）'),
    (gen_random_uuid(), 'post:manage_own', '允许在管理界面管理自己的文章'),
    (gen_random_uuid(), 'post:manage_any', '允许在管理界面管理任意用户的文章')
ON CONFLICT (name) DO NOTHING;

-- 3. 为所有角色分配公开阅读权限（包括未来可能的 guest 角色）
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r, permissions p
WHERE p.name = 'post:read_published'
  AND NOT EXISTS (
    SELECT 1 FROM role_permissions rp 
    WHERE rp.role_id = r.id AND rp.permission_id = p.id
  );

-- 4. 为 user 角色分配管理自己文章的权限
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r, permissions p
WHERE r.name = 'user'
  AND p.name = 'post:manage_own'
  AND NOT EXISTS (
    SELECT 1 FROM role_permissions rp 
    WHERE rp.role_id = r.id AND rp.permission_id = p.id
  );

-- 5. 为管理角色分配管理任意文章的权限
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r, permissions p
WHERE r.name IN ('admin', 'editor', 'superadmin')
  AND p.name IN ('post:manage_any', 'post:read_any')
  AND NOT EXISTS (
    SELECT 1 FROM role_permissions rp 
    WHERE rp.role_id = r.id AND rp.permission_id = p.id
  );
