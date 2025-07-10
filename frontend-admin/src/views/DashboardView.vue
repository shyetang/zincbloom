<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useAuthStore } from '@/stores/auth.ts'
import AdminLayout from '../../layouts/AdminLayout.vue'
import apiClient from '@/api'
import type { DashboardStats, UserStats } from '@/types'

const authStore = useAuthStore()

// 统计数据
const stats = ref<DashboardStats>({
  total_posts: 0,
  published_posts: 0,
  draft_posts: 0,
  total_categories: 0,
  total_tags: 0,
  total_users: 0,
  verified_users: 0,
  unverified_users: 0,
})

const userStats = ref<UserStats | null>(null)

const loading = ref(true)
const error = ref<string | null>(null)

// 获取统计数据
const fetchStats = async () => {
  try {
    loading.value = true
    error.value = null

    // 根据用户权限获取不同的统计数据
    if (authStore.hasPermission('admin:view_statistics')) {
      // 管理员：获取仪表板统计数据和用户统计
      const [dashboardRes, userStatsRes] = await Promise.allSettled([
        apiClient.get('/admin/stats/dashboard'),
        authStore.hasPermission('admin:user_management')
          ? apiClient.get('/admin/stats/users')
          : Promise.resolve({ data: null }),
      ])

      // 处理仪表板统计
      if (dashboardRes.status === 'fulfilled') {
        stats.value = dashboardRes.value.data
      }

      // 处理用户统计（仅管理员可见）
      if (userStatsRes.status === 'fulfilled' && userStatsRes.value.data) {
        userStats.value = userStatsRes.value.data
      }
    } else {
      // 普通用户：获取个人统计数据
      const response = await apiClient.get('/me/stats')
      stats.value = response.data
    }
  } catch (err) {
    console.error('获取统计数据失败:', err)
    error.value = '获取统计数据失败'
  } finally {
    loading.value = false
  }
}

// 快捷操作
const quickActions = [
  {
    name: '写新文章',
    icon: '✍️',
    route: '/posts/new',
    color: 'primary',
    requiredPermissions: [], // 所有用户都可以写文章
  },
  {
    name: '管理分类',
    icon: '📁',
    route: '/categories',
    color: 'secondary',
    requiredPermissions: [], // 所有用户都可以查看分类
  },
  {
    name: '管理标签',
    icon: '🏷️',
    route: '/tags',
    color: 'secondary',
    requiredPermissions: [], // 所有用户都可以查看标签
  },
  {
    name: '用户管理',
    icon: '👥',
    route: '/users',
    color: 'secondary',
    requiredPermissions: ['admin:user_management'], // 需要用户管理权限
  },
]

// 计算过滤后的快捷操作
const filteredQuickActions = computed(() => {
  return quickActions.filter((action) => {
    // 如果没有权限要求，直接显示
    if (!action.requiredPermissions.length) return true

    // 检查用户是否具有所需权限
    return authStore.hasAnyPermission(action.requiredPermissions)
  })
})

onMounted(() => {
  fetchStats()
})

const formatDate = (date: string) => {
  return new Date(date).toLocaleDateString('zh-CN')
}
</script>

<template>
  <AdminLayout>
    <div class="dashboard">
      <!-- 页面头部 -->
      <div class="page-header">
        <div class="page-title">
          <h1>仪表板</h1>
          <p class="page-subtitle">
            欢迎回来，{{ authStore.user?.username }}！ 今天是
            {{ formatDate(new Date().toISOString()) }}
          </p>
        </div>
      </div>

      <!-- 统计卡片 -->
      <div class="stats-grid">
        <div class="stat-card">
          <div class="stat-icon primary">📝</div>
          <div class="stat-content">
            <div class="stat-label">总文章数</div>
            <div class="stat-value">{{ loading ? '-' : stats.total_posts }}</div>
          </div>
        </div>

        <div class="stat-card">
          <div class="stat-icon success">✅</div>
          <div class="stat-content">
            <div class="stat-label">已发布</div>
            <div class="stat-value">{{ loading ? '-' : stats.published_posts }}</div>
          </div>
        </div>

        <div class="stat-card">
          <div class="stat-icon warning">📄</div>
          <div class="stat-content">
            <div class="stat-label">草稿</div>
            <div class="stat-value">{{ loading ? '-' : stats.draft_posts }}</div>
          </div>
        </div>

        <div class="stat-card">
          <div class="stat-icon secondary">📁</div>
          <div class="stat-content">
            <div class="stat-label">分类数</div>
            <div class="stat-value">{{ loading ? '-' : stats.total_categories }}</div>
          </div>
        </div>

        <div class="stat-card">
          <div class="stat-icon info">🏷️</div>
          <div class="stat-content">
            <div class="stat-label">标签数</div>
            <div class="stat-value">{{ loading ? '-' : stats.total_tags }}</div>
          </div>
        </div>

        <!-- 根据用户权限显示用户统计 -->
        <div v-if="authStore.hasPermission('admin:user_management')" class="stat-card">
          <div class="stat-icon primary">👥</div>
          <div class="stat-content">
            <div class="stat-label">总用户数</div>
            <div class="stat-value">{{ loading ? '-' : stats.total_users }}</div>
          </div>
        </div>

        <div v-if="authStore.hasPermission('admin:user_management')" class="stat-card">
          <div class="stat-icon success">✔️</div>
          <div class="stat-content">
            <div class="stat-label">已验证用户</div>
            <div class="stat-value">{{ loading ? '-' : stats.verified_users }}</div>
          </div>
        </div>

        <div v-if="authStore.hasPermission('admin:user_management')" class="stat-card">
          <div class="stat-icon warning">⏳</div>
          <div class="stat-content">
            <div class="stat-label">未验证用户</div>
            <div class="stat-value">{{ loading ? '-' : stats.unverified_users }}</div>
          </div>
        </div>
      </div>

      <!-- 快捷操作 -->
      <div class="quick-actions-section">
        <h2 class="section-title">快捷操作</h2>
        <div class="quick-actions-grid">
          <router-link
            v-for="action in filteredQuickActions"
            :key="action.name"
            :to="action.route"
            class="quick-action-card"
            :class="`quick-action-${action.color}`"
          >
            <div class="quick-action-icon">{{ action.icon }}</div>
            <div class="quick-action-name">{{ action.name }}</div>
          </router-link>
        </div>
      </div>

      <!-- 最近活动 -->
      <div class="recent-activity-section">
        <h2 class="section-title">最近活动</h2>
        <div class="card">
          <div class="card-body">
            <div v-if="loading" class="loading-state">
              <div class="loading-spinner"></div>
              <p>加载中...</p>
            </div>
            <div v-else-if="error" class="error-state">
              <p class="error-message">{{ error }}</p>
              <button @click="fetchStats" class="btn btn-primary btn-sm">重试</button>
            </div>
            <div v-else class="activity-list">
              <div class="activity-item">
                <div class="activity-icon">📝</div>
                <div class="activity-content">
                  <div class="activity-text">系统初始化完成</div>
                  <div class="activity-time">{{ formatDate(new Date().toISOString()) }}</div>
                </div>
              </div>
              <div class="activity-item">
                <div class="activity-icon">🎉</div>
                <div class="activity-content">
                  <div class="activity-text">欢迎使用 ZincBloom 博客管理系统</div>
                  <div class="activity-time">{{ formatDate(new Date().toISOString()) }}</div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </AdminLayout>
</template>

<style scoped>
.dashboard {
  max-width: 1200px;
}

.page-header {
  margin-bottom: var(--space-8);
}

.page-title h1 {
  font-size: var(--text-3xl);
  font-weight: 700;
  color: var(--color-gray-900);
  margin: 0 0 var(--space-2);
}

.page-subtitle {
  color: var(--color-gray-600);
  font-size: var(--text-base);
  margin: 0;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: var(--space-6);
  margin-bottom: var(--space-8);
}

.stat-card {
  background: var(--color-white);
  border-radius: var(--radius-lg);
  padding: var(--space-6);
  box-shadow: var(--shadow-base);
  display: flex;
  align-items: center;
  gap: var(--space-4);
  transition: all var(--transition-fast);
}

.stat-card:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-lg);
}

.stat-icon {
  width: 60px;
  height: 60px;
  border-radius: var(--radius-xl);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: var(--text-2xl);
  flex-shrink: 0;
}

.stat-icon.primary {
  background: linear-gradient(45deg, var(--color-primary), #8b5cf6);
}

.stat-icon.success {
  background: linear-gradient(45deg, var(--color-success), #059669);
}

.stat-icon.warning {
  background: linear-gradient(45deg, var(--color-warning), #d97706);
}

.stat-icon.secondary {
  background: linear-gradient(45deg, var(--color-secondary), #374151);
}

.stat-icon.info {
  background: linear-gradient(45deg, var(--color-info), #0891b2);
}

.stat-content {
  flex: 1;
}

.stat-label {
  font-size: var(--text-sm);
  color: var(--color-gray-600);
  margin-bottom: var(--space-1);
}

.stat-value {
  font-size: var(--text-2xl);
  font-weight: 700;
  color: var(--color-gray-900);
}

.quick-actions-section,
.recent-activity-section {
  margin-bottom: var(--space-8);
}

.section-title {
  font-size: var(--text-xl);
  font-weight: 600;
  color: var(--color-gray-900);
  margin: 0 0 var(--space-4);
}

.quick-actions-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  gap: var(--space-4);
}

.quick-action-card {
  background: var(--color-white);
  border-radius: var(--radius-lg);
  padding: var(--space-6);
  text-decoration: none;
  color: var(--color-gray-700);
  box-shadow: var(--shadow-base);
  transition: all var(--transition-fast);
  text-align: center;
  border: 2px solid transparent;
}

.quick-action-card:hover {
  transform: translateY(-4px);
  box-shadow: var(--shadow-lg);
}

.quick-action-primary:hover {
  border-color: var(--color-primary);
}

.quick-action-secondary:hover {
  border-color: var(--color-secondary);
}

.quick-action-icon {
  font-size: var(--text-3xl);
  margin-bottom: var(--space-3);
}

.quick-action-name {
  font-weight: 600;
  font-size: var(--text-base);
}

.loading-state,
.error-state {
  text-align: center;
  padding: var(--space-8);
}

.loading-spinner {
  width: 40px;
  height: 40px;
  border: 3px solid var(--color-gray-200);
  border-top: 3px solid var(--color-primary);
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto var(--space-4);
}

@keyframes spin {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}

.error-state .error-message {
  color: var(--color-error);
  margin-bottom: var(--space-4);
}

.activity-item {
  display: flex;
  align-items: center;
  gap: var(--space-4);
  padding: var(--space-4);
  border-left: 3px solid var(--color-primary);
  background-color: var(--color-gray-50);
  border-radius: 0 var(--radius-md) var(--radius-md) 0;
}

.activity-item:not(:last-child) {
  margin-bottom: var(--space-4);
}

.activity-icon {
  font-size: var(--text-lg);
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--color-white);
  border-radius: var(--radius-full);
  flex-shrink: 0;
}

.activity-content {
  flex: 1;
}

.activity-text {
  font-weight: 500;
  color: var(--color-gray-900);
  margin-bottom: var(--space-1);
}

.activity-time {
  font-size: var(--text-sm);
  color: var(--color-gray-500);
}

/* 响应式设计 */
@media (max-width: 768px) {
  .stats-grid {
    grid-template-columns: 1fr;
    gap: var(--space-4);
  }

  .quick-actions-grid {
    grid-template-columns: repeat(2, 1fr);
  }

  .stat-card {
    padding: var(--space-4);
  }

  .quick-action-card {
    padding: var(--space-4);
  }

  .quick-action-icon {
    font-size: var(--text-2xl);
    margin-bottom: var(--space-2);
  }
}
</style>
