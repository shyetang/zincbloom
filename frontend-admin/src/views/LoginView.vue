<script setup lang="ts">
import {useAuthStore} from "@/stores/auth.ts";
import {ref} from "vue";

const authStore = useAuthStore();
// 使用ref创建响应式数据，用于绑定表单输入
const username = ref('');
const password = ref('');
const errorMessage = ref<string | null>(null);

const handleLogin = async () => {
  try {
    errorMessage.value = null;  // 清除旧的错误信息
    await authStore.login({
      username: username.value,
      password: password.value,
    });
    // 登录成功后的跳转逻辑已在 authStore中处理
  } catch (error) {
    errorMessage.value = '登录失败，请检查用户名或密码';
    console.error(error)
  }
};

</script>

<template>
  <div class="login-container">
    <div class="login-form">
      <h1>博客管理后台</h1>
      <form @submit.prevent="handleLogin">
        <div class="form-group">
          <label for="username">用户名</label>
          <input type="text" id="username" v-model="username" required/>
        </div>
        <div class="form-group">
          <label for="password">密码 </label>
          <input type="password" id="password" v-model="password" required/>
        </div>
        <button type="submit">登录</button>
        <p class="error" v-if="errorMessage">{{ errorMessage }}</p>
      </form>
    </div>
  </div>
</template>

<style scoped>
.login-container {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100vh;
  background-color: #f0f2f5;
}

.login-form {
  padding: 2rem;
  background: white;
  border-radius: 8px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  text-align: center;
  width: 350px;
}

.form-group {
  margin-bottom: 1.5rem;
  text-align: left;
}

label {
  display: block;
  margin-bottom: 0.5rem;
}

input {
  width: 100%;
  padding: 0.5rem;
  box-sizing: border-box;
}

button {
  width: 100%;
  padding: 0.75rem;
  background-color: #007bff;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.error {
  color: red;
  margin-top: 1rem;
}
</style>