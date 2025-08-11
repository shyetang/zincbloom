<template>
  <div>
    <!-- Hero 区域 -->
    <section
      class="modern-hero"
      style="padding: 2rem 0 1.5rem 0; background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 100%); position: relative;"
    >
      <!-- 背景装饰 -->
      <div style="position: absolute; inset: 0; background-image: radial-gradient(circle at 30% 60%, rgba(255, 255, 255, 0.1) 0%, transparent 50%); opacity: 0.8;" />

      <div class="relative mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
        <div style="text-align: center; color: white;">
          <!-- 现代化图标 -->
          <div style="margin: 0 auto 1rem; width: 2.5rem; height: 2.5rem; display: flex; align-items: center; justify-content: center; border-radius: 50%; background: rgba(255, 255, 255, 0.2); backdrop-filter: blur(10px);">
            <UIcon
              name="i-heroicons-document-text"
              style="width: 1.25rem; height: 1.25rem; color: white;"
            />
          </div>

          <!-- 现代化标题 -->
          <h1
            style="font-size: 1.875rem; font-weight: 700; line-height: 1.1; margin-bottom: 0.75rem;"
            class="md:text-2xl"
          >
            服务条款
          </h1>

          <!-- 副标题 -->
          <p
            style="font-size: 1rem; margin: 0 auto; line-height: 1.5; opacity: 0.9;"
            class="max-w-2xl"
          >
            使用 ZincBloom 服务前，请仔细阅读并同意以下条款
          </p>
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
          <span style="color: #3b82f6; font-weight: 500;">服务条款</span>
        </div>
      </div>
    </nav>

    <!-- 主要内容区域 -->
    <section
      style="background: linear-gradient(135deg, #f8fafc 0%, #e2e8f0 50%, #f3f4f6 100%); padding: 3rem 0; margin: 0; position: relative;"
    >
      <!-- 背景装饰 -->
      <div style="position: absolute; inset: 0; background-image: radial-gradient(circle at 1px 1px, rgba(99, 102, 241, 0.1) 1px, transparent 0); background-size: 60px 60px; opacity: 0.5;" />

      <div class="relative mx-auto max-w-4xl px-4 sm:px-6 lg:px-8">
        <!-- 更新信息 -->
        <div
          class="modern-content-section mb-8"
          style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 2rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);"
        >
          <div class="flex items-center justify-between mb-4">
            <div class="flex items-center gap-3">
              <div
                class="modern-gradient-icon"
                style="background: linear-gradient(45deg, #6366f1, #8b5cf6); width: 2.5rem; height: 2.5rem;"
              >
                <UIcon
                  name="i-heroicons-information-circle"
                  class="h-5 w-5 text-white"
                />
              </div>
              <h2 class="text-lg font-semibold text-slate-900 dark:text-white">
                条款信息
              </h2>
            </div>
          </div>

          <div class="grid md:grid-cols-2 gap-4 text-sm text-slate-600 dark:text-slate-400">
            <div class="flex items-center gap-2">
              <UIcon
                name="i-heroicons-calendar"
                class="w-4 h-4"
              />
              <span>生效日期：2024年1月1日</span>
            </div>
            <div class="flex items-center gap-2">
              <UIcon
                name="i-heroicons-arrow-path"
                class="w-4 h-4"
              />
              <span>最后更新：2024年1月1日</span>
            </div>
          </div>
        </div>

        <!-- 条款内容 -->
        <div
          class="modern-content-section"
          style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 3rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);"
        >
          <div class="prose prose-slate max-w-none dark:prose-invert">
            <!-- 1. 接受条款 -->
            <section class="mb-8">
              <h2 class="text-xl font-bold text-slate-900 dark:text-white mb-4 flex items-center gap-2">
                <span class="flex-shrink-0 w-6 h-6 rounded-full bg-blue-500 text-white text-sm flex items-center justify-center font-medium">1</span>
                接受条款
              </h2>
              <div class="text-slate-700 dark:text-slate-300 leading-relaxed space-y-4">
                <p>
                  欢迎使用 ZincBloom（"我们"、"平台"）提供的服务。通过访问或使用我们的网站和服务，您（"用户"、"您"）同意遵守并受以下服务条款（"条款"）的约束。
                </p>
                <p>
                  如果您不同意这些条款，请不要使用我们的服务。我们保留随时修改这些条款的权利，修改后的条款将在发布后立即生效。
                </p>
              </div>
            </section>

            <!-- 2. 服务描述 -->
            <section class="mb-8">
              <h2 class="text-xl font-bold text-slate-900 dark:text-white mb-4 flex items-center gap-2">
                <span class="flex-shrink-0 w-6 h-6 rounded-full bg-green-500 text-white text-sm flex items-center justify-center font-medium">2</span>
                服务描述
              </h2>
              <div class="text-slate-700 dark:text-slate-300 leading-relaxed space-y-4">
                <p>
                  ZincBloom 是一个技术博客平台，为用户提供以下服务：
                </p>
                <ul class="list-disc ml-6 space-y-2">
                  <li>文章创作、发布和管理工具</li>
                  <li>技术内容分享和交流平台</li>
                  <li>用户个人主页和资料管理</li>
                  <li>文章搜索、分类和标签功能</li>
                  <li>社区互动功能（评论、点赞、收藏等）</li>
                </ul>
                <p>
                  我们致力于提供高质量、稳定的服务，但不保证服务的绝对连续性和无错误性。
                </p>
              </div>
            </section>

            <!-- 3. 用户账户 -->
            <section class="mb-8">
              <h2 class="text-xl font-bold text-slate-900 dark:text-white mb-4 flex items-center gap-2">
                <span class="flex-shrink-0 w-6 h-6 rounded-full bg-orange-500 text-white text-sm flex items-center justify-center font-medium">3</span>
                用户账户
              </h2>
              <div class="text-slate-700 dark:text-slate-300 leading-relaxed space-y-4">
                <p>
                  为了使用某些功能，您需要创建一个账户。您同意：
                </p>
                <ul class="list-disc ml-6 space-y-2">
                  <li>提供准确、完整的注册信息</li>
                  <li>保护您的账户密码安全</li>
                  <li>对您账户下的所有活动承担责任</li>
                  <li>及时更新您的个人信息</li>
                  <li>不与他人分享您的账户</li>
                </ul>
                <p>
                  我们有权因违反条款、安全考虑或其他合理原因暂停或终止您的账户。
                </p>
              </div>
            </section>

            <!-- 4. 用户内容 -->
            <section class="mb-8">
              <h2 class="text-xl font-bold text-slate-900 dark:text-white mb-4 flex items-center gap-2">
                <span class="flex-shrink-0 w-6 h-6 rounded-full bg-purple-500 text-white text-sm flex items-center justify-center font-medium">4</span>
                用户内容
              </h2>
              <div class="text-slate-700 dark:text-slate-300 leading-relaxed space-y-4">
                <p>
                  您对自己发布的内容（包括文章、评论、个人资料等）负责。您保证：
                </p>
                <ul class="list-disc ml-6 space-y-2">
                  <li>您拥有发布内容的合法权利</li>
                  <li>内容不侵犯他人的知识产权</li>
                  <li>内容不包含非法、有害或冒犯性材料</li>
                  <li>内容符合相关法律法规</li>
                </ul>
                <p>
                  您授权我们使用、展示、分发您的内容，以提供和改进我们的服务。我们有权删除违反条款的内容。
                </p>
              </div>
            </section>

            <!-- 5. 禁止行为 -->
            <section class="mb-8">
              <h2 class="text-xl font-bold text-slate-900 dark:text-white mb-4 flex items-center gap-2">
                <span class="flex-shrink-0 w-6 h-6 rounded-full bg-red-500 text-white text-sm flex items-center justify-center font-medium">5</span>
                禁止行为
              </h2>
              <div class="text-slate-700 dark:text-slate-300 leading-relaxed space-y-4">
                <p>
                  在使用我们的服务时，您不得：
                </p>
                <ul class="list-disc ml-6 space-y-2">
                  <li>发布虚假、误导或恶意信息</li>
                  <li>侵犯他人的隐私权或知识产权</li>
                  <li>进行垃圾邮件或恶意营销活动</li>
                  <li>尝试干扰或破坏我们的服务</li>
                  <li>使用自动化工具恶意访问我们的服务</li>
                  <li>冒充他人或虚假陈述身份</li>
                  <li>从事任何违法违规活动</li>
                </ul>
                <p>
                  违反上述规定可能导致账户被暂停或终止。
                </p>
              </div>
            </section>

            <!-- 6. 知识产权 -->
            <section class="mb-8">
              <h2 class="text-xl font-bold text-slate-900 dark:text-white mb-4 flex items-center gap-2">
                <span class="flex-shrink-0 w-6 h-6 rounded-full bg-indigo-500 text-white text-sm flex items-center justify-center font-medium">6</span>
                知识产权
              </h2>
              <div class="text-slate-700 dark:text-slate-300 leading-relaxed space-y-4">
                <p>
                  ZincBloom 平台的技术架构、设计、代码和商标归我们所有。用户发布的原创内容版权归用户所有。
                </p>
                <p>
                  我们尊重知识产权，如果您认为有内容侵犯了您的权利，请联系我们进行处理。
                </p>
              </div>
            </section>

            <!-- 7. 隐私保护 -->
            <section class="mb-8">
              <h2 class="text-xl font-bold text-slate-900 dark:text-white mb-4 flex items-center gap-2">
                <span class="flex-shrink-0 w-6 h-6 rounded-full bg-pink-500 text-white text-sm flex items-center justify-center font-medium">7</span>
                隐私保护
              </h2>
              <div class="text-slate-700 dark:text-slate-300 leading-relaxed space-y-4">
                <p>
                  我们重视您的隐私。我们收集和使用您的个人信息仅用于提供和改进服务。详细的隐私政策请查看我们的隐私政策页面。
                </p>
                <p>
                  我们采用行业标准的安全措施保护您的个人信息，但不能保证绝对的安全性。
                </p>
              </div>
            </section>

            <!-- 8. 免责声明 -->
            <section class="mb-8">
              <h2 class="text-xl font-bold text-slate-900 dark:text-white mb-4 flex items-center gap-2">
                <span class="flex-shrink-0 w-6 h-6 rounded-full bg-yellow-500 text-white text-sm flex items-center justify-center font-medium">8</span>
                免责声明
              </h2>
              <div class="text-slate-700 dark:text-slate-300 leading-relaxed space-y-4">
                <p>
                  我们的服务按"现状"提供。我们不对以下内容承担责任：
                </p>
                <ul class="list-disc ml-6 space-y-2">
                  <li>用户发布内容的准确性或合法性</li>
                  <li>因使用我们服务而产生的任何损失</li>
                  <li>第三方链接或服务的可用性</li>
                  <li>服务的中断或数据丢失</li>
                </ul>
                <p>
                  在法律允许的最大范围内，我们排除所有明示或暗示的保证。
                </p>
              </div>
            </section>

            <!-- 9. 服务修改 -->
            <section class="mb-8">
              <h2 class="text-xl font-bold text-slate-900 dark:text-white mb-4 flex items-center gap-2">
                <span class="flex-shrink-0 w-6 h-6 rounded-full bg-teal-500 text-white text-sm flex items-center justify-center font-medium">9</span>
                服务修改与终止
              </h2>
              <div class="text-slate-700 dark:text-slate-300 leading-relaxed space-y-4">
                <p>
                  我们保留随时修改、暂停或终止服务的权利。我们会尽力提前通知重大变更，但不承担必须通知的义务。
                </p>
                <p>
                  您可以随时停止使用我们的服务并删除您的账户。
                </p>
              </div>
            </section>

            <!-- 10. 争议解决 -->
            <section class="mb-8">
              <h2 class="text-xl font-bold text-slate-900 dark:text-white mb-4 flex items-center gap-2">
                <span class="flex-shrink-0 w-6 h-6 rounded-full bg-cyan-500 text-white text-sm flex items-center justify-center font-medium">10</span>
                争议解决
              </h2>
              <div class="text-slate-700 dark:text-slate-300 leading-relaxed space-y-4">
                <p>
                  本条款受中华人民共和国法律管辖。因使用我们服务产生的争议，我们优先通过友好协商解决。
                </p>
                <p>
                  如协商不成，争议应提交至我们所在地人民法院管辖。
                </p>
              </div>
            </section>

            <!-- 11. 联系我们 -->
            <section class="mb-8">
              <h2 class="text-xl font-bold text-slate-900 dark:text-white mb-4 flex items-center gap-2">
                <span class="flex-shrink-0 w-6 h-6 rounded-full bg-gray-500 text-white text-sm flex items-center justify-center font-medium">11</span>
                联系我们
              </h2>
              <div class="text-slate-700 dark:text-slate-300 leading-relaxed space-y-4">
                <p>
                  如果您对这些服务条款有任何疑问或建议，请通过以下方式联系我们：
                </p>
                <div class="bg-slate-50 dark:bg-slate-800 rounded-lg p-4">
                  <div class="flex items-center gap-2 mb-2">
                    <UIcon
                      name="i-heroicons-envelope"
                      class="w-4 h-4 text-slate-600"
                    />
                    <span class="font-medium">邮箱：</span>
                    <a
                      href="mailto:legal@zincbloom.com"
                      class="text-blue-600 hover:text-blue-700"
                    >legal@zincbloom.com</a>
                  </div>
                </div>
              </div>
            </section>

            <!-- 条款确认 -->
            <div class="mt-8 pt-8 border-t border-slate-200 dark:border-slate-700">
              <div class="bg-blue-50 dark:bg-blue-900/20 rounded-lg p-6">
                <div class="flex items-start gap-3">
                  <UIcon
                    name="i-heroicons-information-circle"
                    class="w-5 h-5 text-blue-600 mt-0.5"
                  />
                  <div>
                    <h3 class="font-medium text-blue-900 dark:text-blue-100 mb-2">
                      重要提醒
                    </h3>
                    <p class="text-blue-700 dark:text-blue-200 text-sm leading-relaxed">
                      通过继续使用 ZincBloom 的服务，您确认已阅读、理解并同意遵守上述服务条款。
                      我们建议您定期查看本页面以了解最新的条款变更。
                    </p>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- 相关链接 -->
        <div
          class="modern-content-section mt-8 text-center"
          style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 2rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);"
        >
          <h3 class="text-lg font-semibold text-slate-900 dark:text-white mb-4">
            相关政策
          </h3>

          <div class="flex flex-wrap items-center justify-center gap-4">
            <NuxtLink
              to="/privacy"
              class="inline-flex items-center gap-2 px-4 py-2 text-slate-600 hover:text-slate-900 border border-slate-200 hover:border-slate-300 rounded-lg transition-colors"
            >
              <UIcon
                name="i-heroicons-shield-check"
                class="w-4 h-4"
              />
              隐私政策
            </NuxtLink>

            <NuxtLink
              to="/help"
              class="inline-flex items-center gap-2 px-4 py-2 text-slate-600 hover:text-slate-900 border border-slate-200 hover:border-slate-300 rounded-lg transition-colors"
            >
              <UIcon
                name="i-heroicons-question-mark-circle"
                class="w-4 h-4"
              />
              帮助中心
            </NuxtLink>

            <NuxtLink
              to="/about"
              class="inline-flex items-center gap-2 px-4 py-2 text-slate-600 hover:text-slate-900 border border-slate-200 hover:border-slate-300 rounded-lg transition-colors"
            >
              <UIcon
                name="i-heroicons-information-circle"
                class="w-4 h-4"
              />
              关于我们
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
  title: "服务条款 - ZincBloom",
  meta: [
    { name: "description", content: "ZincBloom 服务条款，了解使用我们服务的规则和条件。" },
    { property: "og:title", content: "服务条款 - ZincBloom" },
    { property: "og:description", content: "ZincBloom 服务条款，了解使用我们服务的规则和条件" },
    { property: "og:type", content: "website" },
  ],
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

.prose h2 {
  scroll-margin-top: 2rem;
}

.prose ul {
  margin-top: 0.5rem;
  margin-bottom: 1rem;
}

.prose li {
  margin-top: 0.25rem;
  margin-bottom: 0.25rem;
}
</style>
