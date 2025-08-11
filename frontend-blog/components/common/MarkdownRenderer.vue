<template>
  <div class="markdown-renderer">
    <!-- 目录已完全移除 -->

    <!-- Markdown 内容 -->
    <div
      v-if="renderedHtml"
      class="markdown-content prose prose-gray dark:prose-invert max-w-none"
      v-html="renderedHtml"
    />

    <!-- 空内容提示 -->
    <div
      v-else-if="!loading"
      class="empty-content text-center py-8"
    >
      <UIcon
        name="i-heroicons-document-text"
        class="w-12 h-12 text-gray-400 mx-auto mb-4"
      />
      <p class="text-gray-500 dark:text-gray-400">
        暂无内容
      </p>
    </div>

    <!-- 加载状态 -->
    <div
      v-else
      class="loading-content"
    >
      <USkeleton class="h-4 w-full mb-4" />
      <USkeleton class="h-4 w-3/4 mb-4" />
      <USkeleton class="h-4 w-1/2 mb-4" />
      <USkeleton class="h-32 w-full" />
    </div>
  </div>
</template>

<script setup lang="ts">
import TocTree from "~/components/common/TocTree.vue";

interface Props {
  content?: string;
  showToc?: boolean;
  loading?: boolean;
  options?: {
    html?: boolean;
    breaks?: boolean;
    linkify?: boolean;
    typographer?: boolean;
    highlight?: boolean;
    anchor?: boolean;
    toc?: boolean;
  };
}

const props = withDefaults(defineProps<Props>(), {
  content: "",
  showToc: true,
  loading: false,
  options: () => ({}),
});

// 渲染结果
const renderedHtml = computed(() => {
  if (!props.content || props.loading) return "";

  try {
    // 根据showToc动态配置markdown选项
    const options = {
      ...props.options,
      toc: props.showToc, // 只有在showToc为true时才启用目录插件
    };

    const { render } = useMarkdown(options);
    const result = render(props.content);

    // 如果不显示目录，则从HTML中移除所有目录相关元素
    if (!props.showToc && result.html) {
      const cleanHtml = result.html;

      // HTML内容本身是干净的，无需额外清理
      return cleanHtml;
    }

    return result.html;
  }
  catch (error) {
    console.error("Markdown 渲染失败:", error);
    return "<div class=\"error\">内容渲染失败</div>";
  }
});

// 目录数据 - 强制返回空数组
const toc = computed(() => {
  return []; // 强制禁用目录
});
</script>

<style scoped>
.markdown-renderer {
  width: 100%;
}

.toc-container {
  position: sticky;
  top: 2rem;
  max-height: calc(100vh - 4rem);
  overflow-y: auto;
}

.toc-nav {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.loading-content {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.empty-content {
  text-align: center;
  padding: 3rem 0;
}

/* Markdown 内容样式 */
.markdown-content {
  line-height: 1.625;
}

/* 隐藏markdown-it-table-of-contents插件生成的目录 */
.markdown-content .table-of-contents,
.markdown-content div[class*="table-of-contents"],
.markdown-content nav[class*="table-of-contents"],
.markdown-content ul[class*="table-of-contents"] {
  display: none !important;
}

.markdown-content h1,
.markdown-content h2,
.markdown-content h3,
.markdown-content h4,
.markdown-content h5,
.markdown-content h6 {
  font-weight: 600;
  color: #111827;
  margin-top: 2rem;
  margin-bottom: 1rem;
  scroll-margin-top: 2rem;
}

.markdown-content h1 {
  font-size: 1.875rem;
  border-bottom: 1px solid #e5e7eb;
  padding-bottom: 0.5rem;
}

.markdown-content h2 {
  font-size: 1.5rem;
}

.markdown-content h3 {
  font-size: 1.25rem;
}

.markdown-content h4 {
  font-size: 1.125rem;
}

.markdown-content p {
  margin-bottom: 1rem;
  color: #374151;
}

.markdown-content a {
  color: #2563eb;
  text-decoration: none;
  transition: color 0.2s;
}

.markdown-content a:hover {
  color: #1d4ed8;
  text-decoration: underline;
}

.markdown-content ul,
.markdown-content ol {
  margin-bottom: 1rem;
  margin-left: 1.5rem;
}

.markdown-content li {
  margin-bottom: 0.5rem;
}

.markdown-content blockquote {
  border-left: 4px solid #3b82f6;
  padding-left: 1rem;
  padding-top: 0.5rem;
  padding-bottom: 0.5rem;
  margin: 1rem 0;
  background-color: #f9fafb;
  border-radius: 0 0.375rem 0.375rem 0;
}

.markdown-content code {
  background-color: #f3f4f6;
  padding: 0.125rem 0.25rem;
  border-radius: 0.25rem;
  font-size: 0.875rem;
  color: #dc2626;
}

.markdown-content pre {
  background-color: #1f2937;
  color: #f9fafb;
  padding: 1rem;
  border-radius: 0.5rem;
  overflow-x: auto;
  margin: 1rem 0;
}

.markdown-content pre code {
  background-color: transparent;
  color: inherit;
  padding: 0;
}

.markdown-content table {
  width: 100%;
  border-collapse: collapse;
  margin: 1rem 0;
}

.markdown-content th,
.markdown-content td {
  border: 1px solid #e5e7eb;
  padding: 0.75rem 1rem;
  text-align: left;
}

.markdown-content th {
  background-color: #f9fafb;
  font-weight: 600;
}

.markdown-content hr {
  margin: 2rem 0;
  border-color: #e5e7eb;
}

.markdown-content img {
  max-width: 100%;
  height: auto;
  border-radius: 0.5rem;
  box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1);
  margin: 1rem 0;
}

.markdown-content .header-anchor {
  opacity: 0;
  transition: opacity 0.2s;
  color: #3b82f6;
  text-decoration: none;
  margin-left: 0.5rem;
}

.markdown-content h1:hover .header-anchor,
.markdown-content h2:hover .header-anchor,
.markdown-content h3:hover .header-anchor,
.markdown-content h4:hover .header-anchor,
.markdown-content h5:hover .header-anchor,
.markdown-content h6:hover .header-anchor {
  opacity: 1;
}

.markdown-content .header-anchor:hover {
  color: #1d4ed8;
}

.hljs {
  font-size: 0.875rem;
}

/* 响应式调整 */
@media (max-width: 768px) {
  .toc-container {
    position: static;
    max-height: none;
    overflow-y: visible;
  }

  .markdown-content {
    font-size: 0.875rem;
  }

  .markdown-content h1 {
    font-size: 1.5rem;
  }

  .markdown-content h2 {
    font-size: 1.25rem;
  }

  .markdown-content h3 {
    font-size: 1.125rem;
  }
}

/* 深色主题支持 */
@media (prefers-color-scheme: dark) {
  .markdown-content h1,
  .markdown-content h2,
  .markdown-content h3,
  .markdown-content h4,
  .markdown-content h5,
  .markdown-content h6 {
    color: #f9fafb;
  }

  .markdown-content h1 {
    border-bottom-color: #374151;
  }

  .markdown-content p {
    color: #d1d5db;
  }

  .markdown-content a {
    color: #60a5fa;
  }

  .markdown-content a:hover {
    color: #93c5fd;
  }

  .markdown-content blockquote {
    background-color: #1f2937;
  }

  .markdown-content code {
    background-color: #1f2937;
    color: #f87171;
  }

  .markdown-content th,
  .markdown-content td {
    border-color: #374151;
  }

  .markdown-content th {
    background-color: #1f2937;
  }

  .markdown-content hr {
    border-color: #374151;
  }

  .markdown-content .header-anchor {
    color: #60a5fa;
  }

  .markdown-content .header-anchor:hover {
    color: #93c5fd;
  }
}
</style>
