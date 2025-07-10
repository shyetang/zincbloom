<script setup lang="ts">
import { useAuthStore } from '@/stores/auth.ts'
import { ref, onMounted } from 'vue'

const authStore = useAuthStore()

// 使用ref创建响应式数据，用于绑定表单输入
const username = ref('')
const password = ref('')
const errorMessage = ref<string | null>(null)
const isLoading = ref(false)

// 是否显示了记住的凭据
const hasRememberedCredentials = ref(false)

// 组件挂载时，尝试恢复最后登录的凭据
onMounted(() => {
  const lastCredentials = authStore.getLastLoginCredentials()
  if (lastCredentials) {
    username.value = lastCredentials.username
    password.value = lastCredentials.password
    hasRememberedCredentials.value = true
  }
})

// 清除记住的凭据
const clearRememberedCredentials = () => {
  authStore.clearLastLoginCredentials()
  username.value = ''
  password.value = ''
  hasRememberedCredentials.value = false
}

const handleLogin = async () => {
  if (!username.value || !password.value) {
    errorMessage.value = '请填写用户名和密码'
    return
  }

  try {
    isLoading.value = true
    errorMessage.value = null
    await authStore.login({
      username: username.value,
      password: password.value,
    })
  } catch (error) {
    errorMessage.value = '登录失败，请检查用户名或密码'
    console.error(error)
  } finally {
    isLoading.value = false
  }
}
</script>

<template>
  <div class="login-container">
    <div class="login-card">
      <!-- Logo区域 -->
      <div class="login-header">
        <h1 class="logo">ZincBloom</h1>
        <p class="subtitle">博客管理后台</p>
      </div>

      <!-- 记住凭据提示 -->
      <div v-if="hasRememberedCredentials" class="remembered-credentials-notice">
        <div class="notice-content">
          <span class="notice-icon">🔄</span>
          <span class="notice-text">已自动填写上次登录信息</span>
          <button
            type="button"
            @click="clearRememberedCredentials"
            class="notice-clear-btn"
            title="清除记住的信息"
          >
            ✕
          </button>
        </div>
      </div>

      <!-- 登录表单 -->
      <form @submit.prevent="handleLogin" class="login-form">
        <div class="form-group">
          <label for="username" class="form-label">用户名</label>
          <input
            id="username"
            v-model="username"
            type="text"
            class="form-input"
            placeholder="请输入用户名"
            :disabled="isLoading"
            required
            autocomplete="username"
          />
        </div>

        <div class="form-group">
          <label for="password" class="form-label">密码</label>
          <input
            id="password"
            v-model="password"
            type="password"
            class="form-input"
            placeholder="请输入密码"
            :disabled="isLoading"
            required
            autocomplete="current-password"
          />
        </div>

        <div v-if="errorMessage" class="error-message">
          {{ errorMessage }}
        </div>

        <button type="submit" class="btn btn-primary btn-lg w-full" :disabled="isLoading">
          <span v-if="isLoading">登录中...</span>
          <span v-else>登录</span>
        </button>
      </form>

      <!-- 页脚 -->
      <div class="login-footer">
        <p class="footer-text">基于 <strong>Rust + Vue.js</strong> 构建的现代化博客系统</p>
      </div>
    </div>

    <!-- 背景装饰 -->
    <div class="bg-decoration">
      <div class="bg-circle bg-circle-1"></div>
      <div class="bg-circle bg-circle-2"></div>
      <div class="bg-circle bg-circle-3"></div>
    </div>
  </div>
</template>

<style scoped>
.login-container {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  padding: var(--space-4);
  position: relative;
  overflow: hidden;
}

.login-card {
  width: 100%;
  max-width: 420px;
  background: var(--color-white);
  border-radius: var(--radius-2xl);
  box-shadow: var(--shadow-xl);
  overflow: hidden;
  position: relative;
  z-index: 10;
}

.login-header {
  text-align: center;
  padding: var(--space-8) var(--space-6) var(--space-6);
  background: linear-gradient(135deg, #f8fafc 0%, #e2e8f0 100%);
}

.logo {
  font-size: var(--text-3xl);
  font-weight: 700;
  margin: 0 0 var(--space-2);
  background: linear-gradient(45deg, var(--color-primary), #8b5cf6);
  background-clip: text;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.subtitle {
  color: var(--color-gray-600);
  font-size: var(--text-base);
  margin: 0;
  font-weight: 500;
}

.remembered-credentials-notice {
  margin: 0 var(--space-6) 0;
  padding: var(--space-3);
  background: linear-gradient(135deg, #e0f2fe 0%, #b3e5fc 100%);
  border: 1px solid #81d4fa;
  border-radius: var(--radius-md);
  animation: slideIn 0.3s ease-out;
}

.notice-content {
  display: flex;
  align-items: center;
  gap: var(--space-2);
}

.notice-icon {
  font-size: var(--text-base);
}

.notice-text {
  font-size: var(--text-sm);
  color: #01579b;
  font-weight: 500;
  flex: 1;
}

.notice-clear-btn {
  background: none;
  border: none;
  color: #0277bd;
  font-size: var(--text-sm);
  cursor: pointer;
  padding: var(--space-1);
  border-radius: var(--radius-base);
  transition: all var(--transition-fast);
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
}

.notice-clear-btn:hover {
  background-color: rgba(2, 119, 189, 0.1);
  color: #01579b;
}

@keyframes slideIn {
  from {
    opacity: 0;
    transform: translateY(-10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.login-form {
  padding: var(--space-6);
}

.form-group {
  margin-bottom: var(--space-5);
}

.form-label {
  font-weight: 600;
  color: var(--color-gray-700);
}

.form-input {
  font-size: var(--text-base);
  padding: var(--space-4);
  border-width: 2px;
  transition: all var(--transition-base);
}

.form-input:focus {
  border-color: var(--color-primary);
  box-shadow: 0 0 0 4px rgba(59, 130, 246, 0.1);
  transform: translateY(-1px);
}

.form-input:disabled {
  background-color: var(--color-gray-50);
  opacity: 0.7;
  cursor: not-allowed;
}

.error-message {
  background-color: #fef2f2;
  border: 1px solid #fecaca;
  color: var(--color-error);
  padding: var(--space-3);
  border-radius: var(--radius-md);
  font-size: var(--text-sm);
  margin-bottom: var(--space-4);
  display: flex;
  align-items: center;
  gap: var(--space-2);
}

.error-message::before {
  content: '⚠️';
  font-size: var(--text-base);
}

.btn-lg {
  font-weight: 600;
  position: relative;
  overflow: hidden;
}

.btn-lg::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.2), transparent);
  transition: left 0.5s;
}

.btn-lg:hover::before {
  left: 100%;
}

.btn-lg:disabled {
  transform: none;
}

.login-footer {
  padding: var(--space-4) var(--space-6);
  background-color: var(--color-gray-50);
  border-top: 1px solid var(--color-gray-200);
  text-align: center;
}

.footer-text {
  color: var(--color-gray-600);
  font-size: var(--text-sm);
  margin: 0;
}

/* 背景装饰 */
.bg-decoration {
  position: absolute;
  inset: 0;
  pointer-events: none;
}

.bg-circle {
  position: absolute;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.1);
  animation: float 6s ease-in-out infinite;
}

.bg-circle-1 {
  width: 200px;
  height: 200px;
  top: 10%;
  left: 10%;
  animation-delay: 0s;
}

.bg-circle-2 {
  width: 150px;
  height: 150px;
  top: 60%;
  right: 15%;
  animation-delay: 2s;
}

.bg-circle-3 {
  width: 100px;
  height: 100px;
  bottom: 20%;
  left: 20%;
  animation-delay: 4s;
}

@keyframes float {
  0%,
  100% {
    transform: translateY(0px) rotate(0deg);
    opacity: 0.7;
  }
  50% {
    transform: translateY(-20px) rotate(180deg);
    opacity: 0.3;
  }
}

/* 响应式设计 */
@media (max-width: 480px) {
  .login-container {
    padding: var(--space-2);
  }

  .login-card {
    max-width: 100%;
  }

  .login-header {
    padding: var(--space-6) var(--space-4) var(--space-4);
  }

  .login-form {
    padding: var(--space-4);
  }

  .logo {
    font-size: var(--text-2xl);
  }

  .bg-circle {
    display: none;
  }
}

/* 暗色模式支持 */
@media (prefers-color-scheme: dark) {
  .login-card {
    background: var(--color-gray-800);
    color: var(--color-white);
  }

  .login-header {
    background: linear-gradient(135deg, var(--color-gray-700) 0%, var(--color-gray-600) 100%);
  }

  .subtitle {
    color: var(--color-gray-300);
  }

  .form-label {
    color: var(--color-gray-200);
  }

  .form-input {
    background-color: var(--color-gray-700);
    border-color: var(--color-gray-600);
    color: var(--color-white);
  }

  .form-input::placeholder {
    color: var(--color-gray-400);
  }

  .login-footer {
    background-color: var(--color-gray-700);
    border-color: var(--color-gray-600);
  }

  .footer-text {
    color: var(--color-gray-300);
  }

  .remembered-credentials-notice {
    background: linear-gradient(135deg, #1e3a8a 0%, #1e40af 100%);
    border-color: #3b82f6;
  }

  .notice-text {
    color: #dbeafe;
  }

  .notice-clear-btn {
    color: #93c5fd;
  }

  .notice-clear-btn:hover {
    background-color: rgba(147, 197, 253, 0.1);
    color: #dbeafe;
  }
}
</style>
