<template>
    <header
        class="sticky top-0 z-40 w-full backdrop-blur-xl bg-white/80 dark:bg-gray-900/80 border-b border-gray-200 dark:border-gray-800"
    >
        <nav class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
            <div class="flex h-16 items-center justify-between">
                <!-- Logo 和网站标题 -->
                <div class="flex items-center">
                    <NuxtLink
                        to="/"
                        class="flex items-center space-x-2 text-xl font-bold text-gray-900 dark:text-white hover:text-primary-500 transition-colors"
                    >
                        <UIcon name="i-heroicons-sparkles" class="w-8 h-8" />
                        <span>ZincBloom</span>
                    </NuxtLink>
                </div>

                <!-- 桌面端导航菜单 -->
                <div class="hidden md:block">
                    <div class="ml-10 flex items-baseline space-x-4">
                        <NuxtLink
                            v-for="item in navigationItems"
                            :key="item.name"
                            :to="item.href"
                            class="px-3 py-2 rounded-md text-sm font-medium transition-colors"
                            :class="[
                                $route.path === item.href
                                    ? 'bg-primary-100 text-primary-700 dark:bg-primary-900 dark:text-primary-300'
                                    : 'text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 hover:text-gray-900 dark:hover:text-white',
                            ]"
                        >
                            {{ item.name }}
                        </NuxtLink>
                    </div>
                </div>

                <!-- 搜索、用户状态和主题切换 -->
                <div class="hidden md:flex items-center space-x-4">
                    <!-- 搜索框 -->
                    <div class="relative">
                        <UInput
                            v-model="searchQuery"
                            icon="i-heroicons-magnifying-glass"
                            placeholder="搜索文章..."
                            class="w-64"
                            @keyup.enter="handleSearch"
                        />
                    </div>

                    <!-- 主题切换 -->
                    <UButton
                        :icon="isDark ? 'i-heroicons-sun' : 'i-heroicons-moon'"
                        variant="ghost"
                        color="neutral"
                        @click="toggleColorMode"
                    />

                    <!-- 用户状态 -->
                    <div v-if="authStore.isAuthenticated" class="relative">
                        <UDropdown
                            :items="userMenuItems"
                            :popper="{ placement: 'bottom-end' }"
                        >
                            <UButton
                                variant="ghost"
                                color="neutral"
                                :label="authStore.user?.username"
                                trailing-icon="i-heroicons-chevron-down-20-solid"
                            />
                        </UDropdown>
                    </div>

                    <!-- 未登录状态 -->
                    <div v-else class="flex items-center space-x-2">
                        <UButton to="/auth/login" variant="ghost" color="neutral">
                            登录
                        </UButton>
                        <UButton to="/auth/register" color="primary">
                            注册
                        </UButton>
                    </div>
                </div>

                <!-- 移动端菜单按钮 -->
                <div class="md:hidden">
                    <UButton
                        :icon="
                            isMobileMenuOpen
                                ? 'i-heroicons-x-mark'
                                : 'i-heroicons-bars-3'
                        "
                        variant="ghost"
                        color="neutral"
                        @click="toggleMobileMenu"
                    />
                </div>
            </div>

            <!-- 移动端菜单 -->
            <div v-show="isMobileMenuOpen" class="md:hidden">
                <div
                    class="px-2 pt-2 pb-3 space-y-1 sm:px-3 border-t border-gray-200 dark:border-gray-700"
                >
                    <!-- 移动端导航 -->
                    <NuxtLink
                        v-for="item in navigationItems"
                        :key="item.name"
                        :to="item.href"
                        class="block px-3 py-2 rounded-md text-base font-medium transition-colors"
                        :class="[
                            $route.path === item.href
                                ? 'bg-primary-100 text-primary-700 dark:bg-primary-900 dark:text-primary-300'
                                : 'text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 hover:text-gray-900 dark:hover:text-white',
                        ]"
                        @click="closeMobileMenu"
                    >
                        {{ item.name }}
                    </NuxtLink>

                    <!-- 移动端搜索 -->
                    <div class="px-3 py-2">
                        <UInput
                            v-model="searchQuery"
                            icon="i-heroicons-magnifying-glass"
                            placeholder="搜索文章..."
                            @keyup.enter="handleSearch"
                        />
                    </div>

                    <!-- 移动端用户状态 -->
                    <div class="px-3 py-2">
                        <div v-if="authStore.isAuthenticated" class="space-y-2">
                            <div
                                class="text-sm text-gray-600 dark:text-gray-400"
                            >
                                欢迎，{{ authStore.user?.username }}
                            </div>
                            <div class="space-y-1">
                                <NuxtLink
                                    v-for="item in userMenuItems[0]"
                                    :key="item.label"
                                    :to="item.to"
                                    class="block py-1 text-sm text-gray-700 dark:text-gray-300 hover:text-gray-900 dark:hover:text-white"
                                    @click="closeMobileMenu"
                                >
                                    {{ item.label }}
                                </NuxtLink>
                            </div>
                        </div>
                        <div v-else class="flex flex-col space-y-2">
                            <UButton
                                to="/auth/login"
                                variant="outline"
                                block
                                @click="closeMobileMenu"
                            >
                                登录
                            </UButton>
                            <UButton
                                to="/auth/register"
                                color="primary"
                                block
                                @click="closeMobileMenu"
                            >
                                注册
                            </UButton>
                        </div>
                    </div>

                    <!-- 移动端主题切换 -->
                    <div class="px-3 py-2">
                        <UButton
                            :icon="
                                isDark ? 'i-heroicons-sun' : 'i-heroicons-moon'
                            "
                            :label="
                                isDark ? '切换到浅色模式' : '切换到深色模式'
                            "
                            variant="ghost"
                            color="neutral"
                            block
                            @click="toggleColorMode"
                        />
                    </div>
                </div>
            </div>
        </nav>
    </header>
</template>

<script setup lang="ts">
import type { DropdownItem } from '~/types/ui';

// 由于找不到“~/types/ui”，暂时移除类型导入或请确保该路径存在
// import type { DropdownItem } from "~/types/ui";

// 状态管理
const authStore = useAuthStore();
const colorMode = useColorMode();

// 响应式状态
const searchQuery = ref("");
const isMobileMenuOpen = ref(false);

// 计算属性
const isDark = computed(() => colorMode.value === "dark");

// 导航菜单项
const navigationItems = [
    { name: "首页", href: "/" },
    { name: "文章", href: "/posts" },
    { name: "分类", href: "/categories" },
    { name: "标签", href: "/tags" },
    { name: "关于", href: "/about" },
];

// 用户菜单项
const userMenuItems = computed((): DropdownItem[][] => [
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
const toggleColorMode = () => {
    colorMode.preference = colorMode.value === "dark" ? "light" : "dark";
};

const toggleMobileMenu = () => {
    isMobileMenuOpen.value = !isMobileMenuOpen.value;
};

const closeMobileMenu = () => {
    isMobileMenuOpen.value = false;
};

const handleSearch = () => {
    if (searchQuery.value.trim()) {
        navigateTo(`/search?q=${encodeURIComponent(searchQuery.value)}`);
        searchQuery.value = "";
        closeMobileMenu();
    }
};

const handleLogout = async () => {
    await authStore.logout();
    closeMobileMenu();
};

// 监听路由变化，关闭移动端菜单
watch(
    () => useRoute().path,
    () => {
        closeMobileMenu();
    }
);
</script>
