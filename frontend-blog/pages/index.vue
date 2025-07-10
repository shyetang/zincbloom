<template>
    <div>
        <!-- Hero 区域 -->
        <section
            class="bg-gradient-to-br from-primary-50 to-primary-100 dark:from-primary-900/20 dark:to-primary-800/20"
        >
            <div class="container-blog py-20">
                <div class="text-center">
                    <h1
                        class="text-4xl md:text-6xl font-bold text-gray-900 dark:text-white mb-6"
                    >
                        欢迎来到
                        <span class="text-primary-600 dark:text-primary-400"
                            >ZincBloom</span
                        >
                    </h1>
                    <p
                        class="text-xl text-gray-600 dark:text-gray-300 mb-8 max-w-2xl mx-auto"
                    >
                        一个基于 Nuxt 3 和 Rust
                        构建的现代化博客平台，为创作者和读者提供优雅的体验。
                    </p>
                    <div class="flex flex-col sm:flex-row gap-4 justify-center">
                        <UButton
                            to="/posts"
                            color="primary"
                            size="xl"
                            icon="i-heroicons-book-open"
                        >
                            浏览文章
                        </UButton>
                        <UButton
                            to="/write"
                            variant="outline"
                            color="primary"
                            size="xl"
                            icon="i-heroicons-pencil-square"
                        >
                            开始写作
                        </UButton>
                    </div>
                </div>
            </div>
        </section>

        <!-- 最新文章区域 -->
        <section class="py-16">
            <div class="container-blog">
                <div class="flex items-center justify-between mb-8">
                    <h2
                        class="text-3xl font-bold text-gray-900 dark:text-white"
                    >
                        最新文章
                    </h2>
                    <UButton
                        to="/posts"
                        variant="ghost"
                        color="primary"
                        trailing-icon="i-heroicons-arrow-right"
                    >
                        查看全部
                    </UButton>
                </div>

                <!-- 文章列表 -->
                <div
                    v-if="pending"
                    class="grid md:grid-cols-2 lg:grid-cols-3 gap-6"
                >
                    <USkeleton v-for="i in 6" :key="i" class="h-64 w-full" />
                </div>

                <div
                    v-else-if="Array.isArray(posts) && posts.length"
                    class="grid md:grid-cols-2 lg:grid-cols-3 gap-6"
                >
                    <PostCard
                        v-for="post in posts"
                        :key="post.id"
                        :post="post"
                    />
                </div>

                <div v-else class="text-center py-12">
                    <UIcon
                        name="i-heroicons-document-text"
                        class="w-16 h-16 text-gray-400 mx-auto mb-4"
                    />
                    <p class="text-gray-600 dark:text-gray-400">
                        暂无文章，
                        <NuxtLink
                            to="/write"
                            class="text-primary-600 hover:text-primary-700"
                        >
                            开始写作
                        </NuxtLink>
                        吧！
                    </p>
                </div>
            </div>
        </section>

        <!-- 特色区域 -->
        <section class="bg-gray-50 dark:bg-gray-800/50 py-16">
            <div class="container-blog">
                <div class="text-center mb-12">
                    <h2
                        class="text-3xl font-bold text-gray-900 dark:text-white mb-4"
                    >
                        为什么选择 ZincBloom？
                    </h2>
                    <p
                        class="text-lg text-gray-600 dark:text-gray-300 max-w-2xl mx-auto"
                    >
                        我们致力于提供最佳的博客写作和阅读体验
                    </p>
                </div>

                <div class="grid md:grid-cols-3 gap-8">
                    <div class="text-center">
                        <div
                            class="w-16 h-16 bg-primary-100 dark:bg-primary-900/50 rounded-full flex items-center justify-center mx-auto mb-4"
                        >
                            <UIcon
                                name="i-heroicons-rocket-launch"
                                class="w-8 h-8 text-primary-600"
                            />
                        </div>
                        <h3
                            class="text-xl font-semibold text-gray-900 dark:text-white mb-2"
                        >
                            高性能
                        </h3>
                        <p class="text-gray-600 dark:text-gray-400">
                            基于 Rust 后端和 Nuxt 3，提供闪电般的加载速度
                        </p>
                    </div>

                    <div class="text-center">
                        <div
                            class="w-16 h-16 bg-primary-100 dark:bg-primary-900/50 rounded-full flex items-center justify-center mx-auto mb-4"
                        >
                            <UIcon
                                name="i-heroicons-paint-brush"
                                class="w-8 h-8 text-primary-600"
                            />
                        </div>
                        <h3
                            class="text-xl font-semibold text-gray-900 dark:text-white mb-2"
                        >
                            优雅设计
                        </h3>
                        <p class="text-gray-600 dark:text-gray-400">
                            现代化的界面设计，支持深色模式，完美适配各种设备
                        </p>
                    </div>

                    <div class="text-center">
                        <div
                            class="w-16 h-16 bg-primary-100 dark:bg-primary-900/50 rounded-full flex items-center justify-center mx-auto mb-4"
                        >
                            <UIcon
                                name="i-heroicons-shield-check"
                                class="w-8 h-8 text-primary-600"
                            />
                        </div>
                        <h3
                            class="text-xl font-semibold text-gray-900 dark:text-white mb-2"
                        >
                            安全可靠
                        </h3>
                        <p class="text-gray-600 dark:text-gray-400">
                            完善的权限系统和数据保护，让您安心创作
                        </p>
                    </div>
                </div>
            </div>
        </section>
    </div>
</template>

<script setup lang="ts">
import type { Post } from '~/types';

// SEO 配置
useHead({
    title: "首页",
    meta: [
        {
            name: "description",
            content: "基于 Nuxt 3 和 Rust 构建的现代化博客系统",
        },
    ],
});

// 获取最新文章
const { data: posts, pending } = await useLazyFetch<Post>("/api/posts", {
    query: {
        per_page: 6,
        sort: "created_at",
        order: "desc",
        status: "published",
    }
});
</script>
