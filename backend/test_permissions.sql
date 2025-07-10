-- 检查权限分配状态的测试脚本

-- 1. 检查所有角色
SELECT 'All Roles:' as info, name FROM roles ORDER BY name;

-- 2. 检查所有草稿相关权限
SELECT 'Draft Permissions:' as info, name, description FROM permissions 
WHERE name LIKE 'post:draft:%' ORDER BY name;

-- 3. 检查用户角色的权限分配
SELECT 'User Role Permissions:' as info, p.name as permission_name, p.description
FROM roles r
JOIN role_permissions rp ON r.id = rp.role_id
JOIN permissions p ON rp.permission_id = p.id
WHERE r.name = 'user'
ORDER BY p.name;

-- 4. 检查author角色的权限分配
SELECT 'Author Role Permissions:' as info, p.name as permission_name, p.description
FROM roles r
JOIN role_permissions rp ON r.id = rp.role_id
JOIN permissions p ON rp.permission_id = p.id
WHERE r.name = 'author'
ORDER BY p.name;

-- 5. 检查具体用户的角色分配（替换为实际的用户名或邮箱）
SELECT 'User Role Assignments:' as info, u.username, u.email, r.name as role_name
FROM users u
JOIN user_roles ur ON u.id = ur.user_id
JOIN roles r ON ur.role_id = r.id
ORDER BY u.username;

-- 6. 检查是否有草稿文章
SELECT 'Draft Posts Count:' as info, COUNT(*) as count, author_id
FROM posts 
WHERE published_at IS NULL
GROUP BY author_id;

-- 7. 检查所有文章状态
SELECT 'All Posts:' as info, id, title, author_id, 
       CASE WHEN published_at IS NULL THEN 'DRAFT' ELSE 'PUBLISHED' END as status,
       created_at
FROM posts 
ORDER BY created_at DESC 
LIMIT 10; 