<template>
  <article
    class="bg-white dark:bg-gray-800 rounded-lg shadow-sm hover:shadow-md transition-shadow border border-gray-200 dark:border-gray-700 overflow-hidden"
  >
    <!-- 文章封面 -->
    <div
      v-if="post.thumbnail"
      class="aspect-video overflow-hidden"
    >
      <NuxtLink :to="`/posts/${post.slug}`">
        <img
          :src="post.thumbnail"
          :alt="post.title"
          class="w-full h-full object-cover hover:scale-105 transition-transform duration-300"
          loading="lazy"
          @error="handleImageError"
        >
      </NuxtLink>
    </div>

    <!-- 文章内容 -->
    <div class="p-6">
      <!-- 分类和标签 -->
      <div
        v-if="post.categories?.length"
        class="flex items-center gap-2 mb-3"
      >
        <UBadge
          v-for="category in post.categories"
          :key="category.id"
          :label="category.name"
          color="primary"
          variant="soft"
          size="sm"
        />
      </div>

      <!-- 标题 -->
      <h3
        class="text-xl font-semibold text-gray-900 dark:text-white mb-3 line-clamp-2"
      >
        <NuxtLink
          :to="`/posts/${post.slug}`"
          class="hover:text-primary-600 dark:hover:text-primary-400 transition-colors"
        >
          {{ post.title || "无标题" }}
        </NuxtLink>
      </h3>

      <!-- 摘要 -->
      <p
        class="text-gray-600 dark:text-gray-300 text-sm mb-4 line-clamp-3"
      >
        {{ computedExcerpt }}
      </p>

      <!-- 标签 -->
      <div
        v-if="post.tags?.length"
        class="flex flex-wrap gap-1 mb-4"
      >
        <UBadge
          v-for="tag in displayTags"
          :key="tag.id"
          :label="`#${tag.name}`"
          color="neutral"
          variant="soft"
          size="xs"
        />
        <UBadge
          v-if="post.tags.length > 3"
          :label="`+${post.tags.length - 3}`"
          color="neutral"
          variant="soft"
          size="xs"
        />
      </div>

      <!-- 底部信息 -->
      <div
        class="flex items-center justify-between pt-4 border-t border-gray-200 dark:border-gray-700"
      >
        <!-- 作者信息 -->
        <div class="flex items-center gap-2">
          <div
            class="w-8 h-8 bg-primary-100 dark:bg-primary-900 rounded-full flex items-center justify-center"
          >
            <UIcon
              name="i-heroicons-user"
              class="w-4 h-4 text-primary-600 dark:text-primary-400"
            />
          </div>
          <div class="text-sm">
            <div class="font-medium text-gray-900 dark:text-white">
              {{ post.author?.username || "匿名" }}
            </div>
          </div>
        </div>

        <!-- 发布时间 -->
        <div class="text-xs text-gray-500 dark:text-gray-400">
          {{ formattedDate }}
        </div>
      </div>
    </div>
  </article>
</template>

<script setup lang="ts">
import type { PostWithContent } from "~/types";

const props = defineProps<{
  post: PostWithContent;
}>();

// 计算属性：处理摘要
const computedExcerpt = computed(() => {
  if (props.post.excerpt) {
    return props.post.excerpt;
  }
  return getExcerptFromContent(props.post.content || "");
});

// 计算属性：显示的标签（最多3个）
const displayTags = computed(() => {
  return props.post.tags?.slice(0, 3) || [];
});

// 计算属性：格式化的日期
const formattedDate = computed(() => {
  const dateString = props.post.published_at || props.post.created_at;
  if (!dateString) return "未知时间";
  return formatDate(dateString);
});

// 工具函数：从内容中提取摘要
const getExcerptFromContent = (content: string): string => {
  if (!content) return "暂无内容";

  try {
    // 移除 Markdown 标记
    const plainText = content
      .replace(/#{1,6}\s+/g, "") // 移除标题标记
      .replace(/\*\*(.*?)\*\*/g, "$1") // 移除粗体标记
      .replace(/\*(.*?)\*/g, "$1") // 移除斜体标记
      .replace(/`(.*?)`/g, "$1") // 移除行内代码标记
      .replace(/\[(.*?)\]\(.*?\)/g, "$1") // 移除链接，保留文本
      .replace(/!\[.*?\]\(.*?\)/g, "") // 移除图片
      .replace(/```[\s\S]*?```/g, "") // 移除代码块
      .replace(/\n\s*\n/g, " ") // 替换多个换行为空格
      .trim();

    return plainText.length > 150
      ? plainText.substring(0, 150) + "..."
      : plainText;
  }
  catch (error) {
    console.warn("提取摘要时出错:", error);
    return "内容摘要提取失败";
  }
};

// 工具函数：格式化日期
const formatDate = (dateString: string): string => {
  try {
    const date = new Date(dateString);

    // 检查日期是否有效
    if (isNaN(date.getTime())) {
      return "日期格式错误";
    }

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
      const weeks = Math.floor(diffDays / 7);
      return `${weeks} 周前`;
    }
    else if (diffDays < 365) {
      const months = Math.floor(diffDays / 30);
      return `${months} 个月前`;
    }
    else {
      return date.toLocaleDateString("zh-CN", {
        year: "numeric",
        month: "long",
        day: "numeric",
      });
    }
  }
  catch (error) {
    console.warn("格式化日期时出错:", error);
    return "时间未知";
  }
};

// 图片错误处理
const handleImageError = (event: Event) => {
  const img = event.target as HTMLImageElement;
  if (img) {
    // 可以设置一个默认图片或隐藏图片
    img.style.display = "none";
    console.warn("图片加载失败:", img.src);
  }
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

.line-clamp-3 {
    display: -webkit-box;
    -webkit-line-clamp: 3;
    line-clamp: 3;
    -webkit-box-orient: vertical;
    overflow: hidden;
}
</style>
