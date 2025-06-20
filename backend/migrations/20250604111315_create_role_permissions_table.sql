-- Add migration script here
-- backend/migrations/YYYYMMDDHHMMSS_create_role_permissions_table.sql

-- 创建 role_permissions 表，用于角色和权限之间的多对多关系
CREATE TABLE role_permissions
(
    -- 角色ID，外键关联 roles 表
    role_id       UUID        NOT NULL,
    -- 权限ID，外键关联 permissions 表
    permission_id UUID        NOT NULL,
    -- 权限分配时间，默认为当前时间
    assigned_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- 复合主键，确保一个角色不会重复拥有同一个权限
    PRIMARY KEY (role_id, permission_id),

    -- 外键约束：关联到 roles 表的 id
    CONSTRAINT fk_role_permissions_role
        FOREIGN KEY (role_id)
            REFERENCES roles (id)
            ON DELETE CASCADE, -- 如果角色被删除，则自动删除此关联记录

    -- 外键约束：关联到 permissions 表的 id
    CONSTRAINT fk_role_permissions_permission
        FOREIGN KEY (permission_id)
            REFERENCES permissions (id)
            ON DELETE CASCADE  -- 如果权限被删除，则自动删除此关联记录
);

-- 为之前插入的默认角色分配权限
-- 注意：这种方式依赖于之前 INSERT 语句中 name 的准确性，并且假设这些 name 仍然存在。
-- 在复杂的种子数据场景中，通常会使用代码或更健壮的脚本来处理。

-- 为 'admin' 角色分配所有已定义的权限（这里用一个简单的方式，实际可能需要更精确的选取）
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r,
     permissions p
WHERE r.name = 'admin';
-- 假设 'admin' 角色已在 roles 表中

-- 为 'editor' 角色分配内容管理相关权限
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r
         CROSS JOIN permissions p
WHERE r.name = 'editor'
  AND p.name IN (
                 'post:create', 'post:read_any', 'post:edit_any', 'post:delete_any',
                 'category:manage', 'tag:manage'
    );

-- 为 'user' 角色分配基础权限
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r
         CROSS JOIN permissions p
WHERE r.name = 'user'
  AND p.name IN (
                 'post:create', 'post:read_any', 'post:edit_own', 'post:delete_own'
    );