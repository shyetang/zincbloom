<template>
  <div class="container mx-auto px-4 py-8">
    <div class="max-w-4xl mx-auto">
      <h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-8">
        网站地图
      </h1>

      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
        <!-- 主要页面 -->
        <div>
          <h2 class="text-xl font-semibold mb-4">
            主要页面
          </h2>
          <ul class="space-y-2">
            <li>
              <NuxtLink
                to="/"
                class="text-primary-600 hover:text-primary-700"
              >首页</NuxtLink>
            </li>
            <li>
              <NuxtLink
                to="/posts"
                class="text-primary-600 hover:text-primary-700"
              >文章</NuxtLink>
            </li>
            <li>
              <NuxtLink
                to="/categories"
                class="text-primary-600 hover:text-primary-700"
              >分类</NuxtLink>
            </li>
            <li>
              <NuxtLink
                to="/tags"
                class="text-primary-600 hover:text-primary-700"
              >标签</NuxtLink>
            </li>
            <li>
              <NuxtLink
                to="/authors"
                class="text-primary-600 hover:text-primary-700"
              >作者</NuxtLink>
            </li>
          </ul>
        </div>

        <!-- 用户页面 -->
        <div>
          <h2 class="text-xl font-semibold mb-4">
            用户中心
          </h2>
          <ul class="space-y-2">
            <li>
              <NuxtLink
                to="/auth/login"
                class="text-primary-600 hover:text-primary-700"
              >登录</NuxtLink>
            </li>
            <li>
              <NuxtLink
                to="/auth/register"
                class="text-primary-600 hover:text-primary-700"
              >注册</NuxtLink>
            </li>
            <li>
              <NuxtLink
                to="/user/profile"
                class="text-primary-600 hover:text-primary-700"
              >个人资料</NuxtLink>
            </li>
            <li>
              <NuxtLink
                to="/user/posts"
                class="text-primary-600 hover:text-primary-700"
              >我的文章</NuxtLink>
            </li>
            <li>
              <NuxtLink
                to="/write"
                class="text-primary-600 hover:text-primary-700"
              >写文章</NuxtLink>
            </li>
          </ul>
        </div>

        <!-- 其他页面 -->
        <div>
          <h2 class="text-xl font-semibold mb-4">
            其他
          </h2>
          <ul class="space-y-2">
            <li>
              <NuxtLink
                to="/about"
                class="text-primary-600 hover:text-primary-700"
              >关于我们</NuxtLink>
            </li>
            <li>
              <NuxtLink
                to="/contact"
                class="text-primary-600 hover:text-primary-700"
              >联系我们</NuxtLink>
            </li>
            <li>
              <NuxtLink
                to="/privacy"
                class="text-primary-600 hover:text-primary-700"
              >隐私政策</NuxtLink>
            </li>
            <li>
              <NuxtLink
                to="/terms"
                class="text-primary-600 hover:text-primary-700"
              >服务条款</NuxtLink>
            </li>
          </ul>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
useHead({
  title: "网站地图",
  meta: [
    { name: "description", content: "ZincBloom网站地图，快速找到您需要的页面" },
  ],
});
</script>
