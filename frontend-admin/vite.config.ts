import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueDevTools from 'vite-plugin-vue-devtools'

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    vueDevTools(),
  ],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    },
  },
  // server 配置
  server: {
    proxy: {
      '/api': {
        target: 'http://localhost:8080',
        changeOrigin: true,
        // 将 /api 重写为空字符串，这样请求到后端时就不会带 /api 前缀了
        // 例如：前端请求 /api/posts -> 后端接收到 /posts
        rewrite: (path) => path.replace(/^\/api/,'')
      }
    }
  }
})
