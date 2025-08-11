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
          <div style="margin: 0 auto 1.5rem; width: 3rem; height: 3rem; display: flex; align-items: center; justify-content: center; border-radius: 50%; background: linear-gradient(45deg, #10b981, #06b6d4); box-shadow: 0 6px 24px rgba(16, 185, 129, 0.3);">
            <UIcon
              name="i-heroicons-cog-6-tooth"
              style="width: 1.75rem; height: 1.75rem; color: white;"
            />
          </div>

          <!-- Modern Title -->
          <h1
            style="font-size: 2.25rem; font-weight: 700; line-height: 1.1; margin-bottom: 0.75rem; color: #1e293b;"
            class="dark:text-white"
          >
            用户设置
          </h1>

          <!-- Subtitle -->
          <p
            style="font-size: 1rem; color: #64748b; max-width: 28rem; margin: 0 auto; line-height: 1.5;"
            class="dark:text-slate-300"
          >
            个性化您的账户配置，享受更优质的使用体验
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
            <div class="modern-sidebar-card">
              <div class="modern-sidebar-header">
                <div
                  class="modern-gradient-icon"
                  style="width: 4rem; height: 4rem; border-radius: 50%; background: linear-gradient(45deg, #10b981, #06b6d4);"
                >
                  <UIcon
                    name="i-heroicons-cog-6-tooth"
                    class="w-6 h-6 text-white"
                  />
                </div>
                <div>
                  <h3
                    class="modern-sidebar-title"
                    style="font-size: 1.125rem; margin-bottom: 0.25rem;"
                  >
                    设置中心
                  </h3>
                  <p
                    style="font-size: 0.875rem; color: #64748b;"
                    class="dark:text-slate-400"
                  >
                    {{ user?.username }}
                  </p>
                </div>
              </div>

              <nav class="space-y-1">
                <button
                  v-for="tab in tabs"
                  :key="tab.id"
                  class="w-full text-left px-4 py-3 rounded-lg text-sm font-medium transition-all duration-200 flex items-center gap-3"
                  :class="activeTab === tab.id
                    ? 'bg-gradient-to-r from-emerald-50 to-cyan-50 text-emerald-600 border border-emerald-200 shadow-sm dark:from-emerald-900/20 dark:to-cyan-900/20 dark:text-emerald-400 dark:border-emerald-800'
                    : 'text-slate-600 hover:bg-slate-50 hover:text-slate-900 dark:text-slate-400 dark:hover:bg-slate-800 dark:hover:text-white'"
                  @click="activeTab = tab.id"
                >
                  <div
                    class="flex items-center justify-center w-8 h-8 rounded-lg transition-all"
                    :class="activeTab === tab.id
                      ? 'bg-gradient-to-br from-emerald-500 to-cyan-600 text-white shadow-lg'
                      : 'bg-slate-100 text-slate-500 dark:bg-slate-700 dark:text-slate-400'"
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
            <!-- 通知设置 -->
            <div
              v-if="activeTab === 'notifications'"
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
                        name="i-heroicons-bell"
                        class="h-5 w-5 text-white"
                      />
                    </div>
                    <h2
                      class="modern-section-title"
                      style="font-size: 1.5rem;"
                    >
                      通知设置
                    </h2>
                  </div>
                  <p class="modern-section-subtitle">
                    管理您的通知偏好设置
                  </p>
                </div>
              </div>

              <div
                class="space-y-6"
                style="padding: 0 1rem;"
              >
                <div
                  class="modern-post-card"
                  style="padding: 1.5rem; display: flex; align-items: center; justify-content: space-between;"
                >
                  <div class="flex items-center gap-4 flex-1">
                    <div
                      class="modern-gradient-icon"
                      style="background: linear-gradient(45deg, #3b82f6, #6366f1); width: 3rem; height: 3rem;"
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
                        接收文章评论和回复的邮件通知
                      </p>
                    </div>
                  </div>
                  <UToggle
                    v-model="settings.emailNotifications"
                    @change="saveSettings"
                  />
                </div>

                <div
                  class="modern-post-card"
                  style="padding: 1.5rem; display: flex; align-items: center; justify-content: space-between;"
                >
                  <div class="flex items-center gap-4 flex-1">
                    <div
                      class="modern-gradient-icon"
                      style="background: linear-gradient(45deg, #10b981, #059669); width: 3rem; height: 3rem;"
                    >
                      <UIcon
                        name="i-heroicons-users"
                        class="h-5 w-5 text-white"
                      />
                    </div>
                    <div>
                      <h3 class="font-semibold text-slate-900 dark:text-white text-base">
                        新关注者通知
                      </h3>
                      <p class="text-sm text-slate-600 dark:text-slate-400">
                        有新用户关注您时发送通知
                      </p>
                    </div>
                  </div>
                  <UToggle
                    v-model="settings.followerNotifications"
                    @change="saveSettings"
                  />
                </div>

                <div
                  class="modern-post-card"
                  style="padding: 1.5rem; display: flex; align-items: center; justify-content: space-between;"
                >
                  <div class="flex items-center gap-4 flex-1">
                    <div
                      class="modern-gradient-icon"
                      style="background: linear-gradient(45deg, #f59e0b, #f97316); width: 3rem; height: 3rem;"
                    >
                      <UIcon
                        name="i-heroicons-megaphone"
                        class="h-5 w-5 text-white"
                      />
                    </div>
                    <div>
                      <h3 class="font-semibold text-slate-900 dark:text-white text-base">
                        系统公告
                      </h3>
                      <p class="text-sm text-slate-600 dark:text-slate-400">
                        接收系统维护和更新公告
                      </p>
                    </div>
                  </div>
                  <UToggle
                    v-model="settings.systemNotifications"
                    @change="saveSettings"
                  />
                </div>
              </div>
            </div>

            <!-- 账户设置 -->
            <div
              v-if="activeTab === 'account'"
              class="modern-content-section"
              style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 2rem; margin-bottom: 1rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);"
            >
              <div class="modern-section-header">
                <div>
                  <div class="flex items-center gap-3 mb-2">
                    <div
                      class="modern-gradient-icon"
                      style="background: linear-gradient(45deg, #8b5cf6, #a855f7); width: 2.5rem; height: 2.5rem;"
                    >
                      <UIcon
                        name="i-heroicons-user-circle"
                        class="h-5 w-5 text-white"
                      />
                    </div>
                    <h2
                      class="modern-section-title"
                      style="font-size: 1.5rem;"
                    >
                      账户设置
                    </h2>
                  </div>
                  <p class="modern-section-subtitle">
                    管理基本账户信息
                  </p>
                </div>
              </div>

              <div
                class="space-y-6"
                style="padding: 0 1rem;"
              >
                <div
                  class="modern-post-card"
                  style="padding: 2rem;"
                >
                  <h3 class="font-semibold text-slate-900 dark:text-white mb-4">
                    账户信息
                  </h3>
                  <div class="space-y-3">
                    <div class="flex justify-between items-center">
                      <span class="text-slate-600 dark:text-slate-400">用户名:</span>
                      <span class="font-medium text-slate-900 dark:text-white">{{ user?.username }}</span>
                    </div>
                    <div class="flex justify-between items-center">
                      <span class="text-slate-600 dark:text-slate-400">邮箱:</span>
                      <span class="font-medium text-slate-900 dark:text-white">{{ user?.email }}</span>
                    </div>
                    <div class="flex justify-between items-center">
                      <span class="text-slate-600 dark:text-slate-400">注册时间:</span>
                      <span class="font-medium text-slate-900 dark:text-white">{{ formatDate(user?.created_at) }}</span>
                    </div>
                  </div>
                </div>

                <div class="space-y-4">
                  <NuxtLink
                    to="/user/profile"
                    class="modern-post-card block p-4 hover:shadow-lg transition-all"
                  >
                    <div class="flex items-center justify-between">
                      <div class="flex items-center gap-3">
                        <div
                          class="modern-gradient-icon"
                          style="background: linear-gradient(45deg, #3b82f6, #6366f1); width: 2.5rem; height: 2.5rem;"
                        >
                          <UIcon
                            name="i-heroicons-user"
                            class="w-4 h-4 text-white"
                          />
                        </div>
                        <span class="font-medium text-slate-900 dark:text-white">编辑个人资料</span>
                      </div>
                      <UIcon
                        name="i-heroicons-chevron-right"
                        class="w-5 h-5 text-slate-400"
                      />
                    </div>
                  </NuxtLink>
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
// 路由守卫
definePageMeta({
  middleware: "auth",
});

// SEO
useHead({
  title: "用户设置",
});

// 状态
const activeTab = ref("notifications");

const authStore = useAuthStore();
const toast = useToast();

// 用户信息
const user = computed(() => authStore.user);

// 标签页配置
const tabs = [
  { id: "notifications", name: "通知设置", icon: "i-heroicons-bell" },
  { id: "account", name: "账户设置", icon: "i-heroicons-user-circle" },
];

// 设置数据
const settings = reactive({
  emailNotifications: true,
  followerNotifications: true,
  systemNotifications: false,
});

// 方法
const saveSettings = async () => {
  try {
    const api = useApi();
    const response = await api.updatePreferences({
      email_notifications: settings.emailNotifications,
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
    toast.add({
      title: "保存失败",
      description: error instanceof Error ? error.message : "更新设置时发生错误",
      color: "error",
    });
  }
};

const formatDate = (date: string | undefined) => {
  if (!date) return "未知";
  return new Date(date).toLocaleDateString("zh-CN");
};
</script>
