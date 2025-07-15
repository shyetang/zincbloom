<template>
  <div class="space-y-4">
    <div class="modern-sidebar-header">
      <div
        class="modern-gradient-icon"
        style="background: linear-gradient(45deg, #8b5cf6, #a855f7); width: 1.5rem; height: 1.5rem;"
      >
        <UIcon
          name="i-heroicons-users"
          class="h-3 w-3 text-white"
        />
      </div>
      <h3 class="modern-sidebar-title">
        活跃作者
      </h3>
    </div>

    <!-- 加载状态 -->
    <div
      v-if="pending"
      class="space-y-3"
    >
      <div
        v-for="i in 5"
        :key="i"
        class="flex items-center space-x-3"
      >
        <USkeleton class="w-10 h-10 rounded-full" />
        <div class="flex-1">
          <USkeleton class="h-4 w-24 mb-1" />
          <USkeleton class="h-3 w-16" />
        </div>
      </div>
    </div>

    <!-- 作者列表 -->
    <div
      v-else-if="authors && authors.length"
      class="space-y-3"
    >
      <div
        v-for="author in authors"
        :key="author.id"
        class="flex items-center space-x-3 p-3 rounded-lg border border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors"
      >
        <!-- 头像 -->
        <div class="relative">
          <div
            class="w-10 h-10 bg-primary-100 dark:bg-primary-900 rounded-full flex items-center justify-center"
          >
            <UIcon
              name="i-heroicons-user"
              class="w-5 h-5 text-primary-600 dark:text-primary-400"
            />
          </div>
          <!-- 在线状态指示器 -->
          <div
            v-if="author.is_online"
            class="absolute -bottom-1 -right-1 w-3 h-3 bg-green-500 border-2 border-white dark:border-gray-800 rounded-full"
          />
        </div>

        <!-- 作者信息 -->
        <div class="flex-1 min-w-0">
          <div class="flex items-center space-x-2">
            <p class="text-sm font-medium text-gray-900 dark:text-white truncate">
              {{ author.username }}
            </p>
            <UBadge
              v-if="author.post_count >= 10"
              label="活跃"
              color="success"
              variant="soft"
              size="xs"
            />
          </div>
          <p class="text-xs text-gray-500 dark:text-gray-400">
            {{ author.post_count }} 篇文章
          </p>
        </div>

        <!-- 操作按钮 -->
        <NuxtLink
          :to="`/authors/${author.id}`"
          class="text-primary-600 dark:text-primary-400 hover:text-primary-700 dark:hover:text-primary-300 transition-colors"
        >
          <UIcon
            name="i-heroicons-arrow-right"
            class="w-4 h-4"
          />
        </NuxtLink>
      </div>
    </div>

    <!-- 空状态 -->
    <div
      v-else
      class="text-center py-8"
    >
      <UIcon
        name="i-heroicons-users"
        class="w-12 h-12 text-gray-400 mx-auto mb-2"
      />
      <p class="text-gray-500 dark:text-gray-400 text-sm">
        暂无活跃作者
      </p>
    </div>

    <!-- 查看更多链接 -->
    <div
      v-if="authors && authors.length >= 5"
      class="text-center"
    >
      <NuxtLink
        to="/authors"
        class="text-sm text-primary-600 dark:text-primary-400 hover:text-primary-700 dark:hover:text-primary-300 transition-colors"
      >
        查看更多作者 →
      </NuxtLink>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { User } from "~/types";

interface AuthorWithStats extends User {
  post_count: number;
  is_online?: boolean;
}

// 获取活跃作者
const { data: authors, pending } = await useLazyFetch<AuthorWithStats[]>("/api/authors/active", {
  query: {
    limit: 5,
    with_stats: true,
  },
});
</script>

<style scoped>
/* 确保卡片在悬停时有微妙的阴影效果 */
.author-card:hover {
  box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
}
</style>
