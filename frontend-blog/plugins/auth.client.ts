// 客户端认证初始化插件
export default defineNuxtPlugin(async () => {
  const authStore = useAuthStore();

  // 只在客户端初始化认证状态
  if (typeof window !== "undefined") {
    await authStore.initialize();
  }
});
