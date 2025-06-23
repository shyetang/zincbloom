<script setup lang="ts">
import {useAuthStore} from "@/stores/auth.ts";

const authStore = useAuthStore();

const handleLogout = () => {
  authStore.logout();
};
</script>

<template>
  <div>
    <h1>欢迎，{{ authStore.user?.username || '管理员' }}!</h1>
    <p>你已成功登录。</p>
    <button @click="handleLogout">登出</button>
  </div>
</template>

<style scoped>

</style>