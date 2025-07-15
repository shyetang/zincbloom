<template>
  <aside class="w-full space-y-6">
    <!-- 搜索框 -->
    <div class="modern-sidebar-card">
      <div class="modern-sidebar-header">
        <div
          class="modern-gradient-icon"
          style="background: linear-gradient(45deg, #64748b, #475569); width: 1.5rem; height: 1.5rem;"
        >
          <UIcon
            name="i-heroicons-magnifying-glass"
            class="h-3 w-3 text-white"
          />
        </div>
        <h3 class="modern-sidebar-title">
          搜索
        </h3>
      </div>
      <div class="relative">
        <UInput
          v-model="searchQuery"
          icon="i-heroicons-magnifying-glass"
          placeholder="搜索文章、标签、作者..."
          class="w-full rounded-xl border-slate-200 bg-slate-50 focus:border-slate-300 focus:bg-white dark:border-slate-600 dark:bg-slate-700 dark:focus:border-slate-500 dark:focus:bg-slate-800"
          @keyup.enter="handleSearch"
        />
        <UButton
          v-if="searchQuery"
          icon="i-heroicons-x-mark"
          variant="ghost"
          color="neutral"
          size="xs"
          class="absolute right-2 top-1/2 -translate-y-1/2"
          @click="clearSearch"
        />
      </div>
      <div class="mt-3 text-xs text-slate-500 dark:text-slate-400">
        提示: 按 <kbd class="rounded bg-slate-100 px-1.5 py-0.5 text-xs font-medium text-slate-900 dark:bg-slate-700 dark:text-slate-300">Enter</kbd> 搜索
      </div>
    </div>

    <!-- 热门标签云 -->
    <div class="modern-sidebar-card">
      <TagCloud />
    </div>

    <!-- 活跃作者 -->
    <div class="modern-sidebar-card">
      <AuthorCard />
    </div>

    <!-- 统计信息 -->
    <div class="modern-sidebar-card">
      <div class="modern-sidebar-header">
        <div
          class="modern-gradient-icon"
          style="background: linear-gradient(45deg, #10b981, #059669); width: 1.5rem; height: 1.5rem;"
        >
          <UIcon
            name="i-heroicons-chart-bar"
            class="h-3 w-3 text-white"
          />
        </div>
        <h3 class="modern-sidebar-title">
          网站统计
        </h3>
      </div>

      <div
        v-if="statsPending"
        class="space-y-3"
      >
        <div
          v-for="i in 4"
          :key="i"
          class="flex justify-between"
        >
          <USkeleton class="h-4 w-20" />
          <USkeleton class="h-4 w-12" />
        </div>
      </div>

      <div
        v-else
        class="space-y-3"
      >
        <div class="flex justify-between items-center">
          <span class="text-sm text-gray-600 dark:text-gray-400">总文章数</span>
          <span class="text-sm font-medium text-gray-900 dark:text-white">
            {{ stats?.total_posts || 0 }}
          </span>
        </div>
        <div class="flex justify-between items-center">
          <span class="text-sm text-gray-600 dark:text-gray-400">总用户数</span>
          <span class="text-sm font-medium text-gray-900 dark:text-white">
            {{ stats?.total_users || 0 }}
          </span>
        </div>
        <div class="flex justify-between items-center">
          <span class="text-sm text-gray-600 dark:text-gray-400">总标签数</span>
          <span class="text-sm font-medium text-gray-900 dark:text-white">
            {{ stats?.total_tags || 0 }}
          </span>
        </div>
        <div class="flex justify-between items-center">
          <span class="text-sm text-gray-600 dark:text-gray-400">总分类数</span>
          <span class="text-sm font-medium text-gray-900 dark:text-white">
            {{ stats?.total_categories || 0 }}
          </span>
        </div>
      </div>
    </div>

    <!-- 最新评论 -->
    <div class="modern-sidebar-card">
      <div class="modern-sidebar-header">
        <div
          class="modern-gradient-icon"
          style="background: linear-gradient(45deg, #6366f1, #8b5cf6); width: 1.5rem; height: 1.5rem;"
        >
          <UIcon
            name="i-heroicons-chat-bubble-left-right"
            class="h-3 w-3 text-white"
          />
        </div>
        <h3 class="modern-sidebar-title">
          最新评论
        </h3>
      </div>

      <div class="space-y-4">
        <div class="text-center py-8">
          <UIcon
            name="i-heroicons-chat-bubble-left-right"
            class="w-12 h-12 text-slate-400 mx-auto mb-2"
          />
          <p class="text-slate-500 dark:text-slate-400 text-sm">
            评论功能即将上线
          </p>
        </div>
      </div>
    </div>

    <!-- 友情链接 -->
    <div class="modern-sidebar-card">
      <div class="modern-sidebar-header">
        <div
          class="modern-gradient-icon"
          style="background: linear-gradient(45deg, #f43f5e, #ec4899); width: 1.5rem; height: 1.5rem;"
        >
          <UIcon
            name="i-heroicons-link"
            class="h-3 w-3 text-white"
          />
        </div>
        <h3 class="modern-sidebar-title">
          友情链接
        </h3>
      </div>

      <div class="space-y-2">
        <a
          v-for="link in friendLinks"
          :key="link.name"
          :href="link.url"
          target="_blank"
          rel="noopener noreferrer"
          class="block rounded-lg px-3 py-2 text-sm text-slate-600 transition-colors hover:bg-slate-100 hover:text-slate-900 dark:text-slate-400 dark:hover:bg-slate-700 dark:hover:text-slate-200"
        >
          {{ link.name }}
        </a>
      </div>
    </div>
  </aside>
</template>

<script setup lang="ts">
import TagCloud from "~/components/common/TagCloud.vue";
import AuthorCard from "~/components/common/AuthorCard.vue";

const router = useRouter();

// 搜索相关
const searchQuery = ref("");

// 获取网站统计信息
interface SiteStats {
  total_posts: number;
  total_users: number;
  total_tags: number;
  total_categories: number;
}

const { data: stats, pending: statsPending } = await useLazyFetch<SiteStats>("/api/stats");

// 友情链接数据
const friendLinks = ref([
  { name: "Vue.js", url: "https://vuejs.org" },
  { name: "Nuxt.js", url: "https://nuxt.com" },
  { name: "Rust", url: "https://www.rust-lang.org" },
  { name: "Axum", url: "https://github.com/tokio-rs/axum" },
  { name: "Tailwind CSS", url: "https://tailwindcss.com" },
]);

// 搜索功能
const handleSearch = () => {
  if (searchQuery.value.trim()) {
    router.push({
      path: "/posts",
      query: { search: searchQuery.value.trim() },
    });
  }
};

// 清除搜索
const clearSearch = () => {
  searchQuery.value = "";
};
</script>

<style scoped>
kbd {
  display: inline-block;
  padding: 0.125rem 0.25rem;
  font-size: 0.75rem;
  font-weight: 600;
  line-height: 1;
  color: rgb(55 65 81);
  background-color: rgb(243 244 246);
  border: 1px solid rgb(209 213 219);
  border-radius: 0.25rem;
  box-shadow: inset 0 -1px 0 rgb(209 213 219);
}

.dark kbd {
  color: rgb(209 213 219);
  background-color: rgb(55 65 81);
  border-color: rgb(75 85 99);
  box-shadow: inset 0 -1px 0 rgb(75 85 99);
}
</style>
