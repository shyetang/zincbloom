<template>
    <div class="container mx-auto px-4 py-8 max-w-4xl">
        <!-- 页面标题 -->
        <div class="text-center mb-12">
            <h1 class="text-4xl font-bold text-gray-900 dark:text-white mb-4">
                开始创作
            </h1>
            <p class="text-lg text-gray-600 dark:text-gray-400">
                分享您的想法，记录您的经历，与世界分享您的故事
            </p>
        </div>

        <!-- 创作选项 -->
        <div class="grid md:grid-cols-2 gap-8 mb-12">
            <!-- 新建文章 -->
            <div
                class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 p-8 hover:shadow-lg transition-shadow"
            >
                <div
                    class="flex items-center justify-center w-16 h-16 bg-primary-100 dark:bg-primary-900 rounded-full mb-6 mx-auto"
                >
                    <UIcon
                        name="i-heroicons-pencil-square"
                        class="w-8 h-8 text-primary-600 dark:text-primary-400"
                    />
                </div>
                <h3
                    class="text-xl font-semibold text-gray-900 dark:text-white text-center mb-4"
                >
                    写新文章
                </h3>
                <p class="text-gray-600 dark:text-gray-400 text-center mb-6">
                    创建一篇全新的文章，分享您的想法和见解
                </p>
                <UButton
                    @click="createNewPost"
                    color="primary"
                    size="lg"
                    block
                    :loading="creating"
                >
                    开始写作
                </UButton>
            </div>

            <!-- 草稿箱 -->
            <div
                class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 p-8 hover:shadow-lg transition-shadow"
            >
                <div
                    class="flex items-center justify-center w-16 h-16 bg-yellow-100 dark:bg-yellow-900 rounded-full mb-6 mx-auto"
                >
                    <UIcon
                        name="i-heroicons-document-text"
                        class="w-8 h-8 text-yellow-600 dark:text-yellow-400"
                    />
                </div>
                <h3
                    class="text-xl font-semibold text-gray-900 dark:text-white text-center mb-4"
                >
                    继续草稿
                </h3>
                <p class="text-gray-600 dark:text-gray-400 text-center mb-6">
                    继续编辑您未完成的草稿文章
                </p>
                <UButton
                    to="/user/posts?status=draft"
                    variant="outline"
                    size="lg"
                    block
                >
                    查看草稿 ({{ draftCount }})
                </UButton>
            </div>
        </div>

        <!-- 最近草稿 -->
        <div v-if="(recentDrafts as any)?.data?.length" class="mb-12">
            <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-6">
                最近的草稿
            </h2>
            <div class="space-y-4">
                <div
                    v-for="draft in (recentDrafts as any)?.data || []"
                    :key="draft.id"
                    class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6 hover:shadow-md transition-shadow"
                >
                    <div class="flex items-start justify-between">
                        <div class="flex-1">
                            <h3
                                class="text-lg font-semibold text-gray-900 dark:text-white mb-2"
                            >
                                {{ draft.title || "无标题草稿" }}
                            </h3>
                            <p
                                class="text-sm text-gray-600 dark:text-gray-400 mb-4"
                            >
                                {{ formatDate(draft.updated_at) }} 更新
                            </p>
                            <p
                                class="text-gray-600 dark:text-gray-400 line-clamp-2"
                            >
                                {{ getExcerpt(draft.content) }}
                            </p>
                        </div>
                        <div class="ml-4 flex-shrink-0">
                            <UButton
                                :to="`/write/${draft.id}`"
                                variant="outline"
                                size="sm"
                            >
                                继续编辑
                            </UButton>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <!-- 写作统计 -->
        <div
            class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 p-8"
        >
            <h2
                class="text-2xl font-bold text-gray-900 dark:text-white mb-6 text-center"
            >
                写作统计
            </h2>
            <div class="grid grid-cols-2 md:grid-cols-4 gap-6">
                <div class="text-center">
                    <div
                        class="text-3xl font-bold text-primary-600 dark:text-primary-400"
                    >
                        {{ (stats as any)?.totalPosts || 0 }}
                    </div>
                    <div class="text-sm text-gray-600 dark:text-gray-400">
                        总文章数
                    </div>
                </div>
                <div class="text-center">
                    <div
                        class="text-3xl font-bold text-green-600 dark:text-green-400"
                    >
                        {{ (stats as any)?.publishedPosts || 0 }}
                    </div>
                    <div class="text-sm text-gray-600 dark:text-gray-400">
                        已发布
                    </div>
                </div>
                <div class="text-center">
                    <div
                        class="text-3xl font-bold text-yellow-600 dark:text-yellow-400"
                    >
                        {{ (stats as any)?.draftPosts || 0 }}
                    </div>
                    <div class="text-sm text-gray-600 dark:text-gray-400">
                        草稿
                    </div>
                </div>
                <div class="text-center">
                    <div
                        class="text-3xl font-bold text-blue-600 dark:text-blue-400"
                    >
                        {{ (stats as any)?.totalWords || 0 }}
                    </div>
                    <div class="text-sm text-gray-600 dark:text-gray-400">
                        总字数
                    </div>
                </div>
            </div>
        </div>

        <!-- 写作技巧 -->
        <div
            class="mt-12 bg-gradient-to-r from-primary-50 to-blue-50 dark:from-primary-900/20 dark:to-blue-900/20 rounded-xl p-8"
        >
            <h2
                class="text-2xl font-bold text-gray-900 dark:text-white mb-6 text-center"
            >
                写作小贴士
            </h2>
            <div class="grid md:grid-cols-2 gap-6">
                <div class="flex items-start space-x-3">
                    <UIcon
                        name="i-heroicons-light-bulb"
                        class="w-6 h-6 text-yellow-500 mt-1"
                    />
                    <div>
                        <h3 class="font-semibold text-gray-900 dark:text-white">
                            明确主题
                        </h3>
                        <p class="text-sm text-gray-600 dark:text-gray-400">
                            在开始写作前，先确定文章的核心主题和要传达的信息。
                        </p>
                    </div>
                </div>
                <div class="flex items-start space-x-3">
                    <UIcon
                        name="i-heroicons-user-group"
                        class="w-6 h-6 text-blue-500 mt-1"
                    />
                    <div>
                        <h3 class="font-semibold text-gray-900 dark:text-white">
                            了解读者
                        </h3>
                        <p class="text-sm text-gray-600 dark:text-gray-400">
                            考虑您的目标读者，用他们能理解的语言写作。
                        </p>
                    </div>
                </div>
                <div class="flex items-start space-x-3">
                    <UIcon
                        name="i-heroicons-pencil"
                        class="w-6 h-6 text-green-500 mt-1"
                    />
                    <div>
                        <h3 class="font-semibold text-gray-900 dark:text-white">
                            结构清晰
                        </h3>
                        <p class="text-sm text-gray-600 dark:text-gray-400">
                            使用标题、段落和列表来组织内容，让文章易于阅读。
                        </p>
                    </div>
                </div>
                <div class="flex items-start space-x-3">
                    <UIcon
                        name="i-heroicons-arrow-path"
                        class="w-6 h-6 text-purple-500 mt-1"
                    />
                    <div>
                        <h3 class="font-semibold text-gray-900 dark:text-white">
                            反复修改
                        </h3>
                        <p class="text-sm text-gray-600 dark:text-gray-400">
                            好文章是改出来的，多次修改和完善您的内容。
                        </p>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
// 路由守卫
definePageMeta({
    middleware: "auth",
});

// SEO
useHead({
    title: "开始写作",
});

// 状态
const creating = ref(false);
const router = useRouter();
const toast = useToast();

// 获取草稿数据
const { data: recentDrafts } = await useLazyFetch("/api/posts", {
    query: {
        status: "draft",
        per_page: 3,
        sort: "updated_at",
        order: "desc",
        author: "me",
    },
    default: () => ({ data: [] as any[] }),
});

// 获取统计数据
const { data: stats } = await useLazyFetch("/api/posts/stats", {
    default: () =>
        ({
            totalPosts: 0,
            publishedPosts: 0,
            draftPosts: 0,
            totalWords: 0,
        } as any),
});

// 计算草稿数量
const draftCount = computed(() => (stats.value as any)?.draftPosts || 0);

// 方法
const createNewPost = async () => {
    creating.value = true;

    try {
        // 创建新的草稿文章
        const response = await $fetch("/api/posts", {
            method: "POST",
            body: {
                title: "",
                content: "",
                status: "draft",
            },
        });

        if ((response as any).success) {
            router.push(`/write/${(response as any).data.id}`);
        } else {
            throw new Error((response as any).error?.message || "创建失败");
        }
    } catch (error) {
        toast.add({
            title: "创建失败",
            description: "无法创建新文章，请稍后重试",
            color: "error",
        });
    } finally {
        creating.value = false;
    }
};

const getExcerpt = (content: string): string => {
    if (!content) return "空白草稿";
    const plainText = content.replace(/[#*`\[\]]/g, "").trim();
    return plainText.length > 100
        ? plainText.substring(0, 100) + "..."
        : plainText;
};

const formatDate = (dateString: string): string => {
    const date = new Date(dateString);
    const now = new Date();
    const diffTime = Math.abs(now.getTime() - date.getTime());
    const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24));

    if (diffDays === 1) {
        return "昨天";
    } else if (diffDays < 7) {
        return `${diffDays} 天前`;
    } else {
        return date.toLocaleDateString("zh-CN");
    }
};
</script>

<style scoped>
.line-clamp-2 {
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
}
</style>
