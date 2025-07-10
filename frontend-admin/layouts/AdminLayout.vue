<script setup lang="ts">
import { ref, computed } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

const router = useRouter()
const route = useRoute()
const authStore = useAuthStore()

const sidebarOpen = ref(false)

const navigation = [
  {
    name: '仪表板',
    href: '/dashboard',
    icon: '📊',
    enabled: true,
    requiredPermissions: [], // 所有用户都可以访问仪表板
  },
  {
    name: '文章管理',
    href: '/posts',
    icon: '📝',
    enabled: true,
    requiredPermissions: [], // 所有登录用户都可以管理文章
  },
  {
    name: '分类管理',
    href: '/categories',
    icon: '📁',
    enabled: true,
    requiredPermissions: [], // 所有登录用户都可以查看分类
  },
  {
    name: '标签管理',
    href: '/tags',
    icon: '🏷️',
    enabled: true,
    requiredPermissions: [], // 所有登录用户都可以查看标签
  },
  {
    name: '用户管理',
    href: '/users',
    icon: '👥',
    enabled: true,
    requiredPermissions: ['admin:user_management'], // 需要用户管理权限
  },
  {
    name: '角色权限',
    href: '/roles',
    icon: '🔐',
    enabled: true,
    requiredPermissions: ['admin:role_management'], // 需要角色管理权限
  },
]

// 计算过滤后的导航菜单
const filteredNavigation = computed(() => {
  return navigation.filter((item) => {
    // 如果没有权限要求，直接显示
    if (!item.requiredPermissions.length) return item.enabled

    // 检查用户是否具有所需权限
    return item.enabled && authStore.hasAnyPermission(item.requiredPermissions)
  })
})

const isActiveRoute = (href: string) => {
  return route.path.startsWith(href)
}

const handleLogout = async () => {
  await authStore.logout()
}

const toggleSidebar = () => {
  sidebarOpen.value = !sidebarOpen.value
}
</script>

<template>
  <div class="admin-layout">
    <!-- 移动端遮罩 -->
    <div v-if="sidebarOpen" class="sidebar-overlay" @click="toggleSidebar"></div>

    <!-- 侧边栏 -->
    <aside class="sidebar" :class="{ 'sidebar-open': sidebarOpen }">
      <div class="sidebar-header">
        <h2 class="sidebar-logo">ZincBloom</h2>
        <span class="sidebar-subtitle">管理后台</span>
      </div>

      <nav class="sidebar-nav">
        <template v-for="item in filteredNavigation" :key="item.name">
          <router-link
            v-if="item.enabled"
            :to="item.href"
            class="nav-item"
            :class="{ 'nav-item-active': isActiveRoute(item.href) }"
            @click="sidebarOpen = false"
          >
            <span class="nav-icon">{{ item.icon }}</span>
            <span class="nav-text">{{ item.name }}</span>
          </router-link>
          <div v-else class="nav-item nav-item-disabled" :title="`${item.name} - 即将推出`">
            <span class="nav-icon">{{ item.icon }}</span>
            <span class="nav-text">{{ item.name }}</span>
            <span class="nav-badge">即将推出</span>
          </div>
        </template>
      </nav>

      <div class="sidebar-footer">
        <div class="user-info">
          <div class="user-avatar">
            {{ authStore.user?.username?.charAt(0).toUpperCase() }}
          </div>
          <div class="user-details">
            <div class="user-name">{{ authStore.user?.username }}</div>
            <div class="user-email">{{ authStore.user?.email }}</div>
          </div>
        </div>
        <button @click="handleLogout" class="logout-btn">
          <span>🚪</span>
          <span>退出登录</span>
        </button>
      </div>
    </aside>

    <!-- 主内容区域 -->
    <div class="main-content">
      <!-- 顶部栏 -->
      <header class="top-bar">
        <button @click="toggleSidebar" class="menu-toggle">
          <span class="hamburger"></span>
          <span class="hamburger"></span>
          <span class="hamburger"></span>
        </button>

        <div class="top-bar-right">
          <div class="user-menu">
            <span>欢迎，{{ authStore.user?.username }}</span>
          </div>
        </div>
      </header>

      <!-- 页面内容 -->
      <main class="page-content">
        <slot />
      </main>
    </div>
  </div>
</template>

<style scoped>
.admin-layout {
  display: flex;
  min-height: 100vh;
  background-color: var(--color-gray-50);
}

/* 侧边栏 */
.sidebar {
  position: fixed;
  top: 0;
  left: 0;
  bottom: 0;
  width: 280px;
  background: linear-gradient(180deg, var(--color-gray-900) 0%, var(--color-gray-800) 100%);
  color: var(--color-white);
  transform: translateX(-100%);
  transition: transform var(--transition-base);
  z-index: 50;
  display: flex;
  flex-direction: column;
}

.sidebar-open {
  transform: translateX(0);
}

.sidebar-overlay {
  position: fixed;
  inset: 0;
  background-color: rgba(0, 0, 0, 0.5);
  z-index: 40;
}

.sidebar-header {
  padding: var(--space-6);
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.sidebar-logo {
  font-size: var(--text-2xl);
  font-weight: 700;
  margin: 0;
  background: linear-gradient(45deg, #60a5fa, #34d399);
  background-clip: text;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.sidebar-subtitle {
  font-size: var(--text-sm);
  color: var(--color-gray-400);
}

.sidebar-nav {
  flex: 1;
  padding: var(--space-4) 0;
}

.nav-item {
  display: flex;
  align-items: center;
  padding: var(--space-3) var(--space-6);
  color: var(--color-gray-300);
  text-decoration: none;
  transition: all var(--transition-fast);
  border-left: 3px solid transparent;
}

.nav-item:hover {
  background-color: rgba(255, 255, 255, 0.05);
  color: var(--color-white);
}

.nav-item-active {
  background-color: rgba(59, 130, 246, 0.1);
  color: var(--color-white);
  border-left-color: var(--color-primary);
}

.nav-item-disabled {
  opacity: 0.5;
  cursor: not-allowed;
  position: relative;
}

.nav-item-disabled:hover {
  background-color: transparent;
  color: var(--color-gray-300);
}

.nav-badge {
  font-size: var(--text-xs);
  background-color: rgba(255, 255, 255, 0.2);
  color: var(--color-gray-300);
  padding: var(--space-1) var(--space-2);
  border-radius: var(--radius-full);
  margin-left: auto;
}

.nav-icon {
  font-size: var(--text-lg);
  margin-right: var(--space-3);
  width: 24px;
  text-align: center;
}

.nav-text {
  font-weight: 500;
}

.sidebar-footer {
  padding: var(--space-4);
  border-top: 1px solid rgba(255, 255, 255, 0.1);
}

.user-info {
  display: flex;
  align-items: center;
  padding: var(--space-3);
  margin-bottom: var(--space-3);
  background-color: rgba(255, 255, 255, 0.05);
  border-radius: var(--radius-lg);
}

.user-avatar {
  width: 40px;
  height: 40px;
  border-radius: var(--radius-full);
  background: linear-gradient(45deg, var(--color-primary), var(--color-success));
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 600;
  margin-right: var(--space-3);
}

.user-details {
  flex: 1;
}

.user-name {
  font-weight: 600;
  font-size: var(--text-sm);
}

.user-email {
  font-size: var(--text-xs);
  color: var(--color-gray-400);
}

.logout-btn {
  display: flex;
  align-items: center;
  width: 100%;
  padding: var(--space-3);
  background: none;
  border: 1px solid rgba(255, 255, 255, 0.2);
  color: var(--color-gray-300);
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all var(--transition-fast);
  font-size: var(--text-sm);
}

.logout-btn:hover {
  background-color: rgba(239, 68, 68, 0.1);
  border-color: var(--color-error);
  color: var(--color-white);
}

.logout-btn span:first-child {
  margin-right: var(--space-2);
}

/* 主内容区域 */
.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  margin-left: 0;
}

.top-bar {
  height: 64px;
  background-color: var(--color-white);
  border-bottom: 1px solid var(--color-gray-200);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 var(--space-6);
  box-shadow: var(--shadow-sm);
  z-index: 30;
}

.menu-toggle {
  display: flex;
  flex-direction: column;
  justify-content: space-around;
  width: 24px;
  height: 24px;
  background: none;
  border: none;
  cursor: pointer;
  padding: 0;
}

.hamburger {
  width: 100%;
  height: 2px;
  background-color: var(--color-gray-600);
  transition: all var(--transition-fast);
}

.menu-toggle:hover .hamburger {
  background-color: var(--color-gray-900);
}

.top-bar-right {
  display: flex;
  align-items: center;
}

.user-menu {
  color: var(--color-gray-700);
  font-size: var(--text-sm);
  font-weight: 500;
}

.page-content {
  flex: 1;
  padding: var(--space-6);
  overflow-y: auto;
}

/* 响应式设计 */
@media (min-width: 1024px) {
  .sidebar {
    position: static;
    transform: translateX(0);
  }

  .sidebar-overlay {
    display: none;
  }

  .menu-toggle {
    display: none;
  }
}

@media (max-width: 768px) {
  .page-content {
    padding: var(--space-4);
  }

  .top-bar {
    padding: 0 var(--space-4);
  }
}
</style>
