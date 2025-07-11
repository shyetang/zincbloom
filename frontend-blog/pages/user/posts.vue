<template>
  <div class="container mx-auto px-4 py-8 max-w-6xl">
    <!-- 页面标题和操作按钮 -->
    <div class="flex justify-between items-center mb-8">
      <div>
        <h1 class="text-3xl font-bold text-gray-900 dark:text-white">
          我的文章
        </h1>
        <p class="mt-2 text-gray-600 dark:text-gray-400">
          管理您的文章内容
        </p>
      </div>
      <UButton
        to="/write"
        color="primary"
        icon="i-heroicons-pencil-square"
      >
        写新文章
      </UButton>
    </div>

    <!-- 筛选和搜索 -->
    <div
      class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6 mb-8"
    >
      <div class="grid md:grid-cols-4 gap-4">
        <UInput
          v-model="searchQuery"
          placeholder="搜索文章..."
          icon="i-heroicons-magnifying-glass"
          @keyup.enter="handleSearch"
        />
        <USelect
          v-model="statusFilter"
          :options="statusOptions"
          placeholder="选择状态"
        />
        <USelect
          v-model="sortBy"
          :options="sortOptions"
          placeholder="排序方式"
        />
        <UButton
          variant="outline"
          color="neutral"
          icon="i-heroicons-arrow-path"
          @click="resetFilters"
        >
          重置
        </UButton>
      </div>
    </div>

    <!-- 文章列表 -->
    <div
      v-if="pending"
      class="space-y-4"
    >
      <USkeleton
        v-for="i in 5"
        :key="i"
        class="h-24 w-full"
      />
    </div>

    <div
      v-else-if="(posts as any)?.data?.length"
      class="space-y-6"
    >
      <!-- 批量操作 -->
      <div
        v-if="selectedPosts.length > 0"
        class="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-4"
      >
        <div class="flex items-center justify-between">
          <span class="text-sm text-blue-700 dark:text-blue-300">
            已选择 {{ selectedPosts.length }} 篇文章
          </span>
          <div class="flex items-center space-x-2">
            <UButton
              size="sm"
              color="success"
              variant="outline"
              :loading="bulkActionLoading"
              @click="bulkPublish"
            >
              批量发布
            </UButton>
            <UButton
              size="sm"
              color="warning"
              variant="outline"
              :loading="bulkActionLoading"
              @click="bulkDraft"
            >
              批量草稿
            </UButton>
            <UButton
              size="sm"
              color="error"
              variant="outline"
              :loading="bulkActionLoading"
              @click="bulkDelete"
            >
              批量删除
            </UButton>
          </div>
        </div>
      </div>

      <!-- 文章项目 -->
      <div class="space-y-4">
        <div
          v-for="post in (posts as any)?.data || []"
          :key="post.id"
          class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6"
        >
          <div class="flex items-start space-x-4">
            <!-- 选择框 -->
            <UCheckbox
              :model-value="selectedPosts.includes(post.id)"
              class="mt-1"
              @update:model-value="togglePostSelection(post.id)"
            />

            <!-- 缩略图 -->
            <div
              v-if="post.thumbnail"
              class="flex-shrink-0"
            >
              <img
                :src="post.thumbnail"
                :alt="post.title"
                class="w-20 h-20 object-cover rounded-lg"
              >
            </div>
            <div
              v-else
              class="flex-shrink-0"
            >
              <div
                class="w-20 h-20 bg-gray-200 dark:bg-gray-700 rounded-lg flex items-center justify-center"
              >
                <UIcon
                  name="i-heroicons-document-text"
                  class="w-8 h-8 text-gray-400"
                />
              </div>
            </div>

            <!-- 文章信息 -->
            <div class="flex-1 min-w-0">
              <div class="flex items-start justify-between">
                <div class="flex-1">
                  <h3
                    class="text-lg font-semibold text-gray-900 dark:text-white truncate"
                  >
                    {{ post.title }}
                  </h3>
                  <p
                    class="mt-1 text-sm text-gray-600 dark:text-gray-400 line-clamp-2"
                  >
                    {{
                      post.excerpt
                        || getExcerpt(post.content)
                    }}
                  </p>
                  <div
                    class="mt-2 flex items-center space-x-4 text-xs text-gray-500 dark:text-gray-400"
                  >
                    <span>{{
                      formatDate(post.created_at)
                    }}</span>
                    <span>{{
                      formatDate(post.updated_at)
                    }}</span>
                    <span v-if="post.published_at">发布于
                      {{
                        formatDate(post.published_at)
                      }}</span>
                  </div>
                </div>

                <!-- 状态和操作 -->
                <div class="flex items-center space-x-3 ml-4">
                  <UBadge
                    :color="
                      post.status === 'published'
                        ? 'success'
                        : 'warning'
                    "
                    variant="soft"
                  >
                    {{
                      post.status === "published"
                        ? "已发布"
                        : "草稿"
                    }}
                  </UBadge>

                  <UDropdown :items="getPostActions(post)">
                    <UButton
                      icon="i-heroicons-ellipsis-vertical"
                      variant="ghost"
                      color="neutral"
                      size="sm"
                    />
                  </UDropdown>
                </div>
              </div>

              <!-- 标签 -->
              <div
                v-if="post.tags?.length"
                class="mt-3 flex flex-wrap gap-1"
              >
                <UBadge
                  v-for="tag in post.tags.slice(0, 3)"
                  :key="tag.id"
                  variant="soft"
                  size="xs"
                >
                  {{ tag.name }}
                </UBadge>
                <UBadge
                  v-if="post.tags.length > 3"
                  variant="soft"
                  size="xs"
                  color="neutral"
                >
                  +{{ post.tags.length - 3 }}
                </UBadge>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 分页 -->
      <div class="flex justify-center">
        <UPagination
          v-model="currentPage"
          :page-count="(posts as any)?.pagination?.per_page || 10"
          :total="(posts as any)?.pagination?.total || 0"
          :max="7"
        />
      </div>
    </div>

    <!-- 空状态 -->
    <div
      v-else
      class="text-center py-16"
    >
      <UIcon
        name="i-heroicons-document-text"
        class="w-16 h-16 text-gray-400 mx-auto mb-4"
      />
      <h3 class="text-lg font-medium text-gray-900 dark:text-white mb-2">
        还没有文章
      </h3>
      <p class="text-gray-600 dark:text-gray-400 mb-4">
        开始您的创作之旅，写第一篇文章吧！
      </p>
      <UButton
        to="/write"
        color="primary"
        icon="i-heroicons-pencil-square"
      >
        写第一篇文章
      </UButton>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Post } from "~/types";

// 路由守卫
definePageMeta({
  middleware: "auth",
});

// SEO
useHead({
  title: "我的文章",
});

// 状态
const searchQuery = ref("");
const statusFilter = ref("");
const sortBy = ref("created_at");
const currentPage = ref(1);
const selectedPosts = ref<string[]>([]);
const bulkActionLoading = ref(false);

const router = useRouter();
const toast = useToast();

// 选项
const statusOptions = [
  { label: "全部状态", value: "" },
  { label: "已发布", value: "published" },
  { label: "草稿", value: "draft" },
];

const sortOptions = [
  { label: "创建时间", value: "created_at" },
  { label: "更新时间", value: "updated_at" },
  { label: "发布时间", value: "published_at" },
  { label: "标题", value: "title" },
];

// 查询参数
const queryParams = computed(() => ({
  page: currentPage.value,
  per_page: 10,
  search: searchQuery.value || undefined,
  status: statusFilter.value || undefined,
  sort: sortBy.value,
  order: "desc",
  author: "me", // 只获取当前用户的文章
}));

// 获取文章数据
const {
  data: posts,
  pending,
  refresh,
} = await useLazyFetch("/api/posts", {
  query: queryParams,
  default: () => ({
    data: [],
    pagination: { page: 1, per_page: 10, total: 0, total_pages: 0 },
  }),
});

// 方法
const handleSearch = () => {
  currentPage.value = 1;
  refresh();
};

const resetFilters = () => {
  searchQuery.value = "";
  statusFilter.value = "";
  sortBy.value = "created_at";
  currentPage.value = 1;
  refresh();
};

const togglePostSelection = (postId: string) => {
  const index = selectedPosts.value.indexOf(postId);
  if (index > -1) {
    selectedPosts.value.splice(index, 1);
  }
  else {
    selectedPosts.value.push(postId);
  }
};

const getPostActions = (post: Post) => [
  [
    {
      label: "查看",
      icon: "i-heroicons-eye",
      click: () => router.push(`/posts/${post.slug}`),
    },
    {
      label: "编辑",
      icon: "i-heroicons-pencil",
      click: () => router.push(`/write/${post.id}`),
    },
  ],
  [
    {
      label: post.status === "published" ? "转为草稿" : "发布",
      icon:
                post.status === "published"
                  ? "i-heroicons-archive-box"
                  : "i-heroicons-rocket-launch",
      click: () => togglePostStatus(post),
    },
  ],
  [
    {
      label: "删除",
      icon: "i-heroicons-trash",
      click: () => deletePost(post),
      class: "text-red-600 dark:text-red-400",
    },
  ],
];

const togglePostStatus = async (post: Post) => {
  try {
    // TODO: 调用API切换文章状态
    await new Promise(resolve => setTimeout(resolve, 1000));

    toast.add({
      title: "状态更新成功",
      description: `文章已${
        post.status === "published" ? "转为草稿" : "发布"
      }`,
      color: "success",
    });

    refresh();
  }
  catch (error) {
    toast.add({
      title: "操作失败",
      description: "更新文章状态时发生错误",
      color: "error",
    });
  }
};

const deletePost = async (post: Post) => {
  // TODO: 添加确认对话框
  try {
    // TODO: 调用API删除文章
    await new Promise(resolve => setTimeout(resolve, 1000));

    toast.add({
      title: "删除成功",
      description: "文章已删除",
      color: "success",
    });

    refresh();
  }
  catch (error) {
    toast.add({
      title: "删除失败",
      description: "删除文章时发生错误",
      color: "error",
    });
  }
};

const bulkPublish = async () => {
  bulkActionLoading.value = true;
  try {
    // TODO: 调用API批量发布
    await new Promise(resolve => setTimeout(resolve, 1000));

    toast.add({
      title: "批量发布成功",
      color: "success",
    });

    selectedPosts.value = [];
    refresh();
  }
  catch (error) {
    toast.add({
      title: "批量操作失败",
      color: "error",
    });
  }
  finally {
    bulkActionLoading.value = false;
  }
};

const bulkDraft = async () => {
  bulkActionLoading.value = true;
  try {
    // TODO: 调用API批量转为草稿
    await new Promise(resolve => setTimeout(resolve, 1000));

    toast.add({
      title: "批量操作成功",
      color: "success",
    });

    selectedPosts.value = [];
    refresh();
  }
  catch (error) {
    toast.add({
      title: "批量操作失败",
      color: "error",
    });
  }
  finally {
    bulkActionLoading.value = false;
  }
};

const bulkDelete = async () => {
  // TODO: 添加确认对话框
  bulkActionLoading.value = true;
  try {
    // TODO: 调用API批量删除
    await new Promise(resolve => setTimeout(resolve, 1000));

    toast.add({
      title: "批量删除成功",
      color: "success",
    });

    selectedPosts.value = [];
    refresh();
  }
  catch (error) {
    toast.add({
      title: "批量操作失败",
      color: "error",
    });
  }
  finally {
    bulkActionLoading.value = false;
  }
};

const getExcerpt = (content: string): string => {
  return content.substring(0, 150) + "...";
};

const formatDate = (dateString: string): string => {
  return new Date(dateString).toLocaleDateString("zh-CN");
};

// 监听分页变化
watch(currentPage, () => {
  refresh();
});

// 监听筛选条件变化
watch([statusFilter, sortBy], () => {
  currentPage.value = 1;
  refresh();
});
</script>

<style scoped>
.line-clamp-2 {
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
}
</style>
