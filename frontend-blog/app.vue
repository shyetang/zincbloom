<template>
    <div>
        <NuxtRouteAnnouncer />
        <NuxtPage />
    </div>
</template>

<script setup lang="ts">
// 初始化认证状态
const authStore = useAuthStore();

onMounted(() => {
    // 应用启动时初始化认证状态
    authStore.initialize();
});
</script>
