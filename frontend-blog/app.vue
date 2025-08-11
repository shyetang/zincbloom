<template>
  <div>
    <NuxtRouteAnnouncer />
    <NuxtLayout>
      <NuxtPage />
    </NuxtLayout>
    <!-- 全局通知组件 -->
    <UNotifications />
  </div>
</template>

<script setup lang="ts">
// 认证状态将通过插件自动初始化
</script>
