-- Add migration script here

-- 权限系统重构 v2：高优先级改进
-- 1. 统一权限命名规范
-- 2. 添加文章发布和状态管理权限  
-- 3. 重新设计角色层次（guest角色在代码层面处理）

-- ================================
-- 第一部分：添加新的角色
-- ================================

-- 添加新的角色层次（不包括guest，guest在代码层面处理）
INSERT INTO roles (id, name, description)
VALUES 
    (gen_random_uuid(), 'author', '作者，可以创建和管理自己的内容'),
    (gen_random_uuid(), 'moderator', '审核员，可以审核和管理内容'),
    (gen_random_uuid(), 'superadmin', '超级管理员，拥有所有权限')
ON CONFLICT (name) DO NOTHING;

-- ================================
-- 第二部分：添加新的权限（统一命名规范）
-- ================================

-- 文章发布和状态管理权限
INSERT INTO permissions (id, name, description)
VALUES 
    (gen_random_uuid(), 'post:publish:own', '发布自己的文章'),
    (gen_random_uuid(), 'post:publish:any', '发布任意文章'),
    (gen_random_uuid(), 'post:unpublish:own', '撤回自己的文章'),
    (gen_random_uuid(), 'post:unpublish:any', '撤回任意文章'),
    (gen_random_uuid(), 'post:draft:manage', '管理草稿'),
    (gen_random_uuid(), 'post:schedule', '定时发布文章')
ON CONFLICT (name) DO NOTHING;

-- 统一命名规范的新权限
INSERT INTO permissions (id, name, description)
VALUES 
    (gen_random_uuid(), 'category:read:any', '查看所有分类'),
    (gen_random_uuid(), 'category:edit:any', '编辑任意分类'),
    (gen_random_uuid(), 'category:delete:any', '删除任意分类'),
    (gen_random_uuid(), 'tag:read:any', '查看所有标签'),
    (gen_random_uuid(), 'tag:edit:any', '编辑任意标签'),
    (gen_random_uuid(), 'tag:delete:any', '删除任意标签')
ON CONFLICT (name) DO NOTHING;

-- 管理权限重新规范化
INSERT INTO permissions (id, name, description)
VALUES 
    (gen_random_uuid(), 'user:read:any', '查看所有用户信息'),
    (gen_random_uuid(), 'user:create', '创建新用户'),
    (gen_random_uuid(), 'user:edit:any', '编辑任意用户'),
    (gen_random_uuid(), 'user:delete:any', '删除任意用户'),
    (gen_random_uuid(), 'user:role:assign', '分配用户角色'),
    (gen_random_uuid(), 'role:create', '创建新角色'),
    (gen_random_uuid(), 'role:edit:any', '编辑任意角色'),
    (gen_random_uuid(), 'role:delete:any', '删除任意角色'),
    (gen_random_uuid(), 'role:permission:assign', '分配角色权限'),
    (gen_random_uuid(), 'system:dashboard', '访问系统仪表板'),
    (gen_random_uuid(), 'system:statistics', '查看系统统计'),
    (gen_random_uuid(), 'system:logs', '查看系统日志'),
    (gen_random_uuid(), 'system:config', '系统配置管理')
ON CONFLICT (name) DO NOTHING;

-- ================================
-- 第三部分：角色权限分配
-- ================================

-- author 角色权限（基础内容创作者）
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r, permissions p
WHERE r.name = 'author'
  AND p.name IN (
    'post:read_published',
    'post:create',
    'post:edit_own',
    'post:delete_own',
    'post:publish:own',
    'post:unpublish:own',
    'post:draft:manage',
    'post:manage_own',
    'category:create',
    'tag:create'
  )
  AND NOT EXISTS (
    SELECT 1 FROM role_permissions rp 
    WHERE rp.role_id = r.id AND rp.permission_id = p.id
  );

-- moderator 角色权限（继承 author 权限 + 审核权限）
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r, permissions p
WHERE r.name = 'moderator'
  AND p.name IN (
    'post:read_published',
    'post:create',
    'post:edit_own',
    'post:delete_own',
    'post:publish:own',
    'post:unpublish:own',
    'post:draft:manage',
    'post:manage_own',
    'post:moderate',
    'post:read_any',
    'category:create',
    'category:read:any',
    'tag:create',
    'tag:read:any'
  )
  AND NOT EXISTS (
    SELECT 1 FROM role_permissions rp 
    WHERE rp.role_id = r.id AND rp.permission_id = p.id
  );

-- 将现有 user 角色的用户迁移到 author 角色
UPDATE user_roles 
SET role_id = (SELECT id FROM roles WHERE name = 'author')
WHERE role_id = (SELECT id FROM roles WHERE name = 'user');

-- 将 user 角色的权限迁移到 author 角色
INSERT INTO role_permissions (role_id, permission_id)
SELECT 
  (SELECT id FROM roles WHERE name = 'author'),
  permission_id
FROM role_permissions 
WHERE role_id = (SELECT id FROM roles WHERE name = 'user')
  AND NOT EXISTS (
    SELECT 1 FROM role_permissions rp2 
    WHERE rp2.role_id = (SELECT id FROM roles WHERE name = 'author')
      AND rp2.permission_id = role_permissions.permission_id
  );

-- 删除 user 角色的权限分配
DELETE FROM role_permissions 
WHERE role_id = (SELECT id FROM roles WHERE name = 'user');

-- editor 角色权限（继承 moderator 权限 + 内容管理权限）
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r, permissions p
WHERE r.name = 'editor'
  AND p.name IN (
    'post:read_published',
    'post:create',
    'post:edit_own',
    'post:delete_own',
    'post:publish:own',
    'post:unpublish:own',
    'post:draft:manage',
    'post:manage_own',
    'post:moderate',
    'post:read_any',
    'post:edit_any',
    'post:delete_any',
    'post:publish:any',
    'post:unpublish:any',
    'post:manage_any',
    'post:schedule',
    'category:create',
    'category:read:any',
    'category:manage',
    'category:edit:any',
    'category:delete:any',
    'tag:create',
    'tag:read:any',
    'tag:manage',
    'tag:edit:any',
    'tag:delete:any'
  )
  AND NOT EXISTS (
    SELECT 1 FROM role_permissions rp 
    WHERE rp.role_id = r.id AND rp.permission_id = p.id
  );

-- admin 角色权限（继承 editor 权限 + 用户管理权限）
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r, permissions p
WHERE r.name = 'admin'
  AND p.name IN (
    -- 继承所有 editor 权限
    'post:read_published', 'post:create', 'post:edit_own', 'post:delete_own',
    'post:publish:own', 'post:unpublish:own', 'post:draft:manage', 'post:manage_own',
    'post:moderate', 'post:read_any', 'post:edit_any', 'post:delete_any',
    'post:publish:any', 'post:unpublish:any', 'post:manage_any', 'post:schedule',
    'category:create', 'category:read:any', 'category:manage', 'category:edit:any', 'category:delete:any',
    'tag:create', 'tag:read:any', 'tag:manage', 'tag:edit:any', 'tag:delete:any',
    -- 新增用户和系统管理权限
    'user:read:any',
    'user:create',
    'user:edit:any',
    'user:delete:any',
    'user:role:assign',
    'role:create',
    'role:edit:any',
    'role:delete:any',
    'role:permission:assign',
    'system:dashboard',
    'system:statistics',
    'admin:user_management',
    'admin:role_management',
    'admin:view_permissions',
    'admin:user_list'
  )
  AND NOT EXISTS (
    SELECT 1 FROM role_permissions rp 
    WHERE rp.role_id = r.id AND rp.permission_id = p.id
  );

-- superadmin 角色权限（所有权限）
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r, permissions p
WHERE r.name = 'superadmin'
  AND NOT EXISTS (
    SELECT 1 FROM role_permissions rp 
    WHERE rp.role_id = r.id AND rp.permission_id = p.id
  );

-- ================================
-- 第四部分：清理和向后兼容
-- ================================

-- 更新 user 角色的描述以反映其废弃状态
UPDATE roles 
SET description = '已废弃：用户已迁移到author角色，保留此角色仅为向后兼容'
WHERE name = 'user';

-- 更新其他角色的描述
UPDATE roles 
SET description = '编辑人员，可以管理所有内容、分类和标签'
WHERE name = 'editor';

UPDATE roles 
SET description = '系统管理员，可以管理用户、角色和系统配置'
WHERE name = 'admin';

-- 添加注释说明游客权限处理方式
COMMENT ON TABLE roles IS '角色表：游客(guest)角色在代码层面处理，不存储在数据库中。所有游客自动拥有post:read_published权限。';
COMMENT ON TABLE permissions IS '权限表：post:read_published权限供游客和认证用户使用，游客权限在代码层面处理。';
