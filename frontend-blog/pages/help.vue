<template>
  <div>
    <!-- Hero 区域 -->
    <section
      class="modern-hero"
      style="padding: 2rem 0 1.5rem 0;"
    >
      <div class="relative mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
        <div style="text-align: center;">
          <!-- 现代化图标 -->
          <div style="margin: 0 auto 1rem; width: 2.5rem; height: 2.5rem; display: flex; align-items: center; justify-content: center; border-radius: 50%; background: linear-gradient(45deg, #10b981, #059669); box-shadow: 0 6px 24px rgba(16, 185, 129, 0.3);">
            <UIcon
              name="i-heroicons-question-mark-circle"
              style="width: 1.25rem; height: 1.25rem; color: white;"
            />
          </div>

          <!-- 现代化标题 -->
          <h1
            style="font-size: 1.875rem; font-weight: 700; line-height: 1.1; margin-bottom: 0.75rem; color: #1e293b;"
            class="dark:text-white"
          >
            帮助
            <span
              class="modern-text-gradient"
              style="font-weight: 800; background: linear-gradient(45deg, #10b981, #059669); -webkit-background-clip: text; -webkit-text-fill-color: transparent;"
            >中心</span>
          </h1>

          <!-- 副标题 -->
          <p
            style="font-size: 1rem; color: #64748b; max-width: 28rem; margin: 0 auto 1.5rem; line-height: 1.5;"
            class="dark:text-slate-300"
          >
            快速找到答案，解决您在使用过程中遇到的问题
          </p>

          <!-- 搜索框 -->
          <div style="max-width: 24rem; margin: 0 auto;">
            <UInput
              v-model="searchQuery"
              icon="i-heroicons-magnifying-glass"
              placeholder="搜索帮助文档..."
              size="lg"
              style="background: rgba(255, 255, 255, 0.9); border: 1px solid rgba(226, 232, 240, 0.6);"
              class="focus:border-green-500"
              @input="filterFAQs"
            />
          </div>
        </div>
      </div>
    </section>

    <!-- 面包屑导航 -->
    <nav style="background: rgba(248, 250, 252, 0.8); border-bottom: 1px solid rgba(226, 232, 240, 0.6); backdrop-filter: blur(10px);">
      <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
        <div style="display: flex; align-items: center; gap: 0.5rem; padding: 0.75rem 0; font-size: 0.875rem;">
          <NuxtLink
            to="/"
            style="display: flex; align-items: center; gap: 0.25rem; color: #64748b; text-decoration: none; transition: all 0.2s ease;"
            class="hover:text-blue-600"
          >
            <UIcon
              name="i-heroicons-home"
              class="w-4 h-4"
            />
            首页
          </NuxtLink>
          <UIcon
            name="i-heroicons-chevron-right"
            class="w-4 h-4 text-slate-400"
          />
          <span style="color: #3b82f6; font-weight: 500;">帮助中心</span>
        </div>
      </div>
    </nav>

    <!-- 主要内容区域 -->
    <section
      style="background: linear-gradient(135deg, #f8fafc 0%, #e2e8f0 50%, #ecfdf5 100%); padding: 2rem 0; margin: 0; position: relative; min-height: 80vh;"
    >
      <!-- 背景装饰 -->
      <div style="position: absolute; inset: 0; background-image: radial-gradient(circle at 1px 1px, rgba(16, 185, 129, 0.1) 1px, transparent 0); background-size: 60px 60px; opacity: 0.5;" />

      <div class="relative mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
        <!-- 快速导航 -->
        <div class="grid md:grid-cols-2 lg:grid-cols-4 gap-6 mb-12">
          <div
            v-for="category in helpCategories"
            :key="category.id"
            class="modern-content-section group cursor-pointer"
            style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 2rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05); transition: all 0.3s ease;"
            @click="scrollToCategory(category.id)"
          >
            <div
              class="modern-gradient-icon mb-4"
              :style="{ background: category.gradient, width: '3rem', height: '3rem' }"
            >
              <UIcon
                :name="category.icon"
                class="h-6 w-6 text-white"
              />
            </div>
            <h3 class="text-lg font-semibold text-slate-900 dark:text-white mb-2">
              {{ category.name }}
            </h3>
            <p class="text-sm text-slate-600 dark:text-slate-400">
              {{ category.description }}
            </p>
            <div class="mt-3 text-xs text-slate-500 dark:text-slate-400">
              {{ category.count }} 个问题
            </div>
          </div>
        </div>

        <!-- FAQ 内容 -->
        <div class="space-y-12">
          <div
            v-for="category in helpCategories"
            :id="`category-${category.id}`"
            :key="category.id"
            class="modern-content-section"
            style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 2rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);"
          >
            <div class="modern-section-header mb-8">
              <div class="flex items-center gap-3 mb-2">
                <div
                  class="modern-gradient-icon"
                  :style="{ background: category.gradient, width: '2.5rem', height: '2.5rem' }"
                >
                  <UIcon
                    :name="category.icon"
                    class="h-5 w-5 text-white"
                  />
                </div>
                <h2 class="text-2xl font-bold text-slate-900 dark:text-white">
                  {{ category.name }}
                </h2>
              </div>
              <p class="text-slate-600 dark:text-slate-400">
                {{ category.description }}
              </p>
            </div>

            <div class="space-y-4">
              <div
                v-for="faq in getFilteredFAQs(category.id)"
                :key="faq.id"
                class="border border-slate-200 dark:border-slate-700 rounded-lg overflow-hidden"
              >
                <button
                  class="w-full text-left p-4 hover:bg-slate-50 dark:hover:bg-slate-800 transition-colors flex items-center justify-between"
                  @click="toggleFAQ(faq.id)"
                >
                  <span class="font-medium text-slate-900 dark:text-white">
                    {{ faq.question }}
                  </span>
                  <UIcon
                    :name="expandedFAQ === faq.id ? 'i-heroicons-chevron-up' : 'i-heroicons-chevron-down'"
                    class="w-5 h-5 text-slate-500 transition-transform"
                  />
                </button>

                <div
                  v-if="expandedFAQ === faq.id"
                  class="p-4 border-t border-slate-200 dark:border-slate-700 bg-slate-50 dark:bg-slate-800"
                >
                  <div
                    class="text-slate-600 dark:text-slate-400 leading-relaxed prose prose-sm max-w-none"
                    v-html="faq.answer"
                  />

                  <div
                    v-if="faq.relatedLinks"
                    class="mt-4 pt-4 border-t border-slate-200 dark:border-slate-600"
                  >
                    <h4 class="text-sm font-medium text-slate-900 dark:text-white mb-2">
                      相关链接：
                    </h4>
                    <div class="flex flex-wrap gap-2">
                      <NuxtLink
                        v-for="link in faq.relatedLinks"
                        :key="link.url"
                        :to="link.url"
                        class="inline-flex items-center gap-1 text-sm text-blue-600 hover:text-blue-700 dark:text-blue-400 dark:hover:text-blue-300"
                      >
                        <UIcon
                          name="i-heroicons-arrow-top-right-on-square"
                          class="w-3 h-3"
                        />
                        {{ link.title }}
                      </NuxtLink>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- 联系支持 -->
        <div
          class="modern-content-section mt-12 text-center"
          style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 3rem 2rem; border: 1px solid #e2e8f0; box-shadow: 0 4px 6px rgba(0, 0, 0, 0.05);"
        >
          <div
            class="modern-gradient-icon mx-auto mb-6"
            style="background: linear-gradient(45deg, #10b981, #059669); width: 4rem; height: 4rem;"
          >
            <UIcon
              name="i-heroicons-chat-bubble-left-ellipsis"
              class="h-8 w-8 text-white"
            />
          </div>

          <h2 class="text-2xl font-bold text-slate-900 dark:text-white mb-4">
            还有其他问题？
          </h2>

          <p class="text-lg text-slate-600 dark:text-slate-400 mb-6 max-w-2xl mx-auto">
            如果您在帮助文档中没有找到答案，欢迎联系我们的支持团队
          </p>

          <div class="flex flex-wrap items-center justify-center gap-4">
            <a
              href="mailto:support@zincbloom.com"
              class="inline-flex items-center gap-2 px-6 py-3 text-white rounded-lg font-medium transition-all hover:transform hover:scale-105"
              style="background: linear-gradient(45deg, #10b981, #059669); box-shadow: 0 4px 12px rgba(16, 185, 129, 0.3);"
            >
              <UIcon
                name="i-heroicons-envelope"
                class="w-4 h-4"
              />
              邮件支持
            </a>

            <NuxtLink
              to="/about"
              class="inline-flex items-center gap-2 px-6 py-3 bg-slate-100 hover:bg-slate-200 text-slate-700 rounded-lg font-medium transition-all hover:transform hover:scale-105"
            >
              <UIcon
                name="i-heroicons-information-circle"
                class="w-4 h-4"
              />
              了解更多
            </NuxtLink>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
// SEO 配置
useHead({
  title: "帮助中心 - ZincBloom",
  meta: [
    { name: "description", content: "ZincBloom 帮助中心，快速找到答案，解决您在使用过程中遇到的问题。包括账户管理、文章发布、技术支持等常见问题。" },
    { property: "og:title", content: "帮助中心 - ZincBloom" },
    { property: "og:description", content: "ZincBloom 帮助中心，快速找到答案，解决您在使用过程中遇到的问题" },
    { property: "og:type", content: "website" },
  ],
});

// 状态管理
const searchQuery = ref("");
const expandedFAQ = ref<string | null>(null);
const filteredFAQs = ref<any[]>([]);

// 帮助分类
const helpCategories = [
  {
    id: "account",
    name: "账户管理",
    description: "注册、登录、个人资料设置等",
    icon: "i-heroicons-user-circle",
    gradient: "linear-gradient(45deg, #3b82f6, #6366f1)",
    count: 8,
  },
  {
    id: "writing",
    name: "文章创作",
    description: "发布、编辑、管理文章内容",
    icon: "i-heroicons-document-text",
    gradient: "linear-gradient(45deg, #10b981, #059669)",
    count: 12,
  },
  {
    id: "features",
    name: "功能使用",
    description: "搜索、收藏、评论等功能",
    icon: "i-heroicons-cog-6-tooth",
    gradient: "linear-gradient(45deg, #f59e0b, #f97316)",
    count: 10,
  },
  {
    id: "technical",
    name: "技术支持",
    description: "常见技术问题和解决方案",
    icon: "i-heroicons-wrench-screwdriver",
    gradient: "linear-gradient(45deg, #ec4899, #f43f5e)",
    count: 6,
  },
];

// FAQ 数据
const faqs = ref([
  // 账户管理
  {
    id: "account-1",
    categoryId: "account",
    question: "如何注册新账户？",
    answer: `
      <p>注册 ZincBloom 账户非常简单：</p>
      <ol>
        <li>点击页面右上角的"注册"按钮</li>
        <li>填写用户名、邮箱地址和密码</li>
        <li>点击"创建账户"按钮</li>
        <li>查看您的邮箱，点击验证链接完成注册</li>
      </ol>
      <p>注册完成后，您就可以开始创作和分享您的技术文章了！</p>
    `,
    relatedLinks: [
      { title: "立即注册", url: "/auth/register" },
    ],
  },
  {
    id: "account-2",
    categoryId: "account",
    question: "忘记密码怎么办？",
    answer: `
      <p>如果您忘记了密码，可以通过以下步骤重置：</p>
      <ol>
        <li>点击登录页面的"忘记密码"链接</li>
        <li>输入您注册时使用的邮箱地址</li>
        <li>查看邮箱中的密码重置邮件</li>
        <li>点击邮件中的链接设置新密码</li>
      </ol>
      <p>如果您没有收到重置邮件，请检查垃圾邮件文件夹或联系我们的支持团队。</p>
    `,
  },
  {
    id: "account-3",
    categoryId: "account",
    question: "如何修改个人资料？",
    answer: `
      <p>修改个人资料的步骤：</p>
      <ol>
        <li>登录后点击右上角的头像菜单</li>
        <li>选择"个人资料"</li>
        <li>在资料页面点击"编辑资料"按钮</li>
        <li>修改您的个人信息</li>
        <li>点击"保存更改"</li>
      </ol>
      <p>您可以修改用户名、头像、个人简介等信息。</p>
    `,
    relatedLinks: [
      { title: "个人中心", url: "/user/profile" },
    ],
  },

  // 文章创作
  {
    id: "writing-1",
    categoryId: "writing",
    question: "如何发布我的第一篇文章？",
    answer: `
      <p>发布您的第一篇文章：</p>
      <ol>
        <li>登录后点击"写文章"按钮</li>
        <li>填写文章标题</li>
        <li>在编辑器中编写文章内容（支持 Markdown 格式）</li>
        <li>选择合适的分类和标签</li>
        <li>添加文章摘要和封面图片（可选）</li>
        <li>预览文章效果</li>
        <li>点击"发布文章"</li>
      </ol>
      <p>发布后的文章会立即显示在您的个人主页和网站首页。</p>
    `,
    relatedLinks: [
      { title: "开始写作", url: "/write" },
    ],
  },
  {
    id: "writing-2",
    categoryId: "writing",
    question: "支持哪些文档格式？",
    answer: `
      <p>ZincBloom 主要支持 Markdown 格式的文章：</p>
      <ul>
        <li><strong>标题</strong>：# ## ### 等</li>
        <li><strong>文本格式</strong>：粗体、斜体、删除线</li>
        <li><strong>列表</strong>：有序和无序列表</li>
        <li><strong>链接和图片</strong>：内联和引用式</li>
        <li><strong>代码</strong>：行内代码和代码块</li>
        <li><strong>表格</strong>：Markdown 表格语法</li>
        <li><strong>引用</strong>：> 引用块</li>
      </ul>
      <p>编辑器提供实时预览功能，让您随时查看文章的最终效果。</p>
    `,
  },
  {
    id: "writing-3",
    categoryId: "writing",
    question: "如何管理草稿？",
    answer: `
      <p>草稿管理功能：</p>
      <ul>
        <li><strong>自动保存</strong>：编辑过程中会自动保存草稿</li>
        <li><strong>手动保存</strong>：点击"保存草稿"按钮</li>
        <li><strong>草稿列表</strong>：在个人中心查看所有草稿</li>
        <li><strong>继续编辑</strong>：随时回到草稿继续编辑</li>
        <li><strong>发布草稿</strong>：草稿完成后可以直接发布</li>
      </ul>
      <p>草稿会一直保存，直到您删除或发布它们。</p>
    `,
    relatedLinks: [
      { title: "我的文章", url: "/user/posts" },
    ],
  },

  // 功能使用
  {
    id: "features-1",
    categoryId: "features",
    question: "如何搜索感兴趣的文章？",
    answer: `
      <p>ZincBloom 提供多种搜索方式：</p>
      <ul>
        <li><strong>全局搜索</strong>：顶部搜索框可以搜索文章、作者、标签</li>
        <li><strong>分类浏览</strong>：按技术分类浏览相关文章</li>
        <li><strong>标签筛选</strong>：点击标签查看相关文章</li>
        <li><strong>作者主页</strong>：查看特定作者的所有文章</li>
        <li><strong>高级搜索</strong>：在搜索页面使用筛选条件</li>
      </ul>
      <p>搜索结果会按相关性排序，您也可以选择其他排序方式。</p>
    `,
    relatedLinks: [
      { title: "全局搜索", url: "/search" },
      { title: "文章分类", url: "/categories" },
    ],
  },
  {
    id: "features-2",
    categoryId: "features",
    question: "如何收藏文章？",
    answer: `
      <p>收藏功能让您保存喜欢的文章：</p>
      <ol>
        <li>在文章页面点击心形收藏按钮</li>
        <li>收藏的文章会保存到您的收藏列表</li>
        <li>在个人中心的"我的收藏"中查看</li>
        <li>支持按分类筛选收藏内容</li>
        <li>可以随时取消收藏</li>
      </ol>
      <p>收藏功能需要登录后才能使用。</p>
    `,
    relatedLinks: [
      { title: "我的收藏", url: "/user/favorites" },
    ],
  },

  // 技术支持
  {
    id: "technical-1",
    categoryId: "technical",
    question: "网站加载很慢怎么办？",
    answer: `
      <p>如果网站加载缓慢，可以尝试以下解决方案：</p>
      <ul>
        <li><strong>检查网络</strong>：确保网络连接稳定</li>
        <li><strong>清除缓存</strong>：清理浏览器缓存和 Cookie</li>
        <li><strong>更换浏览器</strong>：尝试使用其他浏览器访问</li>
        <li><strong>禁用扩展</strong>：临时禁用浏览器扩展</li>
        <li><strong>检查 DNS</strong>：尝试更换 DNS 服务器</li>
      </ul>
      <p>如果问题持续存在，请联系我们的技术支持团队。</p>
    `,
  },
  {
    id: "technical-2",
    categoryId: "technical",
    question: "移动端使用有什么注意事项？",
    answer: `
      <p>ZincBloom 完全支持移动端访问：</p>
      <ul>
        <li><strong>响应式设计</strong>：自动适配各种屏幕尺寸</li>
        <li><strong>触摸优化</strong>：所有交互都针对触摸屏优化</li>
        <li><strong>快速加载</strong>：移动端页面经过优化</li>
        <li><strong>完整功能</strong>：支持写作、编辑、管理等所有功能</li>
        <li><strong>离线支持</strong>：基础内容可以离线访问</li>
      </ul>
      <p>建议使用最新版本的现代浏览器以获得最佳体验。</p>
    `,
  },
]);

// 方法
const toggleFAQ = (id: string) => {
  expandedFAQ.value = expandedFAQ.value === id ? null : id;
};

const scrollToCategory = (categoryId: string) => {
  const element = document.getElementById(`category-${categoryId}`);
  if (element) {
    element.scrollIntoView({ behavior: "smooth", block: "start" });
  }
};

const getFilteredFAQs = (categoryId: string) => {
  const categoryFAQs = faqs.value.filter(faq => faq.categoryId === categoryId);

  if (!searchQuery.value.trim()) {
    return categoryFAQs;
  }

  const query = searchQuery.value.toLowerCase();
  return categoryFAQs.filter(faq =>
    faq.question.toLowerCase().includes(query)
    || faq.answer.toLowerCase().includes(query),
  );
};

const filterFAQs = () => {
  // 实时筛选，已经在 getFilteredFAQs 中处理
};
</script>

<style scoped>
.modern-gradient-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  transition: all 0.3s ease;
}

.group:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 25px rgba(0, 0, 0, 0.15);
}

.group:hover .modern-gradient-icon {
  transform: scale(1.1);
}

.prose ol {
  list-style: decimal;
  margin-left: 1.5rem;
  margin-bottom: 1rem;
}

.prose ul {
  list-style: disc;
  margin-left: 1.5rem;
  margin-bottom: 1rem;
}

.prose li {
  margin-bottom: 0.5rem;
}

.prose strong {
  font-weight: 600;
  color: #1e293b;
}

.dark .prose strong {
  color: #f1f5f9;
}
</style>
