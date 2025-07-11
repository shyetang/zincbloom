<template>
  <ul class="toc-list">
    <li
      v-for="item in items"
      :key="item.anchor"
      class="toc-item"
      :class="{ active: currentActiveAnchor === item.anchor }"
    >
      <a
        :href="`#${item.anchor}`"
        class="toc-link"
        :class="[
          `toc-level-${item.level}`,
          { active: currentActiveAnchor === item.anchor },
        ]"
        @click="handleClick(item.anchor)"
      >
        {{ item.title }}
      </a>

      <!-- 递归渲染子项 -->
      <TocTree
        v-if="item.children.length > 0"
        :items="item.children"
        :active-anchor="currentActiveAnchor"
        class="toc-children"
      />
    </li>
  </ul>
</template>

<script setup lang="ts">
import type { TocItem } from "~/composables/useMarkdown";

interface Props {
  items?: TocItem[];
  activeAnchor?: string;
}

withDefaults(defineProps<Props>(), {
  items: () => [],
  activeAnchor: "",
});

// 监听滚动位置，高亮当前章节
const currentActiveAnchor = ref("");

// 处理点击事件
const handleClick = (anchor: string) => {
  const element = document.getElementById(anchor);
  if (element) {
    element.scrollIntoView({
      behavior: "smooth",
      block: "start",
    });
  }
};

// 监听页面滚动，更新激活的锚点
const updateActiveAnchor = () => {
  const headings = document.querySelectorAll("h1[id], h2[id], h3[id], h4[id], h5[id], h6[id]");
  const scrollTop = window.scrollY + 100; // 偏移量

  let currentAnchor = "";

  headings.forEach((heading) => {
    const element = heading as HTMLElement;
    const offsetTop = element.offsetTop;

    if (offsetTop <= scrollTop) {
      currentAnchor = element.id;
    }
  });

  currentActiveAnchor.value = currentAnchor;
};

// 生命周期管理
onMounted(() => {
  window.addEventListener("scroll", updateActiveAnchor, { passive: true });
  updateActiveAnchor(); // 初始化
});

onUnmounted(() => {
  window.removeEventListener("scroll", updateActiveAnchor);
});
</script>

<style scoped>
.toc-list {
  list-style: none;
  padding-left: 0;
  margin: 0;
}

.toc-list > li:not(:last-child) {
  margin-bottom: 0.25rem;
}

.toc-item {
  position: relative;
}

.toc-link {
  display: block;
  padding: 0.25rem 0.5rem;
  font-size: 0.875rem;
  border-radius: 0.25rem;
  transition: all 0.2s;
  color: #6b7280;
  text-decoration: none;
}

.toc-link:hover {
  color: #2563eb;
  background-color: #f3f4f6;
}

.toc-link.active {
  color: #2563eb;
  background-color: #eff6ff;
  border-left: 2px solid #3b82f6;
}

/* 不同层级的缩进 */
.toc-level-1 {
  font-weight: 500;
}

.toc-level-2 {
  padding-left: 0.75rem;
}

.toc-level-3 {
  padding-left: 1.5rem;
  font-size: 0.75rem;
}

.toc-level-4 {
  padding-left: 2.25rem;
  font-size: 0.75rem;
}

.toc-level-5 {
  padding-left: 3rem;
  font-size: 0.75rem;
}

.toc-level-6 {
  padding-left: 3.75rem;
  font-size: 0.75rem;
}

.toc-children {
  margin-top: 0.25rem;
  margin-left: 0.5rem;
}

/* 激活状态的层级指示器 */
.toc-item.active::before {
  content: '';
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 0.125rem;
  background-color: #3b82f6;
  border-radius: 0 0.125rem 0.125rem 0;
}

/* 深色主题支持 */
@media (prefers-color-scheme: dark) {
  .toc-link {
    color: #9ca3af;
  }

  .toc-link:hover {
    color: #60a5fa;
    background-color: #374151;
  }

  .toc-link.active {
    color: #60a5fa;
    background-color: #1e3a8a;
  }
}
</style>
