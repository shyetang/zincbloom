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
              创建新账户
            </h1>
            <p class="modern-auth-subtitle">
              已有账户？
              <NuxtLink
                to="/auth/login"
                class="modern-auth-link"
              >
                立即登录
              </NuxtLink>
            </p>
          </div>
        </div>

        <!-- 现代化注册表单 -->
        <div class="modern-auth-form-container">
          <UForm
            ref="formRef"
            :schema="registerSchema"
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
                  placeholder="请输入用户名（3-20个字符）"
                  :disabled="pending"
                  class="modern-enhanced-input"
                  :ui="{
                    base: 'w-full h-auto min-h-0 border-0 bg-transparent placeholder:text-gray-400 focus:ring-0 focus:outline-none shadow-none p-0 m-0',
                  }"
                  @input="checkUsernameAvailability"
                />
              </div>
              <p class="modern-form-help">
                用户名将作为您的唯一标识符
              </p>
              <div
                v-if="usernameStatus"
                class="modern-validation-feedback"
                :class="usernameStatusClass"
              >
                <UIcon
                  :name="usernameStatusIcon"
                  class="w-4 h-4"
                />
                {{ usernameStatus }}
              </div>
            </div>

            <!-- 现代化邮箱字段 -->
            <div class="modern-form-group">
              <label class="modern-form-label">
                <UIcon
                  name="i-heroicons-envelope"
                  class="modern-form-icon"
                />
                邮箱地址
              </label>
              <div class="modern-input-wrapper">
                <UInput
                  v-model="formData.email"
                  type="email"
                  placeholder="请输入您的邮箱地址"
                  :disabled="pending"
                  class="modern-enhanced-input"
                  :ui="{
                    base: 'w-full h-auto min-h-0 border-0 bg-transparent placeholder:text-gray-400 focus:ring-0 focus:outline-none shadow-none p-0 m-0',
                  }"
                />
              </div>
              <p class="modern-form-help">
                邮箱将用于验证和重要通知
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
                  :type="showPassword ? 'text' : 'password'"
                  placeholder="请输入密码（至少8个字符）"
                  :disabled="pending"
                  class="modern-enhanced-input"
                  :ui="{
                    base: 'w-full h-auto min-h-0 border-0 bg-transparent placeholder:text-gray-400 focus:ring-0 focus:outline-none shadow-none p-0 m-0',
                  }"
                  @input="updatePasswordStrength"
                />
                <button
                  type="button"
                  class="modern-password-toggle"
                  @click="showPassword = !showPassword"
                >
                  <UIcon
                    :name="showPassword ? 'i-heroicons-eye-slash' : 'i-heroicons-eye'"
                    class="w-5 h-5 text-gray-400 hover:text-gray-600"
                  />
                </button>
              </div>
              <!-- 密码强度指示器 -->
              <div
                v-if="formData.password"
                class="modern-password-strength"
              >
                <div class="modern-strength-bar">
                  <div
                    class="modern-strength-fill"
                    :class="passwordStrengthClass"
                    :style="{ width: passwordStrengthPercentage + '%' }"
                  />
                </div>
                <div
                  class="modern-strength-text"
                  :class="passwordStrengthTextClass"
                >
                  {{ passwordStrengthText }}
                </div>
              </div>
              <div class="modern-password-requirements">
                <div
                  class="modern-requirement"
                  :class="{ met: passwordRequirements.length }"
                >
                  <UIcon
                    name="i-heroicons-check"
                    class="w-3 h-3"
                  />
                  至少8个字符
                </div>
                <div
                  class="modern-requirement"
                  :class="{ met: passwordRequirements.uppercase }"
                >
                  <UIcon
                    name="i-heroicons-check"
                    class="w-3 h-3"
                  />
                  包含大写字母
                </div>
                <div
                  class="modern-requirement"
                  :class="{ met: passwordRequirements.lowercase }"
                >
                  <UIcon
                    name="i-heroicons-check"
                    class="w-3 h-3"
                  />
                  包含小写字母
                </div>
                <div
                  class="modern-requirement"
                  :class="{ met: passwordRequirements.number }"
                >
                  <UIcon
                    name="i-heroicons-check"
                    class="w-3 h-3"
                  />
                  包含数字
                </div>
              </div>
            </div>

            <!-- 现代化确认密码字段 -->
            <div class="modern-form-group">
              <label class="modern-form-label">
                <UIcon
                  name="i-heroicons-lock-closed"
                  class="modern-form-icon"
                />
                确认密码
              </label>
              <div class="modern-input-wrapper">
                <UInput
                  v-model="formData.confirmPassword"
                  :type="showConfirmPassword ? 'text' : 'password'"
                  placeholder="请再次输入密码"
                  :disabled="pending"
                  class="modern-enhanced-input"
                  :ui="{
                    base: 'w-full h-auto min-h-0 border-0 bg-transparent placeholder:text-gray-400 focus:ring-0 focus:outline-none shadow-none p-0 m-0',
                  }"
                />
                <button
                  type="button"
                  class="modern-password-toggle"
                  @click="showConfirmPassword = !showConfirmPassword"
                >
                  <UIcon
                    :name="showConfirmPassword ? 'i-heroicons-eye-slash' : 'i-heroicons-eye'"
                    class="w-5 h-5 text-gray-400 hover:text-gray-600"
                  />
                </button>
              </div>
              <div
                v-if="formData.confirmPassword && formData.password !== formData.confirmPassword"
                class="modern-validation-feedback error"
              >
                <UIcon
                  name="i-heroicons-x-circle"
                  class="w-4 h-4"
                />
                两次输入的密码不一致
              </div>
              <div
                v-if="formData.confirmPassword && formData.password === formData.confirmPassword"
                class="modern-validation-feedback success"
              >
                <UIcon
                  name="i-heroicons-check-circle"
                  class="w-4 h-4"
                />
                密码匹配
              </div>
            </div>

            <!-- 现代化服务条款选项 -->
            <div class="modern-form-options">
              <div class="modern-terms-wrapper">
                <input
                  id="acceptTerms"
                  v-model="formData.acceptTerms"
                  type="checkbox"
                  class="modern-checkbox-input"
                  :disabled="pending"
                >
                <label
                  for="acceptTerms"
                  class="modern-checkbox-label"
                >
                  <div class="modern-checkbox-box">
                    <UIcon
                      name="i-heroicons-check"
                      class="modern-checkbox-icon"
                    />
                  </div>
                  <span class="modern-terms-text">
                    我已阅读并同意
                    <NuxtLink
                      to="/terms"
                      class="modern-auth-link"
                      target="_blank"
                    >
                      服务条款
                    </NuxtLink>
                    和
                    <NuxtLink
                      to="/privacy"
                      class="modern-auth-link"
                      target="_blank"
                    >
                      隐私政策
                    </NuxtLink>
                  </span>
                </label>
              </div>
            </div>

            <!-- 现代化注册按钮 -->
            <button
              type="submit"
              :disabled="pending || !formData.acceptTerms || !isFormValid"
              class="modern-register-button"
            >
              <div class="modern-button-content">
                <div
                  v-if="pending"
                  class="modern-loading-spinner"
                />
                <UIcon
                  v-else
                  name="i-heroicons-user-plus"
                  class="modern-button-icon"
                />
                <span class="modern-button-text">
                  {{ pending ? "注册中..." : "创建账户" }}
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
  title: "用户注册",
  meta: [
    {
      name: "description",
      content: "注册 ZincBloom 账户，开始您的博客之旅",
    },
  ],
});

// 表单验证架构
const registerSchema = z
  .object({
    username: z
      .string()
      .min(3, "用户名至少3个字符")
      .max(20, "用户名不能超过20个字符"),
    email: z.string().email("请输入有效的邮箱地址"),
    password: z
      .string()
      .min(8, "密码至少8个字符")
      .regex(
        /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)/,
        "密码必须包含大小写字母和数字",
      ),
    confirmPassword: z.string(),
    acceptTerms: z
      .boolean()
      .refine(val => val === true, "请同意服务条款和隐私政策"),
  })
  .refine(data => data.password === data.confirmPassword, {
    message: "两次输入的密码不一致",
    path: ["confirmPassword"],
  });

// 表单数据
const formData = reactive({
  username: "",
  email: "",
  password: "",
  confirmPassword: "",
  acceptTerms: false,
});

// 状态管理
const pending = ref(false);
const authStore = useAuthStore();
const router = useRouter();
const toast = useToast();
const formRef = ref();

// 密码显示状态
const showPassword = ref(false);
const showConfirmPassword = ref(false);

// 用户名验证状态
const usernameStatus = ref("");
const usernameStatusClass = computed(() => {
  if (usernameStatus.value.includes("可用")) return "success";
  if (usernameStatus.value.includes("不可用") || usernameStatus.value.includes("无效")) return "error";
  return "";
});
const usernameStatusIcon = computed(() => {
  if (usernameStatus.value.includes("可用")) return "i-heroicons-check-circle";
  if (usernameStatus.value.includes("不可用") || usernameStatus.value.includes("无效")) return "i-heroicons-x-circle";
  return "i-heroicons-clock";
});

// 密码强度检测
const passwordRequirements = computed(() => ({
  length: formData.password.length >= 8,
  uppercase: /[A-Z]/.test(formData.password),
  lowercase: /[a-z]/.test(formData.password),
  number: /\d/.test(formData.password),
}));

const passwordStrength = computed(() => {
  const requirements = passwordRequirements.value;
  const metCount = Object.values(requirements).filter(Boolean).length;

  if (metCount === 0) return 0;
  if (metCount <= 2) return 1;
  if (metCount === 3) return 2;
  return 3;
});

const passwordStrengthPercentage = computed(() => {
  return Math.max(25, (passwordStrength.value + 1) * 25);
});

const passwordStrengthClass = computed(() => {
  const strength = passwordStrength.value;
  if (strength === 0) return "weak";
  if (strength === 1) return "weak";
  if (strength === 2) return "medium";
  return "strong";
});

const passwordStrengthText = computed(() => {
  const strength = passwordStrength.value;
  if (strength === 0) return "密码强度：很弱";
  if (strength === 1) return "密码强度：弱";
  if (strength === 2) return "密码强度：中等";
  return "密码强度：强";
});

const passwordStrengthTextClass = computed(() => {
  return `strength-${passwordStrengthClass.value}`;
});

// 表单验证状态
const isFormValid = computed(() => {
  return (
    formData.username.length >= 3
    && formData.email.includes("@")
    && passwordStrength.value >= 2
    && formData.password === formData.confirmPassword
    && formData.acceptTerms
  );
});

// 用户名可用性检查
const checkUsernameAvailability = async () => {
  const username = formData.username;

  if (!username || username.length < 3) {
    usernameStatus.value = "";
    return;
  }

  if (username.length > 20) {
    usernameStatus.value = "用户名长度不能超过20个字符";
    return;
  }

  // 简单的用户名格式验证
  if (!/^[a-zA-Z0-9_]+$/.test(username)) {
    usernameStatus.value = "用户名只能包含字母、数字和下划线";
    return;
  }

  usernameStatus.value = "检查用户名可用性...";

  // 这里应该调用API检查用户名是否可用
  // 暂时模拟检查结果
  setTimeout(() => {
    // 模拟一些常见用户名不可用
    const unavailableUsernames = ["admin", "root", "user", "test", "demo"];
    if (unavailableUsernames.includes(username.toLowerCase())) {
      usernameStatus.value = "用户名不可用";
    }
    else {
      usernameStatus.value = "用户名可用";
    }
  }, 500);
};

// 更新密码强度
const updatePasswordStrength = () => {
  // 密码强度会通过computed属性自动更新
};

// 处理表单提交
const handleSubmit = async (_event: FormSubmitEvent<unknown>) => {
  if (pending.value) return;

  pending.value = true;

  try {
    const result = await authStore.register({
      username: formData.username,
      email: formData.email,
      password: formData.password,
    });

    if (result.success) {
      toast.add({
        title: "注册成功",
        description: "请查看您的邮箱并点击验证链接",
        icon: "i-heroicons-check-circle",
        color: "success",
      });

      await router.push("/auth/verify");
    }
    else {
      toast.add({
        title: "注册失败",
        description: result.message || "注册过程中发生错误",
        icon: "i-heroicons-x-circle",
        color: "error",
      });
    }
  }
  catch (error: unknown) {
    console.error("注册错误:", error);
    toast.add({
      title: "注册失败",
      description: "网络错误，请稍后重试",
      icon: "i-heroicons-x-circle",
      color: "error",
    });
  }
  finally {
    pending.value = false;
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
    max-width: 30rem;
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
    padding: 2.5rem;
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

/* 认证头部样式 */
.modern-auth-header {
    text-align: center;
    margin-bottom: 2rem;
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
    font-size: 1.875rem;
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

/* 表单容器 */
.modern-auth-form-container {
    margin-bottom: 1rem;
}

.modern-auth-form {
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
}

/* 表单组 */
.modern-form-group {
    position: relative;
    margin-bottom: 0.5rem;
}

.modern-form-group.password-field {
    margin-bottom: 1rem;
}

/* 表单标签 */
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

/* 输入框容器 */
.modern-input-wrapper {
    position: relative;
    background: rgba(248, 250, 252, 0.8);
    border: 2px solid rgba(226, 232, 240, 0.6);
    border-radius: 1rem;
    padding: 0.875rem 1rem;
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

/* 增强输入框 */
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

/* 密码显示切换按钮 */
.modern-password-toggle {
    position: absolute;
    right: 1rem;
    top: 50%;
    transform: translateY(-50%);
    background: transparent;
    border: none;
    cursor: pointer;
    padding: 0.25rem;
    border-radius: 0.375rem;
    transition: all 0.2s ease;
}

.modern-password-toggle:hover {
    background: rgba(107, 114, 128, 0.1);
}

/* 表单帮助文本 */
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

/* 验证反馈 */
.modern-validation-feedback {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.75rem;
    margin-top: 0.5rem;
    padding: 0.5rem;
    border-radius: 0.5rem;
    transition: all 0.3s ease;
}

.modern-validation-feedback.success {
    color: #059669;
    background: rgba(16, 185, 129, 0.1);
    border: 1px solid rgba(16, 185, 129, 0.2);
}

.modern-validation-feedback.error {
    color: #dc2626;
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.2);
}

/* 密码强度指示器 */
.modern-password-strength {
    margin-top: 0.5rem;
}

.modern-strength-bar {
    height: 4px;
    background: rgba(156, 163, 175, 0.3);
    border-radius: 2px;
    overflow: hidden;
    margin-bottom: 0.5rem;
}

.modern-strength-fill {
    height: 100%;
    transition: all 0.3s ease;
    border-radius: 2px;
}

.modern-strength-fill.weak {
    background: linear-gradient(90deg, #ef4444, #f87171);
}

.modern-strength-fill.medium {
    background: linear-gradient(90deg, #f59e0b, #fbbf24);
}

.modern-strength-fill.strong {
    background: linear-gradient(90deg, #10b981, #34d399);
}

.modern-strength-text {
    font-size: 0.75rem;
    font-weight: 500;
    margin-bottom: 0.5rem;
}

.modern-strength-text.strength-weak {
    color: #ef4444;
}

.modern-strength-text.strength-medium {
    color: #f59e0b;
}

.modern-strength-text.strength-strong {
    color: #10b981;
}

/* 密码要求列表 */
.modern-password-requirements {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.25rem;
    margin-top: 0.5rem;
}

.modern-requirement {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    font-size: 0.75rem;
    color: #6b7280;
    transition: all 0.3s ease;
}

.modern-requirement.met {
    color: #10b981;
}

.modern-requirement svg {
    opacity: 0;
    transform: scale(0.8);
    transition: all 0.2s ease;
}

.modern-requirement.met svg {
    opacity: 1;
    transform: scale(1);
}

.dark .modern-requirement {
    color: #9ca3af;
}

.dark .modern-requirement.met {
    color: #34d399;
}

/* 表单选项 */
.modern-form-options {
    margin-bottom: 2rem;
}

/* 服务条款选项 */
.modern-terms-wrapper {
    display: flex;
    align-items: flex-start;
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
    align-items: flex-start;
    gap: 0.75rem;
    font-size: 0.875rem;
    font-weight: 500;
    color: #374151;
    cursor: pointer;
    transition: all 0.2s ease;
    line-height: 1.5;
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
    flex-shrink: 0;
    margin-top: 0.125rem;
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

.modern-terms-wrapper:hover .modern-checkbox-box {
    border-color: rgba(59, 130, 246, 0.4);
    transform: translateY(-1px);
    box-shadow: 0 2px 8px rgba(59, 130, 246, 0.1);
}

.modern-terms-text {
    line-height: 1.6;
}

/* 注册按钮 */
.modern-register-button {
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

.modern-register-button:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 12px 35px rgba(59, 130, 246, 0.4),
        0 0 0 1px rgba(255, 255, 255, 0.2) inset;
}

.modern-register-button:active:not(:disabled) {
    transform: translateY(0);
    box-shadow: 0 4px 15px rgba(59, 130, 246, 0.3),
        0 0 0 1px rgba(255, 255, 255, 0.1) inset;
}

.modern-register-button:disabled {
    opacity: 0.7;
    cursor: not-allowed;
    transform: none;
    box-shadow: 0 4px 12px rgba(59, 130, 246, 0.2),
        0 0 0 1px rgba(255, 255, 255, 0.1) inset;
}

/* 按钮内容 */
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

.modern-register-button:hover:not(:disabled) .modern-button-icon {
    transform: translateX(4px);
}

.modern-button-text {
    font-weight: 600;
    letter-spacing: 0.025em;
}

/* 加载旋转器 */
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

/* 按钮光泽效果 */
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

.modern-register-button:hover:not(:disabled) .modern-button-shimmer {
    left: 100%;
}

/* 认证底部 */
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
        padding: 2rem;
        margin: 1rem;
    }

    .modern-auth-logo {
        font-size: 1.5rem;
    }

    .modern-auth-title {
        font-size: 1.5rem;
    }

    .modern-input-wrapper {
        padding: 0.75rem 1rem;
    }

    .modern-register-button {
        padding: 0.875rem 1.5rem;
        font-size: 0.875rem;
    }

    .modern-form-group {
        margin-bottom: 0.75rem;
    }

    .modern-password-requirements {
        grid-template-columns: 1fr;
    }

    .modern-terms-wrapper {
        align-items: flex-start;
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
</style>
