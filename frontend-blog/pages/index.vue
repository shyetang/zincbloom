<template>
  <div>
    <!-- Hero 区域 -->
    <section
      class="modern-hero"
      style="padding: 4rem 0 3rem 0"
    >
      <div class="relative mx-auto max-w-4xl px-4 sm:px-6 lg:px-8">
        <div style="text-align: center">
          <!-- 现代化图标 -->
          <div
            style="
                            margin: 0 auto 1.5rem;
                            width: 3rem;
                            height: 3rem;
                            display: flex;
                            align-items: center;
                            justify-content: center;
                            border-radius: 50%;
                            background: linear-gradient(
                                45deg,
                                #3b82f6,
                                #8b5cf6
                            );
                            box-shadow: 0 6px 24px rgba(59, 130, 246, 0.3);
                        "
          >
            <UIcon
              name="i-heroicons-sparkles"
              style="width: 1.5rem; height: 1.5rem; color: white"
            />
          </div>

          <!-- 现代化标题 -->
          <h1
            style="
                            font-size: 2.75rem;
                            font-weight: 700;
                            line-height: 1.1;
                            margin-bottom: 1rem;
                            color: #1e293b;
                        "
            class="dark:text-white"
          >
            欢迎来到
            <span
              class="modern-text-gradient"
              style="font-weight: 800"
            >ZincBloom</span>
            博客社区
          </h1>

          <!-- 副标题 -->
          <p
            style="
                            font-size: 1.125rem;
                            color: #64748b;
                            max-width: 32rem;
                            margin: 0 auto 2rem;
                            line-height: 1.5;
                        "
            class="dark:text-slate-300"
          >
            一个现代化的知识分享平台，让创作者与读者在优雅的环境中交流思想，共同成长。
          </p>

          <!-- 现代化按钮 -->
          <div
            style="
                            display: flex;
                            gap: 0.75rem;
                            justify-content: center;
                            flex-wrap: wrap;
                        "
          >
            <a
              href="/posts"
              class="modern-button modern-button-primary"
              style="gap: 0.5rem; padding: 0.625rem 1.25rem"
            >
              <UIcon
                name="i-heroicons-book-open"
                style="width: 1rem; height: 1rem"
              />
              浏览精彩文章
            </a>
            <a
              href="/write"
              class="modern-button modern-button-secondary"
              style="gap: 0.5rem; padding: 0.625rem 1.25rem"
            >
              <UIcon
                name="i-heroicons-pencil-square"
                style="width: 1rem; height: 1rem"
              />
              开始创作
            </a>
          </div>
        </div>
      </div>
    </section>

    <!-- 主要内容区域 -->
    <section
      class="modern-content-section"
      style="
                background: linear-gradient(
                    135deg,
                    #f8fafc 0%,
                    #e2e8f0 50%,
                    #eff6ff 100%
                );
                padding: 2rem 0;
                margin: 0;
            "
    >
      <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
        <div class="layout-container">
          <!-- 主内容区 -->
          <div class="main-content">
            <!-- 编辑推荐 -->
            <div
              class="modern-content-section"
              style="
                                background: rgba(255, 255, 255, 0.9);
                                border-radius: 1rem;
                                padding: 1.5rem;
                                margin-bottom: 1rem;
                                border: 1px solid #e2e8f0;
                                box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
                            "
            >
              <div class="modern-section-header">
                <div>
                  <div class="flex items-center gap-3 mb-2">
                    <div
                      class="modern-gradient-icon"
                      style="
                                                background: linear-gradient(
                                                    45deg,
                                                    #f59e0b,
                                                    #f97316
                                                );
                                                width: 2.5rem;
                                                height: 2.5rem;
                                            "
                    >
                      <UIcon
                        name="i-heroicons-star"
                        class="h-5 w-5 text-white"
                      />
                    </div>
                    <h2 class="modern-section-title">
                      编辑推荐
                    </h2>
                  </div>
                  <p class="modern-section-subtitle">
                    我们为您精心挑选的优质内容
                  </p>
                </div>
                <UButton
                  to="/posts?featured=true"
                  variant="ghost"
                  size="sm"
                  class="text-slate-600 hover:text-slate-900 dark:text-slate-400 dark:hover:text-white"
                  trailing-icon="i-heroicons-arrow-right"
                >
                  查看更多
                </UButton>
              </div>

              <!-- 推荐文章 -->
              <div
                v-if="featuredPending"
                class="modern-grid-cards"
              >
                <div
                  v-for="i in 4"
                  :key="i"
                  class="modern-post-card"
                  style="
                                        height: 250px;
                                        display: flex;
                                        align-items: center;
                                        justify-content: center;
                                    "
                >
                  <USkeleton class="h-64 w-full" />
                </div>
              </div>

              <div
                v-else-if="
                  Array.isArray(featuredPosts)
                    && featuredPosts.length
                "
                class="modern-grid-cards"
              >
                <div
                  v-for="post in featuredPosts"
                  :key="post.id"
                  class="modern-post-card"
                >
                  <PostCard :post="post" />
                </div>
              </div>

              <div
                v-else
                class="text-center py-12"
              >
                <UIcon
                  name="i-heroicons-star"
                  class="w-16 h-16 text-gray-400 mx-auto mb-4"
                />
                <p class="text-gray-600 dark:text-gray-400">
                  暂无推荐文章
                </p>
              </div>
            </div>

            <!-- 最新文章 -->
            <div
              class="modern-content-section"
              style="
                                background: rgba(255, 255, 255, 0.9);
                                border-radius: 1rem;
                                padding: 1.5rem;
                                margin-bottom: 1rem;
                                border: 1px solid #e2e8f0;
                                box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
                            "
            >
              <div class="modern-section-header">
                <div>
                  <div class="flex items-center gap-3 mb-2">
                    <div
                      class="modern-gradient-icon"
                      style="
                                                background: linear-gradient(
                                                    45deg,
                                                    #3b82f6,
                                                    #6366f1
                                                );
                                                width: 2.5rem;
                                                height: 2.5rem;
                                            "
                    >
                      <UIcon
                        name="i-heroicons-newspaper"
                        class="h-5 w-5 text-white"
                      />
                    </div>
                    <h2 class="modern-section-title">
                      最新文章
                    </h2>
                  </div>
                  <p class="modern-section-subtitle">
                    发现社区最新的精彩内容
                  </p>
                </div>
                <UButton
                  to="/posts"
                  variant="ghost"
                  size="sm"
                  class="text-slate-600 hover:text-slate-900 dark:text-slate-400 dark:hover:text-white"
                  trailing-icon="i-heroicons-arrow-right"
                >
                  查看全部
                </UButton>
              </div>

              <!-- 文章列表 -->
              <div
                v-if="pending"
                class="modern-grid-cards"
              >
                <div
                  v-for="i in 6"
                  :key="i"
                  class="modern-post-card"
                  style="
                                        height: 250px;
                                        display: flex;
                                        align-items: center;
                                        justify-content: center;
                                    "
                >
                  <USkeleton class="h-64 w-full" />
                </div>
              </div>

              <div
                v-else-if="Array.isArray(posts) && posts.length"
                class="modern-grid-cards"
              >
                <div
                  v-for="post in posts"
                  :key="post.id"
                  class="modern-post-card"
                >
                  <PostCard :post="post" />
                </div>
              </div>

              <div
                v-else
                class="text-center py-12"
              >
                <UIcon
                  name="i-heroicons-document-text"
                  class="w-16 h-16 text-gray-400 mx-auto mb-4"
                />
                <p class="text-gray-600 dark:text-gray-400">
                  暂无文章，
                  <NuxtLink
                    to="/write"
                    class="text-primary-600 hover:text-primary-700"
                  >
                    开始写作
                  </NuxtLink>
                  吧！
                </p>
              </div>
            </div>

            <!-- 分类浏览 -->
            <div
              class="modern-content-section"
              style="
                                background: rgba(255, 255, 255, 0.9);
                                border-radius: 1rem;
                                padding: 1.5rem;
                                margin-bottom: 1rem;
                                border: 1px solid #e2e8f0;
                                box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
                            "
            >
              <CategoryBrowser />
            </div>
          </div>

          <!-- 侧边栏 -->
          <div class="sidebar modern-sidebar">
            <div class="sidebar-sticky">
              <HomeSidebar />
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- 快捷入口区域 -->
    <section
      style="
                background: rgba(255, 255, 255, 0.6);
                border-top: 1px solid rgba(226, 232, 240, 0.6);
                padding: 1.5rem 0;
            "
    >
      <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
        <div
          style="
                        display: flex;
                        justify-content: center;
                        gap: 1rem;
                        flex-wrap: wrap;
                    "
        >
          <a
            href="/posts"
            style="
                            display: flex;
                            align-items: center;
                            gap: 0.5rem;
                            padding: 0.75rem 1.5rem;
                            background: linear-gradient(
                                45deg,
                                #3b82f6,
                                #6366f1
                            );
                            color: white;
                            text-decoration: none;
                            border-radius: 0.75rem;
                            font-size: 0.875rem;
                            font-weight: 500;
                            transition: all 0.3s ease;
                            box-shadow: 0 2px 4px rgba(59, 130, 246, 0.3);
                            white-space: nowrap;
                        "
            class="hover:transform hover:scale-105"
          >
            <UIcon
              name="i-heroicons-book-open"
              class="w-4 h-4"
            />
            浏览文章
          </a>
          <a
            href="/categories"
            style="
                            display: flex;
                            align-items: center;
                            gap: 0.5rem;
                            padding: 0.75rem 1.5rem;
                            background: rgba(255, 255, 255, 0.9);
                            color: #8b5cf6;
                            text-decoration: none;
                            border-radius: 0.75rem;
                            font-size: 0.875rem;
                            font-weight: 500;
                            border: 1px solid rgba(139, 92, 246, 0.3);
                            transition: all 0.3s ease;
                            white-space: nowrap;
                        "
            class="hover:bg-purple-50 hover:border-purple-400 hover:transform hover:scale-105"
          >
            <UIcon
              name="i-heroicons-squares-2x2"
              class="w-4 h-4"
            />
            分类探索
          </a>
          <a
            href="/authors"
            style="
                            display: flex;
                            align-items: center;
                            gap: 0.5rem;
                            padding: 0.75rem 1.5rem;
                            background: rgba(255, 255, 255, 0.9);
                            color: #10b981;
                            text-decoration: none;
                            border-radius: 0.75rem;
                            font-size: 0.875rem;
                            font-weight: 500;
                            border: 1px solid rgba(16, 185, 129, 0.3);
                            transition: all 0.3s ease;
                            white-space: nowrap;
                        "
            class="hover:bg-green-50 hover:border-green-400 hover:transform hover:scale-105"
          >
            <UIcon
              name="i-heroicons-users"
              class="w-4 h-4"
            />
            优秀作者
          </a>
          <a
            href="/write"
            style="
                            display: flex;
                            align-items: center;
                            gap: 0.5rem;
                            padding: 0.75rem 1.5rem;
                            background: rgba(255, 255, 255, 0.9);
                            color: #f59e0b;
                            text-decoration: none;
                            border-radius: 0.75rem;
                            font-size: 0.875rem;
                            font-weight: 500;
                            border: 1px solid rgba(245, 158, 11, 0.3);
                            transition: all 0.3s ease;
                            white-space: nowrap;
                        "
            class="hover:bg-orange-50 hover:border-orange-400 hover:transform hover:scale-105"
          >
            <UIcon
              name="i-heroicons-pencil-square"
              class="w-4 h-4"
            />
            开始写作
          </a>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import type { PostWithContent } from "~/types";
import HomeSidebar from "~/components/HomeSidebar.vue";
import CategoryBrowser from "~/components/common/CategoryBrowser.vue";

// SEO 配置
useHead({
  title: "首页",
  meta: [
    {
      name: "description",
      content: "基于 Nuxt 3 和 Rust 构建的现代化博客系统",
    },
  ],
});

const runtimeConfig = useRuntimeConfig();

// 定义后端响应类型
interface BackendPostsResponse {
  items: PostWithContent[];
  page: number;
  page_size: number;
  total_items: number;
  total_pages: number;
}

// 获取推荐文章（使用公开接口）
const { data: featuredPostsResponse, pending: featuredPending }
    = await useLazyFetch<BackendPostsResponse>(
      `${runtimeConfig.public.apiBaseUrl}/blog/posts`,
      {
        query: {
          page_size: 4,
          page: 1,
        },
      },
    );

// 获取最新文章（使用公开接口）
const { data: postsResponse, pending }
    = await useLazyFetch<BackendPostsResponse>(
      `${runtimeConfig.public.apiBaseUrl}/blog/posts`,
      {
        query: {
          page_size: 6,
          page: 1,
        },
      },
    );

// 转换后端数据格式为前端期望的格式
const featuredPosts = computed(() => {
  return featuredPostsResponse.value?.items?.slice(0, 4) || [];
});

const posts = computed(() => {
  return postsResponse.value?.items?.slice(0, 6) || [];
});
</script>

<style scoped>
.layout-container {
    display: flex;
    flex-direction: column;
    gap: 2rem;
}

.main-content {
    flex: 1;
    min-width: 0;
}

.sidebar {
    min-width: 320px;
}

.sidebar-sticky {
    position: relative;
}

/* 大屏幕布局 */
@media (min-width: 1024px) {
    .layout-container {
        flex-direction: row;
        gap: 3rem;
    }

    .main-content {
        flex: 3;
        order: 1;
    }

    .sidebar {
        flex: 1;
        order: 2;
    }

    .sidebar-sticky {
        position: sticky;
        top: 6rem;
    }
}

/* 添加网格背景样式 */
.bg-grid-slate-100 {
    background-image: radial-gradient(
        circle,
        rgba(148, 163, 184, 0.15) 1px,
        transparent 1px
    );
}

.dark .bg-grid-slate-700\/25 {
    background-image: radial-gradient(
        circle,
        rgba(51, 65, 85, 0.25) 1px,
        transparent 1px
    );
}
</style>
