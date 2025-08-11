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
              name="i-heroicons-chart-bar"
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
              style="font-weight: 800; background: linear-gradient(45deg, #8b5cf6, #a855f7); -webkit-background-clip: text; -webkit-text-fill-color: transparent;"
            >统计</span>
          </h1>

          <!-- 副标题 -->
          <p
            style="font-size: 1rem; color: #64748b; max-width: 28rem; margin: 0 auto; line-height: 1.5;"
            class="dark:text-slate-300"
          >
            查看您的创作成果和活跃度数据
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
          <span style="color: #3b82f6; font-weight: 500;">数据统计</span>
        </div>
      </div>
    </nav>

    <!-- 主要内容区域 -->
    <section
      style="background: linear-gradient(135deg, #f8fafc 0%, #e2e8f0 50%, #f3e8ff 100%); padding: 2rem 0; margin: 0; position: relative; min-height: 80vh;"
    >
      <!-- 背景装饰 -->
      <div style="position: absolute; inset: 0; background-image: radial-gradient(circle at 1px 1px, rgba(139, 92, 246, 0.1) 1px, transparent 0); background-size: 60px 60px; opacity: 0.5;" />

      <div class="relative mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
        <!-- 加载状态 -->
        <div
          v-if="pending"
          class="space-y-6"
        >
          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
            <USkeleton
              v-for="i in 4"
              :key="i"
              class="h-32"
            />
          </div>
          <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
            <USkeleton class="h-64" />
            <USkeleton class="h-64" />
          </div>
        </div>

        <!-- 统计数据 -->
        <div
          v-else-if="stats"
          class="space-y-6"
        >
          <!-- 核心指标卡片 -->
          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
            <!-- 总文章数 -->
            <div
              class="modern-content-section group cursor-pointer"
              style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 2rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05); transition: all 0.3s ease;"
            >
              <div class="flex items-center justify-between mb-4">
                <div
                  class="modern-gradient-icon"
                  style="background: linear-gradient(45deg, #3b82f6, #6366f1); width: 3rem; height: 3rem;"
                >
                  <UIcon
                    name="i-heroicons-document-text"
                    class="h-6 w-6 text-white"
                  />
                </div>
                <div class="text-right">
                  <div class="text-2xl font-bold text-slate-900 dark:text-white">
                    {{ stats.totalPosts }}
                  </div>
                  <div class="text-sm text-slate-500 dark:text-slate-400">
                    总文章
                  </div>
                </div>
              </div>
              <div class="text-xs text-slate-600 dark:text-slate-400">
                已发布 {{ stats.publishedPosts }} 篇，草稿 {{ stats.draftPosts }} 篇
              </div>
            </div>

            <!-- 总浏览量 -->
            <div
              class="modern-content-section group cursor-pointer"
              style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 2rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05); transition: all 0.3s ease;"
            >
              <div class="flex items-center justify-between mb-4">
                <div
                  class="modern-gradient-icon"
                  style="background: linear-gradient(45deg, #10b981, #059669); width: 3rem; height: 3rem;"
                >
                  <UIcon
                    name="i-heroicons-eye"
                    class="h-6 w-6 text-white"
                  />
                </div>
                <div class="text-right">
                  <div class="text-2xl font-bold text-slate-900 dark:text-white">
                    {{ formatNumber(stats.totalViews) }}
                  </div>
                  <div class="text-sm text-slate-500 dark:text-slate-400">
                    总浏览
                  </div>
                </div>
              </div>
              <div class="text-xs text-slate-600 dark:text-slate-400">
                平均每篇 {{ Math.round(stats.totalViews / Math.max(stats.publishedPosts, 1)) }} 次浏览
              </div>
            </div>

            <!-- 获赞数 -->
            <div
              class="modern-content-section group cursor-pointer"
              style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 2rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05); transition: all 0.3s ease;"
            >
              <div class="flex items-center justify-between mb-4">
                <div
                  class="modern-gradient-icon"
                  style="background: linear-gradient(45deg, #ec4899, #f43f5e); width: 3rem; height: 3rem;"
                >
                  <UIcon
                    name="i-heroicons-heart"
                    class="h-6 w-6 text-white"
                  />
                </div>
                <div class="text-right">
                  <div class="text-2xl font-bold text-slate-900 dark:text-white">
                    {{ formatNumber(stats.totalLikes) }}
                  </div>
                  <div class="text-sm text-slate-500 dark:text-slate-400">
                    获赞数
                  </div>
                </div>
              </div>
              <div class="text-xs text-slate-600 dark:text-slate-400">
                平均每篇 {{ Math.round(stats.totalLikes / Math.max(stats.publishedPosts, 1)) }} 个赞
              </div>
            </div>

            <!-- 评论数 -->
            <div
              class="modern-content-section group cursor-pointer"
              style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 2rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05); transition: all 0.3s ease;"
            >
              <div class="flex items-center justify-between mb-4">
                <div
                  class="modern-gradient-icon"
                  style="background: linear-gradient(45deg, #f59e0b, #f97316); width: 3rem; height: 3rem;"
                >
                  <UIcon
                    name="i-heroicons-chat-bubble-left-ellipsis"
                    class="h-6 w-6 text-white"
                  />
                </div>
                <div class="text-right">
                  <div class="text-2xl font-bold text-slate-900 dark:text-white">
                    {{ formatNumber(stats.totalComments) }}
                  </div>
                  <div class="text-sm text-slate-500 dark:text-slate-400">
                    评论数
                  </div>
                </div>
              </div>
              <div class="text-xs text-slate-600 dark:text-slate-400">
                平均每篇 {{ Math.round(stats.totalComments / Math.max(stats.publishedPosts, 1)) }} 条评论
              </div>
            </div>
          </div>

          <!-- 图表区域 -->
          <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
            <!-- 创作趋势 -->
            <div
              class="modern-content-section"
              style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 2rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);"
            >
              <div class="modern-section-header mb-6">
                <div class="flex items-center gap-3">
                  <div
                    class="modern-gradient-icon"
                    style="background: linear-gradient(45deg, #3b82f6, #6366f1); width: 2.5rem; height: 2.5rem;"
                  >
                    <UIcon
                      name="i-heroicons-chart-bar"
                      class="h-5 w-5 text-white"
                    />
                  </div>
                  <h3 class="text-lg font-semibold text-slate-900 dark:text-white">
                    创作趋势
                  </h3>
                </div>
              </div>

              <!-- 简单的模拟图表 -->
              <div class="space-y-4">
                <div
                  v-for="(month, index) in stats.monthlyPosts"
                  :key="index"
                  class="flex items-center gap-3"
                >
                  <div class="w-12 text-sm text-slate-600 dark:text-slate-400">
                    {{ month.month }}
                  </div>
                  <div class="flex-1 bg-slate-100 dark:bg-slate-700 rounded-full h-2 overflow-hidden">
                    <div
                      class="h-full bg-gradient-to-r from-blue-500 to-purple-600 rounded-full transition-all duration-1000"
                      :style="{ width: `${(month.count / Math.max(...stats.monthlyPosts.map(m => m.count))) * 100}%` }"
                    />
                  </div>
                  <div class="w-8 text-sm font-medium text-slate-900 dark:text-white">
                    {{ month.count }}
                  </div>
                </div>
              </div>
            </div>

            <!-- 分类分布 -->
            <div
              class="modern-content-section"
              style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 2rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);"
            >
              <div class="modern-section-header mb-6">
                <div class="flex items-center gap-3">
                  <div
                    class="modern-gradient-icon"
                    style="background: linear-gradient(45deg, #10b981, #059669); width: 2.5rem; height: 2.5rem;"
                  >
                    <UIcon
                      name="i-heroicons-squares-2x2"
                      class="h-5 w-5 text-white"
                    />
                  </div>
                  <h3 class="text-lg font-semibold text-slate-900 dark:text-white">
                    分类分布
                  </h3>
                </div>
              </div>

              <div class="space-y-3">
                <div
                  v-for="(category, index) in stats.categoryDistribution"
                  :key="index"
                  class="flex items-center justify-between p-3 rounded-lg bg-slate-50 dark:bg-slate-800"
                >
                  <div class="flex items-center gap-3">
                    <div
                      class="w-3 h-3 rounded-full"
                      :style="{ background: getCategoryColor(index) }"
                    />
                    <span class="text-sm font-medium text-slate-900 dark:text-white">
                      {{ category.name }}
                    </span>
                  </div>
                  <div class="text-right">
                    <div class="text-sm font-bold text-slate-900 dark:text-white">
                      {{ category.count }}
                    </div>
                    <div class="text-xs text-slate-500 dark:text-slate-400">
                      {{ Math.round((category.count / stats.totalPosts) * 100) }}%
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- 成就系统 -->
          <div
            class="modern-content-section"
            style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 2rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);"
          >
            <div class="modern-section-header mb-6">
              <div class="flex items-center gap-3">
                <div
                  class="modern-gradient-icon"
                  style="background: linear-gradient(45deg, #f59e0b, #f97316); width: 2.5rem; height: 2.5rem;"
                >
                  <UIcon
                    name="i-heroicons-trophy"
                    class="h-5 w-5 text-white"
                  />
                </div>
                <h3 class="text-lg font-semibold text-slate-900 dark:text-white">
                  成就徽章
                </h3>
              </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
              <div
                v-for="achievement in stats.achievements"
                :key="achievement.id"
                class="flex items-center gap-4 p-4 rounded-lg border-2 transition-all"
                :class="achievement.unlocked
                  ? 'border-yellow-200 bg-yellow-50 dark:border-yellow-800 dark:bg-yellow-900/20'
                  : 'border-slate-200 bg-slate-50 dark:border-slate-700 dark:bg-slate-800'"
              >
                <div
                  class="flex-shrink-0 w-12 h-12 rounded-full flex items-center justify-center"
                  :class="achievement.unlocked
                    ? 'bg-gradient-to-br from-yellow-400 to-orange-500 text-white'
                    : 'bg-slate-200 dark:bg-slate-600 text-slate-500 dark:text-slate-400'"
                >
                  <UIcon
                    :name="achievement.icon"
                    class="w-6 h-6"
                  />
                </div>
                <div class="flex-1">
                  <h4
                    class="font-medium mb-1"
                    :class="achievement.unlocked
                      ? 'text-yellow-800 dark:text-yellow-200'
                      : 'text-slate-600 dark:text-slate-400'"
                  >
                    {{ achievement.name }}
                  </h4>
                  <p
                    class="text-sm"
                    :class="achievement.unlocked
                      ? 'text-yellow-700 dark:text-yellow-300'
                      : 'text-slate-500 dark:text-slate-500'"
                  >
                    {{ achievement.description }}
                  </p>
                  <div
                    v-if="!achievement.unlocked && achievement.progress"
                    class="mt-2"
                  >
                    <div class="flex justify-between items-center text-xs text-slate-500 dark:text-slate-400 mb-1">
                      <span>进度</span>
                      <span>{{ achievement.progress.current }}/{{ achievement.progress.target }}</span>
                    </div>
                    <div class="w-full bg-slate-200 dark:bg-slate-700 rounded-full h-1.5">
                      <div
                        class="h-1.5 bg-gradient-to-r from-blue-500 to-purple-600 rounded-full"
                        :style="{ width: `${(achievement.progress.current / achievement.progress.target) * 100}%` }"
                      />
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- 最新活动 -->
          <div
            class="modern-content-section"
            style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 2rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);"
          >
            <div class="modern-section-header mb-6">
              <div class="flex items-center gap-3">
                <div
                  class="modern-gradient-icon"
                  style="background: linear-gradient(45deg, #8b5cf6, #a855f7); width: 2.5rem; height: 2.5rem;"
                >
                  <UIcon
                    name="i-heroicons-clock"
                    class="h-5 w-5 text-white"
                  />
                </div>
                <h3 class="text-lg font-semibold text-slate-900 dark:text-white">
                  最新活动
                </h3>
              </div>
            </div>

            <div class="space-y-4">
              <div
                v-for="activity in stats.recentActivities"
                :key="activity.id"
                class="flex items-start gap-4 p-4 rounded-lg bg-slate-50 dark:bg-slate-800"
              >
                <div
                  class="flex-shrink-0 w-8 h-8 rounded-full flex items-center justify-center"
                  :style="{ background: getActivityColor(activity.type) }"
                >
                  <UIcon
                    :name="getActivityIcon(activity.type)"
                    class="w-4 h-4 text-white"
                  />
                </div>
                <div class="flex-1">
                  <p class="text-sm text-slate-900 dark:text-white">
                    {{ activity.description }}
                  </p>
                  <p class="text-xs text-slate-500 dark:text-slate-400 mt-1">
                    {{ formatDate(activity.date) }}
                  </p>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- 错误状态 -->
        <div
          v-else
          style="text-align: center; padding: 4rem 0;"
        >
          <div style="margin: 0 auto 2rem; width: 4rem; height: 4rem; display: flex; align-items: center; justify-content: center; border-radius: 50%; background: linear-gradient(45deg, #ef4444, #f87171); box-shadow: 0 8px 32px rgba(239, 68, 68, 0.2);">
            <UIcon
              name="i-heroicons-exclamation-triangle"
              style="width: 2rem; height: 2rem; color: white;"
            />
          </div>
          <h3
            style="font-size: 1.5rem; font-weight: 700; color: #1e293b; margin-bottom: 1rem;"
            class="dark:text-white"
          >
            数据加载失败
          </h3>
          <p
            style="font-size: 1rem; color: #64748b; margin-bottom: 2rem; line-height: 1.6;"
            class="dark:text-slate-300"
          >
            无法获取统计数据，请稍后重试
          </p>
          <UButton
            icon="i-heroicons-arrow-path"
            @click="() => refresh()"
          >
            重新加载
          </UButton>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
// 路由守卫
definePageMeta({
  middleware: "auth",
});

// SEO 配置
useHead({
  title: "数据统计",
  meta: [{ name: "description", content: "查看您的创作成果和活跃度数据" }],
});

// 状态管理
interface UserStats {
  totalPosts: number;
  publishedPosts: number;
  draftPosts: number;
  totalViews: number;
  totalLikes: number;
  totalComments: number;
  monthlyPosts: { month: string; count: number }[];
  categoryDistribution: { name: string; count: number }[];
  achievements: {
    id: string;
    name: string;
    description: string;
    icon: string;
    unlocked: boolean;
    progress?: { current: number; target: number };
  }[];
  recentActivities: {
    id: string;
    type: string;
    description: string;
    date: string;
  }[];
}

// API 客户端
const api = useApi();

// 获取统计数据
const {
  data: stats,
  pending,
  refresh,
} = await useLazyAsyncData(
  "user-stats",
  async () => {
    // 实际应该调用 API 获取统计数据
    // const response = await api.getUserStats();

    // 模拟统计数据
    const mockStats: UserStats = {
      totalPosts: 28,
      publishedPosts: 25,
      draftPosts: 3,
      totalViews: 12580,
      totalLikes: 1234,
      totalComments: 456,
      monthlyPosts: [
        { month: "1月", count: 8 },
        { month: "2月", count: 12 },
        { month: "3月", count: 15 },
        { month: "4月", count: 10 },
        { month: "5月", count: 18 },
        { month: "6月", count: 22 },
      ],
      categoryDistribution: [
        { name: "前端开发", count: 12 },
        { name: "后端技术", count: 8 },
        { name: "移动开发", count: 5 },
        { name: "数据库", count: 3 },
      ],
      achievements: [
        {
          id: "first-post",
          name: "初试啼声",
          description: "发布第一篇文章",
          icon: "i-heroicons-pencil-square",
          unlocked: true,
        },
        {
          id: "ten-posts",
          name: "勤奋创作者",
          description: "发布10篇文章",
          icon: "i-heroicons-document-text",
          unlocked: true,
        },
        {
          id: "hundred-views",
          name: "初有人气",
          description: "文章总浏览量达到100",
          icon: "i-heroicons-eye",
          unlocked: true,
        },
        {
          id: "thousand-views",
          name: "人气作者",
          description: "文章总浏览量达到1000",
          icon: "i-heroicons-fire",
          unlocked: true,
        },
        {
          id: "fifty-posts",
          name: "高产作家",
          description: "发布50篇文章",
          icon: "i-heroicons-bookmark",
          unlocked: false,
          progress: { current: 28, target: 50 },
        },
        {
          id: "hundred-likes",
          name: "深受喜爱",
          description: "获得100个点赞",
          icon: "i-heroicons-heart",
          unlocked: true,
        },
      ],
      recentActivities: [
        {
          id: "1",
          type: "publish",
          description: "发布了文章《Vue.js 3.0 新特性详解》",
          date: "2024-01-15T10:00:00Z",
        },
        {
          id: "2",
          type: "like",
          description: "您的文章《TypeScript 高级类型》获得了 5 个新赞",
          date: "2024-01-14T15:30:00Z",
        },
        {
          id: "3",
          type: "comment",
          description: "您的文章《React 性能优化》收到了新评论",
          date: "2024-01-14T09:20:00Z",
        },
        {
          id: "4",
          type: "achievement",
          description: "解锁成就：勤奋创作者",
          date: "2024-01-13T14:00:00Z",
        },
        {
          id: "5",
          type: "draft",
          description: "保存了草稿《GraphQL 实战指南》",
          date: "2024-01-12T16:45:00Z",
        },
      ],
    };

    return mockStats;
  },
);

// 方法
const formatNumber = (num: number): string => {
  if (num >= 10000) {
    return `${(num / 10000).toFixed(1)}万`;
  }
  else if (num >= 1000) {
    return `${(num / 1000).toFixed(1)}k`;
  }
  return num.toString();
};

const getCategoryColor = (index: number): string => {
  const colors = [
    "linear-gradient(45deg, #3b82f6, #6366f1)",
    "linear-gradient(45deg, #10b981, #059669)",
    "linear-gradient(45deg, #f59e0b, #f97316)",
    "linear-gradient(45deg, #ec4899, #f43f5e)",
    "linear-gradient(45deg, #8b5cf6, #a855f7)",
  ];
  return colors[index % colors.length];
};

const getActivityColor = (type: string): string => {
  const colors = {
    publish: "linear-gradient(45deg, #3b82f6, #6366f1)",
    like: "linear-gradient(45deg, #ec4899, #f43f5e)",
    comment: "linear-gradient(45deg, #10b981, #059669)",
    achievement: "linear-gradient(45deg, #f59e0b, #f97316)",
    draft: "linear-gradient(45deg, #8b5cf6, #a855f7)",
  };
  return colors[type as keyof typeof colors] || colors.publish;
};

const getActivityIcon = (type: string): string => {
  const icons = {
    publish: "i-heroicons-document-text",
    like: "i-heroicons-heart",
    comment: "i-heroicons-chat-bubble-left-ellipsis",
    achievement: "i-heroicons-trophy",
    draft: "i-heroicons-document",
  };
  return icons[type as keyof typeof icons] || icons.publish;
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
.modern-gradient-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  transition: all 0.3s ease;
}

.group:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 25px rgba(0, 0, 0, 0.15);
}
</style>
