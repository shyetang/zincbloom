<template>
  <div class="min-h-screen bg-gray-50 dark:bg-gray-900">
    <!-- 加载状态 -->
    <div
      v-if="pending"
      class="container-blog py-8"
    >
      <div class="max-w-4xl mx-auto">
        <USkeleton class="h-8 w-3/4 mb-4" />
        <USkeleton class="h-4 w-1/2 mb-8" />
        <USkeleton class="h-64 w-full mb-8" />
        <USkeleton class="h-4 w-full mb-2" />
        <USkeleton class="h-4 w-full mb-2" />
        <USkeleton class="h-4 w-3/4 mb-2" />
      </div>
    </div>

    <!-- 错误状态 -->
    <div
      v-else-if="error"
      class="container-blog py-16"
    >
      <div class="text-center">
        <UIcon
          name="i-heroicons-exclamation-triangle"
          class="w-16 h-16 text-red-500 mx-auto mb-4"
        />
        <h1
          class="text-2xl font-bold text-gray-900 dark:text-white mb-2"
        >
          文章加载失败
        </h1>
        <p class="text-gray-600 dark:text-gray-400 mb-6">
          {{ error.message || "未知错误" }}
        </p>
        <UButton
          to="/posts"
          icon="i-heroicons-arrow-left"
        >
          返回文章列表
        </UButton>
      </div>
    </div>

    <!-- 文章内容 -->
    <article
      v-else-if="post"
      class="container-blog py-8"
    >
      <div class="max-w-4xl mx-auto">
        <!-- 文章头部 -->
        <header class="mb-8">
          <!-- 面包屑导航 -->
          <nav
            class="flex items-center space-x-2 text-sm text-gray-500 dark:text-gray-400 mb-4"
          >
            <NuxtLink
              to="/"
              class="hover:text-primary-600"
            >首页</NuxtLink>
            <UIcon
              name="i-heroicons-chevron-right"
              class="w-4 h-4"
            />
            <NuxtLink
              to="/posts"
              class="hover:text-primary-600"
            >文章</NuxtLink>
            <UIcon
              name="i-heroicons-chevron-right"
              class="w-4 h-4"
            />
            <span class="text-gray-700 dark:text-gray-300">{{
              post.title
            }}</span>
          </nav>

          <!-- 标题 -->
          <h1
            class="text-3xl md:text-4xl lg:text-5xl font-bold text-gray-900 dark:text-white mb-4"
          >
            {{ post.title }}
          </h1>

          <!-- 文章元信息 -->
          <div
            class="flex flex-wrap items-center gap-4 text-sm text-gray-600 dark:text-gray-400"
          >
            <!-- 作者 -->
            <div class="flex items-center gap-2">
              <div
                class="w-8 h-8 bg-primary-100 dark:bg-primary-900 rounded-full flex items-center justify-center"
              >
                <UIcon
                  name="i-heroicons-user"
                  class="w-4 h-4 text-primary-600 dark:text-primary-400"
                />
              </div>
              <span class="font-medium">{{
                post.author.username
              }}</span>
            </div>

            <!-- 发布时间 -->
            <div class="flex items-center gap-2">
              <UIcon
                name="i-heroicons-clock"
                class="w-4 h-4"
              />
              <span>{{
                formatDate(post.published_at || post.created_at)
              }}</span>
            </div>

            <!-- 阅读时间 -->
            <div class="flex items-center gap-2">
              <UIcon
                name="i-heroicons-book-open"
                class="w-4 h-4"
              />
              <span>{{ readingTime }} 分钟阅读</span>
            </div>
          </div>

          <!-- 分类和标签 -->
          <div class="flex flex-wrap items-center gap-4 mt-4">
            <!-- 分类 -->
            <div
              v-if="post.categories?.length"
              class="flex items-center gap-2"
            >
              <UBadge
                v-for="category in post.categories"
                :key="category.id"
                :label="category.name"
                color="primary"
                variant="soft"
              />
            </div>

            <!-- 标签 -->
            <div
              v-if="post.tags?.length"
              class="flex flex-wrap gap-2"
            >
              <UBadge
                v-for="tag in post.tags"
                :key="tag.id"
                :label="`#${tag.name}`"
                color="neutral"
                variant="soft"
                size="sm"
              />
            </div>
          </div>
        </header>

        <!-- 文章内容区域 -->
        <div class="md:flex md:gap-8">
          <!-- 主内容区 -->
          <main class="md:flex-1 min-w-0">
            <!-- 文章封面 -->
            <div
              v-if="post.thumbnail"
              class="mb-8"
            >
              <img
                :src="post.thumbnail"
                :alt="post.title"
                class="w-full max-h-96 object-cover rounded-lg shadow-lg"
                loading="lazy"
              >
            </div>

            <!-- Markdown 渲染 -->
            <div class="mb-8">
              <MarkdownRenderer
                v-if="post?.content"
                :content="post.content"
                :show-toc="false"
              />
              <div
                v-else
                class="text-gray-500 italic border border-gray-300 p-4 rounded"
              >
                暂无文章内容
              </div>
            </div>

            <!-- 文章底部信息 -->
            <footer
              class="border-t border-gray-200 dark:border-gray-700 pt-8"
            >
              <div class="flex items-center justify-between">
                <div
                  class="text-sm text-gray-500 dark:text-gray-400"
                >
                  最后更新于 {{ formatDate(post.updated_at) }}
                </div>

                <!-- 分享按钮 -->
                <div class="flex items-center gap-2">
                  <UButton
                    variant="outline"
                    size="sm"
                    icon="i-heroicons-share"
                    @click="sharePost"
                  >
                    分享
                  </UButton>
                  <UButton
                    variant="outline"
                    size="sm"
                    icon="i-heroicons-heart"
                    @click="likePost"
                  >
                    点赞
                  </UButton>
                </div>
              </div>
            </footer>
          </main>

          <!-- 侧边栏 -->
          <aside class="md:w-64 mt-8 md:mt-0">
            <!-- 目录 -->
            <div class="sticky top-8 space-y-6">
              <div
                class="bg-white dark:bg-gray-800 rounded-lg p-4 border border-gray-200 dark:border-gray-700"
              >
                <h3
                  class="font-semibold text-gray-900 dark:text-white mb-4 flex items-center"
                >
                  <UIcon
                    name="i-heroicons-list-bullet"
                    class="w-5 h-5 mr-2"
                  />
                  文章目录
                </h3>
                <TocTree :items="toc" />
                <div
                  v-if="toc.length === 0"
                  class="text-gray-500 text-sm"
                >
                  目录数据为空 (调试信息)
                </div>
                <div class="text-xs text-gray-400 mt-2">
                  目录项数量: {{ toc.length }}
                </div>
              </div>

              <!-- 相关文章 -->
              <div
                v-if="relatedPosts?.length"
                class="bg-white dark:bg-gray-800 rounded-lg p-4 border border-gray-200 dark:border-gray-700"
              >
                <h3
                  class="font-semibold text-gray-900 dark:text-white mb-4 flex items-center"
                >
                  <UIcon
                    name="i-heroicons-document-duplicate"
                    class="w-5 h-5 mr-2"
                  />
                  相关文章
                </h3>
                <div class="space-y-3">
                  <NuxtLink
                    v-for="relatedPost in relatedPosts"
                    :key="relatedPost.id"
                    :to="`/posts/${relatedPost.slug}`"
                    class="block p-3 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors"
                  >
                    <h4
                      class="font-medium text-sm text-gray-900 dark:text-white line-clamp-2"
                    >
                      {{ relatedPost.title }}
                    </h4>
                    <p
                      class="text-xs text-gray-500 dark:text-gray-400 mt-1"
                    >
                      {{
                        formatDate(
                          relatedPost.published_at
                            || relatedPost.created_at,
                        )
                      }}
                    </p>
                  </NuxtLink>
                </div>
              </div>
            </div>
          </aside>
        </div>
      </div>
    </article>

    <!-- 未找到文章 -->
    <div
      v-else
      class="container-blog py-16"
    >
      <div class="text-center">
        <UIcon
          name="i-heroicons-document-text"
          class="w-16 h-16 text-gray-400 mx-auto mb-4"
        />
        <h1
          class="text-2xl font-bold text-gray-900 dark:text-white mb-2"
        >
          文章不存在
        </h1>
        <p class="text-gray-600 dark:text-gray-400 mb-6">
          您访问的文章可能已被删除或不存在
        </p>
        <UButton
          to="/posts"
          icon="i-heroicons-arrow-left"
        >
          返回文章列表
        </UButton>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import MarkdownRenderer from "~/components/common/MarkdownRenderer.vue";
import TocTree from "~/components/common/TocTree.vue";

// 获取路由参数
const route = useRoute();
const slug = route.params.slug as string;

// 获取文章数据
const api = useApi();

// 使用 useLazyAsyncData 获取文章数据
const {
  data: post,
  pending,
  error,
} = await useLazyAsyncData(
  `post-${slug}`,
  async () => {
    const response = await api.getPost(slug);
    if (response.success) {
      return response.data;
    }
    throw createError({
      statusCode: 404,
      statusMessage: "文章不存在",
    });
  },
  {
    // 强制客户端渲染，避免SSR时的API调用问题
    server: false,
    // 默认值
    default: () => null,
  },
);

// 获取相关文章 - 依赖于主文章加载完成
const { data: relatedPosts } = await useLazyAsyncData(
  `related-posts-${slug}`,
  async () => {
    // 只有在主文章加载完成且有分类信息时才获取相关文章
    if (
      !post.value
      || !post.value.categories
      || post.value.categories.length === 0
    ) {
      return [];
    }

    const response = await api.getPosts({
      per_page: 5,
      status: "published",
      category: post.value.categories[0].slug,
    });
    if (response.success) {
      return response.data?.data || [];
    }
    return [];
  },
  {
    // 当主文章数据变化时重新获取相关文章
    server: false,
    default: () => [],
  },
);

// 计算阅读时间
const readingTime = computed(() => {
  if (!post.value?.content) return 0;

  const wordsPerMinute = 200; // 中文阅读速度（字符/分钟）
  const content = post.value.content;
  const wordCount = content.length;
  const minutes = Math.ceil(wordCount / wordsPerMinute);

  return Math.max(1, minutes);
});

// 使用 Markdown 渲染器获取目录
const { render } = useMarkdown({ toc: true });
const toc = computed(() => {
  if (!post.value?.content) {
    console.log("页面TOC: 没有文章内容");
    return [];
  }

  try {
    const result = render(post.value.content);
    console.log("页面TOC: 提取结果", result.toc);
    return result.toc;
  }
  catch (error) {
    console.error("目录提取失败:", error);
    return [];
  }
});

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

// 分享文章
const sharePost = async () => {
  if (!post.value) return;

  try {
    if (navigator.share) {
      await navigator.share({
        title: post.value.title,
        text:
                    post.value.excerpt
                    || `来自 ZincBloom 的文章：${post.value.title}`,
        url: window.location.href,
      });
    }
    else {
      // 复制链接到剪贴板
      await navigator.clipboard.writeText(window.location.href);
      useToast().add({
        title: "链接已复制",
        description: "文章链接已复制到剪贴板",
        icon: "i-heroicons-check-circle",
        color: "success",
      });
    }
  }
  catch (error) {
    console.error("分享失败:", error);
  }
};

// 点赞文章（占位符功能）
const likePost = () => {
  useToast().add({
    title: "感谢点赞！",
    description: "您的支持是我们前进的动力",
    icon: "i-heroicons-heart",
    color: "success",
  });
};

// SEO 配置
watchEffect(() => {
  if (post.value) {
    useHead({
      title: post.value.title,
      meta: [
        {
          name: "description",
          content:
                        post.value.excerpt
                        || `来自 ZincBloom 的文章：${post.value.title}`,
        },
        { property: "og:title", content: post.value.title },
        {
          property: "og:description",
          content:
                        post.value.excerpt
                        || `来自 ZincBloom 的文章：${post.value.title}`,
        },
        {
          property: "og:image",
          content: post.value.thumbnail || "/og-image.jpg",
        },
        { property: "og:type", content: "article" },
        { property: "og:url", content: window.location.href },
        { name: "twitter:card", content: "summary_large_image" },
        { name: "twitter:title", content: post.value.title },
        {
          name: "twitter:description",
          content:
                        post.value.excerpt
                        || `来自 ZincBloom 的文章：${post.value.title}`,
        },
        {
          name: "twitter:image",
          content: post.value.thumbnail || "/og-image.jpg",
        },
        { name: "article:author", content: post.value.author.username },
        {
          name: "article:published_time",
          content: post.value.published_at || post.value.created_at,
        },
        {
          name: "article:modified_time",
          content: post.value.updated_at,
        },
      ],
    });
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
</style>
