// https://nuxt.com/docs/api/configuration/nuxt-config

// import path from "node:path";
export default defineNuxtConfig({
  // 模块配置
  modules: [
    "@nuxt/eslint",
    "@nuxt/fonts",
    "@nuxt/icon",
    "@nuxt/ui",
    "@pinia/nuxt",
    "@vueuse/nuxt",
  ],

  // 日志配置
  ssr: true,
  devtools: { enabled: true },

  // 应用配置
  app: {
    head: {
      charset: "utf-8",
      viewport: "width=device-width, initial-scale=1",
      title: "ZincBloom - 现代化博客系统",
      meta: [
        {
          name: "description",
          content: "基于 Nuxt 3 和 Rust 构建的现代化博客系统",
        },
        {
          name: "keywords",
          content: "blog, nuxt, vue, rust, typescript",
        },
      ],
    },
  },

  // CSS配置
  css: ["~/assets/css/main.css"],

  // 运行时配置
  runtimeConfig: {
    // 私有键（仅在服务器端可用）
    // public keys that are exposed to the client-side
    public: {
      apiBaseUrl:
        process.env.NUXT_PUBLIC_API_BASE_URL || "http://localhost:8080",
    },
  },

  compatibilityDate: "2025-05-15",

  // Nitro 配置
  nitro: {
    compressPublicAssets: true,
    // 减少构建警告
    rollupConfig: {
      external: [],
    },
    // 优化输出
    minify: true,
  },

  // Vite 配置
  vite: {
    build: {
      // 减少 CommonJS 插件警告
      rollupOptions: {
        output: {
          manualChunks: undefined,
        },
      },
      // 减少 sourcemap 相关警告
      sourcemap: process.env.NODE_ENV === "development",
      // 优化构建
      chunkSizeWarningLimit: 1000,
    },
    optimizeDeps: {
      include: ["vue", "vue-router"],
    },
    // CSS 配置
    css: {
      devSourcemap: process.env.NODE_ENV === "development",
    },
  },

  // TypeScript配置
  typescript: {
    strict: true,
    typeCheck: true,
  },

  // ESLint 配置
  eslint: {
    config: {
      stylistic: {
        semi: true,
        quotes: "double",
      },
    },
  },

  // 路径别名配置
  // alias: {
  //     "@shared": path.resolve(__dirname, "../shared"),
  // },
});
