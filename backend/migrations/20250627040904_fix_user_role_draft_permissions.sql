-- Add migration script here

-- 修复用户角色草稿权限问题
-- 问题：在草稿权限系统中，我们忽略了给基础的 'user' 角色分配草稿权限
-- 修复：为 user 角色分配基本的草稿权限

-- 为 user 角色分配基本草稿权限
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r, permissions p
WHERE r.name = 'user'
  AND p.name IN (
    'post:draft:create',
    'post:draft:read_own',
    'post:draft:edit_own',
    'post:draft:delete_own',
    'post:draft:share',
    'post:draft:access_shared'
  )
  AND NOT EXISTS (
    SELECT 1 FROM role_permissions rp 
    WHERE rp.role_id = r.id AND rp.permission_id = p.id
  );

-- 添加说明注释
COMMENT ON TABLE role_permissions IS '角色权限关联表：user角色现在具有基本的草稿管理权限，可以创建、查看、编辑、删除自己的草稿，以及分享和访问他人分享的草稿';
