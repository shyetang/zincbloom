-- Add migration script here

-- 改进草稿权限系统迁移
-- 目标：保护用户草稿隐私，同时保留必要的管理功能
-- 设计原则：草稿默认完全私有，用户可主动分享，管理员仅在紧急情况下可访问

-- ================================
-- 第一部分：数据库结构改进
-- ================================

-- 为文章表添加草稿分享相关字段
ALTER TABLE posts 
ADD COLUMN IF NOT EXISTS draft_shared_with UUID[], -- 分享给哪些用户（UUID数组）
ADD COLUMN IF NOT EXISTS is_draft_public BOOLEAN DEFAULT FALSE; -- 是否允许有权限的编辑查看

-- 创建草稿访问日志表（用于审计管理员访问他人草稿）
CREATE TABLE IF NOT EXISTS draft_access_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    post_id UUID NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    accessed_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    access_type TEXT NOT NULL, -- 'view', 'edit', 'delete'
    access_reason TEXT, -- 访问原因说明
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 为草稿访问日志表创建索引
CREATE INDEX IF NOT EXISTS idx_draft_access_logs_post_id ON draft_access_logs (post_id);
CREATE INDEX IF NOT EXISTS idx_draft_access_logs_accessed_by ON draft_access_logs (accessed_by);
CREATE INDEX IF NOT EXISTS idx_draft_access_logs_created_at ON draft_access_logs (created_at);

-- ================================
-- 第二部分：添加细粒度草稿权限
-- ================================

-- 1. 添加更细粒度的草稿权限
INSERT INTO permissions (id, name, description)
VALUES 
    (gen_random_uuid(), 'post:draft:read_own', '查看自己的草稿'),
    (gen_random_uuid(), 'post:draft:read_any', '查看任意用户的草稿（仅限紧急维护）'),
    (gen_random_uuid(), 'post:draft:create', '创建草稿'),
    (gen_random_uuid(), 'post:draft:edit_own', '编辑自己的草稿'),
    (gen_random_uuid(), 'post:draft:edit_any', '编辑任意草稿（仅限紧急维护）'),
    (gen_random_uuid(), 'post:draft:delete_own', '删除自己的草稿'),
    (gen_random_uuid(), 'post:draft:delete_any', '删除任意草稿（仅限紧急维护）'),
    (gen_random_uuid(), 'post:draft:share', '分享草稿给他人查看'),
    (gen_random_uuid(), 'post:draft:access_shared', '访问他人分享的草稿')
ON CONFLICT (name) DO NOTHING;

-- ================================
-- 第三部分：清理旧权限
-- ================================

-- 2. 移除旧的泛泛权限，替换为细分权限
DELETE FROM role_permissions 
WHERE permission_id = (SELECT id FROM permissions WHERE name = 'post:draft:manage');

-- 删除旧的草稿管理权限（如果存在）
DELETE FROM permissions WHERE name = 'post:draft:manage';

-- ================================
-- 第四部分：角色权限重新分配
-- ================================

-- 3. 为各角色分配新的草稿权限

-- author: 只能管理自己的草稿 + 可以分享草稿
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r, permissions p
WHERE r.name = 'author'
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

-- moderator: 自己的草稿 + 可以访问分享的草稿（不能主动查看他人私人草稿）
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r, permissions p
WHERE r.name = 'moderator'
  AND p.name IN (
    'post:draft:create',
    'post:draft:read_own',
    'post:draft:edit_own',
    'post:draft:delete_own',
    'post:draft:share',
    'post:draft:access_shared'
    -- 注意：不给 post:draft:read_any，尊重作者隐私
  )
  AND NOT EXISTS (
    SELECT 1 FROM role_permissions rp 
    WHERE rp.role_id = r.id AND rp.permission_id = p.id
  );

-- editor: 自己的草稿 + 可以访问分享的草稿
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r, permissions p
WHERE r.name = 'editor'
  AND p.name IN (
    'post:draft:create',
    'post:draft:read_own',
    'post:draft:edit_own',
    'post:draft:delete_own',
    'post:draft:share',
    'post:draft:access_shared'
    -- 同样不给 post:draft:read_any，需要作者主动分享
  )
  AND NOT EXISTS (
    SELECT 1 FROM role_permissions rp 
    WHERE rp.role_id = r.id AND rp.permission_id = p.id
  );

-- admin: 完整权限，但访问他人草稿时会记录日志
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r, permissions p
WHERE r.name = 'admin'
  AND p.name IN (
    'post:draft:create',
    'post:draft:read_own',
    'post:draft:edit_own',
    'post:draft:delete_own',
    'post:draft:share',
    'post:draft:access_shared',
    'post:draft:read_any',  -- 仅限紧急技术维护，会记录访问日志
    'post:draft:edit_any',  -- 仅限紧急技术维护，会记录访问日志
    'post:draft:delete_any' -- 仅限紧急技术维护，会记录访问日志
  )
  AND NOT EXISTS (
    SELECT 1 FROM role_permissions rp 
    WHERE rp.role_id = r.id AND rp.permission_id = p.id
  );

-- superadmin: 所有权限
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r, permissions p
WHERE r.name = 'superadmin'
  AND p.name IN (
    'post:draft:create',
    'post:draft:read_own',
    'post:draft:edit_own',
    'post:draft:delete_own',
    'post:draft:share',
    'post:draft:access_shared',
    'post:draft:read_any',
    'post:draft:edit_any',
    'post:draft:delete_any'
  )
  AND NOT EXISTS (
    SELECT 1 FROM role_permissions rp 
    WHERE rp.role_id = r.id AND rp.permission_id = p.id
  );

-- ================================
-- 第五部分：添加注释和说明
-- ================================

-- 添加表注释
COMMENT ON TABLE posts IS '文章表：包含草稿分享功能，draft_shared_with存储分享给的用户ID数组，is_draft_public控制是否允许有权限的编辑查看';
COMMENT ON COLUMN posts.draft_shared_with IS '草稿分享给的用户ID数组，仅对未发布文章有效';
COMMENT ON COLUMN posts.is_draft_public IS '是否允许有相应权限的编辑查看此草稿，默认false保护隐私';
COMMENT ON TABLE draft_access_logs IS '草稿访问日志表：记录管理员访问他人草稿的操作，用于审计和隐私保护';

-- 添加权限说明注释
COMMENT ON TABLE permissions IS '权限表：post:draft:read_any和post:draft:edit_any等高级权限仅供紧急技术维护使用，会记录访问日志';
