<template>
  <div>
    <!-- Modern Hero Section -->
    <section
      class="modern-hero"
      style="padding: 3rem 0 2rem 0;"
    >
      <div class="relative mx-auto max-w-4xl px-4 sm:px-6 lg:px-8">
        <div style="text-align: center;">
          <!-- Modern Icon -->
          <div style="margin: 0 auto 1.5rem; width: 3rem; height: 3rem; display: flex; align-items: center; justify-content: center; border-radius: 50%; background: linear-gradient(45deg, #3b82f6, #8b5cf6); box-shadow: 0 6px 24px rgba(59, 130, 246, 0.3);">
            <UIcon
              name="i-heroicons-user-circle"
              style="width: 1.75rem; height: 1.75rem; color: white;"
            />
          </div>

          <!-- Modern Title -->
          <h1
            style="font-size: 2.25rem; font-weight: 700; line-height: 1.1; margin-bottom: 0.75rem; color: #1e293b;"
            class="dark:text-white"
          >
            个人资料
          </h1>

          <!-- Subtitle -->
          <p
            style="font-size: 1rem; color: #64748b; max-width: 28rem; margin: 0 auto; line-height: 1.5;"
            class="dark:text-slate-300"
          >
            管理您的账户信息和偏好设置，让体验更加个性化
          </p>
        </div>
      </div>
    </section>

    <!-- Main Content Section -->
    <section
      class="modern-content-section"
      style="background: linear-gradient(135deg, #f8fafc 0%, #e2e8f0 50%, #eff6ff 100%); padding: 2rem 0; margin: 0;"
    >
      <div class="mx-auto max-w-4xl px-4 sm:px-6 lg:px-8">
        <div class="grid lg:grid-cols-3 gap-8">
          <!-- 侧边栏导航 -->
          <div class="lg:col-span-1">
            <div
              class="modern-sidebar-card"
            >
              <div class="modern-sidebar-header">
                <div
                  class="modern-gradient-icon"
                  style="width: 4rem; height: 4rem; border-radius: 50%; background: linear-gradient(45deg, #3b82f6, #8b5cf6);"
                >
                  <UIcon
                    name="i-heroicons-user"
                    class="w-6 h-6 text-white"
                  />
                </div>
                <div>
                  <h3
                    class="modern-sidebar-title"
                    style="font-size: 1.125rem; margin-bottom: 0.25rem;"
                  >
                    {{ user?.username }}
                  </h3>
                  <p
                    style="font-size: 0.875rem; color: #64748b;"
                    class="dark:text-slate-400"
                  >
                    {{ user?.email }}
                  </p>
                </div>
              </div>

              <nav class="space-y-1">
                <button
                  v-for="tab in tabs"
                  :key="tab.id"
                  :class="[
                    'w-full text-left px-4 py-3 rounded-lg text-sm font-medium transition-all duration-200 flex items-center gap-3',
                    activeTab === tab.id
                      ? 'bg-gradient-to-r from-blue-50 to-purple-50 text-blue-600 border border-blue-200 shadow-sm dark:from-blue-900/20 dark:to-purple-900/20 dark:text-blue-400 dark:border-blue-800'
                      : 'text-slate-600 hover:bg-slate-50 hover:text-slate-900 dark:text-slate-400 dark:hover:bg-slate-800 dark:hover:text-white',
                  ]"
                  @click="activeTab = tab.id"
                >
                  <div
                    :class="[
                      'flex items-center justify-center w-8 h-8 rounded-lg transition-all',
                      activeTab === tab.id
                        ? 'bg-gradient-to-br from-blue-500 to-purple-600 text-white shadow-lg'
                        : 'bg-slate-100 text-slate-500 dark:bg-slate-700 dark:text-slate-400',
                    ]"
                  >
                    <UIcon
                      :name="tab.icon"
                      class="w-4 h-4"
                    />
                  </div>
                  {{ tab.name }}
                </button>
              </nav>
            </div>
          </div>

          <!-- 主内容区域 -->
          <div class="lg:col-span-2">
            <!-- 基本信息 -->
            <div
              v-if="activeTab === 'basic'"
              class="modern-content-section"
              style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 2rem; margin-bottom: 1rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);"
            >
              <div class="modern-section-header">
                <div>
                  <div class="flex items-center gap-3 mb-2">
                    <div
                      class="modern-gradient-icon"
                      style="background: linear-gradient(45deg, #3b82f6, #6366f1); width: 2.5rem; height: 2.5rem;"
                    >
                      <UIcon
                        name="i-heroicons-user"
                        class="h-5 w-5 text-white"
                      />
                    </div>
                    <h2
                      class="modern-section-title"
                      style="font-size: 1.5rem;"
                    >
                      基本信息
                    </h2>
                  </div>
                  <p class="modern-section-subtitle">
                    管理您的基本账户信息
                  </p>
                </div>
              </div>

              <div style="padding: 0 1rem;">
                <UForm
                  :state="profileForm"
                  :schema="profileFormSchema"
                  class="space-y-6"
                  @submit="updateProfile"
                >
                  <UFormGroup
                    label="用户名"
                    name="username"
                  >
                    <UInput
                      v-model="profileForm.username"
                      class="modern-form-input"
                    />
                  </UFormGroup>

                  <UFormGroup
                    label="邮箱地址"
                    name="email"
                  >
                    <UInput
                      v-model="profileForm.email"
                      type="email"
                      class="modern-form-input"
                    />
                  </UFormGroup>

                  <UFormGroup
                    label="个人简介"
                    name="bio"
                  >
                    <UTextarea
                      v-model="profileForm.bio"
                      :rows="4"
                      placeholder="简单介绍一下自己..."
                      class="modern-form-input"
                    />
                  </UFormGroup>

                  <UFormGroup
                    label="网站"
                    name="website"
                  >
                    <UInput
                      v-model="profileForm.website"
                      placeholder="https://example.com"
                      class="modern-form-input"
                    />
                  </UFormGroup>

                  <div class="flex justify-end pt-4">
                    <UButton
                      type="submit"
                      :loading="saving"
                      class="modern-button modern-button-primary"
                      style="padding: 0.75rem 2rem; background: linear-gradient(45deg, #3b82f6, #6366f1); color: white; border: none; font-weight: 500;"
                    >
                      <UIcon
                        name="i-heroicons-check"
                        class="w-4 h-4 mr-2"
                      />
                      保存更改
                    </UButton>
                  </div>
                </UForm>
              </div>
            </div>

            <!-- 安全设置 -->
            <div
              v-if="activeTab === 'security'"
              class="modern-content-section"
              style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 2rem; margin-bottom: 1rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);"
            >
              <div class="modern-section-header">
                <div>
                  <div class="flex items-center gap-3 mb-2">
                    <div
                      class="modern-gradient-icon"
                      style="background: linear-gradient(45deg, #ef4444, #f97316); width: 2.5rem; height: 2.5rem;"
                    >
                      <UIcon
                        name="i-heroicons-lock-closed"
                        class="h-5 w-5 text-white"
                      />
                    </div>
                    <h2
                      class="modern-section-title"
                      style="font-size: 1.5rem;"
                    >
                      安全设置
                    </h2>
                  </div>
                  <p class="modern-section-subtitle">
                    管理您的账户安全和密码
                  </p>
                </div>
              </div>

              <div style="padding: 0 1rem;">
                <UForm
                  :state="passwordForm"
                  :schema="passwordFormSchema"
                  class="space-y-6"
                  @submit="changePassword"
                >
                  <UFormGroup
                    label="当前密码"
                    name="currentPassword"
                  >
                    <UInput
                      v-model="passwordForm.currentPassword"
                      type="password"
                      class="modern-form-input"
                    />
                  </UFormGroup>

                  <UFormGroup
                    label="新密码"
                    name="newPassword"
                  >
                    <UInput
                      v-model="passwordForm.newPassword"
                      type="password"
                      class="modern-form-input"
                    />
                  </UFormGroup>

                  <UFormGroup
                    label="确认新密码"
                    name="confirmPassword"
                  >
                    <UInput
                      v-model="passwordForm.confirmPassword"
                      type="password"
                      class="modern-form-input"
                    />
                  </UFormGroup>

                  <div class="flex justify-end pt-4">
                    <UButton
                      type="submit"
                      :loading="changingPassword"
                      class="modern-button modern-button-primary"
                      style="padding: 0.75rem 2rem; background: linear-gradient(45deg, #ef4444, #f97316); color: white; border: none; font-weight: 500;"
                    >
                      <UIcon
                        name="i-heroicons-shield-check"
                        class="w-4 h-4 mr-2"
                      />
                      更改密码
                    </UButton>
                  </div>
                </UForm>
              </div>
            </div>

            <!-- 偏好设置 -->
            <div
              v-if="activeTab === 'preferences'"
              class="modern-content-section"
              style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 2rem; margin-bottom: 1rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);"
            >
              <div class="modern-section-header">
                <div>
                  <div class="flex items-center gap-3 mb-2">
                    <div
                      class="modern-gradient-icon"
                      style="background: linear-gradient(45deg, #10b981, #06b6d4); width: 2.5rem; height: 2.5rem;"
                    >
                      <UIcon
                        name="i-heroicons-cog-6-tooth"
                        class="h-5 w-5 text-white"
                      />
                    </div>
                    <h2
                      class="modern-section-title"
                      style="font-size: 1.5rem;"
                    >
                      偏好设置
                    </h2>
                  </div>
                  <p class="modern-section-subtitle">
                    个性化您的使用体验
                  </p>
                </div>
              </div>

              <div
                class="space-y-6"
                style="padding: 0 1rem;"
              >
                <div
                  class="modern-post-card"
                  style="padding: 1.5rem; display: flex; align-items: center; justify-content: between;"
                >
                  <div class="flex items-center gap-4 flex-1">
                    <div
                      class="modern-gradient-icon"
                      style="background: linear-gradient(45deg, #6366f1, #8b5cf6); width: 3rem; height: 3rem;"
                    >
                      <UIcon
                        name="i-heroicons-moon"
                        class="h-5 w-5 text-white"
                      />
                    </div>
                    <div>
                      <h3 class="font-semibold text-slate-900 dark:text-white text-base">
                        深色模式
                      </h3>
                      <p class="text-sm text-slate-600 dark:text-slate-400">
                        切换界面主题，保护您的眼睛
                      </p>
                    </div>
                  </div>
                  <UToggle
                    v-model="isDarkMode"
                    @change="toggleTheme"
                  />
                </div>

                <div
                  class="modern-post-card"
                  style="padding: 1.5rem; display: flex; align-items: center; justify-content: between;"
                >
                  <div class="flex items-center gap-4 flex-1">
                    <div
                      class="modern-gradient-icon"
                      style="background: linear-gradient(45deg, #f59e0b, #f97316); width: 3rem; height: 3rem;"
                    >
                      <UIcon
                        name="i-heroicons-envelope"
                        class="h-5 w-5 text-white"
                      />
                    </div>
                    <div>
                      <h3 class="font-semibold text-slate-900 dark:text-white text-base">
                        邮件通知
                      </h3>
                      <p class="text-sm text-slate-600 dark:text-slate-400">
                        接收文章评论和系统重要通知
                      </p>
                    </div>
                  </div>
                  <UToggle
                    v-model="preferences.emailNotifications"
                    @change="updatePreferences"
                  />
                </div>

                <div
                  class="modern-post-card"
                  style="padding: 1.5rem; display: flex; align-items: center; justify-content: between;"
                >
                  <div class="flex items-center gap-4 flex-1">
                    <div
                      class="modern-gradient-icon"
                      style="background: linear-gradient(45deg, #10b981, #06b6d4); width: 3rem; height: 3rem;"
                    >
                      <UIcon
                        name="i-heroicons-eye"
                        class="h-5 w-5 text-white"
                      />
                    </div>
                    <div>
                      <h3 class="font-semibold text-slate-900 dark:text-white text-base">
                        公开个人资料
                      </h3>
                      <p class="text-sm text-slate-600 dark:text-slate-400">
                        允许其他用户查看您的个人资料页面
                      </p>
                    </div>
                  </div>
                  <UToggle
                    v-model="preferences.publicProfile"
                    @change="updatePreferences"
                  />
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { z } from "zod";

// 路由守卫
definePageMeta({
  middleware: "auth",
});

// SEO
useHead({
  title: "个人资料",
});

// 状态
const activeTab = ref("basic");
const saving = ref(false);
const changingPassword = ref(false);

const authStore = useAuthStore();
const colorMode = useColorMode();
const toast = useToast();

// 用户信息
const user = computed(() => authStore.user);

// 验证模式
const profileFormSchema = z.object({
  username: z.string()
    .min(3, "用户名至少需要3个字符")
    .max(20, "用户名不能超过20个字符")
    .regex(/^[a-zA-Z0-9_]+$/, "用户名只能包含字母、数字和下划线"),
  email: z.string()
    .email("请输入有效的邮箱地址"),
  bio: z.string()
    .max(500, "个人简介不能超过500个字符")
    .optional(),
  website: z.string()
    .url("请输入有效的网址")
    .optional()
    .or(z.literal("")),
});

const passwordFormSchema = z.object({
  currentPassword: z.string()
    .min(1, "请输入当前密码"),
  newPassword: z.string()
    .min(6, "新密码至少需要6个字符")
    .max(50, "密码不能超过50个字符"),
  confirmPassword: z.string()
    .min(1, "请确认新密码"),
}).refine(data => data.newPassword === data.confirmPassword, {
  message: "新密码和确认密码不匹配",
  path: ["confirmPassword"],
});

// 标签页配置
const tabs = [
  { id: "basic", name: "基本信息", icon: "i-heroicons-user" },
  { id: "security", name: "安全设置", icon: "i-heroicons-lock-closed" },
  { id: "preferences", name: "偏好设置", icon: "i-heroicons-cog-6-tooth" },
];

// 表单数据
const profileForm = reactive({
  username: user.value?.username || "",
  email: user.value?.email || "",
  bio: "",
  website: "",
});

const passwordForm = reactive({
  currentPassword: "",
  newPassword: "",
  confirmPassword: "",
});

const preferences = reactive({
  emailNotifications: true,
  publicProfile: true,
});

// 计算属性
const isDarkMode = computed({
  get: () => colorMode.value === "dark",
  set: (value) => {
    colorMode.preference = value ? "dark" : "light";
  },
});

// 方法
const updateProfile = async () => {
  saving.value = true;

  try {
    const api = useApi();
    const response = await api.updateProfile({
      username: profileForm.username,
      email: profileForm.email,
      bio: profileForm.bio,
      website: profileForm.website,
    });

    if (response.success) {
      // 更新本地用户信息
      if (response.data) {
        authStore.user = response.data;
      }

      toast.add({
        title: "保存成功",
        description: "个人资料已更新",
        color: "success",
      });
    }
    else {
      throw new Error(response.error?.message || "更新失败");
    }
  }
  catch (error: unknown) {
    const errorMessage = error instanceof Error ? error.message : "更新个人资料时发生错误";
    toast.add({
      title: "保存失败",
      description: errorMessage,
      color: "error",
    });
  }
  finally {
    saving.value = false;
  }
};

const changePassword = async () => {
  changingPassword.value = true;

  try {
    const api = useApi();
    const response = await api.changePassword({
      current_password: passwordForm.currentPassword,
      new_password: passwordForm.newPassword,
    });

    if (response.success) {
      // 清空表单
      passwordForm.currentPassword = "";
      passwordForm.newPassword = "";
      passwordForm.confirmPassword = "";

      toast.add({
        title: "密码更改成功",
        description: "您的密码已更新",
        color: "success",
      });
    }
    else {
      throw new Error(response.error?.message || "更改失败");
    }
  }
  catch (error: unknown) {
    const errorMessage = error instanceof Error ? error.message : "更改密码时发生错误";
    toast.add({
      title: "更改失败",
      description: errorMessage,
      color: "error",
    });
  }
  finally {
    changingPassword.value = false;
  }
};

const toggleTheme = () => {
  // 主题切换已通过计算属性处理
};

const updatePreferences = async () => {
  try {
    const api = useApi();
    const response = await api.updatePreferences({
      email_notifications: preferences.emailNotifications,
      public_profile: preferences.publicProfile,
      theme: colorMode.value,
    });

    if (response.success) {
      toast.add({
        title: "设置已保存",
        color: "success",
      });
    }
    else {
      throw new Error(response.error?.message || "保存失败");
    }
  }
  catch (error: unknown) {
    const errorMessage = error instanceof Error ? error.message : "更新设置时发生错误";
    toast.add({
      title: "保存失败",
      description: errorMessage,
      color: "error",
    });
  }
};

// 初始化数据
onMounted(() => {
  if (user.value) {
    profileForm.username = user.value.username;
    profileForm.email = user.value.email;
  }
});
</script>
