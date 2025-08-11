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
              name="i-heroicons-users"
              style="width: 1.25rem; height: 1.25rem; color: white;"
            />
          </div>

          <!-- 现代化标题 -->
          <h1
            style="font-size: 1.875rem; font-weight: 700; line-height: 1.1; margin-bottom: 0.75rem; color: #1e293b;"
            class="dark:text-white"
          >
            优秀
            <span
              class="modern-text-gradient"
              style="font-weight: 800;"
            >作者</span>
          </h1>

          <!-- 副标题 -->
          <p
            style="font-size: 1rem; color: #64748b; max-width: 28rem; margin: 0 auto; line-height: 1.5;"
            class="dark:text-slate-300"
          >
            发现创作社区中的优秀作者，关注他们的精彩作品
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
          <span style="color: #3b82f6; font-weight: 500;">优秀作者</span>
        </div>
      </div>
    </nav>

    <!-- 主要内容区域 -->
    <section
      style="background: linear-gradient(135deg, #f8fafc 0%, #e2e8f0 50%, #dbeafe 100%); padding: 2rem 0; margin: 0; position: relative;"
    >
      <!-- 背景装饰 -->
      <div style="position: absolute; inset: 0; background-image: radial-gradient(circle at 1px 1px, rgba(59, 130, 246, 0.1) 1px, transparent 0); background-size: 60px 60px; opacity: 0.5;" />

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
              {{ error.message || "获取作者列表时发生错误" }}
            </p>
            <button
              style="display: inline-flex; align-items: center; gap: 0.5rem; padding: 0.75rem 2rem; background: linear-gradient(45deg, #ef4444, #f87171); color: white; border-radius: 2rem; font-size: 0.875rem; font-weight: 600; border: none; cursor: pointer; transition: all 0.3s ease; box-shadow: 0 4px 12px rgba(239, 68, 68, 0.3);"
              class="hover:transform hover:scale-105 hover:shadow-lg"
              @click="() => refresh()"
            >
              <UIcon
                name="i-heroicons-arrow-path"
                class="w-4 h-4"
              />
              重新加载
            </button>
          </div>
        </div>

        <!-- 作者列表 -->
        <div
          class="modern-content-section"
          style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 1.5rem; margin-bottom: 1rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);"
        >
          <div class="modern-section-header">
            <div>
              <div class="flex items-center gap-3 mb-2">
                <div
                  class="modern-gradient-icon"
                  style="background: linear-gradient(45deg, #3b82f6, #6366f1); width: 2.5rem; height: 2.5rem;"
                >
                  <UIcon
                    name="i-heroicons-users"
                    class="h-5 w-5 text-white"
                  />
                </div>
                <h2 class="modern-section-title">
                  活跃作者
                </h2>
              </div>
              <p class="modern-section-subtitle">
                {{ authors.length }} 位优秀作者为社区贡献精彩内容
              </p>
            </div>

            <!-- 搜索和排序 -->
            <div class="flex items-center gap-3">
              <!-- 搜索框 -->
              <UInput
                v-model="searchQuery"
                icon="i-heroicons-magnifying-glass"
                placeholder="搜索作者..."
                size="sm"
                style="min-width: 180px; background: rgba(248, 250, 252, 0.8); border: 1px solid rgba(226, 232, 240, 0.6);"
                class="focus:border-blue-500"
              />

              <!-- 排序选项 -->
              <USelectMenu
                v-model="sortOption"
                :options="sortOptions"
                size="sm"
                style="min-width: 120px; background: rgba(248, 250, 252, 0.8); border: 1px solid rgba(226, 232, 240, 0.6);"
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
              v-for="i in 9"
              :key="i"
              class="modern-post-card"
              style="padding: 2rem; height: 280px; display: flex; align-items: center; justify-content: center;"
            >
              <USkeleton class="h-48 w-full" />
            </div>
          </div>

          <!-- 作者网格 -->
          <div
            v-else-if="filteredAuthors.length"
            class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6"
            style="padding: 1rem 0;"
          >
            <div
              v-for="author in filteredAuthors"
              :key="author.id"
              class="modern-post-card group cursor-pointer author-card"
              style="padding: 2rem; transition: all 0.3s ease;"
              @click="goToAuthor(author.username)"
            >
              <!-- 作者头像 -->
              <div style="text-align: center; margin-bottom: 1.5rem;">
                <div
                  class="author-avatar"
                  style="margin: 0 auto; width: 4rem; height: 4rem; display: flex; align-items: center; justify-content: center; border-radius: 50%; background: linear-gradient(45deg, #3b82f6, #6366f1); color: white; font-size: 1.5rem; font-weight: 700; position: relative; overflow: hidden;"
                >
                  <span v-if="!author.avatar">
                    {{ getInitials(author.username) }}
                  </span>
                  <img
                    v-else
                    :src="author.avatar"
                    :alt="author.username"
                    style="width: 100%; height: 100%; object-cover;"
                  >
                </div>
              </div>

              <!-- 作者信息 -->
              <div style="text-align: center;">
                <h3 class="text-xl font-bold text-slate-900 dark:text-white mb-2">
                  {{ author.username }}
                </h3>

                <p
                  v-if="author.bio"
                  class="text-slate-600 dark:text-slate-400 mb-4 line-clamp-2"
                  style="min-height: 2.5rem; font-size: 0.875rem;"
                >
                  {{ author.bio }}
                </p>

                <!-- 统计信息 -->
                <div class="grid grid-cols-2 gap-4 mb-4">
                  <div class="text-center">
                    <div class="text-lg font-bold text-slate-900 dark:text-white">
                      {{ author.postCount }}
                    </div>
                    <div class="text-xs text-slate-500 dark:text-slate-400">
                      篇文章
                    </div>
                  </div>
                  <div class="text-center">
                    <div class="text-lg font-bold text-slate-900 dark:text-white">
                      {{ author.categories.length }}
                    </div>
                    <div class="text-xs text-slate-500 dark:text-slate-400">
                      个分类
                    </div>
                  </div>
                </div>

                <!-- 专长标签 -->
                <div
                  v-if="author.categories.length"
                  class="flex flex-wrap gap-1 justify-center mb-4"
                >
                  <span
                    v-for="category in author.categories.slice(0, 3)"
                    :key="category"
                    class="inline-block px-2 py-1 text-xs rounded-full"
                    style="background: rgba(59, 130, 246, 0.1); color: #3b82f6; font-weight: 500;"
                  >
                    {{ category }}
                  </span>
                  <span
                    v-if="author.categories.length > 3"
                    class="inline-block px-2 py-1 text-xs rounded-full text-slate-400"
                    style="background: rgba(148, 163, 184, 0.1);"
                  >
                    +{{ author.categories.length - 3 }}
                  </span>
                </div>

                <!-- 最后活动时间 -->
                <div class="text-xs text-slate-500 dark:text-slate-400 mb-4">
                  最后活跃：{{ formatDate(author.lastActiveAt) }}
                </div>

                <!-- 查看按钮 -->
                <div
                  class="inline-flex items-center gap-2 px-4 py-2 text-sm font-medium rounded-lg transition-all"
                  style="background: rgba(59, 130, 246, 0.1); color: #3b82f6; border: 1px solid rgba(59, 130, 246, 0.2);"
                >
                  <span>查看主页</span>
                  <UIcon
                    name="i-heroicons-arrow-right"
                    class="w-4 h-4 transition-transform group-hover:translate-x-1"
                  />
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
                name="i-heroicons-users"
                style="width: 2rem; height: 2rem; color: white;"
              />
            </div>
            <h3
              style="font-size: 1.5rem; font-weight: 700; color: #1e293b; margin-bottom: 1rem;"
              class="dark:text-white"
            >
              没有找到作者
            </h3>
            <p
              style="font-size: 1rem; color: #64748b; margin-bottom: 2rem; line-height: 1.6;"
              class="dark:text-slate-300"
            >
              {{ searchQuery ? '尝试调整搜索条件' : '暂无活跃作者' }}
            </p>
            <div style="display: flex; gap: 1rem; flex-wrap: wrap; justify-content: center;">
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
              <button
                v-if="searchQuery"
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
            </div>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import type { Author } from "~/types";

// SEO 配置
useHead({
  title: "优秀作者",
  meta: [{ name: "description", content: "发现创作社区中的优秀作者，关注他们的精彩作品" }],
});

// 状态管理
const router = useRouter();
const searchQuery = ref("");
const sortOption = ref("postCount");

// API 客户端
const api = useApi();

// 获取所有文章来推导作者列表
const {
  data: posts,
  pending,
  refresh,
  error,
} = await useLazyAsyncData(
  "all-posts-for-authors",
  async () => {
    const response = await api.getPosts({
      per_page: 1000, // 获取足够多的文章来分析作者
      status: "published",
    });
    if (response.success) {
      return response.data;
    }
    throw createError({
      statusCode: 500,
      statusMessage: response.error?.message || "获取文章数据失败",
    });
  },
);

// 从文章数据中提取作者信息
const authors = computed((): Author[] => {
  if (!posts.value?.data) return [];

  const authorMap = new Map<string, {
    id: string;
    username: string;
    email?: string;
    posts: any[];
    categories: Set<string>;
    tags: Set<string>;
    lastActiveAt: string;
  }>();

  // 分析每篇文章的作者信息
  posts.value.data.forEach((post) => {
    const authorId = post.author.id;

    if (!authorMap.has(authorId)) {
      authorMap.set(authorId, {
        id: post.author.id,
        username: post.author.username,
        email: post.author.email,
        posts: [],
        categories: new Set(),
        tags: new Set(),
        lastActiveAt: post.published_at || post.created_at,
      });
    }

    const authorData = authorMap.get(authorId)!;
    authorData.posts.push(post);

    // 收集分类和标签
    post.categories?.forEach(cat => authorData.categories.add(cat.name));
    post.tags?.forEach(tag => authorData.tags.add(tag.name));

    // 更新最后活跃时间
    const postDate = new Date(post.published_at || post.created_at);
    const currentLastActive = new Date(authorData.lastActiveAt);
    if (postDate > currentLastActive) {
      authorData.lastActiveAt = post.published_at || post.created_at;
    }
  });

  // 转换为Author类型
  return Array.from(authorMap.values()).map(authorData => ({
    id: authorData.id,
    username: authorData.username,
    email: authorData.email,
    bio: `专注于${Array.from(authorData.categories).slice(0, 2).join("、")}等领域的内容创作`, // 模拟简介
    created_at: authorData.posts[0]?.created_at || new Date().toISOString(),
    postCount: authorData.posts.length,
    categories: Array.from(authorData.categories),
    tags: Array.from(authorData.tags),
    lastActiveAt: authorData.lastActiveAt,
  }));
});

// 搜索和排序
const filteredAuthors = computed(() => {
  let result = [...authors.value];

  // 搜索过滤
  if (searchQuery.value.trim()) {
    const query = searchQuery.value.toLowerCase().trim();
    result = result.filter(author =>
      author.username.toLowerCase().includes(query)
      || (author.bio && author.bio.toLowerCase().includes(query))
      || author.categories.some(cat => cat.toLowerCase().includes(query)),
    );
  }

  // 排序
  result.sort((a, b) => {
    switch (sortOption.value) {
      case "postCount":
        return b.postCount - a.postCount;
      case "username":
        return a.username.localeCompare(b.username);
      case "lastActive":
        return new Date(b.lastActiveAt).getTime() - new Date(a.lastActiveAt).getTime();
      default:
        return 0;
    }
  });

  return result;
});

// 排序选项
const sortOptions = [
  { label: "按文章数排序", value: "postCount" },
  { label: "按用户名排序", value: "username" },
  { label: "按最后活跃", value: "lastActive" },
];

// 工具函数
const getInitials = (username: string): string => {
  return username.slice(0, 2).toUpperCase();
};

const formatDate = (dateString: string): string => {
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
};

const clearSearch = () => {
  searchQuery.value = "";
};

const goToAuthor = (username: string) => {
  router.push(`/authors/${username}`);
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

.author-card:hover {
  transform: translateY(-8px);
  box-shadow: 0 12px 30px rgba(0, 0, 0, 0.15);
}

.author-avatar {
  transition: all 0.3s ease;
  box-shadow: 0 4px 20px rgba(59, 130, 246, 0.3);
}

.group:hover .author-avatar {
  transform: scale(1.1);
  box-shadow: 0 8px 30px rgba(59, 130, 246, 0.4);
}

.modern-gradient-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  transition: all 0.3s ease;
}
</style>
