<template>
    <div class="container-blog py-8">
        <!-- 页面标题 -->
        <div class="mb-8">
            <h1
                class="text-3xl md:text-4xl font-bold text-gray-900 dark:text-white mb-4"
            >
                文章列表
            </h1>
            <p class="text-lg text-gray-600 dark:text-gray-400">
                探索我们精彩的文章内容
            </p>
        </div>

        <!-- 筛选和搜索区域 -->
        <div
            class="bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 p-6 mb-8"
        >
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                <!-- 搜索框 -->
                <UInput
                    v-model="searchQuery"
                    icon="i-heroicons-magnifying-glass"
                    placeholder="搜索文章..."
                    @keyup.enter="handleSearch"
                />

                <!-- 分类筛选 -->
                <USelectMenu
                    v-model="selectedCategory"
                    :options="categoryOptions"
                    placeholder="选择分类"
                    clear-search-on-close
                />

                <!-- 排序选择 -->
                <USelectMenu
                    v-model="sortOption"
                    :options="sortOptions"
                    placeholder="排序方式"
                />
            </div>

            <!-- 标签筛选 -->
            <div v-if="popularTags.length" class="mt-4">
                <div
                    class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
                >
                    热门标签：
                </div>
                <div class="flex flex-wrap gap-2">
                    <UBadge
                        v-for="tag in popularTags"
                        :key="tag.id"
                        :label="`#${tag.name}`"
                        :color="
                            selectedTags.includes(tag.id)
                                ? 'primary'
                                : 'neutral'
                        "
                        :variant="
                            selectedTags.includes(tag.id) ? 'solid' : 'soft'
                        "
                        class="cursor-pointer"
                        @click="toggleTag(tag.id)"
                    />
                </div>
            </div>

            <!-- 搜索按钮 -->
            <div class="mt-4 flex justify-end">
                <UButton
                    icon="i-heroicons-magnifying-glass"
                    @click="handleSearch"
                >
                    搜索
                </UButton>
            </div>
        </div>

        <!-- 文章列表 -->
        <div v-if="pending" class="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
            <USkeleton v-for="i in 9" :key="i" class="h-80 w-full" />
        </div>

        <div v-else-if="posts?.data?.length" class="space-y-8">
            <!-- 文章网格 -->
            <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
                <PostCard
                    v-for="post in posts.data"
                    :key="post.id"
                    :post="post"
                />
            </div>

            <!-- 分页组件 -->
            <div class="flex justify-center">
                <UPagination
                    v-model="currentPage"
                    :page-count="posts.pagination.per_page"
                    :total="posts.pagination.total"
                    :max="7"
                />
            </div>
        </div>

        <!-- 空状态 -->
        <div v-else class="text-center py-16">
            <UIcon
                name="i-heroicons-document-text"
                class="w-16 h-16 text-gray-400 mx-auto mb-4"
            />
            <h3 class="text-lg font-medium text-gray-900 dark:text-white mb-2">
                没有找到文章
            </h3>
            <p class="text-gray-600 dark:text-gray-400 mb-4">
                尝试调整搜索条件或
                <button
                    class="text-primary-600 hover:text-primary-700"
                    @click="clearFilters"
                >
                    清除筛选
                </button>
            </p>
            <UButton to="/write" icon="i-heroicons-pencil-square">
                写第一篇文章
            </UButton>
        </div>
    </div>
</template>

<script setup lang="ts">
import type { PostQueryParams, Category } from "~/types";
import type {
    PostsResponse,
    CategoriesResponse,
    TagsResponse,
} from "~/types/api";

// SEO 配置
useHead({
    title: "文章列表",
    meta: [{ name: "description", content: "浏览所有博客文章" }],
});

// 响应式数据
const route = useRoute();
const router = useRouter();

const searchQuery = ref((route.query.search as string) || "");
const selectedCategory = ref((route.query.category as string) || "");
const selectedTags = ref<string[]>([]);
const sortOption = ref((route.query.sort as string) || "created_at");
const currentPage = ref(Number(route.query.page) || 1);

// 查询参数
const queryParams = computed(
    (): PostQueryParams => ({
        page: currentPage.value,
        per_page: 9,
        search: searchQuery.value || undefined,
        category: selectedCategory.value || undefined,
        tag: selectedTags.value.length
            ? selectedTags.value.join(",")
            : undefined,
        sort: sortOption.value as PostQueryParams["sort"],
        order: "desc",
        status: "published",
    })
);

// 获取文章数据
const {
    data: posts,
    pending,
    refresh,
} = await useLazyFetch<PostsResponse>("/api/posts", {
    query: queryParams,
    default: () => ({
        data: [],
        pagination: { page: 1, per_page: 9, total: 0, total_pages: 0 },
    }),
});

// 获取分类选项
const { data: categories } = await useLazyFetch<CategoriesResponse>(
    "/api/categories",
    {
        default: () => ({
            data: [],
            pagination: { page: 1, per_page: 100, total: 0, total_pages: 0 },
        }),
    }
);

const categoryOptions = computed(() => [
    { label: "全部分类", value: "" },
    ...(categories.value?.data?.map((cat: Category) => ({
        label: cat.name,
        value: cat.slug,
    })) || []),
]);

// 获取热门标签
const { data: tagsData } = await useLazyFetch<TagsResponse>("/api/tags", {
    query: { per_page: 10, sort: "post_count", order: "desc" },
    default: () => ({
        data: [],
        pagination: { page: 1, per_page: 10, total: 0, total_pages: 0 },
    }),
});

const popularTags = computed(() => tagsData.value?.data || []);

// 排序选项
const sortOptions = [
    { label: "最新发布", value: "created_at" },
    { label: "最近更新", value: "updated_at" },
    { label: "标题排序", value: "title" },
];

// 方法
const handleSearch = () => {
    currentPage.value = 1;
    updateURL();
    refresh();
};

const toggleTag = (tagId: string) => {
    const index = selectedTags.value.indexOf(tagId);
    if (index > -1) {
        selectedTags.value.splice(index, 1);
    } else {
        selectedTags.value.push(tagId);
    }
    handleSearch();
};

const clearFilters = () => {
    searchQuery.value = "";
    selectedCategory.value = "";
    selectedTags.value = [];
    sortOption.value = "created_at";
    currentPage.value = 1;
    updateURL();
    refresh();
};

const updateURL = () => {
    const query: Record<string, string | number> = {};

    if (searchQuery.value) query.search = searchQuery.value;
    if (selectedCategory.value) query.category = selectedCategory.value;
    if (selectedTags.value.length) query.tags = selectedTags.value.join(",");
    if (sortOption.value !== "created_at") query.sort = sortOption.value;
    if (currentPage.value > 1) query.page = currentPage.value;

    router.push({ query });
};

// 监听分页变化
watch(currentPage, () => {
    updateURL();
    refresh();
});

// 监听排序变化
watch(sortOption, () => {
    currentPage.value = 1;
    updateURL();
    refresh();
});

// 监听分类变化
watch(selectedCategory, () => {
    currentPage.value = 1;
    updateURL();
    refresh();
});
</script>
