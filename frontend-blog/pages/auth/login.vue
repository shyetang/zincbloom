<template>
  <div class="modern-auth-container">
    <!-- 现代化背景 -->
    <div class="modern-auth-background" />

    <!-- 主要内容 -->
    <div class="modern-auth-content">
      <div class="modern-auth-card">
        <!-- 现代化品牌区域 -->
        <div class="modern-auth-header">
          <NuxtLink
            to="/"
            class="modern-auth-logo"
          >
            <div class="modern-logo-icon">
              <UIcon
                name="i-heroicons-sparkles"
                style="
                                    width: 1.5rem;
                                    height: 1.5rem;
                                    color: white;
                                "
              />
            </div>
            <span class="modern-text-gradient">ZincBloom</span>
          </NuxtLink>

          <div class="modern-auth-title-section">
            <h1 class="modern-auth-title">
              登录您的账户
            </h1>
            <p class="modern-auth-subtitle">
              还没有账户？
              <NuxtLink
                to="/auth/register"
                class="modern-auth-link"
              >
                立即注册
              </NuxtLink>
            </p>
          </div>
        </div>

        <!-- 现代化登录表单 -->
        <div class="modern-auth-form-container">
          <UForm
            ref="formRef"
            :schema="loginSchema"
            :state="formData"
            class="modern-auth-form"
            @submit="handleSubmit"
          >
            <!-- 现代化用户名字段 -->
            <div class="modern-form-group">
              <label class="modern-form-label">
                <UIcon
                  name="i-heroicons-user"
                  class="modern-form-icon"
                />
                用户名
              </label>
              <div class="modern-input-wrapper">
                <UInput
                  v-model="formData.username"
                  placeholder="请输入用户名或邮箱"
                  :disabled="loading"
                  class="modern-enhanced-input"
                  :ui="{
                    base: 'w-full h-auto min-h-0 border-0 bg-transparent placeholder:text-gray-400 focus:ring-0 focus:outline-none shadow-none p-0 m-0',
                  }"
                />
              </div>
              <p class="modern-form-help">
                请输入您的用户名或邮箱地址
              </p>
            </div>

            <!-- 现代化密码字段 -->
            <div class="modern-form-group password-field">
              <label class="modern-form-label">
                <UIcon
                  name="i-heroicons-lock-closed"
                  class="modern-form-icon"
                />
                密码
              </label>
              <div class="modern-input-wrapper">
                <UInput
                  v-model="formData.password"
                  type="password"
                  placeholder="请输入您的密码"
                  :disabled="loading"
                  class="modern-enhanced-input"
                  :ui="{
                    base: 'w-full h-auto min-h-0 border-0 bg-transparent placeholder:text-gray-400 focus:ring-0 focus:outline-none shadow-none p-0 m-0',
                  }"
                />
              </div>
            </div>

            <!-- 现代化选项区域 -->
            <div class="modern-form-options">
              <div class="modern-remember-wrapper">
                <input
                  id="remember"
                  v-model="formData.remember"
                  type="checkbox"
                  class="modern-checkbox-input"
                  :disabled="loading"
                >
                <label
                  for="remember"
                  class="modern-checkbox-label"
                >
                  <div class="modern-checkbox-box">
                    <UIcon
                      name="i-heroicons-check"
                      class="modern-checkbox-icon"
                    />
                  </div>
                  记住我
                </label>
              </div>
              <NuxtLink
                to="/auth/forgot-password"
                class="modern-forgot-link"
              >
                忘记密码？
              </NuxtLink>
            </div>

            <!-- 现代化登录按钮 -->
            <button
              type="submit"
              :disabled="loading"
              class="modern-login-button"
            >
              <div class="modern-button-content">
                <div
                  v-if="loading"
                  class="modern-loading-spinner"
                />
                <UIcon
                  v-else
                  name="i-heroicons-arrow-right"
                  class="modern-button-icon"
                />
                <span class="modern-button-text">
                  {{ loading ? "登录中..." : "登录" }}
                </span>
              </div>
              <div class="modern-button-shimmer" />
            </button>
          </UForm>
        </div>

        <!-- 装饰性底部 -->
        <div class="modern-auth-footer">
          <div class="modern-auth-decoration" />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { z } from "zod";
import type { FormSubmitEvent } from "~/types/ui";

// 禁用默认布局
definePageMeta({
  layout: false,
});

// SEO 配置
useHead({
  title: "登录",
  meta: [{ name: "description", content: "登录您的 ZincBloom 账户" }],
});

// 状态管理
const authStore = useAuthStore();
const toast = useToast();
const router = useRouter();
const route = useRoute();

// 响应式数据
const loading = ref(false);
const formRef = ref();

// 从localStorage获取记住的登录信息
const getRememberedCredentials = () => {
  if (import.meta.client) {
    const remembered = localStorage.getItem("rememberedCredentials");
    if (remembered) {
      try {
        return JSON.parse(remembered);
      }
      catch {
        return { username: "", password: "", remember: false };
      }
    }
  }
  return { username: "", password: "", remember: false };
};

// 表单数据
const formData = reactive(getRememberedCredentials());

// 表单验证规则
const loginSchema = z.object({
  username: z
    .string()
    .min(1, "请输入用户名或邮箱")
    .max(100, "用户名长度不能超过100个字符"),
  password: z
    .string()
    .min(1, "请输入密码")
    .min(6, "密码长度至少6个字符")
    .max(100, "密码长度不能超过100个字符"),
});

// 处理表单提交
const handleSubmit = async (_event: FormSubmitEvent<unknown>) => {
  if (loading.value) return;

  loading.value = true;

  try {
    const result = await authStore.login({
      username: formData.username,
      password: formData.password,
    });

    if (result.success) {
      // 处理记住凭据
      if (import.meta.client) {
        if (formData.remember) {
          // 如果选择记住我，保存凭据到localStorage
          localStorage.setItem(
            "rememberedCredentials",
            JSON.stringify({
              username: formData.username,
              password: formData.password,
              remember: true,
            }),
          );
        }
        else {
          // 如果没有选择记住我，清除保存的凭据
          localStorage.removeItem("rememberedCredentials");
        }
      }

      toast.add({
        title: "登录成功",
        description: "欢迎回来！",
        icon: "i-heroicons-check-circle",
        color: "success",
      });

      // 跳转到目标页面或首页
      const redirectTo = (route.query.redirect as string) || "/";
      await router.push(redirectTo);
    }
    else {
      toast.add({
        title: "登录失败",
        description: result.message || "用户名或密码错误",
        icon: "i-heroicons-x-circle",
        color: "error",
      });
    }
  }
  catch (error: unknown) {
    console.error("登录错误:", error);
    toast.add({
      title: "登录失败",
      description: "网络错误，请稍后重试",
      icon: "i-heroicons-x-circle",
      color: "error",
    });
  }
  finally {
    loading.value = false;
  }
};

// 如果已经登录，重定向到首页
onMounted(() => {
  if (authStore.isAuthenticated) {
    router.push("/");
  }
});
</script>

<style scoped>
/* 现代化认证容器 */
.modern-auth-container {
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 2rem 1rem;
    position: relative;
    overflow: hidden;
}

/* 现代化背景 */
.modern-auth-background {
    position: absolute;
    inset: 0;
    background: linear-gradient(135deg, #eff6ff 0%, #e0e7ff 50%, #f3e8ff 100%);
    z-index: -2;
}

.dark .modern-auth-background {
    background: linear-gradient(135deg, #1e293b 0%, #1e3a8a 50%, #581c87 100%);
}

.modern-auth-background::before {
    content: "";
    position: absolute;
    inset: 0;
    background-image: radial-gradient(
        circle at 1px 1px,
        rgba(148, 163, 184, 0.15) 1px,
        transparent 0
    );
    background-size: 75px 75px;
    mask-image: linear-gradient(0deg, white, rgba(255, 255, 255, 0.6));
    z-index: -1;
}

.dark .modern-auth-background::before {
    background-image: radial-gradient(
        circle at 1px 1px,
        rgba(51, 65, 85, 0.25) 1px,
        transparent 0
    );
    mask-image: linear-gradient(
        0deg,
        rgba(255, 255, 255, 0.1),
        rgba(255, 255, 255, 0.5)
    );
}

/* 现代化内容区域 */
.modern-auth-content {
    width: 100%;
    max-width: 28rem;
    position: relative;
    z-index: 10;
}

/* 现代化认证卡片 */
.modern-auth-card {
    background: rgba(255, 255, 255, 0.95);
    backdrop-filter: blur(20px);
    border-radius: 1.5rem;
    border: 1px solid rgba(226, 232, 240, 0.6);
    box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25),
        0 0 0 1px rgba(255, 255, 255, 0.05);
    padding: 2rem;
    position: relative;
    overflow: hidden;
    transition: all 0.3s ease;
}

.dark .modern-auth-card {
    background: rgba(30, 41, 59, 0.95);
    border-color: rgba(51, 65, 85, 0.6);
    box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5),
        0 0 0 1px rgba(255, 255, 255, 0.1);
}

.modern-auth-card::before {
    content: "";
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 4px;
    background: linear-gradient(90deg, #3b82f6, #8b5cf6, #6366f1);
    border-radius: 1.5rem 1.5rem 0 0;
}

/* 现代化认证头部 */
.modern-auth-header {
    text-align: center;
    margin-bottom: 1.5rem;
}

.modern-auth-logo {
    display: inline-flex;
    align-items: center;
    gap: 0.75rem;
    font-size: 1.875rem;
    font-weight: 800;
    text-decoration: none;
    margin-bottom: 1rem;
    transition: all 0.3s ease;
}

.modern-auth-logo:hover {
    transform: translateY(-2px);
}

.modern-auth-logo:hover .modern-logo-icon {
    transform: rotate(5deg) scale(1.05);
    box-shadow: 0 8px 16px rgba(59, 130, 246, 0.4);
}

.modern-auth-title-section {
    margin-top: 0.5rem;
}

.modern-auth-title {
    font-size: 1.75rem;
    font-weight: 700;
    color: #1e293b;
    margin-bottom: 0.25rem;
    line-height: 1.2;
}

.dark .modern-auth-title {
    color: white;
}

.modern-auth-subtitle {
    font-size: 0.875rem;
    color: #64748b;
    margin: 0;
}

.dark .modern-auth-subtitle {
    color: #cbd5e1;
}

.modern-auth-link {
    color: #3b82f6;
    text-decoration: none;
    font-weight: 600;
    transition: all 0.2s ease;
    position: relative;
}

.modern-auth-link:hover {
    color: #2563eb;
}

.modern-auth-link::after {
    content: "";
    position: absolute;
    bottom: -2px;
    left: 0;
    width: 0;
    height: 2px;
    background: linear-gradient(90deg, #3b82f6, #8b5cf6);
    transition: width 0.3s ease;
}

.modern-auth-link:hover::after {
    width: 100%;
}

/* 现代化表单容器 */
.modern-auth-form-container {
    margin-bottom: 1rem;
}

.modern-auth-form {
    display: flex;
    flex-direction: column;
    gap: 1rem;
}

/* 现代化表单组 */
.modern-form-group {
    position: relative;
    margin-bottom: 0.75rem;
}

/* 密码字段特殊处理，距离更近 */
.modern-form-group.password-field {
    margin-bottom: 1rem;
}

/* 现代化表单标签 */
.modern-form-label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.875rem;
    font-weight: 600;
    color: #374151;
    margin-bottom: 0.5rem;
    transition: all 0.2s ease;
}

.dark .modern-form-label {
    color: #e5e7eb;
}

.modern-form-icon {
    width: 1rem;
    height: 1rem;
    color: #6b7280;
    transition: all 0.2s ease;
}

.dark .modern-form-icon {
    color: #9ca3af;
}

/* 现代化输入框容器 */
.modern-input-wrapper {
    position: relative;
    background: rgba(248, 250, 252, 0.8);
    border: 2px solid rgba(226, 232, 240, 0.6);
    border-radius: 1rem;
    padding: 0.75rem 1rem;
    transition: all 0.3s ease;
    backdrop-filter: blur(10px);
}

.dark .modern-input-wrapper {
    background: rgba(51, 65, 85, 0.5);
    border-color: rgba(71, 85, 105, 0.6);
}

.modern-input-wrapper:hover {
    border-color: rgba(59, 130, 246, 0.4);
    background: rgba(255, 255, 255, 0.9);
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(59, 130, 246, 0.1);
}

.dark .modern-input-wrapper:hover {
    background: rgba(51, 65, 85, 0.7);
    border-color: rgba(59, 130, 246, 0.6);
}

.modern-input-wrapper:focus-within {
    border-color: #3b82f6;
    background: white;
    transform: translateY(-2px);
    box-shadow: 0 8px 25px rgba(59, 130, 246, 0.15),
        0 0 0 4px rgba(59, 130, 246, 0.1);
}

.dark .modern-input-wrapper:focus-within {
    background: rgba(51, 65, 85, 0.9);
    box-shadow: 0 8px 25px rgba(59, 130, 246, 0.25),
        0 0 0 4px rgba(59, 130, 246, 0.2);
}

.modern-input-wrapper:focus-within .modern-form-icon {
    color: #3b82f6;
    transform: scale(1.1);
}

/* 现代化增强输入框 */
.modern-enhanced-input {
    font-size: 1rem;
    font-weight: 500;
    color: #1f2937;
    line-height: 1.5;
    padding: 0 !important;
    margin: 0 !important;
    border: none !important;
    outline: none !important;
    box-shadow: none !important;
    background: transparent !important;
    width: 100% !important;
    height: auto !important;
    min-height: 2.5rem;
}

.modern-enhanced-input * {
    border: none !important;
    outline: none !important;
    box-shadow: none !important;
    background: transparent !important;
}

.dark .modern-enhanced-input {
    color: #f9fafb;
}

.modern-enhanced-input::placeholder {
    color: #9ca3af;
    font-weight: 400;
}

.dark .modern-enhanced-input::placeholder {
    color: #6b7280;
}

/* 现代化表单帮助文本 */
.modern-form-help {
    font-size: 0.75rem;
    color: #6b7280;
    margin-top: 0.25rem;
    padding-left: 0.5rem;
    opacity: 0;
    transform: translateY(-5px);
    transition: all 0.3s ease;
}

.dark .modern-form-help {
    color: #9ca3af;
}

.modern-form-group:focus-within .modern-form-help {
    opacity: 1;
    transform: translateY(0);
}

/* 现代化表单选项 */
.modern-form-options {
    display: flex;
    align-items: center;
    justify-content: space-between;
    flex-wrap: wrap;
    gap: 1rem;
    margin-bottom: 1.5rem;
}

/* 现代化记住我选项 */
.modern-remember-wrapper {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    cursor: pointer;
    user-select: none;
}

.modern-checkbox-input {
    position: absolute;
    opacity: 0;
    pointer-events: none;
}

.modern-checkbox-label {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    font-size: 0.875rem;
    font-weight: 500;
    color: #374151;
    cursor: pointer;
    transition: all 0.2s ease;
}

.dark .modern-checkbox-label {
    color: #e5e7eb;
}

.modern-checkbox-box {
    position: relative;
    width: 1.25rem;
    height: 1.25rem;
    background: rgba(248, 250, 252, 0.8);
    border: 2px solid rgba(226, 232, 240, 0.6);
    border-radius: 0.375rem;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.3s ease;
    backdrop-filter: blur(10px);
}

.dark .modern-checkbox-box {
    background: rgba(51, 65, 85, 0.5);
    border-color: rgba(71, 85, 105, 0.6);
}

.modern-checkbox-icon {
    width: 0.875rem;
    height: 0.875rem;
    color: white;
    opacity: 0;
    transform: scale(0.5);
    transition: all 0.2s ease;
}

.modern-checkbox-input:checked + .modern-checkbox-label .modern-checkbox-box {
    background: linear-gradient(135deg, #3b82f6, #6366f1);
    border-color: #3b82f6;
    transform: scale(1.05);
}

.modern-checkbox-input:checked + .modern-checkbox-label .modern-checkbox-icon {
    opacity: 1;
    transform: scale(1);
}

.modern-remember-wrapper:hover .modern-checkbox-box {
    border-color: rgba(59, 130, 246, 0.4);
    transform: translateY(-1px);
    box-shadow: 0 2px 8px rgba(59, 130, 246, 0.1);
}

/* 现代化忘记密码链接 */
.modern-forgot-link {
    font-size: 0.875rem;
    font-weight: 500;
    color: #3b82f6;
    text-decoration: none;
    position: relative;
    transition: all 0.2s ease;
}

.modern-forgot-link:hover {
    color: #2563eb;
    transform: translateY(-1px);
}

.modern-forgot-link::after {
    content: "";
    position: absolute;
    bottom: -2px;
    left: 0;
    width: 0;
    height: 2px;
    background: linear-gradient(90deg, #3b82f6, #8b5cf6);
    transition: width 0.3s ease;
}

.modern-forgot-link:hover::after {
    width: 100%;
}

/* 现代化登录按钮 */
.modern-login-button {
    width: 100%;
    position: relative;
    background: linear-gradient(135deg, #3b82f6 0%, #6366f1 50%, #8b5cf6 100%);
    border: none;
    border-radius: 1rem;
    padding: 1rem 2rem;
    font-size: 1rem;
    font-weight: 600;
    color: white;
    cursor: pointer;
    overflow: hidden;
    transition: all 0.3s ease;
    box-shadow: 0 8px 25px rgba(59, 130, 246, 0.3),
        0 0 0 1px rgba(255, 255, 255, 0.1) inset;
}

.modern-login-button:hover {
    transform: translateY(-2px);
    box-shadow: 0 12px 35px rgba(59, 130, 246, 0.4),
        0 0 0 1px rgba(255, 255, 255, 0.2) inset;
}

.modern-login-button:active {
    transform: translateY(0);
    box-shadow: 0 4px 15px rgba(59, 130, 246, 0.3),
        0 0 0 1px rgba(255, 255, 255, 0.1) inset;
}

.modern-login-button:disabled {
    opacity: 0.7;
    cursor: not-allowed;
    transform: none;
    box-shadow: 0 4px 12px rgba(59, 130, 246, 0.2),
        0 0 0 1px rgba(255, 255, 255, 0.1) inset;
}

.modern-login-button:disabled:hover {
    transform: none;
    box-shadow: 0 4px 12px rgba(59, 130, 246, 0.2),
        0 0 0 1px rgba(255, 255, 255, 0.1) inset;
}

/* 现代化按钮内容 */
.modern-button-content {
    position: relative;
    z-index: 2;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.75rem;
}

.modern-button-icon {
    width: 1.125rem;
    height: 1.125rem;
    transition: all 0.3s ease;
}

.modern-login-button:hover .modern-button-icon {
    transform: translateX(4px);
}

.modern-button-text {
    font-weight: 600;
    letter-spacing: 0.025em;
}

/* 现代化加载旋转器 */
.modern-loading-spinner {
    width: 1.125rem;
    height: 1.125rem;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-top: 2px solid white;
    border-radius: 50%;
    animation: spin 1s linear infinite;
}

@keyframes spin {
    to {
        transform: rotate(360deg);
    }
}

/* 现代化按钮光泽效果 */
.modern-button-shimmer {
    position: absolute;
    top: 0;
    left: -100%;
    width: 100%;
    height: 100%;
    background: linear-gradient(
        90deg,
        transparent,
        rgba(255, 255, 255, 0.3),
        transparent
    );
    transition: left 0.8s ease;
    z-index: 1;
}

.modern-login-button:hover .modern-button-shimmer {
    left: 100%;
}

/* 现代化认证底部 */
.modern-auth-footer {
    margin-top: 1.25rem;
    padding-top: 1rem;
    border-top: 1px solid rgba(226, 232, 240, 0.6);
    text-align: center;
}

.dark .modern-auth-footer {
    border-top-color: rgba(51, 65, 85, 0.6);
}

.modern-auth-decoration {
    width: 3rem;
    height: 3px;
    background: linear-gradient(90deg, #3b82f6, #8b5cf6, #6366f1);
    border-radius: 1.5px;
    margin: 0 auto;
    animation: shimmer 2s ease-in-out infinite;
}

@keyframes shimmer {
    0%,
    100% {
        opacity: 1;
        transform: scaleX(1);
    }
    50% {
        opacity: 0.7;
        transform: scaleX(1.1);
    }
}

/* 响应式调整 */
@media (max-width: 640px) {
    .modern-auth-card {
        padding: 1.5rem;
        margin: 1rem;
    }

    .modern-auth-logo {
        font-size: 1.5rem;
    }

    .modern-auth-title {
        font-size: 1.5rem;
    }

    .modern-form-options {
        flex-direction: column;
        align-items: flex-start;
        gap: 1rem;
    }

    .modern-input-wrapper {
        padding: 0.625rem 0.875rem;
    }

    .modern-login-button {
        padding: 0.875rem 1.5rem;
        font-size: 0.875rem;
    }

    .modern-form-group {
        margin-bottom: 0.5rem;
    }

    .modern-form-group.password-field {
        margin-bottom: 0.75rem;
    }
}

/* 动画效果 */
.modern-auth-card {
    animation: fadeInUp 0.6s ease-out;
}

@keyframes fadeInUp {
    from {
        opacity: 0;
        transform: translateY(30px);
    }
    to {
        opacity: 1;
        transform: translateY(0);
    }
}

/* 聚焦状态增强 */
.modern-form-group:focus-within {
    z-index: 10;
}

/* 加载状态 */
.modern-submit-button[disabled] {
    opacity: 0.8;
    transform: none;
    box-shadow: 0 2px 4px rgba(59, 130, 246, 0.2);
}

.modern-submit-button[disabled]:hover {
    transform: none;
    box-shadow: 0 2px 4px rgba(59, 130, 246, 0.2);
}
</style>
