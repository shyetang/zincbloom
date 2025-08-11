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
          <div style="margin: 0 auto 1rem; width: 2.5rem; height: 2.5rem; display: flex; align-items: center; justify-content: center; border-radius: 50%; background: linear-gradient(45deg, #3b82f6, #8b5cf6); box-shadow: 0 6px 24px rgba(59, 130, 246, 0.3);">
            <UIcon
              name="i-heroicons-document-text"
              style="width: 1.25rem; height: 1.25rem; color: white;"
            />
          </div>

          <!-- 现代化标题 -->
          <h1
            style="font-size: 1.875rem; font-weight: 700; line-height: 1.1; margin-bottom: 0.75rem; color: #1e293b;"
            class="dark:text-white"
          >
            发现
            <span
              class="modern-text-gradient"
              style="font-weight: 800;"
            >精彩文章</span>
          </h1>

          <!-- 副标题 -->
          <p
            style="font-size: 1rem; color: #64748b; max-width: 28rem; margin: 0 auto; line-height: 1.5;"
            class="dark:text-slate-300"
          >
            探索社区中最有价值的内容，与优秀的创作者们一起成长
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
          <span style="color: #3b82f6; font-weight: 500;">文章列表</span>
        </div>
      </div>
    </nav>

    <!-- 主要内容区域 -->
    <section
      style="background: linear-gradient(135deg, #f8fafc 0%, #e2e8f0 50%, #eff6ff 100%); padding: 2rem 0; margin: 0; position: relative;"
    >
      <!-- 背景装饰 -->
      <div style="position: absolute; inset: 0; background-image: radial-gradient(circle at 1px 1px, rgba(148, 163, 184, 0.1) 1px, transparent 0); background-size: 60px 60px; opacity: 0.5;" />

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
              {{ error.message || "获取文章列表时发生错误" }}
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

        <!-- 文章列表 -->
        <div
          class="modern-content-section"
          style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 1.5rem; margin-bottom: 1rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);"
        >
          <div
            class="modern-section-header"
            style="flex-direction: column; align-items: stretch; gap: 1.5rem;"
          >
            <!-- 标题区域 -->
            <div style="display: flex; align-items: center; justify-content: space-between;">
              <div>
                <div class="flex items-center gap-3 mb-2">
                  <div
                    class="modern-gradient-icon"
                    style="background: linear-gradient(45deg, #3b82f6, #6366f1); width: 2.5rem; height: 2.5rem;"
                  >
                    <UIcon
                      name="i-heroicons-squares-2x2"
                      class="h-5 w-5 text-white"
                    />
                  </div>
                  <h2 class="modern-section-title">
                    文章列表
                  </h2>
                </div>
                <p class="modern-section-subtitle">
                  {{ posts?.pagination?.total || 0 }} 篇文章等待您的发现
                </p>
              </div>

              <!-- 快速搜索 -->
              <div style="min-width: 240px;">
                <UInput
                  v-model="searchQuery"
                  icon="i-heroicons-magnifying-glass"
                  placeholder="快速搜索..."
                  size="sm"
                  style="background: rgba(248, 250, 252, 0.8); border: 1px solid rgba(226, 232, 240, 0.6);"
                  class="focus:border-blue-500"
                  @keyup.enter="handleSearch"
                />
              </div>
            </div>

            <!-- 筛选控件行 -->
            <div style="display: flex; flex-direction: column; gap: 1rem;">
              <!-- 主要筛选控件 -->
              <div style="display: flex; flex-wrap: wrap; align-items: center; gap: 1rem; padding: 1rem; background: rgba(248, 250, 252, 0.5); border-radius: 0.75rem; border: 1px solid rgba(226, 232, 240, 0.4);">
                <!-- 分类筛选 -->
                <div style="display: flex; align-items: center; gap: 0.5rem;">
                  <span style="font-size: 0.875rem; color: #64748b; font-weight: 500;">分类:</span>
                  <USelectMenu
                    v-model="selectedCategory"
                    :options="categoryOptions"
                    placeholder="全部"
                    size="sm"
                    style="min-width: 120px; background: white; border: 1px solid rgba(226, 232, 240, 0.6);"
                    clear-search-on-close
                  />
                </div>

                <!-- 排序选择 -->
                <div style="display: flex; align-items: center; gap: 0.5rem;">
                  <span style="font-size: 0.875rem; color: #64748b; font-weight: 500;">排序:</span>
                  <USelectMenu
                    v-model="sortOption"
                    :options="sortOptions"
                    size="sm"
                    style="min-width: 120px; background: white; border: 1px solid rgba(226, 232, 240, 0.6);"
                  />
                </div>

                <!-- 搜索按钮 -->
                <button
                  style="display: flex; align-items: center; gap: 0.5rem; padding: 0.5rem 1rem; background: linear-gradient(45deg, #3b82f6, #6366f1); color: white; border-radius: 0.5rem; font-size: 0.875rem; font-weight: 500; border: none; cursor: pointer; transition: all 0.2s ease;"
                  class="hover:transform hover:scale-105"
                  @click="handleSearch"
                >
                  <UIcon
                    name="i-heroicons-magnifying-glass"
                    class="w-4 h-4"
                  />
                  搜索
                </button>

                <!-- 清除筛选 -->
                <button
                  v-if="searchQuery || selectedCategory || selectedTags.length"
                  style="padding: 0.5rem 1rem; background: rgba(255, 255, 255, 0.8); color: #64748b; border-radius: 0.5rem; font-size: 0.875rem; border: 1px solid rgba(226, 232, 240, 0.6); cursor: pointer; transition: all 0.2s ease;"
                  class="hover:bg-white hover:text-slate-900"
                  @click="clearFilters"
                >
                  清除筛选
                </button>
              </div>

              <!-- 标签云 -->
              <div
                v-if="popularTags.length"
                style="display: flex; align-items: center; gap: 0.75rem; flex-wrap: wrap; padding: 0.75rem 1rem; background: rgba(255, 255, 255, 0.6); border-radius: 0.75rem; border: 1px solid rgba(226, 232, 240, 0.3);"
              >
                <div style="display: flex; align-items: center; gap: 0.5rem; margin-right: 0.5rem;">
                  <UIcon
                    name="i-heroicons-hashtag"
                    class="w-4 h-4 text-slate-500"
                  />
                  <span style="font-size: 0.875rem; color: #64748b; font-weight: 500; white-space: nowrap;">热门标签:</span>
                </div>
                <div style="display: flex; flex-wrap: wrap; gap: 0.5rem;">
                  <button
                    v-for="tag in popularTags.slice(0, 8)"
                    :key="tag.id"
                    :style="{
                      padding: '0.25rem 0.75rem',
                      borderRadius: '1rem',
                      fontSize: '0.75rem',
                      fontWeight: '500',
                      border: '1px solid',
                      borderColor: selectedTags.includes(tag.id) ? '#3b82f6' : 'rgba(226, 232, 240, 0.6)',
                      background: selectedTags.includes(tag.id) ? 'linear-gradient(45deg, #3b82f6, #6366f1)' : 'rgba(255, 255, 255, 0.8)',
                      color: selectedTags.includes(tag.id) ? 'white' : '#64748b',
                      cursor: 'pointer',
                      transition: 'all 0.2s ease',
                    }"
                    class="hover:transform hover:scale-105"
                    @click="toggleTag(tag.id)"
                  >
                    #{{ tag.name }}
                  </button>
                </div>
              </div>
            </div>
          </div>

          <!-- 加载状态 -->
          <div
            v-if="pending"
            class="modern-grid-cards"
          >
            <div
              v-for="i in 9"
              :key="i"
              class="modern-post-card"
              style="height: 280px; display: flex; align-items: center; justify-content: center;"
            >
              <USkeleton class="h-64 w-full" />
            </div>
          </div>

          <!-- 文章网格 -->
          <div
            v-else-if="posts?.data?.length"
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
            <div style="display: flex; justify-content: center; margin-top: 3rem;">
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
              没有找到文章
            </h3>
            <p
              style="font-size: 1rem; color: #64748b; margin-bottom: 2rem; line-height: 1.6;"
              class="dark:text-slate-300"
            >
              尝试调整搜索条件或
              <button
                style="color: #3b82f6; font-weight: 500; background: none; border: none; cursor: pointer; text-decoration: underline;"
                class="hover:text-blue-700"
                @click="clearFilters"
              >
                清除筛选
              </button>
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
                <a
                  href="/write"
                  style="display: inline-flex; align-items: center; gap: 0.5rem; padding: 0.75rem 1.5rem; background: linear-gradient(45deg, #3b82f6, #6366f1); color: white; text-decoration: none; border-radius: 2rem; font-size: 0.875rem; font-weight: 600; transition: all 0.3s ease; box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);"
                  class="hover:transform hover:scale-105 hover:shadow-lg"
                >
                  <UIcon
                    name="i-heroicons-pencil-square"
                    class="w-4 h-4"
                  />
                  写第一篇文章
                </a>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import type { PostQueryParams, Category } from "~/types";

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
  }),
);

// 获取文章数据
const api = useApi();
const {
  data: posts,
  pending,
  refresh,
  error,
} = await useLazyAsyncData(
  "posts",
  async () => {
    const response = await api.getPosts(queryParams.value);
    if (response.success) {
      return response.data;
    }
    throw createError({
      statusCode: 500,
      statusMessage: response.error?.message || "获取文章列表失败",
    });
  },
  {
    watch: [queryParams],
  },
);

// 获取分类选项
const { data: categories } = await useLazyAsyncData(
  "categories",
  async () => {
    const response = await api.getCategories({ per_page: 100 });
    if (response.success) {
      return response.data;
    }
    return {
      data: [],
      pagination: { page: 1, per_page: 100, total: 0, total_pages: 0 },
    };
  },
);

const categoryOptions = computed(() => [
  { label: "全部分类", value: "" },
  ...(categories.value?.data?.map((cat: Category) => ({
    label: cat.name,
    value: cat.slug,
  })) || []),
]);

// 获取热门标签
const { data: tagsData } = await useLazyAsyncData(
  "tags",
  async () => {
    const response = await api.getTags({ per_page: 10, sort: "post_count", order: "desc" });
    if (response.success) {
      return response.data;
    }
    return {
      data: [],
      pagination: { page: 1, per_page: 10, total: 0, total_pages: 0 },
    };
  },
);

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
  }
  else {
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
