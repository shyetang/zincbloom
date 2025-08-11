<template>
  <div>
    <!-- 加载状态 -->
    <div
      v-if="pending"
      class="container-blog py-8"
    >
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <USkeleton class="h-8 w-3/4 mb-4" />
        <USkeleton class="h-4 w-1/2 mb-8" />
        <USkeleton class="h-64 w-full mb-8" />
        <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
          <USkeleton
            v-for="i in 6"
            :key="i"
            class="h-64 w-full"
          />
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
          分类加载失败
        </h1>
        <p class="text-gray-600 dark:text-gray-400 mb-6">
          {{ error.message || '未知错误' }}
        </p>
        <div class="flex gap-4 justify-center">
          <UButton
            to="/categories"
            icon="i-heroicons-arrow-left"
          >
            返回分类列表
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

    <!-- 分类详情内容 -->
    <div
      v-else-if="category && posts"
      class="min-h-screen"
    >
      <!-- Hero 区域 -->
      <section
        class="modern-hero"
        style="padding: 2rem 0 1.5rem 0;"
      >
        <div class="relative mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
          <div style="text-align: center;">
            <!-- 现代化图标 -->
            <div style="margin: 0 auto 1rem; width: 2.5rem; height: 2.5rem; display: flex; align-items: center; justify-content: center; border-radius: 50%; background: linear-gradient(45deg, #8b5cf6, #a855f7); box-shadow: 0 6px 24px rgba(139, 92, 246, 0.3);">
              <UIcon
                name="i-heroicons-folder"
                style="width: 1.25rem; height: 1.25rem; color: white;"
              />
            </div>

            <!-- 分类标题 -->
            <h1
              style="font-size: 1.875rem; font-weight: 700; line-height: 1.1; margin-bottom: 0.75rem; color: #1e293b;"
              class="dark:text-white"
            >
              {{ category.name }}
            </h1>

            <!-- 分类描述 -->
            <p
              v-if="category.description"
              style="font-size: 1rem; color: #64748b; max-width: 40rem; margin: 0 auto 1rem; line-height: 1.5;"
              class="dark:text-slate-300"
            >
              {{ category.description }}
            </p>

            <!-- 统计信息 -->
            <div class="flex items-center justify-center gap-4 text-sm text-slate-500 dark:text-slate-400">
              <div class="flex items-center gap-1">
                <UIcon
                  name="i-heroicons-document-text"
                  class="w-4 h-4"
                />
                <span>{{ posts.pagination.total }} 篇文章</span>
              </div>
              <div class="flex items-center gap-1">
                <UIcon
                  name="i-heroicons-clock"
                  class="w-4 h-4"
                />
                <span>{{ formatDate(category.created_at) }} 创建</span>
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
              to="/categories"
              style="color: #64748b; text-decoration: none; transition: all 0.2s ease;"
              class="hover:text-blue-600"
            >
              分类
            </NuxtLink>
            <UIcon
              name="i-heroicons-chevron-right"
              class="w-4 h-4 text-slate-400"
            />
            <span style="color: #8b5cf6; font-weight: 500;">{{ category.name }}</span>
          </div>
        </div>
      </nav>

      <!-- 主要内容区域 -->
      <section
        style="background: linear-gradient(135deg, #f8fafc 0%, #e2e8f0 50%, #eff6ff 100%); padding: 2rem 0; margin: 0; position: relative;"
      >
        <!-- 背景装饰 -->
        <div style="position: absolute; inset: 0; background-image: radial-gradient(circle at 1px 1px, rgba(139, 92, 246, 0.1) 1px, transparent 0); background-size: 60px 60px; opacity: 0.5;" />

        <div class="relative mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
          <!-- 文章列表 -->
          <div
            class="modern-content-section"
            style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 1.5rem; margin-bottom: 1rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);"
          >
            <div class="modern-section-header">
              <div>
                <div class="flex items-center gap-3 mb-2">
                  <div
                    class="modern-gradient-icon"
                    style="background: linear-gradient(45deg, #8b5cf6, #a855f7); width: 2.5rem; height: 2.5rem;"
                  >
                    <UIcon
                      name="i-heroicons-document-text"
                      class="h-5 w-5 text-white"
                    />
                  </div>
                  <h2 class="modern-section-title">
                    分类文章
                  </h2>
                </div>
                <p class="modern-section-subtitle">
                  {{ category.name }} 分类下的所有文章
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

            <!-- 加载状态 -->
            <div
              v-if="postsPending"
              class="modern-grid-cards"
            >
              <div
                v-for="i in 6"
                :key="i"
                class="modern-post-card"
                style="height: 280px; display: flex; align-items: center; justify-content: center;"
              >
                <USkeleton class="h-64 w-full" />
              </div>
            </div>

            <!-- 文章网格 -->
            <div
              v-else-if="posts.data.length"
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
                暂无文章
              </h3>
              <p
                style="font-size: 1rem; color: #64748b; margin-bottom: 2rem; line-height: 1.6;"
                class="dark:text-slate-300"
              >
                该分类下还没有发布任何文章
              </p>
              <div style="display: flex; flex-direction: column; gap: 1rem; align-items: center;">
                <div style="display: flex; gap: 1rem; flex-wrap: wrap; justify-content: center;">
                  <NuxtLink
                    to="/categories"
                    style="display: inline-flex; align-items: center; gap: 0.5rem; padding: 0.75rem 1.5rem; background: rgba(255, 255, 255, 0.9); color: #64748b; text-decoration: none; border-radius: 2rem; font-size: 0.875rem; font-weight: 500; border: 1px solid rgba(226, 232, 240, 0.6); transition: all 0.3s ease;"
                    class="hover:bg-white hover:border-slate-300 hover:transform hover:scale-105"
                  >
                    <UIcon
                      name="i-heroicons-arrow-left"
                      class="w-4 h-4"
                    />
                    返回分类列表
                  </NuxtLink>
                  <NuxtLink
                    to="/posts"
                    style="display: inline-flex; align-items: center; gap: 0.5rem; padding: 0.75rem 1.5rem; background: linear-gradient(45deg, #8b5cf6, #a855f7); color: white; text-decoration: none; border-radius: 2rem; font-size: 0.875rem; font-weight: 600; transition: all 0.3s ease; box-shadow: 0 4px 12px rgba(139, 92, 246, 0.3);"
                    class="hover:transform hover:scale-105 hover:shadow-lg"
                  >
                    <UIcon
                      name="i-heroicons-document-text"
                      class="w-4 h-4"
                    />
                    浏览全部文章
                  </NuxtLink>
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>
    </div>

    <!-- 未找到分类 -->
    <div
      v-else
      class="container-blog py-16"
    >
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
        <UIcon
          name="i-heroicons-folder"
          class="w-16 h-16 text-gray-400 mx-auto mb-4"
        />
        <h1 class="text-2xl font-bold text-gray-900 dark:text-white mb-2">
          分类不存在
        </h1>
        <p class="text-gray-600 dark:text-gray-400 mb-6">
          您访问的分类可能已被删除或不存在
        </p>
        <UButton
          to="/categories"
          icon="i-heroicons-arrow-left"
        >
          返回分类列表
        </UButton>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { PostQueryParams } from "~/types";

// 获取路由参数
const route = useRoute();
const slug = route.params.slug as string;

// 状态管理
const currentPage = ref(1);
const sortOption = ref("created_at");

// API 客户端
const api = useApi();

// 获取分类数据
const { data: category, pending, error, refresh } = await useLazyAsyncData(
  `category-${slug}`,
  async () => {
    const response = await api.getCategory(slug);
    if (response.success) {
      return response.data;
    }
    throw createError({
      statusCode: 404,
      statusMessage: "分类不存在",
    });
  },
);

// 查询参数
const queryParams = computed((): PostQueryParams => ({
  page: currentPage.value,
  per_page: 9,
  category: slug,
  sort: sortOption.value as PostQueryParams["sort"],
  order: "desc",
  status: "published",
}));

// 获取分类文章
const {
  data: posts,
  pending: postsPending,
  refresh: refreshPosts,
} = await useLazyAsyncData(
  `category-posts-${slug}`,
  async () => {
    const response = await api.getPostsByCategory(slug, queryParams.value);
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

// 排序选项
const sortOptions = [
  { label: "最新发布", value: "created_at" },
  { label: "最近更新", value: "updated_at" },
  { label: "标题排序", value: "title" },
];

// 格式化日期
const formatDate = (dateString: string) => {
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
  if (category.value) {
    useHead({
      title: `${category.value.name} - 分类`,
      meta: [
        {
          name: "description",
          content: category.value.description || `浏览 ${category.value.name} 分类下的所有文章`,
        },
        { property: "og:title", content: `${category.value.name} - 分类` },
        {
          property: "og:description",
          content: category.value.description || `浏览 ${category.value.name} 分类下的所有文章`,
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
