<template>
  <div>
    <NuxtRouteAnnouncer />
    <NuxtLayout>
      <NuxtPage />
    </NuxtLayout>
    <!-- 移除这行，Nuxt UI v3 会自动处理通知 -->
    <!-- <UNotifications /> -->
  </div>
</template>

<script setup lang="ts">
// 认证状态将通过插件自动初始化
</script>
