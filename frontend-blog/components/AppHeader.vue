<template>
  <header style="position: sticky; top: 0; z-index: 50; width: 100%; background: rgba(255, 255, 255, 0.95); backdrop-filter: blur(20px); border-bottom: 1px solid rgba(226, 232, 240, 0.6); box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.05);">
    <nav style="margin: 0 auto; max-width: 80rem; padding: 0 1rem;">
      <div style="display: flex; height: 4rem; align-items: center; justify-content: space-between; flex-wrap: nowrap; overflow: hidden;">
        <!-- Logo -->
        <NuxtLink
          to="/"
          style="display: flex; align-items: center; gap: 0.75rem; text-decoration: none; flex-shrink: 0;"
        >
          <div style="display: flex; align-items: center; justify-content: center; width: 2.5rem; height: 2.5rem; background: linear-gradient(45deg, #3b82f6, #8b5cf6); border-radius: 0.75rem; box-shadow: 0 4px 8px rgba(59, 130, 246, 0.3);">
            <UIcon
              name="i-heroicons-sparkles"
              class="w-5 h-5 text-white"
            />
          </div>
          <span style="font-size: 1.5rem; font-weight: 800; background: linear-gradient(45deg, #3b82f6, #8b5cf6, #6366f1); -webkit-background-clip: text; background-clip: text; -webkit-text-fill-color: transparent;">ZincBloom</span>
        </NuxtLink>

        <!-- 导航菜单 -->
        <div style="display: flex; align-items: center; gap: 0.25rem; flex-shrink: 0;">
          <NuxtLink
            v-for="item in navigationItems"
            :key="item.name"
            :to="item.href"
            :style="{
              padding: '0.5rem 0.75rem',
              borderRadius: '0.75rem',
              fontSize: '0.875rem',
              fontWeight: '500',
              textDecoration: 'none',
              color: $route.path === item.href ? '#3b82f6' : '#64748b',
              background: $route.path === item.href ? 'rgba(59, 130, 246, 0.1)' : 'transparent',
              whiteSpace: 'nowrap',
              flexShrink: '0',
              transition: 'all 0.2s ease',
            }"
          >
            {{ item.name }}
          </NuxtLink>
        </div>

        <!-- 右侧区域 -->
        <div style="display: flex; align-items: center; gap: 0.75rem; flex-shrink: 0;">
          <!-- 搜索框 -->
          <div style="position: relative; flex-shrink: 0;">
            <input
              v-model="searchQuery"
              type="text"
              placeholder="搜索..."
              style="width: 8rem; background: rgba(248, 250, 252, 0.8); border: 1px solid rgba(226, 232, 240, 0.6); border-radius: 0.75rem; padding: 0.5rem 0.75rem 0.5rem 2rem; font-size: 0.875rem; outline: none;"
              @keyup.enter="handleSearch"
            >
            <UIcon
              name="i-heroicons-magnifying-glass"
              style="position: absolute; left: 0.5rem; top: 50%; transform: translateY(-50%); width: 1rem; height: 1rem; color: #9ca3af;"
            />
          </div>

          <!-- 用户状态 -->
          <div
            v-if="authStore.isAuthenticated"
            style="flex-shrink: 0;"
          >
            <UDropdown
              :items="userMenuItems"
              :popper="{ placement: 'bottom-end' }"
            >
              <button style="display: flex; align-items: center; gap: 0.5rem; padding: 0.5rem 1rem; border-radius: 0.75rem; font-size: 0.875rem; background: transparent; border: none; color: #64748b; cursor: pointer;">
                <div style="width: 1.5rem; height: 1.5rem; background: linear-gradient(45deg, #3b82f6, #8b5cf6); border-radius: 50%; display: flex; align-items: center; justify-content: center;">
                  <UIcon
                    name="i-heroicons-user"
                    class="w-3 h-3 text-white"
                  />
                </div>
                {{ authStore.user?.username }}
                <UIcon
                  name="i-heroicons-chevron-down"
                  class="w-4 h-4"
                />
              </button>
            </UDropdown>
          </div>

          <!-- 未登录状态 -->
          <div
            v-else
            style="display: flex; align-items: center; gap: 0.5rem; flex-shrink: 0;"
          >
            <NuxtLink
              to="/auth/login"
              style="padding: 0.5rem 1rem; border-radius: 0.75rem; font-size: 0.875rem; text-decoration: none; color: #64748b; background: transparent; white-space: nowrap; transition: all 0.2s ease;"
            >
              登录
            </NuxtLink>
            <NuxtLink
              to="/auth/register"
              style="padding: 0.5rem 1rem; border-radius: 0.75rem; font-size: 0.875rem; text-decoration: none; color: white; background: linear-gradient(45deg, #3b82f6, #6366f1); white-space: nowrap; box-shadow: 0 2px 4px rgba(59, 130, 246, 0.3); transition: all 0.2s ease;"
            >
              注册
            </NuxtLink>
          </div>
        </div>
      </div>
    </nav>
  </header>
</template>

<script setup lang="ts">
// 状态管理
const authStore = useAuthStore();
const router = useRouter();

// 响应式数据
const searchQuery = ref("");

// 导航菜单项
const navigationItems = [
  { name: "首页", href: "/" },
  { name: "文章", href: "/posts" },
  { name: "分类", href: "/categories" },
  { name: "标签", href: "/tags" },
  { name: "作者", href: "/authors" },
  { name: "关于", href: "/about" },
];

// 用户菜单项
const userMenuItems = computed(() => [
  [
    {
      label: "个人资料",
      icon: "i-heroicons-user",
      to: "/user/profile",
    },
    {
      label: "我的文章",
      icon: "i-heroicons-document-text",
      to: "/user/posts",
    },
    {
      label: "写文章",
      icon: "i-heroicons-pencil-square",
      to: "/write",
    },
  ],
  [
    {
      label: "设置",
      icon: "i-heroicons-cog-6-tooth",
      to: "/user/settings",
    },
  ],
  [
    {
      label: "退出登录",
      icon: "i-heroicons-arrow-right-on-rectangle",
      click: handleLogout,
    },
  ],
]);

// 方法
const handleSearch = () => {
  if (searchQuery.value.trim()) {
    router.push({
      path: "/posts",
      query: { search: searchQuery.value.trim() },
    });
  }
};

const handleLogout = async () => {
  try {
    await authStore.logout();
    router.push("/");
  }
  catch (error) {
    console.error("退出登录失败:", error);
  }
};
</script>
