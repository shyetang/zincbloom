export default defineNuxtRouteMiddleware(async (_to, _from) => {
  // 在服务器端跳过认证检查
  if (import.meta.server) {
    return;
  }

  const authStore = useAuthStore();

  // 等待认证状态初始化完成
  if (authStore.isLoading) {
    let attempts = 0;
    while (authStore.isLoading && attempts < 50) {
      await new Promise(resolve => setTimeout(resolve, 100));
      attempts++;
    }
  }

  // 检查本地存储中是否有token
  const token = typeof window !== "undefined" ? localStorage.getItem("access_token") : null;

  // 如果没有token也没有认证状态，重定向到登录页
  if (!token && !authStore.isAuthenticated) {
    return navigateTo("/auth/login");
  }

  // 如果有token但没有认证状态，尝试重新初始化
  if (token && !authStore.isAuthenticated) {
    await authStore.initialize();

    // 如果初始化后仍然没有认证，重定向到登录页
    if (!authStore.isAuthenticated) {
      return navigateTo("/auth/login");
    }
  }
});
