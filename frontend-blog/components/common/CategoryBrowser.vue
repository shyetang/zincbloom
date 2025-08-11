<template>
  <div>
    <!-- 标题区域 -->
    <div class="modern-section-header">
      <div>
        <div
          style="
                        display: flex;
                        align-items: center;
                        gap: 0.75rem;
                        margin-bottom: 0.5rem;
                    "
        >
          <div
            class="modern-gradient-icon"
            style="
                            background: linear-gradient(
                                45deg,
                                #06b6d4,
                                #0891b2
                            );
                            width: 2.5rem;
                            height: 2.5rem;
                        "
          >
            <UIcon
              name="i-heroicons-squares-2x2"
              class="h-5 w-5 text-white"
            />
          </div>
          <h2 class="modern-section-title">
            分类浏览
          </h2>
        </div>
        <p class="modern-section-subtitle">
          探索不同主题的精彩内容
        </p>
      </div>
      <a
        href="/categories"
        style="
                    padding: 0.5rem 1rem;
                    border-radius: 0.75rem;
                    font-size: 0.875rem;
                    text-decoration: none;
                    color: #64748b;
                    background: rgba(248, 250, 252, 0.8);
                    border: 1px solid rgba(226, 232, 240, 0.6);
                    transition: all 0.2s ease;
                    display: flex;
                    align-items: center;
                    gap: 0.5rem;
                "
        class="hover:text-blue-600 hover:border-blue-300"
      >
        查看全部
        <UIcon
          name="i-heroicons-arrow-right"
          class="w-4 h-4"
        />
      </a>
    </div>

    <!-- 加载状态 -->
    <div
      v-if="pending"
      class="modern-grid-cards"
    >
      <div
        v-for="i in 6"
        :key="i"
        class="modern-post-card"
        style="padding: 1.5rem; height: auto"
      >
        <div
          style="
                        display: flex;
                        align-items: center;
                        gap: 0.75rem;
                        margin-bottom: 0.75rem;
                    "
        >
          <USkeleton class="w-10 h-10 rounded-lg" />
          <USkeleton class="h-5 w-24" />
        </div>
        <USkeleton class="h-4 w-full mb-2" />
        <USkeleton class="h-3 w-16" />
      </div>
    </div>

    <!-- 分类网格 -->
    <div
      v-else-if="categories && categories.length"
      class="modern-grid-cards"
    >
      <NuxtLink
        v-for="category in categories"
        :key="category.id"
        :to="`/posts?category=${category.slug}`"
        class="modern-post-card"
        style="
                    display: block;
                    text-decoration: none;
                    padding: 1.5rem;
                    height: auto;
                    transition: all 0.3s ease;
                "
      >
        <!-- 分类图标和标题 -->
        <div
          style="
                        display: flex;
                        align-items: center;
                        gap: 0.75rem;
                        margin-bottom: 0.75rem;
                    "
        >
          <div
            style="
                            width: 2.5rem;
                            height: 2.5rem;
                            border-radius: 0.75rem;
                            display: flex;
                            align-items: center;
                            justify-content: center;
                            flex-shrink: 0;
                        "
            :style="{
              background: getCategoryGradient(category.slug),
            }"
          >
            <UIcon
              :name="getCategoryIcon(category.slug)"
              class="w-5 h-5 text-white"
            />
          </div>
          <div style="flex: 1; min-width: 0">
            <h3
              style="
                                font-size: 1rem;
                                font-weight: 600;
                                color: #1e293b;
                                margin: 0;
                            "
              class="dark:text-white"
            >
              {{ category.name }}
            </h3>
          </div>
        </div>

        <!-- 分类描述 -->
        <p
          style="
                        font-size: 0.875rem;
                        color: #64748b;
                        margin-bottom: 0.75rem;
                        line-height: 1.5;
                        display: -webkit-box;
                        -webkit-line-clamp: 2;
                        -webkit-box-orient: vertical;
                        overflow: hidden;
                    "
          class="dark:text-slate-400"
        >
          {{ category.description || "暂无描述" }}
        </p>

        <!-- 文章数量 -->
        <div
          style="
                        display: flex;
                        align-items: center;
                        justify-content: space-between;
                    "
        >
          <span
            style="font-size: 0.875rem; color: #94a3b8"
            class="dark:text-slate-500"
          >
            {{ category.post_count }} 篇文章
          </span>
          <UIcon
            name="i-heroicons-arrow-right"
            style="
                            width: 1rem;
                            height: 1rem;
                            color: #94a3b8;
                            transition: all 0.2s ease;
                        "
            class="group-hover:text-blue-600 group-hover:transform group-hover:translate-x-1"
          />
        </div>
      </NuxtLink>
    </div>

    <!-- 空状态 -->
    <div
      v-else
      style="text-align: center; padding: 3rem 0"
    >
      <div
        style="
                    width: 4rem;
                    height: 4rem;
                    margin: 0 auto 1rem;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    background: rgba(148, 163, 175, 0.1);
                    border-radius: 50%;
                "
      >
        <UIcon
          name="i-heroicons-folder"
          class="w-8 h-8 text-gray-400"
        />
      </div>
      <p
        style="color: #64748b; font-size: 0.875rem"
        class="dark:text-gray-400"
      >
        暂无分类
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Category } from "~/types";

interface CategoryWithCount extends Category {
  post_count: number;
}

// 获取所有分类（后端没有featured参数，使用基础接口）
const runtimeConfig = useRuntimeConfig();
const { data: categoriesResponse, pending } = await useLazyFetch(
  `${runtimeConfig.public.apiBaseUrl}/categories`,
);

// 转换为带数量的分类格式（模拟数据）
const categories = computed(() => {
  if (!categoriesResponse.value || !Array.isArray(categoriesResponse.value))
    return [];
  return categoriesResponse.value.slice(0, 6).map((category: any) => ({
    ...category,
    post_count: Math.floor(Math.random() * 20) + 1, // 模拟数据，后续需要后端支持
  }));
});

// 根据分类标识获取图标
const getCategoryIcon = (slug: string) => {
  const iconMap: Record<string, string> = {
    tech: "i-heroicons-computer-desktop",
    technology: "i-heroicons-computer-desktop",
    programming: "i-heroicons-code-bracket",
    life: "i-heroicons-heart",
    lifestyle: "i-heroicons-heart",
    travel: "i-heroicons-map-pin",
    food: "i-heroicons-cake",
    photography: "i-heroicons-camera",
    music: "i-heroicons-musical-note",
    book: "i-heroicons-book-open",
    notes: "i-heroicons-pencil-square",
    tutorial: "i-heroicons-academic-cap",
    news: "i-heroicons-newspaper",
    review: "i-heroicons-star",
    opensource: "i-heroicons-code-bracket-square",
    random: "i-heroicons-chat-bubble-left-right",
    thoughts: "i-heroicons-light-bulb",
    default: "i-heroicons-folder",
  };
  return iconMap[slug] || iconMap.default;
};

// 根据分类标识获取渐变背景
const getCategoryGradient = (slug: string) => {
  const gradientMap: Record<string, string> = {
    tech: "linear-gradient(45deg, #3b82f6, #1d4ed8)",
    technology: "linear-gradient(45deg, #3b82f6, #1d4ed8)",
    programming: "linear-gradient(45deg, #10b981, #059669)",
    life: "linear-gradient(45deg, #ec4899, #be185d)",
    lifestyle: "linear-gradient(45deg, #ec4899, #be185d)",
    travel: "linear-gradient(45deg, #6366f1, #4f46e5)",
    food: "linear-gradient(45deg, #f97316, #ea580c)",
    photography: "linear-gradient(45deg, #8b5cf6, #7c3aed)",
    music: "linear-gradient(45deg, #eab308, #ca8a04)",
    book: "linear-gradient(45deg, #ef4444, #dc2626)",
    notes: "linear-gradient(45deg, #6b7280, #4b5563)",
    tutorial: "linear-gradient(45deg, #059669, #047857)",
    news: "linear-gradient(45deg, #06b6d4, #0891b2)",
    review: "linear-gradient(45deg, #f59e0b, #d97706)",
    opensource: "linear-gradient(45deg, #14b8a6, #0d9488)",
    random: "linear-gradient(45deg, #f43f5e, #e11d48)",
    thoughts: "linear-gradient(45deg, #8b5cf6, #7c3aed)",
    default: "linear-gradient(45deg, #6b7280, #4b5563)",
  };
  return gradientMap[slug] || gradientMap.default;
};
</script>
