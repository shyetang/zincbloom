<script setup lang="ts">
import {RouterView} from 'vue-router'
import {useAuthStore} from "@/stores/auth.ts";
import {onMounted} from "vue";

const authStore = useAuthStore();

onMounted(() => {
  if (authStore.isAuthenticated) {
    authStore.fetchUserProfile();
  }
})

</script>

<template>
  <RouterView/>
</template>

<style scoped>

</style>
