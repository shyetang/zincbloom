<template>
    <footer
        class="bg-white dark:bg-gray-900 border-t border-gray-200 dark:border-gray-800"
    >
        <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
            <!-- 主要内容区域 -->
            <div
                class="py-12 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8"
            >
                <!-- 网站信息 -->
                <div class="lg:col-span-1">
                    <div class="flex items-center space-x-2 mb-4">
                        <UIcon
                            name="i-heroicons-sparkles"
                            class="w-8 h-8 text-primary-500"
                        />
                        <span
                            class="text-xl font-bold text-gray-900 dark:text-white"
                            >ZincBloom</span
                        >
                    </div>
                    <p class="text-gray-600 dark:text-gray-400 text-sm mb-4">
                        基于 Nuxt 3 和 Rust
                        构建的现代化博客系统，提供优雅的写作体验和阅读体验。
                    </p>
                    <div class="flex space-x-4">
                        <UButton
                            v-for="social in socialLinks"
                            :key="social.name"
                            :icon="social.icon"
                            variant="ghost"
                            color="neutral"
                            size="sm"
                            :to="social.url"
                            external
                            :aria-label="social.name"
                        />
                    </div>
                </div>

                <!-- 快速链接 -->
                <div>
                    <h3
                        class="text-sm font-semibold text-gray-900 dark:text-white uppercase tracking-wider mb-4"
                    >
                        快速链接
                    </h3>
                    <ul class="space-y-2">
                        <li v-for="link in quickLinks" :key="link.name">
                            <NuxtLink
                                :to="link.href"
                                class="text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white text-sm transition-colors"
                            >
                                {{ link.name }}
                            </NuxtLink>
                        </li>
                    </ul>
                </div>

                <!-- 分类链接 -->
                <div>
                    <h3
                        class="text-sm font-semibold text-gray-900 dark:text-white uppercase tracking-wider mb-4"
                    >
                        分类
                    </h3>
                    <ul class="space-y-2">
                        <li v-for="category in categories" :key="category.id">
                            <NuxtLink
                                :to="`/posts?category=${category.slug}`"
                                class="text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white text-sm transition-colors"
                            >
                                {{ category.name }}
                            </NuxtLink>
                        </li>
                    </ul>
                </div>

                <!-- 其他链接 -->
                <div>
                    <h3
                        class="text-sm font-semibold text-gray-900 dark:text-white uppercase tracking-wider mb-4"
                    >
                        其他
                    </h3>
                    <ul class="space-y-2">
                        <li v-for="link in otherLinks" :key="link.name">
                            <NuxtLink
                                :to="link.href"
                                class="text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white text-sm transition-colors"
                            >
                                {{ link.name }}
                            </NuxtLink>
                        </li>
                    </ul>
                </div>
            </div>

            <!-- 底部版权信息 -->
            <div class="py-6 border-t border-gray-200 dark:border-gray-800">
                <div
                    class="flex flex-col md:flex-row justify-between items-center space-y-4 md:space-y-0"
                >
                    <div class="text-sm text-gray-600 dark:text-gray-400">
                        © {{ currentYear }} ZincBloom. 保留所有权利。
                    </div>
                    <div class="flex items-center space-x-6">
                        <NuxtLink
                            to="/privacy"
                            class="text-sm text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white transition-colors"
                        >
                            隐私政策
                        </NuxtLink>
                        <NuxtLink
                            to="/terms"
                            class="text-sm text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white transition-colors"
                        >
                            服务条款
                        </NuxtLink>
                        <div class="text-sm text-gray-600 dark:text-gray-400">
                            由
                            <a
                                href="https://nuxt.com"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="text-green-600 hover:text-green-700 transition-colors"
                            >
                                Nuxt
                            </a>
                            ❤️
                            <a
                                href="https://www.rust-lang.org"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="text-orange-600 hover:text-orange-700 transition-colors"
                            >
                                Rust
                            </a>
                            驱动
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </footer>
</template>

<script setup lang="ts">
// 当前年份
const currentYear = new Date().getFullYear();

// 社交媒体链接
const socialLinks = [
    {
        name: "GitHub",
        icon: "i-simple-icons-github",
        url: "https://github.com",
    },
    {
        name: "Twitter",
        icon: "i-simple-icons-twitter",
        url: "https://twitter.com",
    },
    {
        name: "RSS",
        icon: "i-heroicons-rss",
        url: "/rss.xml",
    },
];

// 快速链接
const quickLinks = [
    { name: "首页", href: "/" },
    { name: "文章", href: "/posts" },
    { name: "标签", href: "/tags" },
    { name: "关于", href: "/about" },
    { name: "联系我们", href: "/contact" },
];

// 示例分类（后续从API获取）
const categories = ref([
    { id: "1", name: "技术分享", slug: "tech" },
    { id: "2", name: "生活随笔", slug: "life" },
    { id: "3", name: "学习笔记", slug: "notes" },
    { id: "4", name: "开源项目", slug: "opensource" },
]);

// 其他链接
const otherLinks = [
    { name: "网站地图", href: "/sitemap" },
    { name: "友情链接", href: "/links" },
    { name: "意见反馈", href: "/feedback" },
    { name: "帮助中心", href: "/help" },
];

// 未来：从 API 获取分类数据
// onMounted(async () => {
//   try {
//     const api = useApi()
//     const response = await api.getCategories({ per_page: 6 })
//     if (response.data) {
//       categories.value = response.data.data
//     }
//   } catch (error) {
//     console.error('获取分类失败:', error)
//   }
// })
</script>
