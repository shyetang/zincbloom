<template>
  <div class="container mx-auto px-4 py-8">
    <div class="max-w-4xl mx-auto">
      <h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-8">
        友情链接
      </h1>

      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <!-- 技术类链接 -->
        <div class="bg-white dark:bg-gray-800 rounded-lg p-6 shadow-sm">
          <h2 class="text-lg font-semibold mb-4">
            技术社区
          </h2>
          <ul class="space-y-3">
            <li>
              <a
                href="https://vuejs.org"
                target="_blank"
                rel="noopener noreferrer"
                class="flex items-center text-primary-600 hover:text-primary-700"
              >
                <UIcon
                  name="i-heroicons-link"
                  class="w-4 h-4 mr-2"
                />
                Vue.js
              </a>
            </li>
            <li>
              <a
                href="https://nuxt.com"
                target="_blank"
                rel="noopener noreferrer"
                class="flex items-center text-primary-600 hover:text-primary-700"
              >
                <UIcon
                  name="i-heroicons-link"
                  class="w-4 h-4 mr-2"
                />
                Nuxt.js
              </a>
            </li>
            <li>
              <a
                href="https://www.rust-lang.org"
                target="_blank"
                rel="noopener noreferrer"
                class="flex items-center text-primary-600 hover:text-primary-700"
              >
                <UIcon
                  name="i-heroicons-link"
                  class="w-4 h-4 mr-2"
                />
                Rust
              </a>
            </li>
          </ul>
        </div>

        <!-- 工具类链接 -->
        <div class="bg-white dark:bg-gray-800 rounded-lg p-6 shadow-sm">
          <h2 class="text-lg font-semibold mb-4">
            开发工具
          </h2>
          <ul class="space-y-3">
            <li>
              <a
                href="https://github.com"
                target="_blank"
                rel="noopener noreferrer"
                class="flex items-center text-primary-600 hover:text-primary-700"
              >
                <UIcon
                  name="i-heroicons-link"
                  class="w-4 h-4 mr-2"
                />
                GitHub
              </a>
            </li>
            <li>
              <a
                href="https://tailwindcss.com"
                target="_blank"
                rel="noopener noreferrer"
                class="flex items-center text-primary-600 hover:text-primary-700"
              >
                <UIcon
                  name="i-heroicons-link"
                  class="w-4 h-4 mr-2"
                />
                Tailwind CSS
              </a>
            </li>
          </ul>
        </div>

        <!-- 友站链接 -->
        <div class="bg-white dark:bg-gray-800 rounded-lg p-6 shadow-sm">
          <h2 class="text-lg font-semibold mb-4">
            友站推荐
          </h2>
          <p class="text-gray-600 dark:text-gray-400 text-sm">
            如果您想申请友情链接，请通过联系页面与我们联系。
          </p>
        </div>
      </div>

      <!-- 申请友链说明 -->
      <div class="mt-12 bg-blue-50 dark:bg-blue-900/20 rounded-lg p-6">
        <h2 class="text-lg font-semibold mb-3">
          申请友情链接
        </h2>
        <p class="text-gray-700 dark:text-gray-300 mb-4">
          我们欢迎与优质的技术博客和网站建立友情链接关系。
        </p>
        <ul
          class="list-disc pl-6 space-y-1 text-gray-600 dark:text-gray-400"
        >
          <li>网站内容积极向上，无违法违规内容</li>
          <li>技术类或知识分享类网站优先</li>
          <li>网站访问稳定，更新频率较高</li>
          <li>网站设计美观，用户体验良好</li>
        </ul>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
useHead({
  title: "友情链接",
  meta: [
    {
      name: "description",
      content: "ZincBloom友情链接页面，发现更多优质技术资源",
    },
  ],
});
</script>
