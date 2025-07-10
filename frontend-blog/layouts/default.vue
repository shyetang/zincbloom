<template>
    <div class="min-h-screen bg-gray-50 dark:bg-gray-900 transition-colors">
        <!-- 导航栏 -->
        <AppHeader />

        <!-- 主内容区域 -->
        <main class="flex-1">
            <slot />
        </main>

        <!-- 页脚 -->
        <AppFooter />

        <!-- 回到顶部按钮 -->
        <Transition
            enter-active-class="transition ease-out duration-300"
            enter-from-class="transform opacity-0 scale-95"
            enter-to-class="transform opacity-100 scale-100"
            leave-active-class="transition ease-in duration-200"
            leave-from-class="transform opacity-100 scale-100"
            leave-to-class="transform opacity-0 scale-95"
        >
            <UButton
                v-show="showBackToTop"
                icon="i-heroicons-arrow-up"
                size="lg"
                color="primary"
                variant="solid"
                class="fixed bottom-8 right-8 z-50 shadow-lg hover:shadow-xl transition-shadow"
                @click="scrollToTop"
            />
        </Transition>

        <!-- 全局通知组件 -->
        <UNotifications />

        <!-- 页面加载进度条 -->
        <UProgress
            v-if="$route.meta.showProgress !== false"
            :model-value="isLoading ? 30 : 100"
            class="fixed top-0 left-0 right-0 z-50 h-1"
            color="primary"
        />
    </div>
</template>

<script setup lang="ts">
// 回到顶部功能
const showBackToTop = ref(false);
const isLoading = ref(false);

// 监听页面加载状态
const nuxtApp = useNuxtApp();

// 监听路由变化显示加载状态
nuxtApp.hook("page:start", () => {
    isLoading.value = true;
});

nuxtApp.hook("page:finish", () => {
    isLoading.value = false;
});

// 监听滚动位置
const handleScroll = () => {
    showBackToTop.value = window.scrollY > 300;
};

// 平滑滚动到顶部
const scrollToTop = () => {
    window.scrollTo({
        top: 0,
        behavior: "smooth",
    });
};

// 生命周期管理
onMounted(() => {
    window.addEventListener("scroll", handleScroll, { passive: true });
});

onUnmounted(() => {
    window.removeEventListener("scroll", handleScroll);
});

// SEO 默认配置
useHead({
    titleTemplate: "%s - ZincBloom",
    meta: [
        { charset: "utf-8" },
        { name: "viewport", content: "width=device-width, initial-scale=1" },
        { name: "format-detection", content: "telephone=no" },
        { name: "theme-color", content: "#3b82f6" },
    ],
    link: [
        { rel: "icon", type: "image/x-icon", href: "/favicon.ico" },
        { rel: "preconnect", href: "https://fonts.googleapis.com" },
    ],
});

// 默认页面元数据
useSeoMeta({
    description:
        "基于 Nuxt 3 和 Rust 构建的现代化博客系统，提供优雅的写作体验和阅读体验。",
    ogDescription: "基于 Nuxt 3 和 Rust 构建的现代化博客系统",
    ogImage: "/og-image.jpg",
    ogImageWidth: "1200",
    ogImageHeight: "630",
    twitterCard: "summary_large_image",
    twitterImage: "/og-image.jpg",
    themeColor: "#3b82f6",
});

// 全局键盘快捷键
onMounted(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
        // Ctrl/Cmd + K 打开搜索
        if ((event.ctrlKey || event.metaKey) && event.key === "k") {
            event.preventDefault();
            // TODO: 触发全局搜索
        }

        // ESC 关闭模态框或菜单
        if (event.key === "Escape") {
            // TODO: 触发全局ESC处理
        }
    };

    window.addEventListener("keydown", handleKeyDown);

    onUnmounted(() => {
        window.removeEventListener("keydown", handleKeyDown);
    });
});
</script>

<style>
/* 全局样式优化 */
html {
    scroll-behavior: smooth;
}

/* 自定义滚动条样式 */
::-webkit-scrollbar {
    width: 8px;
}

::-webkit-scrollbar-track {
    @apply bg-gray-100 dark:bg-gray-800;
}

::-webkit-scrollbar-thumb {
    @apply bg-gray-400 dark:bg-gray-600 rounded-full;
}

::-webkit-scrollbar-thumb:hover {
    @apply bg-gray-500 dark:bg-gray-500;
}

/* 选择文本的颜色 */
::selection {
    @apply bg-primary-100 text-primary-900;
}

/* 深色模式下的选择文本颜色 */
.dark ::selection {
    @apply bg-primary-900 text-primary-100;
}

/* 焦点状态优化 */
*:focus-visible {
    @apply outline-none ring-2 ring-primary-500 ring-offset-2 ring-offset-white dark:ring-offset-gray-900;
}

/* 图片加载优化 */
img {
    @apply transition-opacity duration-300;
}

img[loading="lazy"] {
    @apply opacity-0;
}

img[loading="lazy"].loaded {
    @apply opacity-100;
}

/* 页面过渡动画 */
.page-enter-active,
.page-leave-active {
    transition: all 0.3s ease;
}

.page-enter-from,
.page-leave-to {
    opacity: 0;
    transform: translateY(10px);
}
</style>
