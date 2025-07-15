<template>
  <footer style="background: linear-gradient(135deg, #f1f5f9 0%, #e2e8f0 50%, #cbd5e1 100%); border-top: 1px solid rgba(226, 232, 240, 0.6); position: relative; overflow: hidden;">
    <!-- 背景装饰 -->
    <div style="position: absolute; inset: 0; background-image: radial-gradient(circle at 1px 1px, rgba(148, 163, 184, 0.1) 1px, transparent 0); background-size: 60px 60px; opacity: 0.5;" />

    <div style="position: relative; z-index: 10; margin: 0 auto; max-width: 80rem; padding: 0 1rem;">
      <!-- 主要内容区域 -->
      <div style="padding: 4rem 0; display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 2rem;">
        <!-- 网站信息 -->
        <div style="grid-column: span 1;">
          <div style="display: flex; align-items: center; gap: 0.75rem; margin-bottom: 1.5rem;">
            <div style="display: flex; align-items: center; justify-content: center; width: 2.5rem; height: 2.5rem; background: linear-gradient(45deg, #3b82f6, #8b5cf6); border-radius: 0.75rem; box-shadow: 0 4px 8px rgba(59, 130, 246, 0.3);">
              <UIcon
                name="i-heroicons-sparkles"
                class="w-5 h-5 text-white"
              />
            </div>
            <span style="font-size: 1.5rem; font-weight: 800; background: linear-gradient(45deg, #3b82f6, #8b5cf6, #6366f1); -webkit-background-clip: text; background-clip: text; -webkit-text-fill-color: transparent;">ZincBloom</span>
          </div>
          <p
            style="color: #64748b; font-size: 0.875rem; margin-bottom: 1.5rem; line-height: 1.6;"
            class="dark:text-slate-400"
          >
            基于 Nuxt 3 和 Rust 构建的现代化博客系统，提供优雅的写作体验和阅读体验。
          </p>
          <div style="display: flex; gap: 0.75rem;">
            <a
              v-for="social in socialLinks"
              :key="social.name"
              :href="social.url"
              target="_blank"
              rel="noopener noreferrer"
              style="display: flex; align-items: center; justify-content: center; width: 2.5rem; height: 2.5rem; background: rgba(255, 255, 255, 0.8); border: 1px solid rgba(226, 232, 240, 0.6); border-radius: 0.75rem; color: #64748b; text-decoration: none; transition: all 0.3s ease; backdrop-filter: blur(10px);"
              class="hover:border-blue-300 hover:text-blue-600 hover:transform hover:scale-105"
              :aria-label="social.name"
            >
              <UIcon
                :name="social.icon"
                class="w-4 h-4"
              />
            </a>
          </div>
        </div>

        <!-- 快速链接 -->
        <div style="background: rgba(255, 255, 255, 0.6); padding: 1.5rem; border-radius: 1rem; border: 1px solid rgba(226, 232, 240, 0.6); backdrop-filter: blur(10px);">
          <h3
            style="font-size: 1rem; font-weight: 600; color: #1e293b; margin-bottom: 1rem; display: flex; align-items: center; gap: 0.5rem;"
            class="dark:text-white"
          >
            <div style="width: 1.5rem; height: 1.5rem; background: linear-gradient(45deg, #10b981, #059669); border-radius: 0.375rem; display: flex; align-items: center; justify-content: center;">
              <UIcon
                name="i-heroicons-link"
                class="w-3 h-3 text-white"
              />
            </div>
            快速链接
          </h3>
          <ul style="display: flex; flex-direction: column; gap: 0.5rem;">
            <li
              v-for="link in quickLinks"
              :key="link.name"
            >
              <NuxtLink
                :to="link.href"
                style="color: #64748b; font-size: 0.875rem; text-decoration: none; transition: all 0.2s ease; display: block; padding: 0.25rem 0;"
                class="hover:text-blue-600 hover:translate-x-1"
              >
                {{ link.name }}
              </NuxtLink>
            </li>
          </ul>
        </div>

        <!-- 分类链接 -->
        <div style="background: rgba(255, 255, 255, 0.6); padding: 1.5rem; border-radius: 1rem; border: 1px solid rgba(226, 232, 240, 0.6); backdrop-filter: blur(10px);">
          <h3
            style="font-size: 1rem; font-weight: 600; color: #1e293b; margin-bottom: 1rem; display: flex; align-items: center; gap: 0.5rem;"
            class="dark:text-white"
          >
            <div style="width: 1.5rem; height: 1.5rem; background: linear-gradient(45deg, #8b5cf6, #a855f7); border-radius: 0.375rem; display: flex; align-items: center; justify-content: center;">
              <UIcon
                name="i-heroicons-tag"
                class="w-3 h-3 text-white"
              />
            </div>
            分类
          </h3>
          <ul style="display: flex; flex-direction: column; gap: 0.5rem;">
            <li
              v-for="category in categories"
              :key="category.id"
            >
              <NuxtLink
                :to="`/posts?category=${category.slug}`"
                style="color: #64748b; font-size: 0.875rem; text-decoration: none; transition: all 0.2s ease; display: block; padding: 0.25rem 0;"
                class="hover:text-purple-600 hover:translate-x-1"
              >
                {{ category.name }}
              </NuxtLink>
            </li>
          </ul>
        </div>

        <!-- 其他链接 -->
        <div style="background: rgba(255, 255, 255, 0.6); padding: 1.5rem; border-radius: 1rem; border: 1px solid rgba(226, 232, 240, 0.6); backdrop-filter: blur(10px);">
          <h3
            style="font-size: 1rem; font-weight: 600; color: #1e293b; margin-bottom: 1rem; display: flex; align-items: center; gap: 0.5rem;"
            class="dark:text-white"
          >
            <div style="width: 1.5rem; height: 1.5rem; background: linear-gradient(45deg, #f59e0b, #f97316); border-radius: 0.375rem; display: flex; align-items: center; justify-content: center;">
              <UIcon
                name="i-heroicons-ellipsis-horizontal"
                class="w-3 h-3 text-white"
              />
            </div>
            其他
          </h3>
          <ul style="display: flex; flex-direction: column; gap: 0.5rem;">
            <li
              v-for="link in otherLinks"
              :key="link.name"
            >
              <NuxtLink
                :to="link.href"
                style="color: #64748b; font-size: 0.875rem; text-decoration: none; transition: all 0.2s ease; display: block; padding: 0.25rem 0;"
                class="hover:text-orange-600 hover:translate-x-1"
              >
                {{ link.name }}
              </NuxtLink>
            </li>
          </ul>
        </div>
      </div>

      <!-- 底部版权信息 -->
      <div style="padding: 2rem 0; border-top: 1px solid rgba(226, 232, 240, 0.6); background: rgba(255, 255, 255, 0.8); margin: 0 -1rem; padding-left: 1rem; padding-right: 1rem; backdrop-filter: blur(10px);">
        <div style="display: flex; flex-direction: column; align-items: center; gap: 1rem;">
          <!-- 版权信息 -->
          <div
            style="font-size: 0.875rem; color: #64748b; text-align: center;"
            class="dark:text-slate-400"
          >
            © {{ currentYear }} ZincBloom. 保留所有权利。
          </div>

          <!-- 底部链接 -->
          <div style="display: flex; align-items: center; gap: 1.5rem; flex-wrap: wrap; justify-content: center;">
            <NuxtLink
              to="/privacy"
              style="font-size: 0.875rem; color: #64748b; text-decoration: none; transition: all 0.2s ease;"
              class="hover:text-blue-600"
            >
              隐私政策
            </NuxtLink>
            <NuxtLink
              to="/terms"
              style="font-size: 0.875rem; color: #64748b; text-decoration: none; transition: all 0.2s ease;"
              class="hover:text-blue-600"
            >
              服务条款
            </NuxtLink>
            <div
              style="font-size: 0.875rem; color: #64748b; display: flex; align-items: center; gap: 0.25rem;"
              class="dark:text-slate-400"
            >
              由
              <a
                href="https://nuxt.com"
                target="_blank"
                rel="noopener noreferrer"
                style="color: #10b981; text-decoration: none; font-weight: 500; transition: all 0.2s ease;"
                class="hover:text-emerald-700"
              >
                Nuxt
              </a>
              <span style="color: #ef4444;">❤️</span>
              <a
                href="https://www.rust-lang.org"
                target="_blank"
                rel="noopener noreferrer"
                style="color: #f97316; text-decoration: none; font-weight: 500; transition: all 0.2s ease;"
                class="hover:text-orange-700"
              >
                Rust
              </a>
              驱动
            </div>
          </div>
        </div>
      </div>
    </div>
  </footer>
</template>

<script setup lang="ts">
// 当前年份
const currentYear = new Date().getFullYear();

// 社交媒体链接
const socialLinks = [
  {
    name: "GitHub",
    icon: "i-simple-icons-github",
    url: "https://github.com",
  },
  {
    name: "Twitter",
    icon: "i-simple-icons-twitter",
    url: "https://twitter.com",
  },
  {
    name: "RSS",
    icon: "i-heroicons-rss",
    url: "/rss.xml",
  },
];

// 快速链接
const quickLinks = [
  { name: "首页", href: "/" },
  { name: "文章", href: "/posts" },
  { name: "标签", href: "/tags" },
  { name: "关于", href: "/about" },
  { name: "联系我们", href: "/contact" },
];

// 示例分类（后续从API获取）
const categories = ref([
  { id: "1", name: "技术分享", slug: "tech" },
  { id: "2", name: "生活随笔", slug: "life" },
  { id: "3", name: "学习笔记", slug: "notes" },
  { id: "4", name: "开源项目", slug: "opensource" },
]);

// 其他链接
const otherLinks = [
  { name: "网站地图", href: "/sitemap" },
  { name: "友情链接", href: "/links" },
  { name: "意见反馈", href: "/feedback" },
  { name: "帮助中心", href: "/help" },
];
</script>
