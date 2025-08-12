<template>
  <div
    class="min-h-screen bg-gradient-to-br from-slate-50 via-white to-blue-50 dark:from-gray-900 dark:via-gray-900 dark:to-blue-950"
  >
    <!-- 加载状态 -->
    <div
      v-if="pending"
      class="flex items-center justify-center min-h-screen"
    >
      <div class="text-center">
        <div class="relative">
          <div
            class="w-16 h-16 mx-auto mb-4 bg-gradient-to-r from-blue-500 to-purple-600 rounded-full flex items-center justify-center shadow-lg"
          >
            <UIcon
              name="i-heroicons-pencil-square"
              class="w-8 h-8 text-white animate-pulse"
            />
          </div>
          <div
            class="absolute inset-0 w-16 h-16 mx-auto border-4 border-blue-200 dark:border-blue-800 border-t-blue-500 rounded-full animate-spin"
          />
        </div>
        <p class="text-sm text-gray-600 dark:text-gray-400">
          正在加载编辑器...
        </p>
      </div>
    </div>

    <!-- 错误状态 -->
    <div
      v-else-if="error"
      class="flex items-center justify-center min-h-screen p-6"
    >
      <div class="text-center max-w-md">
        <div
          class="bg-gradient-to-br from-red-50 to-rose-100 dark:from-red-950/20 dark:to-rose-950/20 rounded-2xl p-8 shadow-xl border border-red-100 dark:border-red-800/30"
        >
          <UIcon
            name="i-heroicons-exclamation-triangle"
            class="w-16 h-16 text-red-500 mx-auto mb-4"
          />
          <h1
            class="text-2xl font-bold text-gray-900 dark:text-white mb-3"
          >
            文章不存在
          </h1>
          <p
            class="text-gray-600 dark:text-gray-400 mb-6 leading-relaxed"
          >
            您要编辑的文章不存在或您没有权限访问。
          </p>
          <UButton
            to="/write"
            color="primary"
            size="lg"
            class="shadow-lg hover:shadow-xl transition-all duration-200"
          >
            <UIcon
              name="i-heroicons-arrow-left"
              class="w-4 h-4 mr-2"
            />
            返回写作页面
          </UButton>
        </div>
      </div>
    </div>

    <!-- 主编辑界面 -->
    <div
      v-else
      class="flex flex-col h-screen"
    >
      <!-- 现代化顶部工具栏 -->
      <div
        class="sticky top-0 z-50 backdrop-blur-xl bg-white/80 dark:bg-gray-900/80 border-b border-gray-200/60 dark:border-gray-700/60 shadow-sm"
      >
        <div class="max-w-full px-6 py-4">
          <div class="flex items-center justify-between">
            <!-- 左侧信息区域 -->
            <div class="flex items-center space-x-6">
              <UButton
                variant="ghost"
                size="sm"
                class="hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors duration-200"
                @click="goBack"
              >
                <UIcon
                  name="i-heroicons-arrow-left"
                  class="w-4 h-4 mr-2"
                />
                返回
              </UButton>

              <div class="hidden md:flex items-center space-x-6">
                <!-- 保存状态 -->
                <div class="flex items-center space-x-2">
                  <div
                    class="flex items-center space-x-2 text-sm"
                  >
                    <div class="relative">
                      <UIcon
                        name="i-heroicons-clock"
                        class="w-4 h-4"
                        :class="
                          saving
                            ? 'text-blue-500'
                            : lastSaved
                              ? 'text-green-500'
                              : 'text-gray-400'
                        "
                      />
                      <div
                        v-if="saving"
                        class="absolute -inset-1 border-2 border-blue-300 border-t-blue-500 rounded-full animate-spin"
                      />
                    </div>
                    <span
                      class="font-medium"
                      :class="
                        saving
                          ? 'text-blue-600 dark:text-blue-400'
                          : lastSaved
                            ? 'text-green-600 dark:text-green-400'
                            : 'text-gray-500 dark:text-gray-400'
                      "
                    >
                      {{
                        saving
                          ? "保存中..."
                          : lastSaved
                            ? `${formatDate(
                              lastSaved,
                            )} 已保存`
                            : "未保存"
                      }}
                    </span>
                  </div>
                </div>

                <!-- 文档统计 -->
                <div
                  class="flex items-center space-x-4 text-xs text-gray-500 dark:text-gray-400"
                >
                  <span>{{
                    (post?.content_markdown || "")
                      .length
                  }}
                    字符</span>
                  <span>{{
                    (
                      post?.content_markdown || ""
                    ).split("\n").length
                  }}
                    行</span>
                </div>
              </div>
            </div>

            <!-- 右侧操作按钮组 -->
            <div class="flex items-center space-x-3">
              <!-- 保存草稿 -->
              <UButton
                variant="ghost"
                size="sm"
                :loading="saving"
                class="hidden sm:inline-flex hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200"
                @click="() => saveAsDraft()"
              >
                <UIcon
                  name="i-heroicons-document-arrow-down"
                  class="w-4 h-4 mr-2"
                />
                保存草稿
              </UButton>

              <!-- 预览切换 -->
              <UButton
                :variant="showPreview ? 'solid' : 'ghost'"
                size="sm"
                class="transition-all duration-200 shadow-sm hover:shadow-md"
                :class="
                  showPreview
                    ? 'bg-gradient-to-r from-blue-500 to-purple-500 text-white'
                    : 'hover:bg-gray-100 dark:hover:bg-gray-800'
                "
                @click="togglePreview"
              >
                <UIcon
                  :name="
                    showPreview
                      ? 'i-heroicons-pencil'
                      : 'i-heroicons-eye'
                  "
                  class="w-4 h-4 mr-2"
                />
                {{ showPreview ? "编辑" : "预览" }}
              </UButton>

              <!-- 发布按钮 -->
              <UButton
                size="sm"
                :loading="publishing"
                class="bg-gradient-to-r from-green-500 to-emerald-600 hover:from-green-600 hover:to-emerald-700 text-white shadow-lg hover:shadow-xl transition-all duration-200 font-medium px-6"
                @click="publishPost"
              >
                <UIcon
                  :name="
                    post?.status === 'published'
                      ? 'i-heroicons-arrow-path'
                      : 'i-heroicons-rocket-launch'
                  "
                  class="w-4 h-4 mr-2"
                />
                {{
                  post?.status === "published"
                    ? "更新文章"
                    : "发布文章"
                }}
              </UButton>
            </div>
          </div>
        </div>
      </div>

      <!-- 主内容区域 -->
      <div class="flex flex-1 overflow-hidden">
        <!-- 编辑器主区域 -->
        <div class="flex-1 flex flex-col overflow-hidden">
          <!-- 编辑模式 -->
          <div
            v-if="!showPreview"
            class="flex-1 flex flex-col overflow-hidden"
          >
            <!-- 标题输入区域 -->
            <div
              class="border-b border-gray-100 dark:border-gray-800 bg-white/50 dark:bg-gray-900/50 backdrop-blur-sm"
            >
              <div class="max-w-4xl mx-auto px-8 py-6">
                <UInput
                  v-model="post.title"
                  placeholder="在此输入文章标题..."
                  size="xl"
                  variant="none"
                  class="text-4xl font-bold text-gray-900 dark:text-white placeholder-gray-400 dark:placeholder-gray-500 bg-transparent border-none focus:ring-0 px-0"
                  @input="handleTitleChange"
                />
              </div>
            </div>

            <!-- 现代化 Markdown 编辑器区域 -->
            <div class="flex-1 overflow-hidden relative">
              <!-- 背景装饰 -->
              <div
                class="absolute inset-0 bg-gradient-to-br from-blue-50/20 via-transparent to-purple-50/20 dark:from-blue-950/10 dark:to-purple-950/10"
              />

              <!-- 编辑器容器 -->
              <div class="relative h-full flex flex-col">
                <!-- 简化的工具栏 -->
                <div
                  class="flex-shrink-0 bg-white/80 dark:bg-gray-900/80 backdrop-blur-sm border-b border-gray-200/60 dark:border-gray-700/60 px-6 py-3"
                >
                  <div
                    class="flex items-center space-x-6 text-sm text-gray-600 dark:text-gray-400"
                  >
                    <div
                      class="flex items-center space-x-2"
                    >
                      <div
                        class="w-2 h-2 bg-green-500 rounded-full animate-pulse"
                      />
                      <span>实时保存</span>
                    </div>
                    <div>Markdown 支持</div>
                    <div
                      class="flex items-center space-x-4"
                    >
                      <span>**粗体**</span>
                      <span>*斜体*</span>
                      <span># 标题</span>
                      <span>- 列表</span>
                      <span>> 引用</span>
                    </div>
                  </div>
                </div>

                <!-- 主编辑区域 -->
                <div class="flex-1 relative">
                  <UTextarea
                    v-model="post.content_markdown"
                    placeholder="开始您的创作之旅... 可以使用
                                    Markdown 语法： # 一级标题 ## 二级标题
                                    **粗体文字** *斜体文字* - 列表项 - 列表项 >
                                    引用文字 ```javascript // 代码块
                                    console.log('Hello World!'); ```
                                    [链接文字](https://example.com)"
                    class="modern-editor-textarea h-full w-full
                                    resize-none border-none focus:ring-0
                                    focus:outline-none bg-transparent
                                    text-gray-900 dark:text-gray-100"
                    :style="{
                      fontSize: '16px',
                      lineHeight: '1.7',
                      fontFamily: '-apple-system, BlinkMacSystemFont, Segoe UI, Roboto, sans-serif',
                      padding: '2rem 3rem',
                      minHeight: '60vh',
                    }"
                    autofocus
                    @input="handleContentChange"
                  />

                  <!-- 编辑器增强效果 -->
                  <div
                    class="absolute inset-0 pointer-events-none"
                  >
                    <div
                      class="absolute top-0 left-0 w-full h-px bg-gradient-to-r from-transparent via-blue-500/20 to-transparent"
                    />
                    <div
                      class="absolute bottom-0 left-0 w-full h-px bg-gradient-to-r from-transparent via-purple-500/20 to-transparent"
                    />
                  </div>
                </div>

                <!-- 编辑器状态栏 -->
                <div
                  class="flex-shrink-0 bg-white/60 dark:bg-gray-900/60 backdrop-blur-sm border-t border-gray-200/60 dark:border-gray-700/60 px-6 py-3"
                >
                  <div
                    class="flex items-center justify-between text-xs text-gray-500 dark:text-gray-400"
                  >
                    <div
                      class="flex items-center space-x-6"
                    >
                      <span>字符数:
                        {{
                          (
                            post?.content_markdown
                            || ""
                          ).length
                        }}</span>
                      <span>行数:
                        {{
                          (
                            post?.content_markdown
                            || ""
                          ).split("\n").length
                        }}</span>
                      <span>预计阅读:
                        {{
                          Math.ceil(
                            (
                              post?.content_markdown
                              || ""
                            ).length / 400,
                          )
                        }}
                        分钟</span>
                    </div>
                    <div
                      class="flex items-center space-x-4"
                    >
                      <div
                        class="flex items-center space-x-1"
                      >
                        <div
                          class="w-1 h-1 bg-green-500 rounded-full"
                        />
                        <span>Markdown 模式</span>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- 预览模式 -->
          <div
            v-else
            class="flex-1 overflow-y-auto bg-white dark:bg-gray-900"
          >
            <article class="max-w-4xl mx-auto px-8 py-12">
              <!-- 文章头部 -->
              <header class="mb-12">
                <h1
                  class="text-4xl md:text-5xl font-bold bg-gradient-to-r from-gray-900 to-gray-600 dark:from-white dark:to-gray-300 bg-clip-text text-transparent mb-4 leading-tight"
                >
                  {{ post?.title || "无标题文章" }}
                </h1>

                <div
                  class="flex items-center space-x-6 text-sm text-gray-500 dark:text-gray-400 border-b border-gray-200 dark:border-gray-700 pb-6"
                >
                  <div class="flex items-center space-x-2">
                    <UIcon
                      name="i-heroicons-calendar"
                      class="w-4 h-4"
                    />
                    <span>{{
                      new Date().toLocaleDateString(
                        "zh-CN",
                      )
                    }}</span>
                  </div>
                  <div class="flex items-center space-x-2">
                    <UIcon
                      name="i-heroicons-document-text"
                      class="w-4 h-4"
                    />
                    <span>{{
                      Math.ceil(
                        (
                          post?.content_markdown
                          || ""
                        ).length / 400,
                      )
                    }}
                      分钟阅读</span>
                  </div>
                  <div class="flex items-center space-x-2">
                    <UIcon
                      name="i-heroicons-eye"
                      class="w-4 h-4"
                    />
                    <span>预览模式</span>
                  </div>
                </div>
              </header>

              <!-- 文章内容 -->
              <div
                class="prose prose-lg prose-gray dark:prose-invert max-w-none"
              >
                <MarkdownRenderer
                  :content="post?.content_markdown || ''"
                  :show-toc="false"
                />
              </div>
            </article>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Post } from "~/types";
import MarkdownRenderer from "~/components/common/MarkdownRenderer.vue";

// 路由守卫
definePageMeta({
  middleware: "auth",
});

// 获取路由参数
const route = useRoute();
const router = useRouter();
const toast = useToast();

const postId = route.params.id as string;

// 状态
const saving = ref(false);
const publishing = ref(false);
const showPreview = ref(false);
const lastSaved = ref<string | null>(null);

const apiClient = useApi();

// 获取文章数据
const post = ref({
  id: "",
  slug: "",
  title: "",
  content_markdown: "",
  excerpt: "",
  thumbnail: "",
  status: "draft" as const,
  tags: [],
  categories: [],
  author: { id: "", username: "" },
  author_id: "",
  created_at: "",
  updated_at: "",
  published_at: undefined,
} as Post);

const pending = ref(true);
const error = ref(false);

// SEO
useHead({
  title: computed(() =>
    post.value?.title ? `编辑: ${post.value.title}` : "编辑文章",
  ),
});

// 自动保存功能
let autoSaveTimer: NodeJS.Timeout | null = null;

const scheduleAutoSave = () => {
  if (autoSaveTimer) {
    clearTimeout(autoSaveTimer);
  }
  autoSaveTimer = setTimeout(() => {
    saveAsDraft(true);
  }, 3000); // 3秒后自动保存
};

// 方法
const goBack = () => {
  router.back();
};

const handleTitleChange = () => {
  scheduleAutoSave();
};

const handleContentChange = () => {
  scheduleAutoSave();
};

const saveAsDraft = async (silent = false) => {
  saving.value = true;

  try {
    const response = await apiClient.updatePost(postId, {
      title: post.value.title,
      content_markdown: post.value.content_markdown,
      excerpt: post.value.excerpt,
      thumbnail: post.value.thumbnail,
      status: "draft",
    });

    if (response.success) {
      lastSaved.value = new Date().toISOString();
      if (!silent) {
        toast.add({
          title: "保存成功",
          description: "草稿已保存",
          color: "success",
        });
      }
    }
  }
  catch {
    if (!silent) {
      toast.add({
        title: "保存失败",
        description: "保存草稿时发生错误",
        color: "error",
      });
    }
  }
  finally {
    saving.value = false;
  }
};

const publishPost = async () => {
  if (!post.value.title.trim()) {
    toast.add({
      title: "发布失败",
      description: "请先输入文章标题",
      color: "error",
    });
    return;
  }

  publishing.value = true;

  try {
    const response = await apiClient.updatePost(postId, {
      title: post.value.title,
      content_markdown: post.value.content_markdown,
      excerpt: post.value.excerpt,
      thumbnail: post.value.thumbnail,
      status: "published",
      published_at: post.value.published_at || new Date().toISOString(),
    });

    if (response.success) {
      post.value.status = "published";
      toast.add({
        title: "发布成功",
        description: "文章已发布",
        color: "success",
      });

      router.push(`/posts/${response.data?.slug || postId}`);
    }
  }
  catch {
    toast.add({
      title: "发布失败",
      description: "发布文章时发生错误",
      color: "error",
    });
  }
  finally {
    publishing.value = false;
  }
};

const togglePreview = () => {
  showPreview.value = !showPreview.value;
};

const formatDate = (dateString: string) => {
  return new Date(dateString).toLocaleTimeString("zh-CN", {
    hour: "2-digit",
    minute: "2-digit",
  });
};

// 加载文章数据
onMounted(async () => {
  try {
    const response = await $fetch(`http://localhost:8080/posts/${postId}`, {
      headers: {
        Authorization: `Bearer ${localStorage.getItem("access_token")}`,
      },
    });
    post.value = response as Post;
  }
  catch (err) {
    console.error("Failed to load post:", err);
    error.value = true;
  }
  finally {
    pending.value = false;
  }
});

onUnmounted(() => {
  if (autoSaveTimer) {
    clearTimeout(autoSaveTimer);
  }
});

// 监听路由离开，提醒保存
onBeforeRouteLeave((to, from, next) => {
  if (lastSaved.value) {
    next();
  }
  else {
    const confirmed = confirm("您有未保存的更改，确定要离开吗？");
    if (confirmed) {
      next();
    }
    else {
      next(false);
    }
  }
});
</script>

<style scoped>
/* 现代化编辑器样式 */
.modern-editor-textarea {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto,
        "Helvetica Neue", Arial, sans-serif;
    font-size: 16px;
    line-height: 1.7;
    color: #374151;
    background: transparent;
}

.dark .modern-editor-textarea {
    color: #f3f4f6;
}

.modern-editor-textarea::placeholder {
    color: #9ca3af;
    font-style: italic;
}

.dark .modern-editor-textarea::placeholder {
    color: #6b7280;
}

.modern-editor-textarea:focus {
    outline: none;
    box-shadow: none;
}

/* 自定义滚动条 */
.overflow-y-auto::-webkit-scrollbar {
    width: 6px;
}

.overflow-y-auto::-webkit-scrollbar-track {
    background: rgba(0, 0, 0, 0.05);
    border-radius: 3px;
}

.overflow-y-auto::-webkit-scrollbar-thumb {
    background: rgba(0, 0, 0, 0.2);
    border-radius: 3px;
}

.overflow-y-auto::-webkit-scrollbar-thumb:hover {
    background: rgba(0, 0, 0, 0.3);
}

.dark .overflow-y-auto::-webkit-scrollbar-track {
    background: rgba(255, 255, 255, 0.05);
}

.dark .overflow-y-auto::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.2);
}

.dark .overflow-y-auto::-webkit-scrollbar-thumb:hover {
    background: rgba(255, 255, 255, 0.3);
}

/* 平滑过渡动画 */
.transition-all {
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

/* 毛玻璃效果增强 */
.backdrop-blur-xl {
    backdrop-filter: blur(24px) saturate(180%);
}

/* 渐变动画 */
@keyframes gradient {
    0%,
    100% {
        background-position: 0% 50%;
    }
    50% {
        background-position: 100% 50%;
    }
}

.bg-gradient-to-br {
    background-size: 400% 400%;
    animation: gradient 15s ease infinite;
}

/* 编辑器聚焦效果 */
.modern-editor-textarea:focus + .absolute {
    opacity: 0.6;
}

/* 响应式设计 */
@media (max-width: 768px) {
    .modern-editor-textarea {
        padding: 1rem 1.5rem;
        font-size: 15px;
    }
}
</style>
