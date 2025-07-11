<template>
  <div class="min-h-screen bg-gray-50 dark:bg-gray-900">
    <div
      v-if="pending"
      class="flex items-center justify-center min-h-screen"
    >
      <UIcon
        name="i-heroicons-arrow-path"
        class="w-8 h-8 animate-spin text-primary-500"
      />
    </div>

    <div
      v-else-if="error"
      class="flex items-center justify-center min-h-screen"
    >
      <div class="text-center">
        <UIcon
          name="i-heroicons-exclamation-triangle"
          class="w-16 h-16 text-red-500 mx-auto mb-4"
        />
        <h1
          class="text-2xl font-bold text-gray-900 dark:text-white mb-2"
        >
          文章不存在
        </h1>
        <p class="text-gray-600 dark:text-gray-400 mb-4">
          您要编辑的文章不存在或您没有权限访问。
        </p>
        <UButton
          to="/write"
          color="primary"
        >
          返回写作页面
        </UButton>
      </div>
    </div>

    <div
      v-else
      class="max-w-7xl mx-auto"
    >
      <!-- 顶部工具栏 -->
      <div
        class="sticky top-0 z-40 bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 px-6 py-4"
      >
        <div class="flex items-center justify-between">
          <!-- 左侧按钮 -->
          <div class="flex items-center space-x-4">
            <UButton
              variant="ghost"
              icon="i-heroicons-arrow-left"
              size="sm"
              @click="goBack"
            >
              返回
            </UButton>

            <div
              class="flex items-center space-x-2 text-sm text-gray-600 dark:text-gray-400"
            >
              <UIcon
                name="i-heroicons-clock"
                class="w-4 h-4"
              />
              <span>{{
                lastSaved
                  ? `${formatDate(lastSaved)} 保存`
                  : "未保存"
              }}</span>
            </div>
          </div>

          <!-- 右侧操作按钮 -->
          <div class="flex items-center space-x-3">
            <UButton
              variant="outline"
              :loading="saving"
              size="sm"
              @click="() => saveAsDraft()"
            >
              保存草稿
            </UButton>

            <UButton
              variant="outline"
              :icon="
                showPreview
                  ? 'i-heroicons-pencil'
                  : 'i-heroicons-eye'
              "
              size="sm"
              @click="togglePreview"
            >
              {{ showPreview ? "编辑" : "预览" }}
            </UButton>

            <UButton
              variant="outline"
              icon="i-heroicons-cog-6-tooth"
              size="sm"
              @click="openSettings"
            >
              设置
            </UButton>

            <UButton
              color="primary"
              :loading="publishing"
              size="sm"
              @click="publishPost"
            >
              {{
                (post as any)?.status === "published"
                  ? "更新"
                  : "发布"
              }}
            </UButton>
          </div>
        </div>
      </div>

      <div class="flex">
        <!-- 主编辑区域 -->
        <div class="flex-1 min-h-screen">
          <div
            v-if="!showPreview"
            class="p-6"
          >
            <!-- 标题输入 -->
            <div class="mb-6">
              <UInput
                v-model="(post as any).title"
                placeholder="文章标题..."
                size="xl"
                variant="none"
                class="text-4xl font-bold"
                @input="handleTitleChange"
              />
            </div>

            <!-- 编辑器区域 -->
            <div class="mb-8">
              <UTextarea
                v-model="(post as any).content"
                placeholder="开始写作..."
                :rows="30"
                variant="none"
                class="min-h-[600px] text-lg leading-relaxed"
                @input="handleContentChange"
              />
            </div>
          </div>

          <!-- 预览模式 -->
          <div
            v-else
            class="p-6"
          >
            <article
              class="prose prose-lg dark:prose-invert max-w-none"
            >
              <h1>{{ (post as any)?.title || "无标题" }}</h1>
              <div v-html="renderedContent" />
            </article>
          </div>
        </div>

        <!-- 侧边栏设置面板 -->
        <div
          v-if="showSettings"
          class="w-80 bg-white dark:bg-gray-800 border-l border-gray-200 dark:border-gray-700 p-6 overflow-y-auto"
        >
          <div class="space-y-6">
            <!-- 发布设置 -->
            <div>
              <h3
                class="text-lg font-semibold text-gray-900 dark:text-white mb-4"
              >
                发布设置
              </h3>

              <UFormGroup
                label="文章状态"
                class="mb-4"
              >
                <USelect
                  v-model="(post as any).status"
                  :options="statusOptions"
                />
              </UFormGroup>

              <UFormGroup
                label="发布时间"
                class="mb-4"
              >
                <UInput
                  v-model="(post as any).published_at"
                  type="datetime-local"
                />
              </UFormGroup>
            </div>

            <!-- 分类和标签 -->
            <div>
              <h3
                class="text-lg font-semibold text-gray-900 dark:text-white mb-4"
              >
                分类标签
              </h3>

              <UFormGroup
                label="分类"
                class="mb-4"
              >
                <USelectMenu
                  v-model="selectedCategories"
                  :options="categoryOptions"
                  multiple
                  placeholder="选择分类"
                />
              </UFormGroup>

              <UFormGroup
                label="标签"
                class="mb-4"
              >
                <UInput
                  v-model="tagInput"
                  placeholder="输入标签，按回车添加"
                  @keyup.enter="addTag"
                />
                <div
                  v-if="(post as any).tags?.length"
                  class="mt-2 flex flex-wrap gap-2"
                >
                  <UBadge
                    v-for="tag in (post as any).tags"
                    :key="tag.id"
                    variant="soft"
                    closable
                    @close="removeTag(tag.id)"
                  >
                    {{ tag.name }}
                  </UBadge>
                </div>
              </UFormGroup>
            </div>

            <!-- 文章摘要 -->
            <div>
              <h3
                class="text-lg font-semibold text-gray-900 dark:text-white mb-4"
              >
                文章摘要
              </h3>
              <UTextarea
                v-model="(post as any).excerpt"
                placeholder="文章摘要（可选）"
                :rows="4"
              />
            </div>

            <!-- 封面图片 -->
            <div>
              <h3
                class="text-lg font-semibold text-gray-900 dark:text-white mb-4"
              >
                封面图片
              </h3>
              <div
                v-if="(post as any).thumbnail"
                class="mb-4"
              >
                <img
                  :src="(post as any).thumbnail"
                  alt="封面图片"
                  class="w-full rounded-lg"
                >
                <UButton
                  variant="ghost"
                  color="error"
                  size="sm"
                  class="mt-2"
                  @click="removeThumbnail"
                >
                  移除封面
                </UButton>
              </div>
              <UButton
                variant="outline"
                icon="i-heroicons-photo"
                block
                @click="uploadThumbnail"
              >
                {{
                  (post as any).thumbnail
                    ? "更换封面"
                    : "添加封面"
                }}
              </UButton>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 文章设置模态框 -->
    <UModal v-model="showSettingsModal">
      <!-- TODO: 添加文章设置模态框内容 -->
    </UModal>
  </div>
</template>

<script setup lang="ts">
import type { Post, Category, Tag } from "~/types";

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
const showSettings = ref(false);
const showSettingsModal = ref(false);
const lastSaved = ref<string | null>(null);
const tagInput = ref("");
const selectedCategories = ref<string[]>([]);

// 获取文章数据
const {
  data: post,
  pending,
  error,
} = await useLazyFetch<Post>(`/api/posts/${postId}`, {
  default: () =>
    ({
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
    } as Post),
});

// SEO
useHead({
  title: computed(() =>
    (post as any).value?.title
      ? `编辑: ${(post as any).value.title}`
      : "编辑文章",
  ),
});

// 选项数据
const statusOptions = [
  { label: "草稿", value: "draft" },
  { label: "已发布", value: "published" },
];

const categoryOptions = ref<Category[]>([]);

// 计算属性
const renderedContent = computed(() => {
  // TODO: 实现 Markdown 渲染
  return (post as any).value?.content?.replace(/\n/g, "<br>") || "";
});

// 自动保存功能
let autoSaveTimer: NodeJS.Timeout | null = null;

const scheduleAutoSave = () => {
  if (autoSaveTimer) {
    clearTimeout(autoSaveTimer);
  }
  autoSaveTimer = setTimeout(() => {
    saveAsDraft(true);
  }, 5000); // 5秒后自动保存
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
    const response = await $fetch(`/api/posts/${postId}`, {
      method: "PUT",
      body: {
        title: (post as any).value.title,
        content: (post as any).value.content,
        excerpt: (post as any).value.excerpt,
        thumbnail: (post as any).value.thumbnail,
        status: "draft",
        categories: selectedCategories.value,
        tags:
                    (post as any).value.tags?.map((tag: any) => tag.name) || [],
      },
    });

    if ((response as any).success) {
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
  catch (error) {
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
  if (!(post as any).value.title.trim()) {
    toast.add({
      title: "发布失败",
      description: "请先输入文章标题",
      color: "error",
    });
    return;
  }

  publishing.value = true;

  try {
    const response = await $fetch(`/api/posts/${postId}`, {
      method: "PUT",
      body: {
        title: (post as any).value.title,
        content: (post as any).value.content,
        excerpt: (post as any).value.excerpt,
        thumbnail: (post as any).value.thumbnail,
        status: "published",
        categories: selectedCategories.value,
        tags:
                    (post as any).value.tags?.map((tag: any) => tag.name) || [],
        published_at:
                    (post as any).value.published_at
                    || new Date().toISOString(),
      },
    });

    if ((response as any).success) {
      ((post as any).value as any).status = "published";
      toast.add({
        title: "发布成功",
        description: "文章已发布",
        color: "success",
      });

      router.push(`/posts/${(response as any).data.slug}`);
    }
  }
  catch (error) {
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

const openSettings = () => {
  showSettings.value = !showSettings.value;
};

const addTag = () => {
  const tagName = tagInput.value.trim();
  if (
    tagName
    && !(post as any).value.tags?.some((tag: any) => tag.name === tagName)
  ) {
    if (!(post as any).value.tags) {
      (post as any).value.tags = [];
    }
    (post as any).value.tags.push({
      id: Date.now().toString(),
      name: tagName,
      slug: tagName.toLowerCase(),
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
    });
    tagInput.value = "";
  }
};

const removeTag = (tagId: string) => {
  if ((post as any).value.tags) {
    (post as any).value.tags = (post as any).value.tags.filter(
      (tag: any) => tag.id !== tagId,
    );
  }
};

const uploadThumbnail = () => {
  // TODO: 实现图片上传功能
  toast.add({
    title: "功能开发中",
    description: "图片上传功能正在开发中",
    color: "warning",
  });
};

const removeThumbnail = () => {
  (post as any).value.thumbnail = "";
};

const formatDate = (dateString: string) => {
  return new Date(dateString).toLocaleTimeString("zh-CN", {
    hour: "2-digit",
    minute: "2-digit",
  });
};

// 生命周期
onMounted(async () => {
  // 获取分类选项
  try {
    const categoriesResponse = await $fetch("/api/categories");
    if ((categoriesResponse as any).success) {
      categoryOptions.value = (categoriesResponse as any).data;
    }
  }
  catch (error) {
    console.error("Failed to load categories:", error);
  }

  // 初始化选中的分类
  if ((post as any).value.categories) {
    selectedCategories.value = (post as any).value.categories.map(
      (cat: any) => cat.id,
    );
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
    // TODO: 添加确认对话框
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
