import { defineStore } from 'pinia'
import type { User, Permission } from '@/types'
import apiClient from '@/api'
import router from '@/router'
import { computed, ref } from 'vue'

// 定义登录凭据的类型，匹配后端的 UserLoginPayload DTO
interface LoginCredentials {
  username: string
  password: string
}

/*// 定义 state 的类型
interface AuthState {
    user: User | null;
    token: string | null;
}*/

/*
// 选项式api风格
export const useAuthStore = defineStore('auth', {
    // 状态： 定义 store 的数据
    state: (): AuthState => ({
        user: null,
        // 从 localStorage 初始化 token，防止刷新页面后丢失登录状态
        token: localStorage.getItem('accessToken') || null
    }),
    // Getters: 类似计算属性，可以派生出新的状态
    getters: {
        isAuthenticated: (state) => !!state.token,
        getUser: (state) => state.user,
    },
    // Actions: 定义可以修改状态的业务逻辑方法
    actions: {
        // 登录方法
        async login(credentials: LoginCredentials) {
            try {
                // 调用后端 /auth/login 接口
                const response = await apiClient.post('/auth/login', credentials);
                const {access_token, refresh_token, user} = response.data;
                // 更新状态
                this.token = access_token;
                this.user = user;
                // 将token存储到 localStorage 以实现持久化登录
                localStorage.setItem('accessToken', access_token);
                localStorage.setItem('refreshToken', refresh_token);
                // 登录成功后跳转到后台主页
                await (router as any).push({name: 'dashboard'});
            } catch (error) {
                // 处理登录失败
                console.error('Login failed:', error);
                throw error;
            }
        },
        // 登出方法
        async logout() {
            try {
                const refreshToken = localStorage.getItem('refreshToken');
                if (refreshToken) {
                    // 调用后端接口
                    await apiClient.post('/auth/logput', {refresh_token: refreshToken});
                }
            } catch (error) {
                console.error('Logout API call failed:', error);
            } finally {
                // 清理状态
                this.user = null;
                this.token = null;
                // 清理 localStorage
                localStorage.removeItem('accessToken');
                localStorage.removeItem('refreshToken');
                // 跳转到登录页
                await (router as any).push({name: 'login'});
            }
        },
        // 应用加载时，尝试获取用户信息
        async fetchUserProfile() {
            if (!this.isAuthenticated) return;
            try {
                const response = await apiClient.get('/me');
                // 后端返回的是 UserPublic
                this.user = response.data as User;
            } catch (error) {
                console.error('Failed to fetch user profile,logging out.', error);
                // 如果 token 失效，自动登出
                await this.logout();
            }
        },
    }
})*/

// 组合式api风格
export const useAuthStore = defineStore('auth', () => {
  const user = ref<User | null>(null)
  const token = ref<string | null>(localStorage.getItem('accessToken'))
  const userPermissions = ref<Permission[]>([])

  // 保存最后登录的凭据（仅在内存中，退出浏览器后会清除）
  const lastLoginCredentials = ref<LoginCredentials | null>(null)

  const isAuthenticated = computed(() => !!token.value)
  const getUser = computed(() => user.value)
  const getPermissions = computed(() => userPermissions.value)

  // 权限检查函数
  const hasPermission = (permission: string): boolean => {
    return userPermissions.value.some((p) => p.name === permission)
  }

  // 批量权限检查函数
  const hasAnyPermission = (permissions: string[]): boolean => {
    return permissions.some((permission) => hasPermission(permission))
  }

  // 检查是否拥有所有指定权限
  const hasAllPermissions = (permissions: string[]): boolean => {
    return permissions.every((permission) => hasPermission(permission))
  }

  // 草稿权限检查方法
  const canCreateDraft = (): boolean => hasPermission('post:draft:create')
  const canReadOwnDrafts = (): boolean => hasPermission('post:draft:read_own')
  const canReadAnyDrafts = (): boolean => hasPermission('post:draft:read_any')
  const canEditOwnDrafts = (): boolean => hasPermission('post:draft:edit_own')
  const canEditAnyDrafts = (): boolean => hasPermission('post:draft:edit_any')
  const canDeleteOwnDrafts = (): boolean => hasPermission('post:draft:delete_own')
  const canDeleteAnyDrafts = (): boolean => hasPermission('post:draft:delete_any')
  const canShareDrafts = (): boolean => hasPermission('post:draft:share')
  const canAccessSharedDrafts = (): boolean => hasPermission('post:draft:access_shared')

  // 检查是否有紧急访问他人草稿的权限（管理员）
  const canEmergencyAccessDrafts = (): boolean => hasPermission('post:draft:read_any')

  // 检查是否可以访问特定草稿
  const canAccessDraft = (post: any): boolean => {
    if (!post || post.published_at) return true // 已发布文章任何人都可以查看

    const currentUserId = user.value?.id
    if (!currentUserId) return false

    // 1. 如果是作者本人，需要有读取自己草稿的权限
    if (post.author_id === currentUserId) {
      return canReadOwnDrafts()
    }

    // 2. 如果被分享给当前用户
    if (post.draft_shared_with?.includes(currentUserId)) return true

    // 3. 如果是公开草稿且用户有权限查看
    if (post.is_draft_public && canAccessSharedDrafts()) return true

    // 注意：移除了管理员随意访问草稿的权限
    // 管理员只能通过特殊的紧急访问功能访问他人草稿，且会记录审计日志

    return false
  }

  // 检查是否可以编辑特定草稿
  const canEditDraft = (post: any): boolean => {
    if (!post) return false

    const currentUserId = user.value?.id
    if (!currentUserId) return false

    // 如果已发布，需要其他权限检查
    if (post.published_at)
      return (
        hasPermission('post:edit_any') ||
        (hasPermission('post:edit_own') && post.author_id === currentUserId)
      )

    // 草稿编辑权限 - 只允许作者编辑自己的草稿
    if (post.author_id === currentUserId) return canEditOwnDrafts()

    // 注意：移除了管理员随意编辑草稿的权限
    // 草稿隐私保护：只有作者才能编辑自己的草稿

    return false
  }

  // 登录方法
  async function login(credentials: LoginCredentials) {
    try {
      // 调用后端 /auth/login 接口
      const response = await apiClient.post('/auth/login', credentials)
      const { access_token, refresh_token, user: userData } = response.data
      // 更新状态
      token.value = access_token
      user.value = userData
      // 保存最后登录的凭据（登录成功后保存）
      lastLoginCredentials.value = { ...credentials }
      // 将token存储到 localStorage 以实现持久化登录
      localStorage.setItem('accessToken', access_token)
      localStorage.setItem('refreshToken', refresh_token)

      // 获取用户权限
      await fetchUserPermissions()

      // 登录成功后跳转到后台主页
      await (router as any).push({ name: 'dashboard' })
    } catch (error) {
      // 处理登录失败
      console.error('Login failed:', error)
      throw error
    }
  }

  // 登出方法
  async function logout() {
    // 如果正在刷新token，等待刷新完成
    if ((window as any).isRefreshing) {
      console.log('正在刷新token，延迟logout...')
      await new Promise((resolve) => setTimeout(resolve, 1000))
    }

    try {
      const refreshToken = localStorage.getItem('refreshToken')
      if (refreshToken) {
        // 调用后端接口
        await apiClient.post('/auth/logout', { refresh_token: refreshToken })
      }
    } catch (error) {
      console.error('Logout API call failed:', error)
      // 如果是401错误（token已失效），这是正常的，不需要再报错
      if (error && typeof error === 'object' && 'response' in error) {
        const axiosError = error as { response?: { status?: number } }
        if (axiosError.response?.status === 401) {
          console.log('Refresh token已失效，这是正常的logout流程')
        }
      }
    } finally {
      // 清理状态
      user.value = null
      token.value = null
      userPermissions.value = []
      // 清理 localStorage
      localStorage.removeItem('accessToken')
      localStorage.removeItem('refreshToken')
      // 跳转到登录页
      await (router as any).push({ name: 'login' })
    }
  }

  // 获取用户权限
  async function fetchUserPermissions() {
    if (!isAuthenticated.value) return
    try {
      const response = await apiClient.get('/me/permissions')
      userPermissions.value = response.data || []
    } catch (error) {
      console.error('Failed to fetch user permissions:', error)
      userPermissions.value = []
    }
  }

  // 应用加载时，尝试获取用户信息
  async function fetchUserProfile() {
    if (!isAuthenticated.value) return
    try {
      const response = await apiClient.get('/me')
      // 后端返回的是 UserPublic
      user.value = response.data as User
      // 同时获取用户权限
      await fetchUserPermissions()
    } catch (error) {
      console.error('Failed to fetch user profile,logging out.', error)
      // 如果 token 失效，自动登出
      await logout()
    }
  }

  // 获取最后登录的凭据
  function getLastLoginCredentials() {
    return lastLoginCredentials.value
  }

  // 清除最后登录的凭据
  function clearLastLoginCredentials() {
    lastLoginCredentials.value = null
  }

  // 手动更新token（用于刷新token时）
  function updateToken(newToken: string) {
    token.value = newToken
    localStorage.setItem('accessToken', newToken)
  }

  return {
    user,
    token,
    userPermissions,
    isAuthenticated,
    getUser,
    getPermissions,
    hasPermission,
    hasAnyPermission,
    hasAllPermissions,
    // 草稿权限检查方法
    canCreateDraft,
    canReadOwnDrafts,
    canReadAnyDrafts,
    canEditOwnDrafts,
    canEditAnyDrafts,
    canDeleteOwnDrafts,
    canDeleteAnyDrafts,
    canShareDrafts,
    canAccessSharedDrafts,
    canEmergencyAccessDrafts,
    canAccessDraft,
    canEditDraft,
    // 认证操作方法
    login,
    logout,
    fetchUserProfile,
    fetchUserPermissions,
    getLastLoginCredentials,
    clearLastLoginCredentials,
    updateToken,
  }
})
