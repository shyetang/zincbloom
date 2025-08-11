<template>
  <div>
    <!-- Modern Hero Section -->
    <section
      class="modern-hero"
      style="padding: 4rem 0 3rem 0;"
    >
      <div class="relative mx-auto max-w-4xl px-4 sm:px-6 lg:px-8">
        <div style="text-align: center;">
          <!-- Modern Icon -->
          <div style="margin: 0 auto 1.5rem; width: 4rem; height: 4rem; display: flex; align-items: center; justify-content: center; border-radius: 50%; background: linear-gradient(45deg, #f59e0b, #f97316); box-shadow: 0 8px 32px rgba(245, 158, 11, 0.3);">
            <UIcon
              name="i-heroicons-pencil-square"
              style="width: 2rem; height: 2rem; color: white;"
            />
          </div>

          <!-- Modern Title -->
          <h1
            style="font-size: 2.75rem; font-weight: 700; line-height: 1.1; margin-bottom: 1rem; color: #1e293b;"
            class="dark:text-white"
          >
            开始创作
          </h1>

          <!-- Subtitle -->
          <p
            style="font-size: 1.125rem; color: #64748b; max-width: 32rem; margin: 0 auto; line-height: 1.5;"
            class="dark:text-slate-300"
          >
            分享您的想法，记录珍贵经历，用文字触动世界的心灵
          </p>
        </div>
      </div>
    </section>

    <!-- Main Content Section -->
    <section
      class="modern-content-section"
      style="background: linear-gradient(135deg, #f8fafc 0%, #e2e8f0 50%, #eff6ff 100%); padding: 2rem 0; margin: 0;"
    >
      <div class="mx-auto max-w-4xl px-4 sm:px-6 lg:px-8">
        <!-- 创作选项 -->
        <div class="grid md:grid-cols-2 gap-8 mb-12">
          <!-- 新建文章 -->
          <div
            class="modern-feature-card group cursor-pointer"
            @click="createNewPost"
          >
            <div
              class="modern-feature-icon"
              style="background: linear-gradient(45deg, #3b82f6, #6366f1); margin-bottom: 1.5rem;"
            >
              <UIcon
                name="i-heroicons-pencil-square"
                class="w-6 h-6 text-white"
              />
            </div>
            <h3 class="text-2xl font-bold text-slate-900 dark:text-white text-center mb-4">
              写新文章
            </h3>
            <p class="text-slate-600 dark:text-slate-400 text-center mb-8 leading-relaxed">
              创建一篇全新的文章，分享您独特的想法和深刻见解
            </p>
            <UButton
              size="lg"
              block
              :loading="creating"
              class="modern-button modern-button-primary"
              style="padding: 1rem 2rem; background: linear-gradient(45deg, #3b82f6, #6366f1); color: white; border: none; font-weight: 600;"
              @click.stop="createNewPost"
            >
              <UIcon
                name="i-heroicons-sparkles"
                class="w-5 h-5 mr-2"
              />
              开始写作
            </UButton>
          </div>

          <!-- 草稿箱 -->
          <div class="modern-feature-card group">
            <div
              class="modern-feature-icon"
              style="background: linear-gradient(45deg, #f59e0b, #f97316); margin-bottom: 1.5rem;"
            >
              <UIcon
                name="i-heroicons-document-text"
                class="w-6 h-6 text-white"
              />
            </div>
            <h3 class="text-2xl font-bold text-slate-900 dark:text-white text-center mb-4">
              继续草稿
            </h3>
            <p class="text-slate-600 dark:text-slate-400 text-center mb-8 leading-relaxed">
              继续完善您未完成的草稿，让创意得到完美呈现
            </p>
            <UButton
              to="/user/posts?status=draft"
              size="lg"
              block
              class="modern-button modern-button-secondary"
              style="padding: 1rem 2rem; border: 2px solid #f59e0b; color: #f59e0b; font-weight: 600;"
            >
              <UIcon
                name="i-heroicons-folder-open"
                class="w-5 h-5 mr-2"
              />
              查看草稿 ({{ draftCount }})
            </UButton>
          </div>
        </div>

        <!-- 最近草稿 -->
        <div
          v-if="(recentDrafts as any)?.data?.length"
          class="modern-content-section mb-12"
          style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 2rem; margin-bottom: 1.5rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);"
        >
          <div class="modern-section-header">
            <div>
              <div class="flex items-center gap-3 mb-2">
                <div
                  class="modern-gradient-icon"
                  style="background: linear-gradient(45deg, #8b5cf6, #a855f7); width: 2.5rem; height: 2.5rem;"
                >
                  <UIcon
                    name="i-heroicons-clock"
                    class="h-5 w-5 text-white"
                  />
                </div>
                <h2
                  class="modern-section-title"
                  style="font-size: 1.5rem;"
                >
                  最近的草稿
                </h2>
              </div>
              <p class="modern-section-subtitle">
                继续您未完成的创作
              </p>
            </div>
          </div>

          <div
            class="space-y-4"
            style="padding: 0 1rem;"
          >
            <div
              v-for="draft in (recentDrafts as any)?.data || []"
              :key="draft.id"
              class="modern-post-card"
              style="padding: 2rem;"
            >
              <div class="flex items-start justify-between">
                <div class="flex-1">
                  <h3 class="text-xl font-bold text-slate-900 dark:text-white mb-3">
                    {{ draft.title || "无标题草稿" }}
                  </h3>
                  <div class="flex items-center gap-2 mb-3">
                    <UIcon
                      name="i-heroicons-calendar"
                      class="w-4 h-4 text-slate-500"
                    />
                    <p class="text-sm text-slate-500 dark:text-slate-400">
                      {{ formatDate(draft.updated_at) }} 更新
                    </p>
                  </div>
                  <p class="text-slate-600 dark:text-slate-400 line-clamp-2 leading-relaxed">
                    {{ getExcerpt(draft.content_markdown) }}
                  </p>
                </div>
                <div class="ml-6 flex-shrink-0">
                  <UButton
                    :to="`/write/${draft.id}`"
                    class="modern-button modern-button-secondary"
                    style="padding: 0.75rem 1.5rem; border: 2px solid #8b5cf6; color: #8b5cf6; font-weight: 500;"
                  >
                    <UIcon
                      name="i-heroicons-pencil"
                      class="w-4 h-4 mr-2"
                    />
                    继续编辑
                  </UButton>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- 写作统计 -->
        <div
          class="modern-content-section mb-12"
          style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 2rem; margin-bottom: 1.5rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);"
        >
          <div class="modern-section-header">
            <div>
              <div class="flex items-center gap-3 mb-2">
                <div
                  class="modern-gradient-icon"
                  style="background: linear-gradient(45deg, #06b6d4, #0891b2); width: 2.5rem; height: 2.5rem;"
                >
                  <UIcon
                    name="i-heroicons-chart-bar"
                    class="h-5 w-5 text-white"
                  />
                </div>
                <h2
                  class="modern-section-title"
                  style="font-size: 1.5rem;"
                >
                  写作统计
                </h2>
              </div>
              <p class="modern-section-subtitle">
                记录您的创作成就
              </p>
            </div>
          </div>

          <div
            class="grid grid-cols-2 md:grid-cols-4 gap-6"
            style="padding: 0 1rem;"
          >
            <div
              class="modern-post-card text-center"
              style="padding: 1.5rem;"
            >
              <div
                class="modern-gradient-icon"
                style="background: linear-gradient(45deg, #3b82f6, #6366f1); width: 3rem; height: 3rem; margin: 0 auto 1rem;"
              >
                <UIcon
                  name="i-heroicons-document-text"
                  class="w-5 h-5 text-white"
                />
              </div>
              <div class="text-3xl font-bold text-slate-900 dark:text-white mb-2">
                {{ (stats as any)?.totalPosts || 0 }}
              </div>
              <div class="text-sm text-slate-600 dark:text-slate-400 font-medium">
                总文章数
              </div>
            </div>

            <div
              class="modern-post-card text-center"
              style="padding: 1.5rem;"
            >
              <div
                class="modern-gradient-icon"
                style="background: linear-gradient(45deg, #10b981, #059669); width: 3rem; height: 3rem; margin: 0 auto 1rem;"
              >
                <UIcon
                  name="i-heroicons-rocket-launch"
                  class="w-5 h-5 text-white"
                />
              </div>
              <div class="text-3xl font-bold text-slate-900 dark:text-white mb-2">
                {{ (stats as any)?.publishedPosts || 0 }}
              </div>
              <div class="text-sm text-slate-600 dark:text-slate-400 font-medium">
                已发布
              </div>
            </div>

            <div
              class="modern-post-card text-center"
              style="padding: 1.5rem;"
            >
              <div
                class="modern-gradient-icon"
                style="background: linear-gradient(45deg, #f59e0b, #d97706); width: 3rem; height: 3rem; margin: 0 auto 1rem;"
              >
                <UIcon
                  name="i-heroicons-document"
                  class="w-5 h-5 text-white"
                />
              </div>
              <div class="text-3xl font-bold text-slate-900 dark:text-white mb-2">
                {{ (stats as any)?.draftPosts || 0 }}
              </div>
              <div class="text-sm text-slate-600 dark:text-slate-400 font-medium">
                草稿
              </div>
            </div>

            <div
              class="modern-post-card text-center"
              style="padding: 1.5rem;"
            >
              <div
                class="modern-gradient-icon"
                style="background: linear-gradient(45deg, #8b5cf6, #7c3aed); width: 3rem; height: 3rem; margin: 0 auto 1rem;"
              >
                <UIcon
                  name="i-heroicons-pencil"
                  class="w-5 h-5 text-white"
                />
              </div>
              <div class="text-3xl font-bold text-slate-900 dark:text-white mb-2">
                {{ (stats as any)?.totalWords || 0 }}
              </div>
              <div class="text-sm text-slate-600 dark:text-slate-400 font-medium">
                总字数
              </div>
            </div>
          </div>
        </div>

        <!-- 写作技巧 -->
        <div
          class="modern-features-section"
          style="background: linear-gradient(135deg, #f8fafc 0%, #e0f2fe 100%); border-radius: 1.5rem; padding: 3rem; position: relative; overflow: hidden;"
        >
          <div
            class="modern-section-header"
            style="text-align: center; margin-bottom: 3rem;"
          >
            <div>
              <div class="flex items-center justify-center gap-3 mb-4">
                <div
                  class="modern-gradient-icon"
                  style="background: linear-gradient(45deg, #f59e0b, #f97316); width: 3rem; height: 3rem;"
                >
                  <UIcon
                    name="i-heroicons-academic-cap"
                    class="h-6 w-6 text-white"
                  />
                </div>
                <h2
                  class="modern-section-title"
                  style="font-size: 2rem;"
                >
                  写作小贴士
                </h2>
              </div>
              <p
                class="modern-section-subtitle"
                style="font-size: 1rem;"
              >
                掌握这些技巧，让您的文章更具魅力
              </p>
            </div>
          </div>

          <div class="grid md:grid-cols-2 gap-8">
            <div
              class="modern-feature-card"
              style="background: rgba(255, 255, 255, 0.8); backdrop-filter: blur(10px); padding: 2rem;"
            >
              <div
                class="modern-feature-icon"
                style="background: linear-gradient(45deg, #fbbf24, #f59e0b); margin-bottom: 1.5rem;"
              >
                <UIcon
                  name="i-heroicons-light-bulb"
                  class="w-6 h-6 text-white"
                />
              </div>
              <h3 class="text-xl font-bold text-slate-900 dark:text-white mb-3">
                明确主题
              </h3>
              <p class="text-slate-600 dark:text-slate-400 leading-relaxed">
                在开始写作前，先确定文章的核心主题和要传达的信息，让读者一目了然。
              </p>
            </div>

            <div
              class="modern-feature-card"
              style="background: rgba(255, 255, 255, 0.8); backdrop-filter: blur(10px); padding: 2rem;"
            >
              <div
                class="modern-feature-icon"
                style="background: linear-gradient(45deg, #3b82f6, #6366f1); margin-bottom: 1.5rem;"
              >
                <UIcon
                  name="i-heroicons-user-group"
                  class="w-6 h-6 text-white"
                />
              </div>
              <h3 class="text-xl font-bold text-slate-900 dark:text-white mb-3">
                了解读者
              </h3>
              <p class="text-slate-600 dark:text-slate-400 leading-relaxed">
                考虑您的目标读者群体，用他们能理解的语言和方式来表达观点。
              </p>
            </div>

            <div
              class="modern-feature-card"
              style="background: rgba(255, 255, 255, 0.8); backdrop-filter: blur(10px); padding: 2rem;"
            >
              <div
                class="modern-feature-icon"
                style="background: linear-gradient(45deg, #10b981, #059669); margin-bottom: 1.5rem;"
              >
                <UIcon
                  name="i-heroicons-squares-2x2"
                  class="w-6 h-6 text-white"
                />
              </div>
              <h3 class="text-xl font-bold text-slate-900 dark:text-white mb-3">
                结构清晰
              </h3>
              <p class="text-slate-600 dark:text-slate-400 leading-relaxed">
                使用标题、段落和列表来组织内容，让文章条理清晰，易于阅读。
              </p>
            </div>

            <div
              class="modern-feature-card"
              style="background: rgba(255, 255, 255, 0.8); backdrop-filter: blur(10px); padding: 2rem;"
            >
              <div
                class="modern-feature-icon"
                style="background: linear-gradient(45deg, #8b5cf6, #7c3aed); margin-bottom: 1.5rem;"
              >
                <UIcon
                  name="i-heroicons-arrow-path"
                  class="w-6 h-6 text-white"
                />
              </div>
              <h3 class="text-xl font-bold text-slate-900 dark:text-white mb-3">
                反复斟酌
              </h3>
              <p class="text-slate-600 dark:text-slate-400 leading-relaxed">
                好文章是改出来的，多次修改和完善您的内容，精益求精。
              </p>
            </div>
          </div>
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

// SEO
useHead({
  title: "开始写作",
});

// 状态
const creating = ref(false);
const router = useRouter();
const toast = useToast();

const apiClient = useApi();

// 获取草稿数据
const recentDrafts = ref({ data: [] as any[] });
const stats = ref({
  totalPosts: 0,
  publishedPosts: 0,
  draftPosts: 0,
  totalWords: 0,
} as any);

// 加载数据
onMounted(async () => {
  try {
    // 获取草稿数据
    const draftsResponse = await apiClient.getPosts({
      status: "draft",
      per_page: 3,
      sort: "updated_at",
      order: "desc",
      author: "me",
    });
    if (draftsResponse.success && draftsResponse.data) {
      recentDrafts.value = draftsResponse.data;
    }

    // 获取统计数据
    const statsResponse = await $fetch("http://localhost:8080/me/stats", {
      headers: {
        Authorization: `Bearer ${localStorage.getItem("access_token")}`,
      },
    });
    stats.value = statsResponse;
  }
  catch (error) {
    console.error("Failed to load data:", error);
  }
});

// 计算草稿数量
const draftCount = computed(() => (stats.value as any)?.draftPosts || 0);

// 方法
const createNewPost = async () => {
  creating.value = true;

  try {
    // 生成唯一的文章标题
    const timestamp = new Date().toISOString().replace(/[:.]/g, "-");
    const response = await apiClient.createPost({
      title: `新建文章_${timestamp}`,
      content_markdown: "# 开始您的创作...\n\n在这里输入您的文章内容。",
      status: "draft",
    });

    if (response.success) {
      router.push(`/write/${response.data?.id || "new"}`);
    }
    else {
      throw new Error(response.error?.message || "创建失败");
    }
  }
  catch (error) {
    toast.add({
      title: "创建失败",
      description: error instanceof Error ? error.message : "无法创建新文章，请稍后重试",
      color: "error",
    });
  }
  finally {
    creating.value = false;
  }
};

const getExcerpt = (content: string): string => {
  if (!content) return "空白草稿";
  const plainText = content.replace(/[#*`\[\]]/g, "").trim();
  return plainText.length > 100
    ? plainText.substring(0, 100) + "..."
    : plainText;
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
  else {
    return date.toLocaleDateString("zh-CN");
  }
};
</script>

<style scoped>
.line-clamp-2 {
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
}
</style>
