-- Add migration script here
-- backend/migrations/YYYYMMDDHHMMSS_create_user_roles_table.sql

-- 创建 user_roles 表，用于用户和角色之间的多对多关系
CREATE TABLE user_roles
(
    -- 用户ID，外键关联 users 表
    user_id     UUID        NOT NULL,
    -- 角色ID，外键关联 roles 表
    role_id     UUID        NOT NULL,
    -- 角色分配时间，默认为当前时间
    assigned_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- 复合主键，确保一个用户不会重复拥有同一个角色
    PRIMARY KEY (user_id, role_id),

    -- 外键约束：关联到 users 表的 id
    CONSTRAINT fk_user_roles_user
        FOREIGN KEY (user_id)
            REFERENCES users (id)
            ON DELETE CASCADE, -- 如果用户被删除，则自动删除此关联记录

    -- 外键约束：关联到 roles 表的 id
    CONSTRAINT fk_user_roles_role
        FOREIGN KEY (role_id)
            REFERENCES roles (id)
            ON DELETE CASCADE  -- 如果角色被删除，则自动删除此关联记录
);