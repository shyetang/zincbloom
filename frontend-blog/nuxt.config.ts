// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
    compatibilityDate: "2025-05-15",
    devtools: { enabled: true },

    // TypeScript配置
    typescript: {
        strict: true,
        typeCheck: true,
    },

    // 模块配置
    modules: [
        "@nuxt/eslint",
        "@nuxt/fonts",
        "@nuxt/icon",
        "@nuxt/image",
        "@nuxt/ui",
        "@pinia/nuxt",
        "@vueuse/nuxt",
    ],

    // CSS配置
    css: ["~/assets/css/main.css"],

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

    // 运行时配置
    runtimeConfig: {
        // 私有键（仅在服务器端可用）
        // public keys that are exposed to the client-side
        public: {
            apiBaseUrl:
                process.env.NUXT_PUBLIC_API_BASE_URL || "http://localhost:8080",
        },
    },
    

    // 构建配置
    nitro: {
        compressPublicAssets: true,
    },
});
