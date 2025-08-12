<template>
  <div>
    <!-- Modern Hero Section -->
    <section
      class="modern-hero"
      style="padding: 3rem 0 2rem 0;"
    >
      <div class="relative mx-auto max-w-6xl px-4 sm:px-6 lg:px-8">
        <div class="flex justify-between items-center">
          <!-- Left side - Title and description -->
          <div>
            <!-- Modern Icon -->
            <div style="display: inline-flex; align-items: center; gap: 1rem; margin-bottom: 1rem;">
              <div style="width: 3rem; height: 3rem; display: flex; align-items: center; justify-content: center; border-radius: 50%; background: linear-gradient(45deg, #3b82f6, #8b5cf6); box-shadow: 0 6px 24px rgba(59, 130, 246, 0.3);">
                <UIcon
                  name="i-heroicons-document-text"
                  style="width: 1.75rem; height: 1.75rem; color: white;"
                />
              </div>
              <h1
                style="font-size: 2.25rem; font-weight: 700; color: #1e293b; margin: 0;"
                class="dark:text-white"
              >
                我的文章
              </h1>
            </div>
            <p
              style="font-size: 1rem; color: #64748b; margin: 0;"
              class="dark:text-slate-300"
            >
              管理您的创作内容，让每一篇文章都能触达更多读者
            </p>
          </div>

          <!-- Right side - Action button -->
          <div>
            <a
              href="/write"
              class="modern-button modern-button-primary"
              style="gap: 0.5rem; padding: 0.75rem 1.5rem;"
            >
              <UIcon
                name="i-heroicons-pencil-square"
                style="width: 1.25rem; height: 1.25rem;"
              />
              写新文章
            </a>
          </div>
        </div>
      </div>
    </section>

    <!-- Main Content Section -->
    <section
      class="modern-content-section"
      style="background: linear-gradient(135deg, #f8fafc 0%, #e2e8f0 50%, #eff6ff 100%); padding: 2rem 0; margin: 0;"
    >
      <div class="mx-auto max-w-6xl px-4 sm:px-6 lg:px-8">
        <!-- 筛选和搜索 -->
        <div
          class="modern-content-section"
          style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 1.5rem; margin-bottom: 1.5rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);"
        >
          <div
            class="modern-section-header"
            style="margin-bottom: 1rem;"
          >
            <div>
              <div class="flex items-center gap-3 mb-2">
                <div
                  class="modern-gradient-icon"
                  style="background: linear-gradient(45deg, #f59e0b, #f97316); width: 2rem; height: 2rem;"
                >
                  <UIcon
                    name="i-heroicons-funnel"
                    class="h-4 w-4 text-white"
                  />
                </div>
                <h2
                  class="modern-section-title"
                  style="font-size: 1.125rem;"
                >
                  搜索和筛选
                </h2>
              </div>
            </div>
          </div>
          <div
            class="grid md:grid-cols-4 gap-4"
            style="padding: 0 1rem;"
          >
            <UInput
              v-model="searchQuery"
              placeholder="搜索文章..."
              icon="i-heroicons-magnifying-glass"
              class="modern-form-input"
              @keyup.enter="handleSearch"
            />
            <USelect
              v-model="statusFilter"
              :options="statusOptions"
              placeholder="选择状态"
              class="modern-form-input"
            />
            <USelect
              v-model="sortBy"
              :options="sortOptions"
              placeholder="排序方式"
              class="modern-form-input"
            />
            <UButton
              variant="outline"
              icon="i-heroicons-arrow-path"
              class="modern-button modern-button-secondary"
              style="padding: 0.75rem 1rem;"
              @click="resetFilters"
            >
              重置
            </UButton>
          </div>
        </div>

        <!-- 文章列表 -->
        <div
          v-if="pending"
          class="modern-content-section"
          style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 1.5rem; margin-bottom: 1rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);"
        >
          <div class="space-y-4">
            <USkeleton
              v-for="i in 5"
              :key="i"
              class="h-32 w-full"
            />
          </div>
        </div>

        <div
          v-else-if="(posts as any)?.data?.length"
          class="space-y-6"
        >
          <!-- 批量操作 -->
          <div
            v-if="selectedPosts.length > 0"
            class="modern-post-card"
            style="background: linear-gradient(135deg, #dbeafe 0%, #e0f2fe 100%); border: 2px solid #3b82f6; padding: 1.5rem;"
          >
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-3">
                <div
                  class="modern-gradient-icon"
                  style="background: linear-gradient(45deg, #3b82f6, #6366f1); width: 2.5rem; height: 2.5rem;"
                >
                  <UIcon
                    name="i-heroicons-check-circle"
                    class="h-5 w-5 text-white"
                  />
                </div>
                <span class="font-medium text-blue-700 dark:text-blue-300">
                  已选择 {{ selectedPosts.length }} 篇文章
                </span>
              </div>
              <div class="flex items-center space-x-3">
                <UButton
                  size="sm"
                  :loading="bulkActionLoading"
                  class="modern-button"
                  style="background: linear-gradient(45deg, #10b981, #06b6d4); color: white; padding: 0.5rem 1rem; border: none;"
                  @click="bulkPublish"
                >
                  <UIcon
                    name="i-heroicons-rocket-launch"
                    class="w-4 h-4 mr-1"
                  />
                  批量发布
                </UButton>
                <UButton
                  size="sm"
                  :loading="bulkActionLoading"
                  class="modern-button"
                  style="background: linear-gradient(45deg, #f59e0b, #f97316); color: white; padding: 0.5rem 1rem; border: none;"
                  @click="bulkDraft"
                >
                  <UIcon
                    name="i-heroicons-archive-box"
                    class="w-4 h-4 mr-1"
                  />
                  批量草稿
                </UButton>
                <UButton
                  size="sm"
                  :loading="bulkActionLoading"
                  class="modern-button"
                  style="background: linear-gradient(45deg, #ef4444, #f97316); color: white; padding: 0.5rem 1rem; border: none;"
                  @click="bulkDelete"
                >
                  <UIcon
                    name="i-heroicons-trash"
                    class="w-4 h-4 mr-1"
                  />
                  批量删除
                </UButton>
              </div>
            </div>
          </div>

          <!-- 文章项目 -->
          <div
            class="modern-grid-cards"
            style="grid-template-columns: 1fr;"
          >
            <div
              v-for="post in (posts as any)?.data || []"
              :key="post.id"
              class="modern-post-card"
              style="padding: 2rem;"
            >
              <div class="flex items-start space-x-6">
                <!-- 选择框 -->
                <div class="flex items-center justify-center mt-2">
                  <UCheckbox
                    :model-value="selectedPosts.includes(post.id)"
                    @update:model-value="togglePostSelection(post.id)"
                  />
                </div>

                <!-- 缩略图 -->
                <div class="flex-shrink-0">
                  <div
                    v-if="post.thumbnail"
                    class="w-24 h-24 rounded-xl overflow-hidden shadow-lg border-2 border-white dark:border-slate-600"
                  >
                    <img
                      :src="post.thumbnail"
                      :alt="post.title"
                      class="w-full h-full object-cover"
                    >
                  </div>
                  <div
                    v-else
                    class="w-24 h-24 rounded-xl flex items-center justify-center shadow-lg border-2 border-white dark:border-slate-600"
                    style="background: linear-gradient(135deg, #f1f5f9 0%, #e2e8f0 100%);"
                  >
                    <UIcon
                      name="i-heroicons-document-text"
                      class="w-8 h-8 text-slate-400"
                    />
                  </div>
                </div>

                <!-- 文章信息 -->
                <div class="flex-1 min-w-0">
                  <div class="flex items-start justify-between">
                    <div class="flex-1">
                      <h3 class="text-xl font-bold text-slate-900 dark:text-white mb-2 line-clamp-1">
                        {{ post.title }}
                      </h3>
                      <p class="text-sm text-slate-600 dark:text-slate-400 line-clamp-2 mb-3 leading-relaxed">
                        {{
                          post.excerpt
                            || getExcerpt(post.content_markdown || "")
                        }}
                      </p>
                      <div class="flex items-center space-x-4 text-xs text-slate-500 dark:text-slate-400 mb-3">
                        <div class="flex items-center gap-1">
                          <UIcon
                            name="i-heroicons-calendar"
                            class="w-3 h-3"
                          />
                          <span>创建：{{ formatDate(post.created_at) }}</span>
                        </div>
                        <div class="flex items-center gap-1">
                          <UIcon
                            name="i-heroicons-pencil"
                            class="w-3 h-3"
                          />
                          <span>更新：{{ formatDate(post.updated_at) }}</span>
                        </div>
                        <div
                          v-if="post.published_at"
                          class="flex items-center gap-1"
                        >
                          <UIcon
                            name="i-heroicons-rocket-launch"
                            class="w-3 h-3"
                          />
                          <span>发布：{{ formatDate(post.published_at) }}</span>
                        </div>
                      </div>
                    </div>

                    <!-- 状态和操作 -->
                    <div class="flex items-center space-x-3 ml-6">
                      <UBadge
                        :color="post.status === 'published' ? 'success' : 'warning'"
                        variant="soft"
                        size="lg"
                        class="px-3 py-1 font-medium"
                      >
                        {{ post.status === "published" ? "已发布" : "草稿" }}
                      </UBadge>

                      <UDropdown :items="getPostActions(post)">
                        <UButton
                          icon="i-heroicons-ellipsis-vertical"
                          variant="ghost"
                          size="sm"
                          class="hover:bg-slate-100 dark:hover:bg-slate-700 rounded-lg p-2"
                        />
                      </UDropdown>
                    </div>
                  </div>

                  <!-- 标签 -->
                  <div
                    v-if="post.tags?.length"
                    class="flex flex-wrap gap-2 mt-2"
                  >
                    <UBadge
                      v-for="tag in post.tags.slice(0, 3)"
                      :key="tag.id"
                      variant="soft"
                      size="sm"
                      class="px-2 py-1 bg-gradient-to-r from-blue-50 to-purple-50 text-blue-600 border border-blue-200 dark:from-blue-900/20 dark:to-purple-900/20 dark:text-blue-400 dark:border-blue-800"
                    >
                      {{ tag.name }}
                    </UBadge>
                    <UBadge
                      v-if="post.tags.length > 3"
                      variant="soft"
                      size="sm"
                      color="neutral"
                      class="px-2 py-1"
                    >
                      +{{ post.tags.length - 3 }}
                    </UBadge>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- 分页 -->
          <div class="flex justify-center mt-8">
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
          class="modern-content-section"
          style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 4rem 2rem; margin-bottom: 1rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05); text-align: center;"
        >
          <div
            class="modern-gradient-icon"
            style="width: 5rem; height: 5rem; margin: 0 auto 2rem; background: linear-gradient(45deg, #6b7280, #9ca3af);"
          >
            <UIcon
              name="i-heroicons-document-text"
              class="w-8 h-8 text-white"
            />
          </div>
          <h3 class="text-2xl font-bold text-slate-900 dark:text-white mb-3">
            还没有文章
          </h3>
          <p class="text-slate-600 dark:text-slate-400 mb-6 max-w-md mx-auto">
            开始您的创作之旅，分享您的想法和见解，让世界听到您的声音！
          </p>
          <a
            href="/write"
            class="modern-button modern-button-primary"
            style="gap: 0.5rem; padding: 1rem 2rem; font-size: 1rem;"
          >
            <UIcon
              name="i-heroicons-pencil-square"
              style="width: 1.25rem; height: 1.25rem;"
            />
            写第一篇文章
          </a>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import type { Post, PaginatedResponse } from "~/types";

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

// 获取文章数据
const posts = ref<{
  data: Post[];
  pagination: { page: number; per_page: number; total: number; total_pages: number };
}>({
  data: [],
  pagination: { page: 1, per_page: 10, total: 0, total_pages: 0 },
});
const pending = ref(false);

const fetchPosts = async () => {
  pending.value = true;
  try {
    // 构建查询字符串，使用后端期望的参数名
    const params = new URLSearchParams();
    if (currentPage.value) params.append("page", currentPage.value.toString());
    if (searchQuery.value) params.append("search", searchQuery.value);
    if (statusFilter.value) params.append("status", statusFilter.value);
    if (sortBy.value) params.append("sort", sortBy.value);
    params.append("order", "desc");
    params.append("page_size", "10"); // 后端使用 page_size 而不是 per_page

    const queryString = params.toString();
    const url = queryString ? `/posts?${queryString}` : "/posts";

    const response = await $fetch<PaginatedResponse<Post>>(`http://localhost:8080${url}`, {
      method: "GET",
      headers: {
        "Authorization": `Bearer ${localStorage.getItem("access_token")}`,
        "Content-Type": "application/json",
      },
    });

    // 后端返回 PaginatedResponse 结构，需要转换为前端期望的格式
    posts.value = {
      data: response.items || [],
      pagination: {
        page: response.page || 1,
        per_page: response.page_size || 10,
        total: response.total_items || 0,
        total_pages: response.total_pages || 0,
      },
    };
  }
  catch (error) {
    console.error("获取文章时发生错误:", error);
    toast.add({
      title: "获取文章失败",
      description: "无法获取文章列表，请稍后重试",
      color: "error",
    });
  }
  finally {
    pending.value = false;
  }
};

const refresh = fetchPosts;

// 初始化时获取数据
onMounted(() => {
  fetchPosts();
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
    const api = useApi();
    const newStatus = post.status === "published" ? "draft" : "published";

    const response = await api.updatePost(post.id, {
      status: newStatus,
    });

    if (response.success) {
      toast.add({
        title: "状态更新成功",
        description: `文章已${
          post.status === "published" ? "转为草稿" : "发布"
        }`,
        color: "success",
      });

      refresh();
    }
    else {
      throw new Error(response.error?.message || "更新失败");
    }
  }
  catch (error: unknown) {
    toast.add({
      title: "操作失败",
      description: error instanceof Error ? error.message : "更新文章状态时发生错误",
      color: "error",
    });
  }
};

const deletePost = async (post: Post) => {
  // 添加确认对话框
  const confirmed = confirm(`确定要删除文章"${post.title}"吗？此操作无法撤销。`);
  if (!confirmed) return;

  try {
    const api = useApi();
    const response = await api.deletePost(post.id);

    if (response.success) {
      toast.add({
        title: "删除成功",
        description: "文章已删除",
        color: "success",
      });

      refresh();
    }
    else {
      throw new Error(response.error?.message || "删除失败");
    }
  }
  catch (error: unknown) {
    toast.add({
      title: "删除失败",
      description: error instanceof Error ? error.message : "删除文章时发生错误",
      color: "error",
    });
  }
};

const bulkPublish = async () => {
  if (selectedPosts.value.length === 0) return;

  bulkActionLoading.value = true;
  try {
    const api = useApi();
    const response = await api.bulkUpdatePosts({
      post_ids: selectedPosts.value,
      action: "publish",
    });

    if (response.success) {
      toast.add({
        title: "批量发布成功",
        description: `已发布 ${selectedPosts.value.length} 篇文章`,
        color: "success",
      });

      selectedPosts.value = [];
      refresh();
    }
    else {
      throw new Error(response.error?.message || "批量操作失败");
    }
  }
  catch (error: unknown) {
    toast.add({
      title: "批量操作失败",
      description: error instanceof Error ? error.message : "批量发布时发生错误",
      color: "error",
    });
  }
  finally {
    bulkActionLoading.value = false;
  }
};

const bulkDraft = async () => {
  if (selectedPosts.value.length === 0) return;

  bulkActionLoading.value = true;
  try {
    const api = useApi();
    const response = await api.bulkUpdatePosts({
      post_ids: selectedPosts.value,
      action: "draft",
    });

    if (response.success) {
      toast.add({
        title: "批量操作成功",
        description: `已将 ${selectedPosts.value.length} 篇文章转为草稿`,
        color: "success",
      });

      selectedPosts.value = [];
      refresh();
    }
    else {
      throw new Error(response.error?.message || "批量操作失败");
    }
  }
  catch (error: unknown) {
    toast.add({
      title: "批量操作失败",
      description: error instanceof Error ? error.message : "批量转草稿时发生错误",
      color: "error",
    });
  }
  finally {
    bulkActionLoading.value = false;
  }
};

const bulkDelete = async () => {
  if (selectedPosts.value.length === 0) return;

  // 添加确认对话框
  const confirmed = confirm(`确定要删除选中的 ${selectedPosts.value.length} 篇文章吗？此操作无法撤销。`);
  if (!confirmed) return;

  bulkActionLoading.value = true;
  try {
    const api = useApi();
    const response = await api.bulkUpdatePosts({
      post_ids: selectedPosts.value,
      action: "delete",
    });

    if (response.success) {
      toast.add({
        title: "批量删除成功",
        description: `已删除 ${selectedPosts.value.length} 篇文章`,
        color: "success",
      });

      selectedPosts.value = [];
      refresh();
    }
    else {
      throw new Error(response.error?.message || "批量操作失败");
    }
  }
  catch (error: unknown) {
    toast.add({
      title: "批量操作失败",
      description: error instanceof Error ? error.message : "批量删除时发生错误",
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
    line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
}
</style>
