<template>
  <div>
    <!-- Hero 区域 -->
    <section
      class="modern-hero"
      style="padding: 4rem 0 3rem 0; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); position: relative; overflow: hidden;"
    >
      <!-- 背景装饰 -->
      <div style="position: absolute; inset: 0; background-image: radial-gradient(circle at 20% 50%, rgba(255, 255, 255, 0.1) 0%, transparent 50%), radial-gradient(circle at 80% 20%, rgba(255, 255, 255, 0.05) 0%, transparent 50%); opacity: 0.8;" />

      <div class="relative mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
        <div style="text-align: center; color: white;">
          <!-- Logo/品牌图标 -->
          <div style="margin: 0 auto 2rem; width: 4rem; height: 4rem; display: flex; align-items: center; justify-content: center; border-radius: 50%; background: rgba(255, 255, 255, 0.2); backdrop-filter: blur(10px); box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);">
            <UIcon
              name="i-heroicons-heart"
              style="width: 2rem; height: 2rem; color: white;"
            />
          </div>

          <!-- 品牌标题 -->
          <h1
            style="font-size: 3rem; font-weight: 800; line-height: 1.1; margin-bottom: 1.5rem; text-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);"
            class="md:text-5xl"
          >
            ZincBloom
          </h1>

          <!-- 品牌口号 -->
          <p
            style="font-size: 1.25rem; margin: 0 auto 2rem; line-height: 1.6; max-width: 36rem; opacity: 0.95;"
            class="md:text-xl"
          >
            让知识绽放，让思想传播<br>
            一个专注于技术分享与交流的博客平台
          </p>

          <!-- 统计数据 -->
          <div class="flex items-center justify-center gap-8 text-white/90">
            <div class="text-center">
              <div class="text-2xl font-bold">
                {{ stats.posts }}+
              </div>
              <div class="text-sm opacity-80">
                优质文章
              </div>
            </div>
            <div class="w-px h-8 bg-white/20" />
            <div class="text-center">
              <div class="text-2xl font-bold">
                {{ stats.authors }}+
              </div>
              <div class="text-sm opacity-80">
                活跃作者
              </div>
            </div>
            <div class="w-px h-8 bg-white/20" />
            <div class="text-center">
              <div class="text-2xl font-bold">
                {{ stats.categories }}+
              </div>
              <div class="text-sm opacity-80">
                技术分类
              </div>
            </div>
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
          <span style="color: #3b82f6; font-weight: 500;">关于我们</span>
        </div>
      </div>
    </nav>

    <!-- 主要内容区域 -->
    <section
      style="background: linear-gradient(135deg, #f8fafc 0%, #e2e8f0 50%, #f1f5f9 100%); padding: 4rem 0; margin: 0; position: relative;"
    >
      <!-- 背景装饰 -->
      <div style="position: absolute; inset: 0; background-image: radial-gradient(circle at 1px 1px, rgba(100, 116, 139, 0.1) 1px, transparent 0); background-size: 60px 60px; opacity: 0.5;" />

      <div class="relative mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
        <!-- 关于我们 -->
        <div class="grid lg:grid-cols-2 gap-12 items-center mb-16">
          <div>
            <div
              class="modern-content-section"
              style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 3rem; border: 1px solid #e2e8f0; box-shadow: 0 4px 6px rgba(0, 0, 0, 0.05);"
            >
              <div class="modern-section-header mb-6">
                <div class="flex items-center gap-3 mb-4">
                  <div
                    class="modern-gradient-icon"
                    style="background: linear-gradient(45deg, #667eea, #764ba2); width: 3rem; height: 3rem;"
                  >
                    <UIcon
                      name="i-heroicons-sparkles"
                      class="h-6 w-6 text-white"
                    />
                  </div>
                  <h2 class="text-2xl font-bold text-slate-900 dark:text-white">
                    关于 ZincBloom
                  </h2>
                </div>
              </div>

              <div class="space-y-6 text-slate-600 dark:text-slate-400 leading-relaxed">
                <p>
                  ZincBloom 是一个专注于技术分享与交流的现代化博客平台。我们致力于为开发者、技术爱好者提供一个优质的内容创作和知识分享空间。
                </p>

                <p>
                  平台名称来源于 "Zinc"（锌）和 "Bloom"（绽放）的结合。锌是一种重要的微量元素，能够促进生长和发展；而绽放代表着知识和思想的蓬勃发展。我们希望每一位用户都能在这里绽放自己的技术才华。
                </p>

                <p>
                  我们相信，通过分享和交流，技术知识能够得到更好的传播和发展。无论您是经验丰富的开发者，还是刚刚起步的技术新手，这里都有属于您的空间。
                </p>
              </div>
            </div>
          </div>

          <div class="space-y-6">
            <!-- 愿景使命卡片 -->
            <div
              class="modern-content-section"
              style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 2rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);"
            >
              <div class="flex items-center gap-3 mb-4">
                <div
                  class="modern-gradient-icon"
                  style="background: linear-gradient(45deg, #3b82f6, #6366f1); width: 2.5rem; height: 2.5rem;"
                >
                  <UIcon
                    name="i-heroicons-eye"
                    class="h-5 w-5 text-white"
                  />
                </div>
                <h3 class="text-xl font-semibold text-slate-900 dark:text-white">
                  我们的愿景
                </h3>
              </div>
              <p class="text-slate-600 dark:text-slate-400 leading-relaxed">
                成为技术领域最具影响力的知识分享平台，让优质内容触达更多人，推动技术社区的繁荣发展。
              </p>
            </div>

            <div
              class="modern-content-section"
              style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 2rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);"
            >
              <div class="flex items-center gap-3 mb-4">
                <div
                  class="modern-gradient-icon"
                  style="background: linear-gradient(45deg, #10b981, #059669); width: 2.5rem; height: 2.5rem;"
                >
                  <UIcon
                    name="i-heroicons-heart"
                    class="h-5 w-5 text-white"
                  />
                </div>
                <h3 class="text-xl font-semibold text-slate-900 dark:text-white">
                  我们的使命
                </h3>
              </div>
              <p class="text-slate-600 dark:text-slate-400 leading-relaxed">
                为技术创作者提供优质的写作环境，为读者提供有价值的技术内容，促进知识的传播与技术的进步。
              </p>
            </div>
          </div>
        </div>

        <!-- 核心特性 -->
        <div class="mb-16">
          <div class="text-center mb-12">
            <h2 class="text-3xl font-bold text-slate-900 dark:text-white mb-4">
              平台特色
            </h2>
            <p class="text-lg text-slate-600 dark:text-slate-400 max-w-2xl mx-auto">
              我们致力于打造最好的技术博客平台体验
            </p>
          </div>

          <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
            <div
              class="modern-content-section group"
              style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 2rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05); transition: all 0.3s ease;"
            >
              <div
                class="modern-gradient-icon mb-4"
                style="background: linear-gradient(45deg, #3b82f6, #6366f1); width: 3rem; height: 3rem;"
              >
                <UIcon
                  name="i-heroicons-document-text"
                  class="h-6 w-6 text-white"
                />
              </div>
              <h3 class="text-xl font-semibold text-slate-900 dark:text-white mb-3">
                优质内容
              </h3>
              <p class="text-slate-600 dark:text-slate-400">
                严格把控内容质量，确保每篇文章都具有实用价值和技术深度，为读者提供有价值的学习资源。
              </p>
            </div>

            <div
              class="modern-content-section group"
              style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 2rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05); transition: all 0.3s ease;"
            >
              <div
                class="modern-gradient-icon mb-4"
                style="background: linear-gradient(45deg, #10b981, #059669); width: 3rem; height: 3rem;"
              >
                <UIcon
                  name="i-heroicons-users"
                  class="h-6 w-6 text-white"
                />
              </div>
              <h3 class="text-xl font-semibold text-slate-900 dark:text-white mb-3">
                活跃社区
              </h3>
              <p class="text-slate-600 dark:text-slate-400">
                汇聚来自各个技术领域的专业人士，形成良好的交流氛围，促进知识分享和技术讨论。
              </p>
            </div>

            <div
              class="modern-content-section group"
              style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 2rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05); transition: all 0.3s ease;"
            >
              <div
                class="modern-gradient-icon mb-4"
                style="background: linear-gradient(45deg, #f59e0b, #f97316); width: 3rem; height: 3rem;"
              >
                <UIcon
                  name="i-heroicons-lightning-bolt"
                  class="h-6 w-6 text-white"
                />
              </div>
              <h3 class="text-xl font-semibold text-slate-900 dark:text-white mb-3">
                现代技术
              </h3>
              <p class="text-slate-600 dark:text-slate-400">
                采用最新的技术栈构建，提供流畅的用户体验、快速的页面加载和优秀的移动端适配。
              </p>
            </div>

            <div
              class="modern-content-section group"
              style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 2rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05); transition: all 0.3s ease;"
            >
              <div
                class="modern-gradient-icon mb-4"
                style="background: linear-gradient(45deg, #ec4899, #f43f5e); width: 3rem; height: 3rem;"
              >
                <UIcon
                  name="i-heroicons-heart"
                  class="h-6 w-6 text-white"
                />
              </div>
              <h3 class="text-xl font-semibold text-slate-900 dark:text-white mb-3">
                用户至上
              </h3>
              <p class="text-slate-600 dark:text-slate-400">
                以用户需求为中心设计产品功能，提供直观易用的界面和贴心的功能服务。
              </p>
            </div>

            <div
              class="modern-content-section group"
              style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 2rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05); transition: all 0.3s ease;"
            >
              <div
                class="modern-gradient-icon mb-4"
                style="background: linear-gradient(45deg, #8b5cf6, #a855f7); width: 3rem; height: 3rem;"
              >
                <UIcon
                  name="i-heroicons-shield-check"
                  class="h-6 w-6 text-white"
                />
              </div>
              <h3 class="text-xl font-semibold text-slate-900 dark:text-white mb-3">
                安全可靠
              </h3>
              <p class="text-slate-600 dark:text-slate-400">
                采用行业标准的安全措施，保护用户数据安全，提供稳定可靠的服务。
              </p>
            </div>

            <div
              class="modern-content-section group"
              style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 2rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05); transition: all 0.3s ease;"
            >
              <div
                class="modern-gradient-icon mb-4"
                style="background: linear-gradient(45deg, #06b6d4, #0891b2); width: 3rem; height: 3rem;"
              >
                <UIcon
                  name="i-heroicons-rocket-launch"
                  class="h-6 w-6 text-white"
                />
              </div>
              <h3 class="text-xl font-semibold text-slate-900 dark:text-white mb-3">
                持续创新
              </h3>
              <p class="text-slate-600 dark:text-slate-400">
                不断迭代产品功能，引入新技术和新理念，为用户提供更好的使用体验。
              </p>
            </div>
          </div>
        </div>

        <!-- 技术栈 -->
        <div class="mb-16">
          <div class="text-center mb-12">
            <h2 class="text-3xl font-bold text-slate-900 dark:text-white mb-4">
              技术架构
            </h2>
            <p class="text-lg text-slate-600 dark:text-slate-400 max-w-2xl mx-auto">
              采用现代化的技术栈，确保平台的高性能和可扩展性
            </p>
          </div>

          <div class="grid md:grid-cols-2 gap-8">
            <div
              class="modern-content-section"
              style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 2rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);"
            >
              <div class="flex items-center gap-3 mb-6">
                <div
                  class="modern-gradient-icon"
                  style="background: linear-gradient(45deg, #3b82f6, #6366f1); width: 2.5rem; height: 2.5rem;"
                >
                  <UIcon
                    name="i-heroicons-computer-desktop"
                    class="h-5 w-5 text-white"
                  />
                </div>
                <h3 class="text-xl font-semibold text-slate-900 dark:text-white">
                  前端技术
                </h3>
              </div>

              <div class="space-y-4">
                <div class="flex items-center gap-3">
                  <div class="w-2 h-2 rounded-full bg-green-500" />
                  <span class="text-slate-700 dark:text-slate-300">Vue.js 3 + Nuxt.js 3</span>
                </div>
                <div class="flex items-center gap-3">
                  <div class="w-2 h-2 rounded-full bg-blue-500" />
                  <span class="text-slate-700 dark:text-slate-300">TypeScript</span>
                </div>
                <div class="flex items-center gap-3">
                  <div class="w-2 h-2 rounded-full bg-purple-500" />
                  <span class="text-slate-700 dark:text-slate-300">Tailwind CSS + Nuxt UI</span>
                </div>
                <div class="flex items-center gap-3">
                  <div class="w-2 h-2 rounded-full bg-orange-500" />
                  <span class="text-slate-700 dark:text-slate-300">Pinia 状态管理</span>
                </div>
              </div>
            </div>

            <div
              class="modern-content-section"
              style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 2rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);"
            >
              <div class="flex items-center gap-3 mb-6">
                <div
                  class="modern-gradient-icon"
                  style="background: linear-gradient(45deg, #10b981, #059669); width: 2.5rem; height: 2.5rem;"
                >
                  <UIcon
                    name="i-heroicons-server"
                    class="h-5 w-5 text-white"
                  />
                </div>
                <h3 class="text-xl font-semibold text-slate-900 dark:text-white">
                  后端技术
                </h3>
              </div>

              <div class="space-y-4">
                <div class="flex items-center gap-3">
                  <div class="w-2 h-2 rounded-full bg-red-500" />
                  <span class="text-slate-700 dark:text-slate-300">Rust + Axum</span>
                </div>
                <div class="flex items-center gap-3">
                  <div class="w-2 h-2 rounded-full bg-blue-500" />
                  <span class="text-slate-700 dark:text-slate-300">PostgreSQL</span>
                </div>
                <div class="flex items-center gap-3">
                  <div class="w-2 h-2 rounded-full bg-green-500" />
                  <span class="text-slate-700 dark:text-slate-300">SQLx</span>
                </div>
                <div class="flex items-center gap-3">
                  <div class="w-2 h-2 rounded-full bg-yellow-500" />
                  <span class="text-slate-700 dark:text-slate-300">JWT 认证</span>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- 联系我们 -->
        <div
          class="modern-content-section text-center"
          style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 3rem 2rem; border: 1px solid #e2e8f0; box-shadow: 0 4px 6px rgba(0, 0, 0, 0.05);"
        >
          <div
            class="modern-gradient-icon mx-auto mb-6"
            style="background: linear-gradient(45deg, #667eea, #764ba2); width: 4rem; height: 4rem;"
          >
            <UIcon
              name="i-heroicons-envelope"
              class="h-8 w-8 text-white"
            />
          </div>

          <h2 class="text-2xl font-bold text-slate-900 dark:text-white mb-4">
            联系我们
          </h2>

          <p class="text-lg text-slate-600 dark:text-slate-400 mb-6 max-w-2xl mx-auto">
            如果您有任何问题、建议或者想要合作，欢迎随时联系我们
          </p>

          <div class="flex flex-wrap items-center justify-center gap-4">
            <a
              href="mailto:contact@zincbloom.com"
              class="inline-flex items-center gap-2 px-6 py-3 text-white rounded-lg font-medium transition-all hover:transform hover:scale-105"
              style="background: linear-gradient(45deg, #667eea, #764ba2); box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);"
            >
              <UIcon
                name="i-heroicons-envelope"
                class="w-4 h-4"
              />
              邮件联系
            </a>

            <NuxtLink
              to="/"
              class="inline-flex items-center gap-2 px-6 py-3 bg-slate-100 hover:bg-slate-200 text-slate-700 rounded-lg font-medium transition-all hover:transform hover:scale-105"
            >
              <UIcon
                name="i-heroicons-home"
                class="w-4 h-4"
              />
              返回首页
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
  title: "关于我们 - ZincBloom",
  meta: [
    { name: "description", content: "了解 ZincBloom 博客平台的愿景、使命和技术架构。我们致力于为技术创作者提供优质的写作环境，为读者提供有价值的技术内容。" },
    { property: "og:title", content: "关于我们 - ZincBloom" },
    { property: "og:description", content: "了解 ZincBloom 博客平台的愿景、使命和技术架构" },
    { property: "og:type", content: "website" },
  ],
});

// 模拟统计数据
const stats = ref({
  posts: 500,
  authors: 120,
  categories: 25,
});

// 可以在实际项目中从 API 获取真实数据
onMounted(async () => {
  // const api = useApi();
  // const response = await api.getStats();
  // if (response.success) {
  //   stats.value = response.data;
  // }
});
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
</style>
