<template>
  <div>
    <!-- Hero 区域 -->
    <section
      class="modern-hero"
      style="padding: 2rem 0 1.5rem 0;"
    >
      <div class="relative mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
        <div style="text-align: center;">
          <!-- 现代化图标 -->
          <div style="margin: 0 auto 1rem; width: 2.5rem; height: 2.5rem; display: flex; align-items: center; justify-content: center; border-radius: 50%; background: linear-gradient(45deg, #ec4899, #f43f5e); box-shadow: 0 6px 24px rgba(236, 72, 153, 0.3);">
            <UIcon
              name="i-heroicons-heart"
              style="width: 1.25rem; height: 1.25rem; color: white;"
            />
          </div>

          <!-- 现代化标题 -->
          <h1
            style="font-size: 1.875rem; font-weight: 700; line-height: 1.1; margin-bottom: 0.75rem; color: #1e293b;"
            class="dark:text-white"
          >
            我的
            <span
              class="modern-text-gradient"
              style="font-weight: 800; background: linear-gradient(45deg, #ec4899, #f43f5e); -webkit-background-clip: text; -webkit-text-fill-color: transparent;"
            >收藏</span>
          </h1>

          <!-- 副标题 -->
          <p
            style="font-size: 1rem; color: #64748b; max-width: 28rem; margin: 0 auto; line-height: 1.5;"
            class="dark:text-slate-300"
          >
            管理您收藏的精彩文章，随时找到感兴趣的内容
          </p>
        </div>
      </div>
    </section>

    <!-- 面包屑导航 -->
    <nav style="background: rgba(248, 250, 252, 0.8); border-bottom: 1px solid rgba(226, 232, 240, 0.6); backdrop-filter: blur(10px);">
      <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
        <div style="display: flex; align-items: center; gap: 0.5rem; padding: 0.75rem 0; font-size: 0.875rem;">
          <NuxtLink
            to="/"
            style="display: flex; align-items: center; gap: 0.25rem; color: #64748b; text-decoration: none; transition: all 0.2s ease;"
            class="hover:text-blue-600"
          >
            <UIcon
              name="i-heroicons-home"
              class="w-4 h-4"
            />
            首页
          </NuxtLink>
          <UIcon
            name="i-heroicons-chevron-right"
            class="w-4 h-4 text-slate-400"
          />
          <NuxtLink
            to="/user/profile"
            style="color: #64748b; text-decoration: none; transition: all 0.2s ease;"
            class="hover:text-blue-600"
          >
            用户中心
          </NuxtLink>
          <UIcon
            name="i-heroicons-chevron-right"
            class="w-4 h-4 text-slate-400"
          />
          <span style="color: #3b82f6; font-weight: 500;">我的收藏</span>
        </div>
      </div>
    </nav>

    <!-- 主要内容区域 -->
    <section
      style="background: linear-gradient(135deg, #f8fafc 0%, #e2e8f0 50%, #fdf2f8 100%); padding: 2rem 0; margin: 0; position: relative; min-height: 60vh;"
    >
      <!-- 背景装饰 -->
      <div style="position: absolute; inset: 0; background-image: radial-gradient(circle at 1px 1px, rgba(236, 72, 153, 0.1) 1px, transparent 0); background-size: 60px 60px; opacity: 0.5;" />

      <div class="relative mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
        <!-- 收藏统计和筛选 -->
        <div
          class="modern-content-section mb-6"
          style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 1.5rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);"
        >
          <div class="modern-section-header">
            <div>
              <div class="flex items-center gap-3 mb-2">
                <div
                  class="modern-gradient-icon"
                  style="background: linear-gradient(45deg, #ec4899, #f43f5e); width: 2.5rem; height: 2.5rem;"
                >
                  <UIcon
                    name="i-heroicons-heart"
                    class="h-5 w-5 text-white"
                  />
                </div>
                <h2 class="modern-section-title">
                  收藏管理
                </h2>
              </div>
              <p class="modern-section-subtitle">
                共收藏了 {{ totalFavorites }} 篇精彩文章
              </p>
            </div>

            <!-- 筛选和排序选项 -->
            <div class="flex items-center gap-3">
              <!-- 分类筛选 -->
              <USelectMenu
                v-model="selectedCategory"
                :options="categoryOptions"
                size="sm"
                placeholder="筛选分类"
                style="min-width: 120px; background: rgba(248, 250, 252, 0.8); border: 1px solid rgba(226, 232, 240, 0.6);"
              />

              <!-- 排序选项 -->
              <USelectMenu
                v-model="sortOption"
                :options="sortOptions"
                size="sm"
                style="min-width: 120px; background: rgba(248, 250, 252, 0.8); border: 1px solid rgba(226, 232, 240, 0.6);"
              />

              <!-- 批量操作 -->
              <UDropdown
                v-if="selectedFavorites.length > 0"
                :items="batchActions"
                :popper="{ placement: 'bottom-start' }"
              >
                <UButton
                  icon="i-heroicons-ellipsis-horizontal"
                  size="sm"
                  color="neutral"
                  variant="ghost"
                >
                  批量操作 ({{ selectedFavorites.length }})
                </UButton>
              </UDropdown>
            </div>
          </div>
        </div>

        <!-- 收藏列表 -->
        <div
          class="modern-content-section"
          style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 1.5rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);"
        >
          <!-- 加载状态 -->
          <div
            v-if="pending"
            class="space-y-4"
          >
            <div
              v-for="i in 5"
              :key="i"
              class="flex items-start gap-4 p-4 border border-slate-200 rounded-lg"
            >
              <USkeleton class="w-4 h-4 rounded" />
              <div class="flex-1 space-y-2">
                <USkeleton class="h-5 w-3/4" />
                <USkeleton class="h-4 w-full" />
                <USkeleton class="h-4 w-1/2" />
              </div>
              <USkeleton class="w-24 h-8" />
            </div>
          </div>

          <!-- 收藏文章列表 -->
          <div
            v-else-if="filteredFavorites.length"
            class="space-y-4"
          >
            <div
              v-for="favorite in filteredFavorites"
              :key="favorite.id"
              class="flex items-start gap-4 p-4 border border-slate-200 hover:border-pink-300 hover:shadow-md rounded-lg transition-all cursor-pointer group"
              style="background: rgba(248, 250, 252, 0.5);"
            >
              <!-- 选择框 -->
              <UCheckbox
                :model-value="selectedFavorites.includes(favorite.id)"
                class="mt-1"
                @update:model-value="toggleFavoriteSelection(favorite.id)"
              />

              <!-- 文章信息 -->
              <div
                class="flex-1"
                @click="$router.push(`/posts/${favorite.post.slug}`)"
              >
                <div class="flex items-start justify-between mb-2">
                  <h3 class="font-medium text-slate-900 dark:text-white group-hover:text-pink-600 transition-colors line-clamp-1">
                    {{ favorite.post.title }}
                  </h3>
                  <span class="text-xs text-slate-500 dark:text-slate-400 ml-4 whitespace-nowrap">
                    {{ formatDate(favorite.created_at) }}
                  </span>
                </div>

                <p
                  v-if="favorite.post.excerpt"
                  class="text-sm text-slate-600 dark:text-slate-400 mb-3 line-clamp-2"
                >
                  {{ favorite.post.excerpt }}
                </p>

                <div class="flex items-center gap-4 text-xs text-slate-500 dark:text-slate-400">
                  <div class="flex items-center gap-1">
                    <UIcon
                      name="i-heroicons-user"
                      class="w-3 h-3"
                    />
                    <span>{{ favorite.post.author.username }}</span>
                  </div>

                  <div
                    v-if="favorite.post.categories.length"
                    class="flex items-center gap-1"
                  >
                    <UIcon
                      name="i-heroicons-squares-2x2"
                      class="w-3 h-3"
                    />
                    <span>{{ favorite.post.categories.map(c => c.name).join(", ") }}</span>
                  </div>

                  <div
                    v-if="favorite.post.tags.length"
                    class="flex items-center gap-1"
                  >
                    <UIcon
                      name="i-heroicons-hashtag"
                      class="w-3 h-3"
                    />
                    <span>{{ favorite.post.tags.slice(0, 3).map(t => t.name).join(", ") }}</span>
                  </div>
                </div>
              </div>

              <!-- 操作按钮 -->
              <div class="flex flex-col gap-2">
                <UButton
                  icon="i-heroicons-heart"
                  size="xs"
                  color="error"
                  variant="ghost"
                  :loading="removingFavorite === favorite.id"
                  @click.stop="removeFavorite(favorite.id)"
                >
                  取消收藏
                </UButton>

                <UButton
                  icon="i-heroicons-share"
                  size="xs"
                  color="neutral"
                  variant="ghost"
                  @click.stop="sharePost(favorite.post)"
                >
                  分享
                </UButton>
              </div>
            </div>
          </div>

          <!-- 空状态 -->
          <div
            v-else
            style="text-align: center; padding: 4rem 0;"
          >
            <div style="margin: 0 auto 2rem; width: 4rem; height: 4rem; display: flex; align-items: center; justify-content: center; border-radius: 50%; background: linear-gradient(45deg, #ec4899, #f43f5e); box-shadow: 0 8px 32px rgba(236, 72, 153, 0.2);">
              <UIcon
                name="i-heroicons-heart"
                style="width: 2rem; height: 2rem; color: white;"
              />
            </div>
            <h3
              style="font-size: 1.5rem; font-weight: 700; color: #1e293b; margin-bottom: 1rem;"
              class="dark:text-white"
            >
              还没有收藏文章
            </h3>
            <p
              style="font-size: 1rem; color: #64748b; margin-bottom: 2rem; line-height: 1.6;"
              class="dark:text-slate-300"
            >
              浏览文章时点击收藏按钮，将精彩内容保存到这里
            </p>
            <div style="display: flex; gap: 1rem; flex-wrap: wrap; justify-content: center;">
              <NuxtLink
                to="/posts"
                style="display: inline-flex; align-items: center; gap: 0.5rem; padding: 0.75rem 2rem; background: linear-gradient(45deg, #ec4899, #f43f5e); color: white; text-decoration: none; border-radius: 2rem; font-size: 0.875rem; font-weight: 600; transition: all 0.3s ease; box-shadow: 0 4px 12px rgba(236, 72, 153, 0.3);"
                class="hover:transform hover:scale-105 hover:shadow-lg"
              >
                <UIcon
                  name="i-heroicons-document-text"
                  class="w-4 h-4"
                />
                浏览文章
              </NuxtLink>
              <NuxtLink
                to="/"
                style="display: inline-flex; align-items: center; gap: 0.5rem; padding: 0.75rem 1.5rem; background: rgba(255, 255, 255, 0.9); color: #64748b; text-decoration: none; border-radius: 2rem; font-size: 0.875rem; font-weight: 500; border: 1px solid rgba(226, 232, 240, 0.6); transition: all 0.3s ease;"
                class="hover:bg-white hover:border-slate-300 hover:transform hover:scale-105"
              >
                <UIcon
                  name="i-heroicons-home"
                  class="w-4 h-4"
                />
                返回首页
              </NuxtLink>
            </div>
          </div>
        </div>

        <!-- 分页 -->
        <div
          v-if="totalPages > 1"
          style="display: flex; justify-content: center; margin-top: 2rem;"
        >
          <UPagination
            v-model="currentPage"
            :page-count="pageSize"
            :total="totalFavorites"
            :max="7"
            style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 1rem; border: 1px solid rgba(226, 232, 240, 0.6); box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);"
          />
        </div>
      </div>
    </section>

    <!-- 分享对话框 -->
    <UModal v-model="shareDialogOpen">
      <div class="p-6">
        <h3 class="text-lg font-semibold text-slate-900 dark:text-white mb-4">
          分享文章
        </h3>

        <div class="space-y-4">
          <div>
            <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
              文章链接
            </label>
            <div class="flex gap-2">
              <UInput
                v-model="shareUrl"
                readonly
                class="flex-1"
              />
              <UButton
                icon="i-heroicons-clipboard-document"
                @click="copyToClipboard"
              >
                复制
              </UButton>
            </div>
          </div>

          <div class="flex justify-end gap-3">
            <UButton
              variant="ghost"
              @click="shareDialogOpen = false"
            >
              取消
            </UButton>
          </div>
        </div>
      </div>
    </UModal>
  </div>
</template>

<script setup lang="ts">
import type { PostWithContent } from "~/types";

// 路由守卫
definePageMeta({
  middleware: "auth",
});

// SEO 配置
useHead({
  title: "我的收藏",
  meta: [{ name: "description", content: "管理您收藏的精彩文章，随时找到感兴趣的内容" }],
});

// 状态管理
const currentPage = ref(1);
const pageSize = ref(10);
const selectedCategory = ref("");
const sortOption = ref("created_at");
const selectedFavorites = ref<string[]>([]);
const removingFavorite = ref<string | null>(null);
const shareDialogOpen = ref(false);
const shareUrl = ref("");

// API 客户端
const api = useApi();
const toast = useToast();

// 模拟收藏数据（实际应该从 API 获取）
interface FavoriteItem {
  id: string;
  post: PostWithContent;
  created_at: string;
  category?: string;
}

const {
  data: favoritesData,
  pending,
  refresh,
} = await useLazyAsyncData(
  "user-favorites",
  async () => {
    // 模拟 API 调用 - 实际应该调用后端收藏接口
    // const response = await api.getFavorites({ page: currentPage.value, per_page: pageSize.value });

    // 为了演示，使用一些示例数据
    const mockFavorites: FavoriteItem[] = [
      {
        id: "fav-1",
        post: {
          id: "1",
          title: "Vue.js 3.0 新特性详解",
          slug: "vue-3-new-features",
          content: "Vue.js 3.0 带来了许多令人兴奋的新特性...",
          excerpt: "深入了解 Vue.js 3.0 的新特性，包括 Composition API、性能改进等",
          author_id: "author-1",
          author: {
            id: "author-1",
            username: "vue_expert",
            email: "expert@vue.com",
          },
          categories: [{ id: "1", name: "前端开发", slug: "frontend", created_at: "", updated_at: "" }],
          tags: [{ id: "1", name: "Vue.js", slug: "vuejs", created_at: "", updated_at: "" }],
          created_at: "2024-01-15T10:00:00Z",
          updated_at: "2024-01-15T10:00:00Z",
          published_at: "2024-01-15T10:00:00Z",
        },
        created_at: "2024-01-16T08:00:00Z",
      },
      {
        id: "fav-2",
        post: {
          id: "2",
          title: "TypeScript 高级类型系统",
          slug: "typescript-advanced-types",
          content: "TypeScript 的类型系统非常强大...",
          excerpt: "探索 TypeScript 高级类型特性，提升代码质量和开发效率",
          author_id: "author-2",
          author: {
            id: "author-2",
            username: "ts_master",
            email: "master@ts.com",
          },
          categories: [{ id: "1", name: "前端开发", slug: "frontend", created_at: "", updated_at: "" }],
          tags: [{ id: "2", name: "TypeScript", slug: "typescript", created_at: "", updated_at: "" }],
          created_at: "2024-01-10T10:00:00Z",
          updated_at: "2024-01-10T10:00:00Z",
          published_at: "2024-01-10T10:00:00Z",
        },
        created_at: "2024-01-12T09:00:00Z",
      },
    ];

    return {
      data: mockFavorites,
      pagination: {
        page: currentPage.value,
        per_page: pageSize.value,
        total: mockFavorites.length,
        total_pages: Math.ceil(mockFavorites.length / pageSize.value),
      },
    };
  },
  {
    watch: [currentPage, pageSize, selectedCategory, sortOption],
  },
);

// 计算属性
const favorites = computed(() => favoritesData.value?.data || []);
const totalFavorites = computed(() => favoritesData.value?.pagination.total || 0);
const totalPages = computed(() => favoritesData.value?.pagination.total_pages || 1);

// 分类选项
const categoryOptions = computed(() => {
  const categories = new Set<string>();
  favorites.value.forEach((fav) => {
    fav.post.categories.forEach(cat => categories.add(cat.name));
  });

  return [
    { label: "全部分类", value: "" },
    ...Array.from(categories).map(name => ({
      label: name,
      value: name,
    })),
  ];
});

// 排序选项
const sortOptions = [
  { label: "收藏时间", value: "created_at" },
  { label: "文章标题", value: "title" },
  { label: "发布时间", value: "published_at" },
];

// 筛选后的收藏列表
const filteredFavorites = computed(() => {
  let result = [...favorites.value];

  // 分类筛选
  if (selectedCategory.value) {
    result = result.filter(fav =>
      fav.post.categories.some(cat => cat.name === selectedCategory.value),
    );
  }

  // 排序
  result.sort((a, b) => {
    switch (sortOption.value) {
      case "title":
        return a.post.title.localeCompare(b.post.title);
      case "published_at":
        return new Date(b.post.published_at || b.post.created_at).getTime()
          - new Date(a.post.published_at || a.post.created_at).getTime();
      case "created_at":
      default:
        return new Date(b.created_at).getTime() - new Date(a.created_at).getTime();
    }
  });

  return result;
});

// 批量操作选项
const batchActions = [
  [{
    label: "批量取消收藏",
    icon: "i-heroicons-heart",
    click: () => batchRemoveFavorites(),
  }],
];

// 方法
const toggleFavoriteSelection = (favoriteId: string) => {
  const index = selectedFavorites.value.indexOf(favoriteId);
  if (index > -1) {
    selectedFavorites.value.splice(index, 1);
  }
  else {
    selectedFavorites.value.push(favoriteId);
  }
};

const removeFavorite = async (favoriteId: string) => {
  removingFavorite.value = favoriteId;

  try {
    // 实际应该调用 API 取消收藏
    // const response = await api.removeFavorite(favoriteId);

    // 模拟 API 调用
    await new Promise(resolve => setTimeout(resolve, 500));

    toast.add({
      title: "已取消收藏",
      color: "success",
    });

    // 刷新列表
    await refresh();
  }
  catch (error) {
    toast.add({
      title: "操作失败",
      description: error instanceof Error ? error.message : "取消收藏失败",
      color: "error",
    });
  }
  finally {
    removingFavorite.value = null;
  }
};

const batchRemoveFavorites = async () => {
  if (selectedFavorites.value.length === 0) return;

  try {
    // 实际应该调用批量取消收藏 API
    // const response = await api.batchRemoveFavorites(selectedFavorites.value);

    // 模拟 API 调用
    await new Promise(resolve => setTimeout(resolve, 1000));

    toast.add({
      title: "批量操作成功",
      description: `已取消收藏 ${selectedFavorites.value.length} 篇文章`,
      color: "success",
    });

    selectedFavorites.value = [];
    await refresh();
  }
  catch (error) {
    toast.add({
      title: "批量操作失败",
      description: error instanceof Error ? error.message : "批量取消收藏失败",
      color: "error",
    });
  }
};

const sharePost = (post: PostWithContent) => {
  shareUrl.value = `${window.location.origin}/posts/${post.slug}`;
  shareDialogOpen.value = true;
};

const copyToClipboard = async () => {
  try {
    await navigator.clipboard.writeText(shareUrl.value);
    toast.add({
      title: "复制成功",
      description: "文章链接已复制到剪贴板",
      color: "success",
    });
    shareDialogOpen.value = false;
  }
  catch (error) {
    toast.add({
      title: "复制失败",
      description: "无法复制到剪贴板",
      color: "error",
    });
  }
};

const formatDate = (dateString: string): string => {
  try {
    const date = new Date(dateString);
    const now = new Date();
    const diffTime = Math.abs(now.getTime() - date.getTime());
    const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24));

    if (diffDays === 1) {
      return "昨天";
    }
    else if (diffDays < 7) {
      return `${diffDays} 天前`;
    }
    else if (diffDays < 30) {
      const weeks = Math.ceil(diffDays / 7);
      return `${weeks} 周前`;
    }
    else {
      return date.toLocaleDateString("zh-CN", { month: "short", day: "numeric" });
    }
  }
  catch {
    return "时间未知";
  }
};
</script>

<style scoped>
.line-clamp-1 {
  display: -webkit-box;
  -webkit-line-clamp: 1;
  line-clamp: 1;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.modern-gradient-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  transition: all 0.3s ease;
}
</style>
