-- Add migration script here

-- 添加文章封禁权限
INSERT INTO permissions (id, name, description)
VALUES 
    (gen_random_uuid(), 'post:ban', '封禁和解封文章')
ON CONFLICT (name) DO NOTHING;

-- 将post:ban权限分配给admin角色
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r, permissions p
WHERE r.name = 'admin'
  AND p.name = 'post:ban'
  AND NOT EXISTS (
    SELECT 1 FROM role_permissions rp 
    WHERE rp.role_id = r.id AND rp.permission_id = p.id
  );

-- 将post:ban权限分配给superadmin角色
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r, permissions p
WHERE r.name = 'superadmin'
  AND p.name = 'post:ban'
  AND NOT EXISTS (
    SELECT 1 FROM role_permissions rp 
    WHERE rp.role_id = r.id AND rp.permission_id = p.id
  );
