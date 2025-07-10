<template>
    <div
        class="min-h-screen flex items-center justify-center bg-gray-50 dark:bg-gray-900"
    >
        <div class="max-w-lg w-full text-center px-4">
            <!-- 错误图标 -->
            <div class="mb-8">
                <UIcon
                    :name="errorIcon"
                    class="w-24 h-24 text-gray-400 mx-auto mb-6"
                />
                <h1
                    class="text-6xl font-bold text-gray-900 dark:text-white mb-4"
                >
                    {{ error?.statusCode || "错误" }}
                </h1>
                <h2
                    class="text-2xl font-semibold text-gray-700 dark:text-gray-300 mb-2"
                >
                    {{ errorTitle }}
                </h2>
                <p class="text-gray-600 dark:text-gray-400 mb-8">
                    {{ errorMessage }}
                </p>
            </div>

            <!-- 操作按钮 -->
            <div class="space-y-4">
                <UButton size="lg" color="primary" @click="handleError">
                    {{ isNotFound ? "返回首页" : "重新加载" }}
                </UButton>

                <div v-if="isNotFound" class="space-x-4">
                    <UButton variant="ghost" color="neutral" to="/posts">
                        浏览文章
                    </UButton>
                    <UButton variant="ghost" color="neutral" to="/write">
                        开始写作
                    </UButton>
                </div>
            </div>

            <!-- 搜索建议 (仅404错误) -->
            <div v-if="isNotFound" class="mt-12">
                <div class="max-w-md mx-auto">
                    <p class="text-sm text-gray-600 dark:text-gray-400 mb-4">
                        没有找到您要访问的页面，试试搜索：
                    </p>
                    <UInput
                        v-model="searchQuery"
                        icon="i-heroicons-magnifying-glass"
                        placeholder="搜索文章..."
                        size="lg"
                        @keyup.enter="handleSearch"
                    />
                </div>
            </div>

            <!-- 开发环境错误详情 -->
            <div v-if="isDevelopment && error?.stack" class="mt-12 text-left">
                <details class="bg-gray-100 dark:bg-gray-800 rounded-lg p-4">
                    <summary
                        class="cursor-pointer text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
                    >
                        错误详情 (开发环境)
                    </summary>
                    <pre
                        class="text-xs text-gray-600 dark:text-gray-400 overflow-auto"
                        >{{ error.stack }}</pre
                    >
                </details>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
// 禁用默认布局
definePageMeta({
    layout: false,
});

interface ErrorProps {
    error: {
        statusCode?: number;
        statusMessage?: string;
        message?: string;
        stack?: string;
        data?: any;
    };
}

const props = defineProps<ErrorProps>();

const router = useRouter();
const searchQuery = ref("");

// 判断错误类型
const isNotFound = computed(() => props.error?.statusCode === 404);
const isDevelopment = computed(() => process.dev);

// 错误图标
const errorIcon = computed(() => {
    switch (props.error?.statusCode) {
        case 404:
            return "i-heroicons-document-magnifying-glass";
        case 403:
            return "i-heroicons-lock-closed";
        case 500:
            return "i-heroicons-exclamation-triangle";
        default:
            return "i-heroicons-x-circle";
    }
});

// 错误标题
const errorTitle = computed(() => {
    switch (props.error?.statusCode) {
        case 404:
            return "页面未找到";
        case 403:
            return "访问被拒绝";
        case 500:
            return "服务器错误";
        default:
            return "出现错误";
    }
});

// 错误消息
const errorMessage = computed(() => {
    if (props.error?.statusMessage) {
        return props.error.statusMessage;
    }

    switch (props.error?.statusCode) {
        case 404:
            return "您访问的页面不存在，可能已被删除或移动。";
        case 403:
            return "您没有权限访问此页面。";
        case 500:
            return "服务器遇到了一个错误，请稍后重试。";
        default:
            return props.error?.message || "发生了未知错误。";
    }
});

// SEO 配置
useHead({
    title: `${props.error?.statusCode || "错误"} - ${errorTitle.value}`,
    meta: [{ name: "description", content: errorMessage.value }],
});

// 处理错误
const handleError = () => {
    if (isNotFound.value) {
        // 404错误返回首页
        router.push("/");
    } else {
        // 其他错误重新加载页面
        clearError({ redirect: "/" });
    }
};

// 处理搜索
const handleSearch = () => {
    if (searchQuery.value.trim()) {
        router.push(`/search?q=${encodeURIComponent(searchQuery.value)}`);
    }
};
</script>
