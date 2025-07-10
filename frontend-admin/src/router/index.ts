import { createRouter, createWebHistory } from 'vue-router'
import LoginView from '@/views/LoginView.vue'
import DashboardView from '@/views/DashboardView.vue'
import { useAuthStore } from '@/stores/auth.ts'

// 声明路由meta的类型
declare module 'vue-router' {
  interface RouteMeta {
    requiresAuth?: boolean
    requiredPermissions?: string[]
  }
}

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/login',
      name: 'login',
      component: LoginView,
      meta: { requiresAuth: false },
    },
    {
      path: '/dashboard',
      name: 'dashboard',
      component: DashboardView,
      meta: { requiresAuth: true },
    },
    {
      path: '/posts',
      name: 'posts',
      component: () => import('@/views/PostsView.vue'),
      meta: { requiresAuth: true },
    },
    {
      path: '/posts/new',
      name: 'posts-new',
      component: () => import('@/views/PostEditView.vue'),
      meta: { requiresAuth: true },
    },
    {
      path: '/posts/:id/edit',
      name: 'posts-edit',
      component: () => import('@/views/PostEditView.vue'),
      meta: { requiresAuth: true },
    },
    // 分类管理
    {
      path: '/categories',
      name: 'categories',
      component: () => import('@/views/CategoriesView.vue'),
      meta: { requiresAuth: true },
    },
    // 标签管理
    {
      path: '/tags',
      name: 'tags',
      component: () => import('@/views/TagsView.vue'),
      meta: { requiresAuth: true },
    },
    // 用户管理
    {
      path: '/users',
      name: 'users',
      component: () => import('@/views/UsersView.vue'),
      meta: {
        requiresAuth: true,
        requiredPermissions: ['admin:user_management'],
      },
    },
    // 角色权限管理
    {
      path: '/roles',
      name: 'roles',
      component: () => import('@/views/RolesView.vue'),
      meta: {
        requiresAuth: true,
        requiredPermissions: ['admin:role_management'],
      },
    },
    {
      path: '/',
      redirect: '/dashboard',
    },
    {
      path: '/:pathMatch(.*)*',
      name: 'not-found',
      component: () => import('@/views/NotFoundView.vue'),
    },
  ],
})

// 全局前置导航守卫
// 在每次路由跳转之前执行
router.beforeEach((to, from, next) => {
  const authStore = useAuthStore()
  const requireAuth = to.meta.requiresAuth !== false // 默认需要认证

  // 如果路由需要认证，但用户未登录
  if (requireAuth && !authStore.isAuthenticated) {
    // 将用户重定向到登录页
    next({ name: 'login' })
    return
  }

  // 如果用户已登录，但想访问登录页，就转到dashboard
  if (to.name === 'login' && authStore.isAuthenticated) {
    next({ name: 'dashboard' })
    return
  }

  // 检查路由权限要求
  const requiredPermissions = to.meta.requiredPermissions as string[] | undefined
  if (requiredPermissions && requiredPermissions.length > 0) {
    // 检查用户是否具有所需权限
    if (!authStore.hasAnyPermission(requiredPermissions)) {
      // 权限不足，重定向到仪表板或显示403页面
      console.warn(`访问 ${to.path} 需要权限: ${requiredPermissions.join(', ')}`)
      next({ name: 'dashboard' })
      return
    }
  }

  // 否则直接放行
  next()
})

export default router
