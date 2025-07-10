import axios, { type InternalAxiosRequestConfig } from 'axios'
import { useAuthStore } from '@/stores/auth.ts'

// 防止并发刷新token的全局状态
let isRefreshing = false
let failedQueue: Array<{
  resolve: (value?: any) => void
  reject: (reason?: any) => void
}> = []

const processQueue = (error: any, token: string | null = null) => {
  failedQueue.forEach(({ resolve, reject }) => {
    if (error) {
      reject(error)
    } else {
      resolve(token)
    }
  })

  failedQueue = []
}

const apiClient = axios.create({
  baseURL: import.meta.env.VITE_API_BASE_URL || 'http://localhost:8080',
  headers: {
    'Content-Type': 'application/json',
  },
  timeout: 10000, // 10秒超时
})

// 请求拦截器
// 在每个请求被发送前，拦截它并附加 Authorization 头
apiClient.interceptors.request.use(
  (config: InternalAxiosRequestConfig) => {
    // 在 Pinia 初始化之前，路由守卫可能会先运行，
    // 所以需要在函数内部获取 store 实例。
    const authStore = useAuthStore()
    const token = authStore.token

    if (token) {
      config.headers.Authorization = `Bearer ${token}`
    }
    return config
  },
  (error) => {
    return Promise.reject(error)
  },
)

// 响应拦截器
apiClient.interceptors.response.use(
  (response) => {
    return response
  },
  async (error) => {
    const originalRequest = error.config

    // 处理认证错误 - 尝试刷新token
    if (error.response?.status === 401 && !originalRequest._retry) {
      if (isRefreshing) {
        // 如果正在刷新，将请求加入队列
        return new Promise((resolve, reject) => {
          failedQueue.push({ resolve, reject })
        })
          .then((token) => {
            originalRequest.headers.Authorization = `Bearer ${token}`
            return apiClient(originalRequest)
          })
          .catch((err) => {
            return Promise.reject(err)
          })
      }

      originalRequest._retry = true
      isRefreshing = true
      // 暴露刷新状态到全局，供logout等函数使用
      ;(window as any).isRefreshing = true

      try {
        const refreshToken = localStorage.getItem('refreshToken')
        if (refreshToken) {
          console.log('尝试刷新过期的token...')
          // 尝试刷新token
          const response = await axios.post(
            `${import.meta.env.VITE_API_BASE_URL || 'http://localhost:8080'}/auth/refresh`,
            { refresh_token: refreshToken },
            { headers: { 'Content-Type': 'application/json' } },
          )

          console.log('token刷新成功')

          const { access_token, refresh_token: newRefreshToken } = response.data

          // 更新存储的token
          localStorage.setItem('refreshToken', newRefreshToken)

          // 更新store中的token
          const authStore = useAuthStore()
          authStore.updateToken(access_token)

          // 处理队列中的请求
          processQueue(null, access_token)

          // 重新发送原始请求
          originalRequest.headers.Authorization = `Bearer ${access_token}`
          return apiClient(originalRequest)
        }
      } catch (refreshError) {
        console.error('刷新token失败:', refreshError)
        // 处理队列中的请求
        processQueue(refreshError, null)

        // 刷新失败，清除所有认证信息并跳转到登录页
        const authStore = useAuthStore()
        authStore.logout()
        return Promise.reject(refreshError)
      } finally {
        isRefreshing = false
        ;(window as any).isRefreshing = false
      }
    }

    // 处理网络错误
    if (!error.response) {
      console.error('网络错误:', error.message)
    }

    return Promise.reject(error)
  },
)

export default apiClient
