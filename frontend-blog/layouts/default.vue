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
        <UButton
            v-show="showBackToTop"
            icon="i-heroicons-arrow-up"
            size="lg"
            color="primary"
            variant="solid"
            class="fixed bottom-8 right-8 z-50 shadow-lg"
            @click="scrollToTop"
        />

        <!-- 通知组件 -->
        <UNotifications />
    </div>
</template>

<script setup lang="ts">
// 回到顶部功能
const showBackToTop = ref(false);

// 监听滚动位置
const handleScroll = () => {
    showBackToTop.value = window.scrollY > 300;
};

// 回到顶部
const scrollToTop = () => {
    window.scrollTo({
        top: 0,
        behavior: "smooth",
    });
};

// 生命周期
onMounted(() => {
    window.addEventListener("scroll", handleScroll);
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
    ],
    link: [{ rel: "icon", type: "image/x-icon", href: "/favicon.ico" }],
});

// 默认页面描述
useSeoMeta({
    description: "基于 Nuxt 3 和 Rust 构建的现代化博客系统",
    ogDescription: "基于 Nuxt 3 和 Rust 构建的现代化博客系统",
    ogImage: "/og-image.jpg",
    twitterCard: "summary_large_image",
});
</script>
