<template>
  <header class="app-header">
    <nav class="nav-container">
      <div class="nav-content">
        <!-- Logo -->
        <NuxtLink
          to="/"
          class="logo-link"
        >
          <div class="logo-icon">
            <UIcon
              name="i-heroicons-sparkles"
              class="w-5 h-5 text-white"
            />
          </div>
          <span class="logo-text">ZincBloom</span>
        </NuxtLink>

        <!-- 导航菜单 -->
        <div class="nav-menu">
          <NuxtLink
            v-for="item in navigationItems"
            :key="item.name"
            :to="item.href"
            class="nav-item"
            :class="{ active: $route.path === item.href }"
          >
            {{ item.name }}
          </NuxtLink>
        </div>

        <!-- 右侧区域 -->
        <div class="nav-actions">
          <!-- 搜索框 -->
          <div class="search-container">
            <input
              v-model="searchQuery"
              type="text"
              placeholder="搜索..."
              class="search-input"
              @keyup.enter="handleSearch"
            >
            <UIcon
              name="i-heroicons-magnifying-glass"
              class="search-icon"
            />
          </div>

          <!-- 主题切换按钮 -->
          <button
            class="theme-toggle"
            :title="isDark ? '切换到亮色模式' : '切换到暗色模式'"
            @click="toggleTheme"
          >
            <UIcon
              :name="
                isDark ? 'i-heroicons-sun' : 'i-heroicons-moon'
              "
              class="w-5 h-5"
            />
          </button>

          <!-- 用户状态 -->
          <div
            v-if="authStore.isAuthenticated"
            class="user-menu"
          >
            <div class="relative">
              <button
                class="user-button"
                @click="showUserMenu = !showUserMenu"
              >
                <div class="user-avatar">
                  <UIcon
                    name="i-heroicons-user"
                    class="w-3 h-3 text-white"
                  />
                </div>
                <span class="user-name">{{
                  authStore.user?.username
                }}</span>
                <UIcon
                  name="i-heroicons-chevron-down"
                  class="w-4 h-4"
                />
              </button>

              <!-- 下拉菜单 -->
              <div
                v-if="showUserMenu"
                class="user-dropdown-menu"
              >
                <div class="py-1">
                  <NuxtLink
                    to="/user/profile"
                    class="dropdown-item"
                    @click="showUserMenu = false"
                  >
                    <UIcon
                      name="i-heroicons-user"
                      class="w-4 h-4"
                    />
                    个人资料
                  </NuxtLink>
                  <NuxtLink
                    to="/user/posts"
                    class="dropdown-item"
                    @click="showUserMenu = false"
                  >
                    <UIcon
                      name="i-heroicons-document-text"
                      class="w-4 h-4"
                    />
                    我的文章
                  </NuxtLink>
                  <NuxtLink
                    to="/write"
                    class="dropdown-item"
                    @click="showUserMenu = false"
                  >
                    <UIcon
                      name="i-heroicons-pencil-square"
                      class="w-4 h-4"
                    />
                    写文章
                  </NuxtLink>
                  <hr
                    class="my-1 border-gray-200 dark:border-gray-700"
                  >
                  <NuxtLink
                    to="/user/settings"
                    class="dropdown-item"
                    @click="showUserMenu = false"
                  >
                    <UIcon
                      name="i-heroicons-cog-6-tooth"
                      class="w-4 h-4"
                    />
                    设置
                  </NuxtLink>
                  <hr
                    class="my-1 border-gray-200 dark:border-gray-700"
                  >
                  <button
                    class="dropdown-item w-full text-left"
                    @click="
                      handleLogout();
                      showUserMenu = false;
                    "
                  >
                    <UIcon
                      name="i-heroicons-arrow-right-on-rectangle"
                      class="w-4 h-4"
                    />
                    退出登录
                  </button>
                </div>
              </div>
            </div>
          </div>

          <!-- 未登录状态 -->
          <div
            v-else
            class="auth-buttons"
          >
            <NuxtLink
              to="/auth/login"
              class="login-btn"
            >
              登录
            </NuxtLink>
            <NuxtLink
              to="/auth/register"
              class="register-btn"
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
const colorMode = useColorMode();

// 响应式数据
const searchQuery = ref("");
const showUserMenu = ref(false);

// 主题相关
const isDark = computed(() => colorMode.value === "dark");

// 导航菜单项
const navigationItems = [
  { name: "首页", href: "/" },
  { name: "文章", href: "/posts" },
  { name: "分类", href: "/categories" },
  { name: "标签", href: "/tags" },
  { name: "作者", href: "/authors" },
  { name: "关于", href: "/about" },
];

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
    await router.push("/");
  }
  catch (error) {
    console.error("退出登录失败:", error);
  }
};

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

const toggleTheme = () => {
  colorMode.preference = colorMode.value === "dark" ? "light" : "dark";
};

// 点击外部关闭菜单
onMounted(() => {
  document.addEventListener("click", (e) => {
    const userMenu = document.querySelector(".user-menu");
    if (userMenu && !userMenu.contains(e.target as Node)) {
      showUserMenu.value = false;
    }
  });
});
</script>

<style scoped>
.app-header {
    position: sticky;
    top: 0;
    z-index: 40;
    width: 100%;
    background: rgba(255, 255, 255, 0.95);
    backdrop-filter: blur(20px);
    border-bottom: 1px solid rgba(226, 232, 240, 0.6);
    box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.05);
}

.dark .app-header {
    background: rgba(17, 24, 39, 0.95);
    border-bottom-color: rgba(55, 65, 81, 0.6);
}

.nav-container {
    margin: 0 auto;
    max-width: 80rem;
    padding: 0 1rem;
}

.nav-content {
    display: flex;
    height: 4rem;
    align-items: center;
    justify-content: space-between;
    flex-wrap: nowrap;
    overflow: visible;
    position: relative;
}

/* Logo 样式 */
.logo-link {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    text-decoration: none;
    flex-shrink: 0;
}

.logo-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 2.5rem;
    height: 2.5rem;
    background: linear-gradient(45deg, #3b82f6, #8b5cf6);
    border-radius: 0.75rem;
    box-shadow: 0 4px 8px rgba(59, 130, 246, 0.3);
}

.logo-text {
    font-size: 1.5rem;
    font-weight: 800;
    background: linear-gradient(45deg, #3b82f6, #8b5cf6, #6366f1);
    -webkit-background-clip: text;
    background-clip: text;
    -webkit-text-fill-color: transparent;
}

/* 导航菜单样式 */
.nav-menu {
    display: none;
    align-items: center;
    gap: 0.25rem;
    flex-shrink: 0;
}

@media (min-width: 768px) {
    .nav-menu {
        display: flex;
    }
}

.nav-item {
    padding: 0.5rem 0.75rem;
    border-radius: 0.75rem;
    font-size: 0.875rem;
    font-weight: 500;
    text-decoration: none;
    color: #64748b;
    background: transparent;
    white-space: nowrap;
    flex-shrink: 0;
    transition: all 0.2s ease;
}

.nav-item:hover {
    color: #3b82f6;
    background: rgba(59, 130, 246, 0.1);
}

.nav-item.active {
    color: #3b82f6;
    background: rgba(59, 130, 246, 0.1);
}

.dark .nav-item {
    color: #9ca3af;
}

.dark .nav-item:hover {
    color: #60a5fa;
    background: rgba(59, 130, 246, 0.2);
}

.dark .nav-item.active {
    color: #60a5fa;
    background: rgba(59, 130, 246, 0.2);
}

/* 右侧区域样式 */
.nav-actions {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    flex-shrink: 0;
}

/* 搜索框样式 */
.search-container {
    position: relative;
    flex-shrink: 0;
    display: none;
}

@media (min-width: 640px) {
    .search-container {
        display: block;
    }
}

.search-input {
    width: 8rem;
    background: rgba(248, 250, 252, 0.8);
    border: 1px solid rgba(226, 232, 240, 0.6);
    border-radius: 0.75rem;
    padding: 0.5rem 0.75rem 0.5rem 2rem;
    font-size: 0.875rem;
    outline: none;
    transition: all 0.2s ease;
}

.search-input:focus {
    border-color: rgba(59, 130, 246, 0.5);
    box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.1);
}

.dark .search-input {
    background: rgba(31, 41, 55, 0.8);
    border-color: rgba(55, 65, 81, 0.6);
    color: #f9fafb;
}

.dark .search-input::placeholder {
    color: #9ca3af;
}

.search-icon {
    position: absolute;
    left: 0.5rem;
    top: 50%;
    transform: translateY(-50%);
    width: 1rem;
    height: 1rem;
    color: #9ca3af;
}

/* 主题切换按钮样式 */
.theme-toggle {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 2.5rem;
    height: 2.5rem;
    border-radius: 0.75rem;
    background: rgba(248, 250, 252, 0.8);
    border: 1px solid rgba(226, 232, 240, 0.6);
    cursor: pointer;
    color: #64748b;
    transition: all 0.2s ease;
    flex-shrink: 0;
}

.theme-toggle:hover {
    color: #3b82f6;
    background: rgba(59, 130, 246, 0.1);
}

.dark .theme-toggle {
    background: rgba(31, 41, 55, 0.8);
    border-color: rgba(55, 65, 81, 0.6);
    color: #9ca3af;
}

.dark .theme-toggle:hover {
    color: #60a5fa;
    background: rgba(59, 130, 246, 0.2);
}

/* 用户菜单样式 */
.user-menu {
    flex-shrink: 0;
    position: relative;
    z-index: 100;
}

.user-button-custom {
    background: transparent !important;
    border: none !important;
    color: #64748b !important;
    transition: all 0.2s ease;
}

.user-button-custom:hover {
    color: #3b82f6 !important;
    background: rgba(59, 130, 246, 0.1) !important;
}

.dark .user-button-custom {
    color: #9ca3af !important;
}

.dark .user-button-custom:hover {
    color: #60a5fa !important;
    background: rgba(59, 130, 246, 0.2) !important;
}

.user-button {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    border-radius: 0.75rem;
    font-size: 0.875rem;
    background: transparent;
    border: none;
    color: #64748b;
    cursor: pointer;
    transition: all 0.2s ease;
}

.user-button:hover {
    color: #3b82f6;
    background: rgba(59, 130, 246, 0.1);
}

.dark .user-button {
    color: #9ca3af;
}

.dark .user-button:hover {
    color: #60a5fa;
    background: rgba(59, 130, 246, 0.2);
}

.user-avatar {
    width: 1.5rem;
    height: 1.5rem;
    background: linear-gradient(45deg, #3b82f6, #8b5cf6);
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
}

.user-name {
    display: none;
}

@media (min-width: 640px) {
    .user-name {
        display: inline;
    }
}

/* 认证按钮样式 */
.auth-buttons {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    flex-shrink: 0;
}

.login-btn {
    padding: 0.5rem 1rem;
    border-radius: 0.75rem;
    font-size: 0.875rem;
    text-decoration: none;
    color: #64748b;
    background: transparent;
    white-space: nowrap;
    transition: all 0.2s ease;
}

.login-btn:hover {
    color: #3b82f6;
    background: rgba(59, 130, 246, 0.1);
}

.register-btn {
    padding: 0.5rem 1rem;
    border-radius: 0.75rem;
    font-size: 0.875rem;
    text-decoration: none;
    color: white;
    background: linear-gradient(45deg, #3b82f6, #6366f1);
    white-space: nowrap;
    box-shadow: 0 2px 4px rgba(59, 130, 246, 0.3);
    transition: all 0.2s ease;
}

.register-btn:hover {
    box-shadow: 0 4px 8px rgba(59, 130, 246, 0.4);
    transform: translateY(-1px);
}

.dark .login-btn {
    color: #9ca3af;
}

.dark .login-btn:hover {
    color: #60a5fa;
    background: rgba(59, 130, 246, 0.2);
}

/* 下拉菜单项样式 */
.dropdown-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    text-decoration: none;
    color: #374151;
    font-size: 0.875rem;
    transition: background-color 0.2s ease;
}

.dropdown-item:hover {
    background-color: #f3f4f6;
}

.dark .dropdown-item {
    color: #d1d5db;
}

.dark .dropdown-item:hover {
    background-color: #374151;
}

/* 确保下拉菜单在最顶层 */
.user-menu .relative {
    position: relative;
}

.user-menu .absolute {
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.15);
    border: 1px solid rgba(0, 0, 0, 0.1);
}

.dark .user-menu .absolute {
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.5);
    border: 1px solid rgba(255, 255, 255, 0.1);
}

/* 用户下拉菜单样式 */
.user-dropdown-menu {
    position: absolute;
    top: 100%;
    right: 0;
    margin-top: 0.5rem;
    width: 12rem; /* w-48 */
    background-color: white;
    border-radius: 0.375rem;
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.15);
    border: 1px solid rgba(0, 0, 0, 0.1);
    z-index: 9999;
    transform-origin: top right;
    animation: dropdown-appear 0.1s ease-out;
}

.dark .user-dropdown-menu {
    background-color: #1f2937; /* gray-800 */
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.5);
    border: 1px solid rgba(255, 255, 255, 0.1);
}

@keyframes dropdown-appear {
    from {
        opacity: 0;
        transform: scale(0.95) translateY(-10px);
    }
    to {
        opacity: 1;
        transform: scale(1) translateY(0);
    }
}
</style>
