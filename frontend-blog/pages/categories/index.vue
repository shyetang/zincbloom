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
          <div style="margin: 0 auto 1rem; width: 2.5rem; height: 2.5rem; display: flex; align-items: center; justify-content: center; border-radius: 50%; background: linear-gradient(45deg, #8b5cf6, #a855f7); box-shadow: 0 6px 24px rgba(139, 92, 246, 0.3);">
            <UIcon
              name="i-heroicons-squares-2x2"
              style="width: 1.25rem; height: 1.25rem; color: white;"
            />
          </div>

          <!-- 现代化标题 -->
          <h1
            style="font-size: 1.875rem; font-weight: 700; line-height: 1.1; margin-bottom: 0.75rem; color: #1e293b;"
            class="dark:text-white"
          >
            文章
            <span
              class="modern-text-gradient"
              style="font-weight: 800;"
            >分类</span>
          </h1>

          <!-- 副标题 -->
          <p
            style="font-size: 1rem; color: #64748b; max-width: 28rem; margin: 0 auto; line-height: 1.5;"
            class="dark:text-slate-300"
          >
            探索不同主题的精彩内容，找到最感兴趣的知识领域
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
          <span style="color: #8b5cf6; font-weight: 500;">分类浏览</span>
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
        <!-- 错误状态 -->
        <div
          v-if="error"
          class="modern-content-section"
          style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 2rem 1.5rem; margin-bottom: 1rem; border: 1px solid #fca5a5; box-shadow: 0 2px 4px rgba(239, 68, 68, 0.1);"
        >
          <div style="text-align: center;">
            <div style="margin: 0 auto 2rem; width: 4rem; height: 4rem; display: flex; align-items: center; justify-content: center; border-radius: 50%; background: linear-gradient(45deg, #ef4444, #f87171); box-shadow: 0 8px 32px rgba(239, 68, 68, 0.3);">
              <UIcon
                name="i-heroicons-exclamation-triangle"
                style="width: 2rem; height: 2rem; color: white;"
              />
            </div>
            <h3
              style="font-size: 1.5rem; font-weight: 700; color: #1e293b; margin-bottom: 1rem;"
              class="dark:text-white"
            >
              加载失败
            </h3>
            <p
              style="font-size: 1rem; color: #64748b; margin-bottom: 2rem; line-height: 1.6;"
              class="dark:text-slate-300"
            >
              {{ error.message || "获取分类列表时发生错误" }}
            </p>
            <button
              style="display: inline-flex; align-items: center; gap: 0.5rem; padding: 0.75rem 2rem; background: linear-gradient(45deg, #ef4444, #f87171); color: white; border-radius: 2rem; font-size: 0.875rem; font-weight: 600; border: none; cursor: pointer; transition: all 0.3s ease; box-shadow: 0 4px 12px rgba(239, 68, 68, 0.3);"
              class="hover:transform hover:scale-105 hover:shadow-lg"
              @click="refresh()"
            >
              <UIcon
                name="i-heroicons-arrow-path"
                class="w-4 h-4"
              />
              重新加载
            </button>
          </div>
        </div>

        <!-- 分类列表 -->
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
                    name="i-heroicons-squares-2x2"
                    class="h-5 w-5 text-white"
                  />
                </div>
                <h2 class="modern-section-title">
                  所有分类
                </h2>
              </div>
              <p class="modern-section-subtitle">
                {{ categories?.pagination?.total || 0 }} 个分类等待您的探索
              </p>
            </div>

            <!-- 搜索框 -->
            <div style="min-width: 240px;">
              <UInput
                v-model="searchQuery"
                icon="i-heroicons-magnifying-glass"
                placeholder="搜索分类..."
                size="sm"
                style="background: rgba(248, 250, 252, 0.8); border: 1px solid rgba(226, 232, 240, 0.6);"
                class="focus:border-purple-500"
                @keyup.enter="handleSearch"
              />
            </div>
          </div>

          <!-- 加载状态 -->
          <div
            v-if="pending"
            class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6"
            style="padding: 1rem 0;"
          >
            <div
              v-for="i in 6"
              :key="i"
              class="modern-post-card"
              style="padding: 2rem; height: 200px; display: flex; align-items: center; justify-content: center;"
            >
              <USkeleton class="h-32 w-full" />
            </div>
          </div>

          <!-- 分类网格 -->
          <div
            v-else-if="filteredCategories.length"
            class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6"
            style="padding: 1rem 0;"
          >
            <div
              v-for="category in filteredCategories"
              :key="category.id"
              class="modern-post-card group cursor-pointer"
              style="padding: 2rem; transition: all 0.3s ease;"
              @click="goToCategory(category.slug)"
            >
              <!-- 分类图标 -->
              <div style="text-align: center; margin-bottom: 1.5rem;">
                <div
                  class="modern-gradient-icon"
                  style="background: linear-gradient(45deg, #8b5cf6, #a855f7); width: 4rem; height: 4rem; margin: 0 auto;"
                >
                  <UIcon
                    name="i-heroicons-folder"
                    class="w-6 h-6 text-white"
                  />
                </div>
              </div>

              <!-- 分类信息 -->
              <div style="text-align: center;">
                <h3 class="text-xl font-bold text-slate-900 dark:text-white mb-2">
                  {{ category.name }}
                </h3>

                <p
                  v-if="category.description"
                  class="text-slate-600 dark:text-slate-400 mb-4 line-clamp-2"
                  style="min-height: 3rem;"
                >
                  {{ category.description }}
                </p>

                <div class="flex items-center justify-center gap-2 text-sm text-slate-500 dark:text-slate-400">
                  <UIcon
                    name="i-heroicons-document-text"
                    class="w-4 h-4"
                  />
                  <span>{{ category.post_count || 0 }} 篇文章</span>
                </div>

                <!-- 查看按钮 -->
                <div style="margin-top: 1.5rem;">
                  <div
                    class="inline-flex items-center gap-2 px-4 py-2 text-sm font-medium rounded-lg transition-all"
                    style="background: rgba(139, 92, 246, 0.1); color: #8b5cf6; border: 1px solid rgba(139, 92, 246, 0.2);"
                  >
                    <span>浏览文章</span>
                    <UIcon
                      name="i-heroicons-arrow-right"
                      class="w-4 h-4 transition-transform group-hover:translate-x-1"
                    />
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- 空状态 -->
          <div
            v-else
            style="text-align: center; padding: 4rem 0;"
          >
            <div style="margin: 0 auto 2rem; width: 4rem; height: 4rem; display: flex; align-items: center; justify-content: center; border-radius: 50%; background: linear-gradient(45deg, #94a3b8, #cbd5e1); box-shadow: 0 8px 32px rgba(148, 163, 184, 0.2);">
              <UIcon
                name="i-heroicons-squares-2x2"
                style="width: 2rem; height: 2rem; color: white;"
              />
            </div>
            <h3
              style="font-size: 1.5rem; font-weight: 700; color: #1e293b; margin-bottom: 1rem;"
              class="dark:text-white"
            >
              没有找到分类
            </h3>
            <p
              style="font-size: 1rem; color: #64748b; margin-bottom: 2rem; line-height: 1.6;"
              class="dark:text-slate-300"
            >
              {{ searchQuery ? '尝试调整搜索条件' : '暂无分类内容' }}
            </p>
            <div style="display: flex; flex-direction: column; gap: 1rem; align-items: center;">
              <div style="display: flex; gap: 1rem; flex-wrap: wrap; justify-content: center;">
                <a
                  href="/"
                  style="display: inline-flex; align-items: center; gap: 0.5rem; padding: 0.75rem 1.5rem; background: rgba(255, 255, 255, 0.9); color: #64748b; text-decoration: none; border-radius: 2rem; font-size: 0.875rem; font-weight: 500; border: 1px solid rgba(226, 232, 240, 0.6); transition: all 0.3s ease;"
                  class="hover:bg-white hover:border-slate-300 hover:transform hover:scale-105"
                >
                  <UIcon
                    name="i-heroicons-home"
                    class="w-4 h-4"
                  />
                  返回首页
                </a>
                <button
                  v-if="searchQuery"
                  style="display: inline-flex; align-items: center; gap: 0.5rem; padding: 0.75rem 1.5rem; background: linear-gradient(45deg, #8b5cf6, #a855f7); color: white; border-radius: 2rem; font-size: 0.875rem; font-weight: 600; border: none; cursor: pointer; transition: all 0.3s ease; box-shadow: 0 4px 12px rgba(139, 92, 246, 0.3);"
                  class="hover:transform hover:scale-105 hover:shadow-lg"
                  @click="clearSearch"
                >
                  <UIcon
                    name="i-heroicons-x-mark"
                    class="w-4 h-4"
                  />
                  清除搜索
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import type { Category } from "~/types";

// SEO 配置
useHead({
  title: "文章分类",
  meta: [{ name: "description", content: "浏览所有文章分类，发现感兴趣的内容主题" }],
});

// 状态管理
const router = useRouter();
const searchQuery = ref("");

// API 客户端
const api = useApi();

// 获取分类数据
const {
  data: categories,
  pending,
  refresh,
  error,
} = await useLazyAsyncData(
  "categories",
  async () => {
    const response = await api.getCategories();
    if (response.success) {
      return response.data;
    }
    throw createError({
      statusCode: 500,
      statusMessage: response.error?.message || "获取分类列表失败",
    });
  },
);

// 搜索过滤
const filteredCategories = computed(() => {
  if (!categories.value?.data) return [];

  if (!searchQuery.value.trim()) {
    return categories.value.data;
  }

  const query = searchQuery.value.toLowerCase().trim();
  return categories.value.data.filter((category: Category) =>
    category.name.toLowerCase().includes(query)
    || (category.description && category.description.toLowerCase().includes(query)),
  );
});

// 方法
const handleSearch = () => {
  // 搜索逻辑已通过计算属性实现
};

const clearSearch = () => {
  searchQuery.value = "";
};

const goToCategory = (slug: string) => {
  router.push(`/categories/${slug}`);
};
</script>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
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

.group:hover .modern-gradient-icon {
  transform: scale(1.1);
  box-shadow: 0 8px 20px rgba(139, 92, 246, 0.4);
}
</style>
