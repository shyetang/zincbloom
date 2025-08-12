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
          <div style="margin: 0 auto 1rem; width: 2.5rem; height: 2.5rem; display: flex; align-items: center; justify-content: center; border-radius: 50%; background: linear-gradient(45deg, #3b82f6, #6366f1); box-shadow: 0 6px 24px rgba(59, 130, 246, 0.3);">
            <UIcon
              name="i-heroicons-magnifying-glass"
              style="width: 1.25rem; height: 1.25rem; color: white;"
            />
          </div>

          <!-- 现代化标题 -->
          <h1
            style="font-size: 1.875rem; font-weight: 700; line-height: 1.1; margin-bottom: 0.75rem; color: #1e293b;"
            class="dark:text-white"
          >
            全局
            <span
              class="modern-text-gradient"
              style="font-weight: 800;"
            >搜索</span>
          </h1>

          <!-- 副标题 -->
          <p
            style="font-size: 1rem; color: #64748b; max-width: 28rem; margin: 0 auto 1.5rem; line-height: 1.5;"
            class="dark:text-slate-300"
          >
            搜索文章、分类、标签、作者，快速找到您需要的内容
          </p>

          <!-- 搜索框 -->
          <div style="max-width: 32rem; margin: 0 auto;">
            <UInput
              ref="searchInputRef"
              v-model="searchQuery"
              icon="i-heroicons-magnifying-glass"
              trailing-icon="i-heroicons-x-mark"
              :trailing="!!searchQuery"
              placeholder="输入关键词开始搜索..."
              size="lg"
              style="background: rgba(255, 255, 255, 0.9); border: 2px solid rgba(226, 232, 240, 0.6); backdrop-filter: blur(10px);"
              class="focus:border-blue-500"
              @keyup.enter="performSearch"
              @trailing-click="clearSearch"
            />
            <div style="margin-top: 1rem;">
              <button
                style="display: inline-flex; align-items: center; gap: 0.5rem; padding: 0.75rem 2rem; background: linear-gradient(45deg, #3b82f6, #6366f1); color: white; border-radius: 2rem; font-size: 0.875rem; font-weight: 600; border: none; cursor: pointer; transition: all 0.3s ease; box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);"
                class="hover:transform hover:scale-105 hover:shadow-lg"
                :disabled="!searchQuery.trim() || isSearching"
                @click="performSearch"
              >
                <UIcon
                  :name="isSearching ? 'i-heroicons-arrow-path' : 'i-heroicons-magnifying-glass'"
                  :class="['w-4 h-4', { 'animate-spin': isSearching }]"
                />
                {{ isSearching ? '搜索中...' : '开始搜索' }}
              </button>
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
          <span style="color: #3b82f6; font-weight: 500;">搜索</span>
          <span
            v-if="currentSearchQuery"
            style="color: #64748b;"
          >
            "{{ currentSearchQuery }}"
          </span>
        </div>
      </div>
    </nav>

    <!-- 搜索历史记录 -->
    <section
      v-if="!hasSearched && searchHistory.length"
      style="background: linear-gradient(135deg, #f8fafc 0%, #e2e8f0 50%, #dbeafe 100%); padding: 2rem 0; margin: 0;"
    >
      <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
        <div
          class="modern-content-section"
          style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 1.5rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);"
        >
          <h2 class="text-lg font-semibold text-slate-900 dark:text-white mb-4 flex items-center">
            <UIcon
              name="i-heroicons-clock"
              class="w-5 h-5 mr-2 text-slate-500"
            />
            搜索历史
          </h2>

          <div class="flex flex-wrap gap-2 mb-4">
            <button
              v-for="(history, index) in searchHistory.slice(0, 8)"
              :key="index"
              class="inline-flex items-center gap-1 px-3 py-2 text-sm rounded-lg bg-slate-100 text-slate-700 hover:bg-slate-200 dark:bg-slate-700 dark:text-slate-300 dark:hover:bg-slate-600 transition-colors"
              @click="searchFromHistory(history)"
            >
              <span>{{ history }}</span>
              <UIcon
                name="i-heroicons-x-mark"
                class="w-3 h-3 ml-1 opacity-50 hover:opacity-100 cursor-pointer"
                @click.stop="removeFromHistory(index)"
              />
            </button>
          </div>

          <div class="flex justify-end">
            <button
              class="text-sm text-slate-500 hover:text-slate-700 dark:text-slate-400 dark:hover:text-slate-300"
              @click="clearSearchHistory"
            >
              清除搜索历史
            </button>
          </div>
        </div>
      </div>
    </section>

    <!-- 搜索结果 -->
    <section
      v-if="hasSearched"
      style="background: linear-gradient(135deg, #f8fafc 0%, #e2e8f0 50%, #dbeafe 100%); padding: 2rem 0; margin: 0; position: relative; min-height: 60vh;"
    >
      <!-- 背景装饰 -->
      <div style="position: absolute; inset: 0; background-image: radial-gradient(circle at 1px 1px, rgba(59, 130, 246, 0.1) 1px, transparent 0); background-size: 60px 60px; opacity: 0.5;" />

      <div class="relative mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
        <!-- 搜索结果概览 -->
        <div
          class="modern-content-section mb-6"
          style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 1.5rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);"
        >
          <div class="flex items-center justify-between">
            <div>
              <h2 class="text-xl font-bold text-slate-900 dark:text-white">
                搜索结果：{{ currentSearchQuery }}
              </h2>
              <p class="text-slate-600 dark:text-slate-400 mt-1">
                找到 {{ totalResults }} 个相关结果
              </p>
            </div>

            <!-- 筛选器 -->
            <div class="flex items-center gap-3">
              <USelectMenu
                v-model="selectedCategory"
                :options="categoryOptions"
                size="sm"
                placeholder="筛选分类"
                style="min-width: 120px; background: rgba(248, 250, 252, 0.8); border: 1px solid rgba(226, 232, 240, 0.6);"
              />
              <USelectMenu
                v-model="sortOption"
                :options="sortOptions"
                size="sm"
                style="min-width: 120px; background: rgba(248, 250, 252, 0.8); border: 1px solid rgba(226, 232, 240, 0.6);"
              />
            </div>
          </div>
        </div>

        <!-- 搜索结果内容 -->
        <div
          v-if="isSearching"
          class="grid grid-cols-1 lg:grid-cols-3 gap-6"
        >
          <!-- 加载骨架 -->
          <div class="lg:col-span-2">
            <USkeleton class="h-96 w-full mb-4" />
          </div>
          <div>
            <USkeleton class="h-48 w-full mb-4" />
            <USkeleton class="h-48 w-full" />
          </div>
        </div>

        <!-- 搜索结果 -->
        <div
          v-else-if="totalResults > 0"
          class="grid grid-cols-1 lg:grid-cols-3 gap-6"
        >
          <!-- 主要搜索结果 -->
          <div class="lg:col-span-2 space-y-6">
            <!-- 文章结果 -->
            <div
              v-if="filteredPosts.length"
              class="modern-content-section"
              style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 1.5rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);"
            >
              <div class="modern-section-header mb-4">
                <div class="flex items-center gap-3">
                  <div
                    class="modern-gradient-icon"
                    style="background: linear-gradient(45deg, #3b82f6, #6366f1); width: 2rem; height: 2rem;"
                  >
                    <UIcon
                      name="i-heroicons-document-text"
                      class="h-4 w-4 text-white"
                    />
                  </div>
                  <h3 class="text-lg font-semibold text-slate-900 dark:text-white">
                    相关文章 ({{ filteredPosts.length }})
                  </h3>
                </div>
              </div>

              <div class="space-y-4">
                <div
                  v-for="post in filteredPosts.slice(0, displayLimit.posts)"
                  :key="post.id"
                  class="p-4 rounded-lg border border-slate-200 hover:border-blue-300 hover:shadow-md transition-all cursor-pointer group"
                  style="background: rgba(248, 250, 252, 0.5);"
                  @click="$router.push(`/posts/${post.slug}`)"
                >
                  <div class="flex items-start justify-between">
                    <div class="flex-1">
                      <h4 class="font-medium text-slate-900 dark:text-white group-hover:text-blue-600 transition-colors mb-1">
                        {{ post.title }}
                      </h4>
                      <p
                        v-if="post.excerpt"
                        class="text-sm text-slate-600 dark:text-slate-400 mb-2 line-clamp-2"
                      >
                        {{ post.excerpt }}
                      </p>
                      <div class="flex items-center gap-2 text-xs text-slate-500 dark:text-slate-400">
                        <span>{{ post.author.username }}</span>
                        <span>·</span>
                        <span>{{ formatDate(post.published_at || post.created_at) }}</span>
                        <div
                          v-if="post.categories.length"
                          class="flex items-center gap-1 ml-2"
                        >
                          <span
                            v-for="category in post.categories.slice(0, 2)"
                            :key="category.id"
                            class="px-2 py-1 rounded-full bg-blue-100 text-blue-700 dark:bg-blue-900 dark:text-blue-200"
                          >
                            {{ category.name }}
                          </span>
                        </div>
                      </div>
                    </div>
                    <UIcon
                      name="i-heroicons-arrow-right"
                      class="w-4 h-4 text-slate-400 group-hover:text-blue-500 transition-colors ml-2"
                    />
                  </div>
                </div>

                <button
                  v-if="filteredPosts.length > displayLimit.posts"
                  class="w-full text-center py-2 text-blue-600 hover:text-blue-700 text-sm font-medium"
                  @click="displayLimit.posts += 5"
                >
                  显示更多文章 ({{ filteredPosts.length - displayLimit.posts }})
                </button>
              </div>
            </div>
          </div>

          <!-- 侧边栏结果 -->
          <div class="space-y-6">
            <!-- 作者结果 -->
            <div
              v-if="searchResults.authors.length"
              class="modern-content-section"
              style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 1.5rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);"
            >
              <div class="modern-section-header mb-4">
                <div class="flex items-center gap-3">
                  <div
                    class="modern-gradient-icon"
                    style="background: linear-gradient(45deg, #10b981, #059669); width: 2rem; height: 2rem;"
                  >
                    <UIcon
                      name="i-heroicons-user"
                      class="h-4 w-4 text-white"
                    />
                  </div>
                  <h3 class="text-lg font-semibold text-slate-900 dark:text-white">
                    相关作者
                  </h3>
                </div>
              </div>

              <div class="space-y-3">
                <div
                  v-for="author in searchResults.authors.slice(0, displayLimit.authors)"
                  :key="author.id"
                  class="flex items-center gap-3 p-3 rounded-lg hover:bg-slate-100 dark:hover:bg-slate-700 cursor-pointer transition-colors"
                  @click="$router.push(`/authors/${author.username}`)"
                >
                  <div
                    class="w-10 h-10 rounded-full flex items-center justify-center text-white font-medium"
                    style="background: linear-gradient(45deg, #3b82f6, #6366f1);"
                  >
                    {{ getInitials(author.username) }}
                  </div>
                  <div class="flex-1">
                    <div class="font-medium text-slate-900 dark:text-white">
                      {{ author.username }}
                    </div>
                    <div class="text-sm text-slate-500 dark:text-slate-400">
                      {{ author.postCount }} 篇文章
                    </div>
                  </div>
                </div>

                <button
                  v-if="searchResults.authors.length > displayLimit.authors"
                  class="w-full text-center py-2 text-blue-600 hover:text-blue-700 text-sm font-medium"
                  @click="displayLimit.authors += 3"
                >
                  显示更多作者
                </button>
              </div>
            </div>

            <!-- 分类结果 -->
            <div
              v-if="searchResults.categories.length"
              class="modern-content-section"
              style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 1.5rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);"
            >
              <div class="modern-section-header mb-4">
                <div class="flex items-center gap-3">
                  <div
                    class="modern-gradient-icon"
                    style="background: linear-gradient(45deg, #f59e0b, #d97706); width: 2rem; height: 2rem;"
                  >
                    <UIcon
                      name="i-heroicons-squares-2x2"
                      class="h-4 w-4 text-white"
                    />
                  </div>
                  <h3 class="text-lg font-semibold text-slate-900 dark:text-white">
                    相关分类
                  </h3>
                </div>
              </div>

              <div class="space-y-2">
                <NuxtLink
                  v-for="category in searchResults.categories.slice(0, displayLimit.categories)"
                  :key="category.id"
                  :to="`/categories/${category.slug}`"
                  class="flex items-center justify-between p-2 rounded-lg hover:bg-slate-100 dark:hover:bg-slate-700 transition-colors"
                >
                  <span class="font-medium text-slate-900 dark:text-white">{{ category.name }}</span>
                  <span class="text-sm text-slate-500 dark:text-slate-400">{{ category.post_count || 0 }}</span>
                </NuxtLink>

                <button
                  v-if="searchResults.categories.length > displayLimit.categories"
                  class="w-full text-center py-2 text-blue-600 hover:text-blue-700 text-sm font-medium"
                  @click="displayLimit.categories += 5"
                >
                  显示更多分类
                </button>
              </div>
            </div>

            <!-- 标签结果 -->
            <div
              v-if="searchResults.tags.length"
              class="modern-content-section"
              style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 1.5rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);"
            >
              <div class="modern-section-header mb-4">
                <div class="flex items-center gap-3">
                  <div
                    class="modern-gradient-icon"
                    style="background: linear-gradient(45deg, #8b5cf6, #7c3aed); width: 2rem; height: 2rem;"
                  >
                    <UIcon
                      name="i-heroicons-hashtag"
                      class="h-4 w-4 text-white"
                    />
                  </div>
                  <h3 class="text-lg font-semibold text-slate-900 dark:text-white">
                    相关标签
                  </h3>
                </div>
              </div>

              <div class="flex flex-wrap gap-2">
                <NuxtLink
                  v-for="tag in searchResults.tags.slice(0, displayLimit.tags)"
                  :key="tag.id"
                  :to="`/tags/${tag.name}`"
                  class="inline-block px-3 py-1 text-sm rounded-full bg-purple-100 text-purple-700 hover:bg-purple-200 dark:bg-purple-900 dark:text-purple-200 dark:hover:bg-purple-800 transition-colors"
                >
                  #{{ tag.name }}
                </NuxtLink>
              </div>

              <button
                v-if="searchResults.tags.length > displayLimit.tags"
                class="w-full text-center py-2 text-blue-600 hover:text-blue-700 text-sm font-medium mt-3"
                @click="displayLimit.tags += 10"
              >
                显示更多标签
              </button>
            </div>
          </div>
        </div>

        <!-- 无结果状态 -->
        <div
          v-else-if="hasSearched && !isSearching"
          style="text-align: center; padding: 4rem 0;"
        >
          <div style="margin: 0 auto 2rem; width: 4rem; height: 4rem; display: flex; align-items: center; justify-content: center; border-radius: 50%; background: linear-gradient(45deg, #94a3b8, #cbd5e1); box-shadow: 0 8px 32px rgba(148, 163, 184, 0.2);">
            <UIcon
              name="i-heroicons-magnifying-glass"
              style="width: 2rem; height: 2rem; color: white;"
            />
          </div>
          <h3
            style="font-size: 1.5rem; font-weight: 700; color: #1e293b; margin-bottom: 1rem;"
            class="dark:text-white"
          >
            未找到相关内容
          </h3>
          <p
            style="font-size: 1rem; color: #64748b; margin-bottom: 2rem; line-height: 1.6;"
            class="dark:text-slate-300"
          >
            尝试使用不同的关键词或筛选条件
          </p>
          <div style="display: flex; gap: 1rem; flex-wrap: wrap; justify-content: center;">
            <button
              style="display: inline-flex; align-items: center; gap: 0.5rem; padding: 0.75rem 1.5rem; background: linear-gradient(45deg, #3b82f6, #6366f1); color: white; border-radius: 2rem; font-size: 0.875rem; font-weight: 600; border: none; cursor: pointer; transition: all 0.3s ease; box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);"
              class="hover:transform hover:scale-105 hover:shadow-lg"
              @click="clearSearch"
            >
              <UIcon
                name="i-heroicons-x-mark"
                class="w-4 h-4"
              />
              清除搜索
            </button>
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
    </section>
  </div>
</template>

<script setup lang="ts">
import type { PostWithContent, Category, Tag, Author } from "~/types";

// SEO 配置
useHead({
  title: "搜索",
  meta: [{ name: "description", content: "搜索文章、分类、标签、作者，快速找到您需要的内容" }],
});

// 状态管理
const route = useRoute();
const router = useRouter();
const searchInputRef = ref();
const searchQuery = ref((route.query.q as string) || "");
const currentSearchQuery = ref("");
const isSearching = ref(false);
const hasSearched = ref(false);
const selectedCategory = ref("");
const sortOption = ref("relevance");

// API 客户端
const api = useApi();

// 搜索结果
const searchResults = ref<{
  posts: PostWithContent[];
  categories: Category[];
  tags: Tag[];
  authors: Author[];
}>({
  posts: [],
  categories: [],
  tags: [],
  authors: [],
});

// 显示限制
const displayLimit = ref({
  posts: 5,
  categories: 5,
  tags: 10,
  authors: 3,
});

// 搜索历史
const searchHistory = ref<string[]>([]);

// 分类选项
const categoryOptions = ref([
  { label: "全部分类", value: "" },
]);

// 排序选项
const sortOptions = [
  { label: "相关性", value: "relevance" },
  { label: "最新发布", value: "created_at" },
  { label: "最近更新", value: "updated_at" },
  { label: "标题排序", value: "title" },
];

// 计算属性
const totalResults = computed(() => {
  return searchResults.value.posts.length
    + searchResults.value.categories.length
    + searchResults.value.tags.length
    + searchResults.value.authors.length;
});

const filteredPosts = computed(() => {
  let posts = [...searchResults.value.posts];

  // 分类筛选
  if (selectedCategory.value) {
    posts = posts.filter(post =>
      post.categories.some(cat => cat.name === selectedCategory.value),
    );
  }

  // 排序
  posts.sort((a, b) => {
    switch (sortOption.value) {
      case "created_at":
        return new Date(b.created_at).getTime() - new Date(a.created_at).getTime();
      case "updated_at":
        return new Date(b.updated_at).getTime() - new Date(a.updated_at).getTime();
      case "title":
        return a.title.localeCompare(b.title);
      case "relevance":
      default: {
        // 简单的相关性排序：标题包含关键词的排在前面
        const queryLower = currentSearchQuery.value.toLowerCase();
        const aRelevant = a.title.toLowerCase().includes(queryLower);
        const bRelevant = b.title.toLowerCase().includes(queryLower);
        if (aRelevant && !bRelevant) return -1;
        if (!aRelevant && bRelevant) return 1;
        return new Date(b.created_at).getTime() - new Date(a.created_at).getTime();
      }
    }
  });

  return posts;
});

// 方法
const performSearch = async () => {
  if (!searchQuery.value.trim() || isSearching.value) return;

  isSearching.value = true;
  hasSearched.value = true;
  currentSearchQuery.value = searchQuery.value.trim();

  // 保存搜索历史
  saveSearchHistory(currentSearchQuery.value);

  // 更新URL
  await router.push({
    path: "/search",
    query: { q: currentSearchQuery.value },
  });

  try {
    // 并行搜索所有类型的数据
    const [postsResponse, categoriesResponse, tagsResponse, authorsResponse] = await Promise.all([
      // 搜索文章
      api.getPosts({
        search: currentSearchQuery.value,
        per_page: 50,
        status: "published",
      }),
      // 搜索分类
      api.getCategories({
        search: currentSearchQuery.value,
        per_page: 20,
      }),
      // 搜索标签
      api.getTags({
        search: currentSearchQuery.value,
        per_page: 30,
      }),
      // 获取所有文章来搜索作者
      api.getPosts({
        per_page: 1000,
        status: "published",
      }),
    ]);

    // 处理文章结果
    searchResults.value.posts = postsResponse.success && postsResponse.data
      ? postsResponse.data.data
      : [];

    // 处理分类结果
    searchResults.value.categories = categoriesResponse.success && categoriesResponse.data
      ? categoriesResponse.data.data.filter(category =>
          category.name.toLowerCase().includes(currentSearchQuery.value.toLowerCase()),
        )
      : [];

    // 处理标签结果
    searchResults.value.tags = tagsResponse.success && tagsResponse.data
      ? tagsResponse.data.data.filter(tag =>
          tag.name.toLowerCase().includes(currentSearchQuery.value.toLowerCase()),
        )
      : [];

    // 处理作者结果 - 从文章数据中提取并搜索
    if (authorsResponse.success && authorsResponse.data) {
      const allPosts = authorsResponse.data.data;
      const authorMap = new Map<string, Author>();
      const queryLower = currentSearchQuery.value.toLowerCase();

      allPosts.forEach((post) => {
        const authorId = post.author.id;

        // 检查作者名是否匹配搜索关键词
        if (post.author.username.toLowerCase().includes(queryLower)) {
          if (!authorMap.has(authorId)) {
            authorMap.set(authorId, {
              id: post.author.id,
              username: post.author.username,
              email: post.author.email,
              bio: `专注于内容创作的作者`,
              created_at: post.created_at,
              postCount: 1,
              categories: [],
              tags: [],
              lastActiveAt: post.published_at || post.created_at,
            });
          }
          else {
            const author = authorMap.get(authorId)!;
            author.postCount += 1;
            if (new Date(post.published_at || post.created_at) > new Date(author.lastActiveAt)) {
              author.lastActiveAt = post.published_at || post.created_at;
            }
          }
        }
      });

      searchResults.value.authors = Array.from(authorMap.values());
    }
    else {
      searchResults.value.authors = [];
    }

    // 更新分类筛选选项
    const allCategories = new Set<string>();
    searchResults.value.posts.forEach((post) => {
      post.categories.forEach(cat => allCategories.add(cat.name));
    });

    categoryOptions.value = [
      { label: "全部分类", value: "" },
      ...Array.from(allCategories).map(name => ({
        label: name,
        value: name,
      })),
    ];
  }
  catch (error) {
    console.error("Search error:", error);
  }
  finally {
    isSearching.value = false;
  }
};

const clearSearch = () => {
  searchQuery.value = "";
  currentSearchQuery.value = "";
  hasSearched.value = false;
  searchResults.value = {
    posts: [],
    categories: [],
    tags: [],
    authors: [],
  };

  // 重置显示限制
  displayLimit.value = {
    posts: 5,
    categories: 5,
    tags: 10,
    authors: 3,
  };

  router.push({ path: "/search" });
};

const searchFromHistory = (query: string) => {
  searchQuery.value = query;
  performSearch();
};

const saveSearchHistory = (query: string) => {
  if (!query.trim()) return;

  const history = [...searchHistory.value];
  const index = history.indexOf(query);

  if (index > -1) {
    history.splice(index, 1);
  }

  history.unshift(query);
  searchHistory.value = history.slice(0, 10); // 保留最新的10个搜索记录

  // 持久化存储
  if (typeof window !== "undefined") {
    localStorage.setItem("search_history", JSON.stringify(searchHistory.value));
  }
};

const removeFromHistory = (index: number) => {
  searchHistory.value.splice(index, 1);
  if (typeof window !== "undefined") {
    localStorage.setItem("search_history", JSON.stringify(searchHistory.value));
  }
};

const clearSearchHistory = () => {
  searchHistory.value = [];
  if (typeof window !== "undefined") {
    localStorage.removeItem("search_history");
  }
};

const getInitials = (username: string): string => {
  return username.slice(0, 2).toUpperCase();
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

// 生命周期
onMounted(() => {
  // 加载搜索历史
  if (typeof window !== "undefined") {
    const stored = localStorage.getItem("search_history");
    if (stored) {
      try {
        searchHistory.value = JSON.parse(stored);
      }
      catch {
        // 忽略解析错误
      }
    }
  }

  // 如果URL中有搜索参数，自动执行搜索
  if (searchQuery.value) {
    performSearch();
  }

  // 聚焦搜索框
  nextTick(() => {
    if (searchInputRef.value) {
      searchInputRef.value.$refs.input?.focus();
    }
  });
});

// 监听路由变化
watch(() => route.query.q, (newQuery) => {
  if (newQuery && typeof newQuery === "string") {
    searchQuery.value = newQuery;
    performSearch();
  }
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

.modern-gradient-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  transition: all 0.3s ease;
}

.modern-post-card:hover {
  transform: translateY(-2px);
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.animate-spin {
  animation: spin 1s linear infinite;
}
</style>
