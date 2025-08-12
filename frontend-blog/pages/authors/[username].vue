<template>
  <div>
    <!-- 加载状态 -->
    <div
      v-if="pending || postsPending"
      class="container-blog py-8"
    >
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="grid lg:grid-cols-4 gap-8">
          <!-- 作者信息骨架 -->
          <div class="lg:col-span-1">
            <USkeleton class="h-32 w-32 rounded-full mx-auto mb-4" />
            <USkeleton class="h-6 w-3/4 mx-auto mb-2" />
            <USkeleton class="h-4 w-1/2 mx-auto mb-4" />
            <USkeleton class="h-20 w-full" />
          </div>
          <!-- 文章列表骨架 -->
          <div class="lg:col-span-3">
            <USkeleton class="h-8 w-1/2 mb-6" />
            <div class="grid md:grid-cols-2 gap-6">
              <USkeleton
                v-for="i in 4"
                :key="i"
                class="h-48 w-full"
              />
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 错误状态 -->
    <div
      v-else-if="error"
      class="container-blog py-16"
    >
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
        <UIcon
          name="i-heroicons-exclamation-triangle"
          class="w-16 h-16 text-red-500 mx-auto mb-4"
        />
        <h1 class="text-2xl font-bold text-gray-900 dark:text-white mb-2">
          作者信息加载失败
        </h1>
        <p class="text-gray-600 dark:text-gray-400 mb-6">
          {{ error.message || '未知错误' }}
        </p>
        <div class="flex gap-4 justify-center">
          <UButton
            to="/authors"
            icon="i-heroicons-arrow-left"
          >
            返回作者列表
          </UButton>
          <UButton
            variant="outline"
            @click="() => refresh()"
          >
            重新加载
          </UButton>
        </div>
      </div>
    </div>

    <!-- 作者详情内容 -->
    <div
      v-else-if="authorInfo && posts"
      class="min-h-screen"
    >
      <!-- Hero 区域 -->
      <section
        class="modern-hero"
        style="padding: 2rem 0 1.5rem 0; background: linear-gradient(135deg, #f8fafc 0%, #e2e8f0 50%, #dbeafe 100%);"
      >
        <div class="relative mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
          <div style="text-align: center;">
            <!-- 作者头像 -->
            <div style="margin: 0 auto 1.5rem;">
              <div
                class="author-avatar-large"
                style="margin: 0 auto; width: 6rem; height: 6rem; display: flex; align-items: center; justify-content: center; border-radius: 50%; background: linear-gradient(45deg, #3b82f6, #6366f1); color: white; font-size: 2rem; font-weight: 700; box-shadow: 0 8px 32px rgba(59, 130, 246, 0.3); position: relative; overflow: hidden;"
              >
                <span v-if="!authorInfo.avatar">
                  {{ getInitials(authorInfo.username) }}
                </span>
                <img
                  v-else
                  :src="authorInfo.avatar"
                  :alt="authorInfo.username"
                  style="width: 100%; height: 100%; object-fit: cover;"
                >
              </div>
            </div>

            <!-- 作者名称 -->
            <h1
              style="font-size: 2rem; font-weight: 700; line-height: 1.1; margin-bottom: 0.75rem; color: #1e293b;"
              class="dark:text-white"
            >
              {{ authorInfo.username }}
            </h1>

            <!-- 作者简介 -->
            <p
              v-if="authorInfo.bio"
              style="font-size: 1rem; color: #64748b; max-width: 40rem; margin: 0 auto 1rem; line-height: 1.5;"
              class="dark:text-slate-300"
            >
              {{ authorInfo.bio }}
            </p>

            <!-- 统计信息 -->
            <div class="flex items-center justify-center gap-6 text-sm text-slate-500 dark:text-slate-400">
              <div class="flex items-center gap-1">
                <UIcon
                  name="i-heroicons-document-text"
                  class="w-4 h-4"
                />
                <span>{{ posts.pagination.total }} 篇文章</span>
              </div>
              <div class="flex items-center gap-1">
                <UIcon
                  name="i-heroicons-squares-2x2"
                  class="w-4 h-4"
                />
                <span>{{ authorStats?.categories.length || 0 }} 个分类</span>
              </div>
              <div class="flex items-center gap-1">
                <UIcon
                  name="i-heroicons-clock"
                  class="w-4 h-4"
                />
                <span>加入于 {{ formatDate(authorInfo.created_at) }}</span>
              </div>
            </div>
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
              to="/authors"
              style="color: #64748b; text-decoration: none; transition: all 0.2s ease;"
              class="hover:text-blue-600"
            >
              作者
            </NuxtLink>
            <UIcon
              name="i-heroicons-chevron-right"
              class="w-4 h-4 text-slate-400"
            />
            <span style="color: #3b82f6; font-weight: 500;">{{ authorInfo.username }}</span>
          </div>
        </div>
      </nav>

      <!-- 主要内容区域 -->
      <section
        style="background: linear-gradient(135deg, #f8fafc 0%, #e2e8f0 50%, #dbeafe 100%); padding: 2rem 0; margin: 0;"
      >
        <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
          <div class="grid lg:grid-cols-4 gap-8">
            <!-- 侧边栏 - 作者信息 -->
            <div class="lg:col-span-1">
              <div class="sticky top-8">
                <!-- 统计卡片 -->
                <div
                  class="modern-content-section mb-6"
                  style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 1.5rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);"
                >
                  <h3 class="font-semibold text-slate-900 dark:text-white mb-4 flex items-center">
                    <UIcon
                      name="i-heroicons-chart-bar"
                      class="w-5 h-5 mr-2 text-blue-500"
                    />
                    统计信息
                  </h3>

                  <div class="space-y-3">
                    <div class="flex justify-between items-center">
                      <span class="text-slate-600 dark:text-slate-400">总文章</span>
                      <span class="font-semibold text-slate-900 dark:text-white">{{ posts.pagination.total }}</span>
                    </div>
                    <div class="flex justify-between items-center">
                      <span class="text-slate-600 dark:text-slate-400">涉及分类</span>
                      <span class="font-semibold text-slate-900 dark:text-white">{{ authorStats?.categories.length || 0 }}</span>
                    </div>
                    <div class="flex justify-between items-center">
                      <span class="text-slate-600 dark:text-slate-400">使用标签</span>
                      <span class="font-semibold text-slate-900 dark:text-white">{{ authorStats?.tags.length || 0 }}</span>
                    </div>
                  </div>
                </div>

                <!-- 专长领域 -->
                <div
                  v-if="authorStats?.categories.length"
                  class="modern-content-section mb-6"
                  style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 1.5rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);"
                >
                  <h3 class="font-semibold text-slate-900 dark:text-white mb-4 flex items-center">
                    <UIcon
                      name="i-heroicons-squares-2x2"
                      class="w-5 h-5 mr-2 text-purple-500"
                    />
                    专长领域
                  </h3>

                  <div class="flex flex-wrap gap-2">
                    <span
                      v-for="category in authorStats.categories"
                      :key="category"
                      class="inline-block px-3 py-1 text-sm rounded-full bg-purple-100 text-purple-700 dark:bg-purple-900 dark:text-purple-200"
                    >
                      {{ category }}
                    </span>
                  </div>
                </div>

                <!-- 常用标签 -->
                <div
                  v-if="authorStats?.tags.length"
                  class="modern-content-section"
                  style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 1.5rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);"
                >
                  <h3 class="font-semibold text-slate-900 dark:text-white mb-4 flex items-center">
                    <UIcon
                      name="i-heroicons-hashtag"
                      class="w-5 h-5 mr-2 text-emerald-500"
                    />
                    常用标签
                  </h3>

                  <div class="flex flex-wrap gap-1">
                    <span
                      v-for="tag in authorStats.tags.slice(0, 10)"
                      :key="tag"
                      class="inline-block px-2 py-1 text-xs rounded-full bg-emerald-100 text-emerald-700 dark:bg-emerald-900 dark:text-emerald-200"
                    >
                      #{{ tag }}
                    </span>
                  </div>
                </div>
              </div>
            </div>

            <!-- 主内容 - 文章列表 -->
            <div class="lg:col-span-3">
              <div
                class="modern-content-section"
                style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 1.5rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);"
              >
                <div class="modern-section-header">
                  <div>
                    <div class="flex items-center gap-3 mb-2">
                      <div
                        class="modern-gradient-icon"
                        style="background: linear-gradient(45deg, #3b82f6, #6366f1); width: 2.5rem; height: 2.5rem;"
                      >
                        <UIcon
                          name="i-heroicons-document-text"
                          class="h-5 w-5 text-white"
                        />
                      </div>
                      <h2 class="modern-section-title">
                        {{ authorInfo.username }} 的文章
                      </h2>
                    </div>
                    <p class="modern-section-subtitle">
                      共 {{ posts.pagination.total }} 篇优质内容
                    </p>
                  </div>

                  <!-- 排序选项 -->
                  <div style="min-width: 160px;">
                    <USelectMenu
                      v-model="sortOption"
                      :options="sortOptions"
                      size="sm"
                      style="background: rgba(248, 250, 252, 0.8); border: 1px solid rgba(226, 232, 240, 0.6);"
                    />
                  </div>
                </div>

                <!-- 文章网格 -->
                <div
                  v-if="posts.data.length"
                  style="margin-bottom: 3rem;"
                >
                  <div class="modern-grid-cards">
                    <div
                      v-for="post in posts.data"
                      :key="post.id"
                      class="modern-post-card"
                    >
                      <PostCard :post="post" />
                    </div>
                  </div>

                  <!-- 分页组件 -->
                  <div
                    v-if="posts.pagination.total_pages > 1"
                    style="display: flex; justify-content: center; margin-top: 3rem;"
                  >
                    <UPagination
                      v-model="currentPage"
                      :page-count="posts.pagination.per_page"
                      :total="posts.pagination.total"
                      :max="7"
                      style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 1rem; border: 1px solid rgba(226, 232, 240, 0.6); box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);"
                    />
                  </div>
                </div>

                <!-- 空状态 -->
                <div
                  v-else
                  style="text-align: center; padding: 4rem 0;"
                >
                  <div style="margin: 0 auto 2rem; width: 4rem; height: 4rem; display: flex; align-items: center; justify-content: center; border-radius: 50%; background: linear-gradient(45deg, #94a3b8, #cbd5e1); box-shadow: 0 8px 32px rgba(148, 163, 184, 0.2);">
                    <UIcon
                      name="i-heroicons-document-text"
                      style="width: 2rem; height: 2rem; color: white;"
                    />
                  </div>
                  <h3
                    style="font-size: 1.5rem; font-weight: 700; color: #1e293b; margin-bottom: 1rem;"
                    class="dark:text-white"
                  >
                    暂无发布文章
                  </h3>
                  <p
                    style="font-size: 1rem; color: #64748b; margin-bottom: 2rem; line-height: 1.6;"
                    class="dark:text-slate-300"
                  >
                    该作者还没有发布任何文章
                  </p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>
    </div>

    <!-- 未找到作者 -->
    <div
      v-else
      class="container-blog py-16"
    >
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
        <UIcon
          name="i-heroicons-user"
          class="w-16 h-16 text-gray-400 mx-auto mb-4"
        />
        <h1 class="text-2xl font-bold text-gray-900 dark:text-white mb-2">
          作者不存在
        </h1>
        <p class="text-gray-600 dark:text-gray-400 mb-6">
          未找到该作者的信息或该作者没有发布过文章
        </p>
        <UButton
          to="/authors"
          icon="i-heroicons-arrow-left"
        >
          返回作者列表
        </UButton>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { PostQueryParams } from "~/types";

// 获取路由参数
const route = useRoute();
const username = route.params.username as string;

// 状态管理
const currentPage = ref(1);
const sortOption = ref("created_at");

// API 客户端
const api = useApi();

// 查询参数
const queryParams = computed((): PostQueryParams => ({
  page: currentPage.value,
  per_page: 9,
  author: username,
  sort: sortOption.value as PostQueryParams["sort"],
  order: "desc",
  status: "published",
}));

// 获取作者文章
const {
  data: posts,
  pending: postsPending,
  refresh: refreshPosts,
} = await useLazyAsyncData(
  `author-posts-${username}`,
  async () => {
    const response = await api.getPostsByAuthor(username, queryParams.value);
    if (response.success) {
      return response.data;
    }
    return {
      data: [],
      pagination: { page: 1, per_page: 9, total: 0, total_pages: 0 },
    };
  },
  {
    watch: [queryParams],
  },
);

// 获取作者统计信息
const { data: authorStats, pending, error, refresh } = await useLazyAsyncData(
  `author-stats-${username}`,
  async () => {
    const response = await api.getAuthorStats(username);
    if (response.success) {
      return response.data;
    }
    throw createError({
      statusCode: 404,
      statusMessage: "作者不存在或没有发布过文章",
    });
  },
);

// 推导作者基本信息（从第一篇文章获取）
const authorInfo = computed(() => {
  if (!posts.value?.data.length) return null;

  const firstPost = posts.value.data[0];
  return {
    id: firstPost.author.id,
    username: firstPost.author.username,
    email: firstPost.author.email,
    bio: `专注于${authorStats.value?.categories.slice(0, 2).join("、")}等领域的内容创作`, // 模拟简介
    created_at: firstPost.created_at,
    avatar: undefined, // 暂无头像数据
  };
});

// 排序选项
const sortOptions = [
  { label: "最新发布", value: "created_at" },
  { label: "最近更新", value: "updated_at" },
  { label: "标题排序", value: "title" },
];

// 工具函数
const getInitials = (username: string): string => {
  return username.slice(0, 2).toUpperCase();
};

const formatDate = (dateString: string): string => {
  try {
    const date = new Date(dateString);
    return date.toLocaleDateString("zh-CN", {
      year: "numeric",
      month: "long",
      day: "numeric",
    });
  }
  catch {
    return "时间未知";
  }
};

// SEO 配置
watchEffect(() => {
  if (authorInfo.value) {
    useHead({
      title: `${authorInfo.value.username} - 作者主页`,
      meta: [
        {
          name: "description",
          content: `查看 ${authorInfo.value.username} 的所有文章，专注于${authorStats.value?.categories.slice(0, 2).join("、")}等领域`,
        },
        { property: "og:title", content: `${authorInfo.value.username} - 作者主页` },
        {
          property: "og:description",
          content: `查看 ${authorInfo.value.username} 的所有文章`,
        },
      ],
    });
  }
});

// 监听页码变化
watch(currentPage, () => {
  refreshPosts();
});

// 监听排序变化
watch(sortOption, () => {
  currentPage.value = 1;
  refreshPosts();
});
</script>

<style scoped>
.author-avatar-large {
  transition: all 0.3s ease;
}

.modern-post-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 25px rgba(0, 0, 0, 0.15);
}

.modern-gradient-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  transition: all 0.3s ease;
}
</style>
