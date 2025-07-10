<script setup lang="ts">
import { useRouter } from 'vue-router'

const router = useRouter()

const goBack = () => {
  router.back()
}

const goHome = () => {
  router.push('/dashboard')
}
</script>

<template>
  <div class="not-found-container">
    <div class="not-found-content">
      <div class="error-illustration">
        <div class="error-code">404</div>
        <div class="error-icon">🔍</div>
      </div>

      <div class="error-message">
        <h1>页面未找到</h1>
        <p>抱歉，您访问的页面不存在或已被移动。</p>
      </div>

      <div class="error-actions">
        <button @click="goBack" class="btn btn-secondary">
          <span>←</span>
          <span>返回上页</span>
        </button>
        <button @click="goHome" class="btn btn-primary">
          <span>🏠</span>
          <span>回到首页</span>
        </button>
      </div>
    </div>

    <!-- 背景装饰 -->
    <div class="bg-decoration">
      <div class="floating-element floating-element-1">📄</div>
      <div class="floating-element floating-element-2">📁</div>
      <div class="floating-element floating-element-3">🏷️</div>
      <div class="floating-element floating-element-4">✨</div>
    </div>
  </div>
</template>

<style scoped>
.not-found-container {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, #f8fafc 0%, #e2e8f0 100%);
  padding: var(--space-4);
  position: relative;
  overflow: hidden;
}

.not-found-content {
  text-align: center;
  max-width: 500px;
  position: relative;
  z-index: 10;
}

.error-illustration {
  position: relative;
  margin-bottom: var(--space-8);
}

.error-code {
  font-size: 8rem;
  font-weight: 900;
  color: var(--color-primary);
  line-height: 1;
  margin-bottom: var(--space-4);
  background: linear-gradient(45deg, var(--color-primary), #8b5cf6);
  background-clip: text;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  filter: drop-shadow(2px 2px 4px rgba(59, 130, 246, 0.2));
}

.error-icon {
  font-size: 4rem;
  opacity: 0.8;
  animation: float 3s ease-in-out infinite;
}

.error-message h1 {
  font-size: var(--text-3xl);
  font-weight: 700;
  color: var(--color-gray-900);
  margin: 0 0 var(--space-4);
}

.error-message p {
  font-size: var(--text-lg);
  color: var(--color-gray-600);
  margin: 0 0 var(--space-8);
  line-height: 1.6;
}

.error-actions {
  display: flex;
  gap: var(--space-4);
  justify-content: center;
}

.error-actions .btn {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-3) var(--space-6);
  font-size: var(--text-base);
  font-weight: 600;
  border-radius: var(--radius-lg);
  transition: all var(--transition-base);
  box-shadow: var(--shadow-base);
}

.error-actions .btn:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-lg);
}

/* 背景装饰 */
.bg-decoration {
  position: absolute;
  inset: 0;
  pointer-events: none;
}

.floating-element {
  position: absolute;
  font-size: 2rem;
  opacity: 0.1;
  animation: float-random 8s ease-in-out infinite;
}

.floating-element-1 {
  top: 20%;
  left: 15%;
  animation-delay: 0s;
}

.floating-element-2 {
  top: 70%;
  right: 20%;
  animation-delay: 2s;
}

.floating-element-3 {
  bottom: 30%;
  left: 20%;
  animation-delay: 4s;
}

.floating-element-4 {
  top: 40%;
  right: 15%;
  animation-delay: 6s;
}

@keyframes float {
  0%,
  100% {
    transform: translateY(0px);
  }
  50% {
    transform: translateY(-10px);
  }
}

@keyframes float-random {
  0%,
  100% {
    transform: translateY(0px) rotate(0deg);
    opacity: 0.1;
  }
  25% {
    transform: translateY(-20px) rotate(90deg);
    opacity: 0.3;
  }
  50% {
    transform: translateY(-10px) rotate(180deg);
    opacity: 0.2;
  }
  75% {
    transform: translateY(-30px) rotate(270deg);
    opacity: 0.25;
  }
}

/* 响应式设计 */
@media (max-width: 768px) {
  .error-code {
    font-size: 6rem;
  }

  .error-icon {
    font-size: 3rem;
  }

  .error-message h1 {
    font-size: var(--text-2xl);
  }

  .error-message p {
    font-size: var(--text-base);
  }

  .error-actions {
    flex-direction: column;
  }

  .error-actions .btn {
    width: 100%;
    justify-content: center;
  }

  .floating-element {
    display: none;
  }
}

@media (max-width: 480px) {
  .not-found-container {
    padding: var(--space-2);
  }

  .error-code {
    font-size: 4rem;
  }

  .error-message h1 {
    font-size: var(--text-xl);
  }

  .error-actions .btn {
    padding: var(--space-3) var(--space-4);
    font-size: var(--text-sm);
  }
}

/* 暗色模式支持 */
@media (prefers-color-scheme: dark) {
  .not-found-container {
    background: linear-gradient(135deg, var(--color-gray-900) 0%, var(--color-gray-800) 100%);
  }

  .error-message h1 {
    color: var(--color-white);
  }

  .error-message p {
    color: var(--color-gray-300);
  }

  .floating-element {
    opacity: 0.05;
  }
}
</style>
