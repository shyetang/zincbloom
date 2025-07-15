<template>
  <div class="space-y-4">
    <div class="modern-sidebar-header">
      <div
        class="modern-gradient-icon"
        style="background: linear-gradient(45deg, #f59e0b, #f97316); width: 1.5rem; height: 1.5rem;"
      >
        <UIcon
          name="i-heroicons-fire"
          class="h-3 w-3 text-white"
        />
      </div>
      <h3 class="modern-sidebar-title">
        热门标签
      </h3>
    </div>

    <!-- 加载状态 -->
    <div
      v-if="pending"
      class="space-y-2"
    >
      <USkeleton
        v-for="i in 8"
        :key="i"
        class="h-6 w-16"
      />
    </div>

    <!-- 标签云 -->
    <div
      v-else-if="tags && tags.length"
      class="flex flex-wrap gap-2"
    >
      <NuxtLink
        v-for="tag in tags"
        :key="tag.id"
        :to="`/posts?tag=${tag.slug}`"
        class="tag-item"
        :class="getTagClass(tag.post_count)"
      >
        <UBadge
          :label="`#${tag.name}`"
          :color="getTagColor(tag.post_count)"
          variant="soft"
          :size="getTagSize(tag.post_count)"
        />
        <span class="text-xs text-gray-500 dark:text-gray-400 ml-1">
          {{ tag.post_count }}
        </span>
      </NuxtLink>
    </div>

    <!-- 空状态 -->
    <div
      v-else
      class="text-center py-8"
    >
      <UIcon
        name="i-heroicons-tag"
        class="w-12 h-12 text-gray-400 mx-auto mb-2"
      />
      <p class="text-gray-500 dark:text-gray-400 text-sm">
        暂无热门标签
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Tag } from "~/types";

interface TagWithCount extends Tag {
  post_count: number;
}

// 获取热门标签
const { data: tags, pending } = await useLazyFetch<TagWithCount[]>("/api/tags/popular", {
  query: {
    limit: 15,
    with_count: true,
  },
});

// 根据文章数量获取标签样式类
const getTagClass = (_count: number) => {
  return "hover:scale-105 transition-transform duration-200";
};

// 根据文章数量获取标签颜色
const getTagColor = (count: number) => {
  if (count >= 10) return "error";
  if (count >= 5) return "warning";
  if (count >= 3) return "primary";
  return "neutral";
};

// 根据文章数量获取标签大小
const getTagSize = (count: number) => {
  if (count >= 10) return "md";
  if (count >= 5) return "sm";
  return "xs";
};
</script>

<style scoped>
.tag-item {
  display: inline-flex;
  align-items: center;
  text-decoration: none;
}

.tag-item:hover {
  transform: translateY(-1px);
}
</style>
