<template>
  <div
    class="min-h-screen flex items-center justify-center bg-gray-50 dark:bg-gray-900 py-12 px-4 sm:px-6 lg:px-8"
  >
    <div class="max-w-md w-full space-y-8">
      <!-- 验证成功状态 -->
      <div
        v-if="verificationStatus === 'success'"
        class="text-center"
      >
        <div class="mx-auto h-12 w-12 flex items-center justify-center">
          <UIcon
            name="i-heroicons-check-circle"
            class="w-12 h-12 text-green-500"
          />
        </div>
        <h2
          class="mt-6 text-center text-3xl font-bold text-gray-900 dark:text-white"
        >
          邮箱验证成功
        </h2>
        <p
          class="mt-2 text-center text-sm text-gray-600 dark:text-gray-400"
        >
          您的邮箱已成功验证，现在可以开始使用您的账户了。
        </p>
        <div class="mt-8">
          <UButton
            to="/auth/login"
            color="primary"
            size="lg"
            block
          >
            前往登录
          </UButton>
        </div>
      </div>

      <!-- 验证失败状态 -->
      <div
        v-else-if="verificationStatus === 'error'"
        class="text-center"
      >
        <div class="mx-auto h-12 w-12 flex items-center justify-center">
          <UIcon
            name="i-heroicons-x-circle"
            class="w-12 h-12 text-red-500"
          />
        </div>
        <h2
          class="mt-6 text-center text-3xl font-bold text-gray-900 dark:text-white"
        >
          验证失败
        </h2>
        <p
          class="mt-2 text-center text-sm text-gray-600 dark:text-gray-400"
        >
          {{ errorMessage }}
        </p>
        <div class="mt-8 space-y-4">
          <UButton
            color="primary"
            size="lg"
            block
            :loading="resending"
            @click="resendVerification"
          >
            重新发送验证邮件
          </UButton>
          <UButton
            to="/auth/register"
            variant="outline"
            size="lg"
            block
          >
            重新注册
          </UButton>
        </div>
      </div>

      <!-- 验证中状态 -->
      <div
        v-else-if="verificationStatus === 'verifying'"
        class="text-center"
      >
        <div class="mx-auto h-12 w-12 flex items-center justify-center">
          <UIcon
            name="i-heroicons-clock"
            class="w-12 h-12 text-yellow-500 animate-spin"
          />
        </div>
        <h2
          class="mt-6 text-center text-3xl font-bold text-gray-900 dark:text-white"
        >
          验证中...
        </h2>
        <p
          class="mt-2 text-center text-sm text-gray-600 dark:text-gray-400"
        >
          正在验证您的邮箱，请稍候。
        </p>
      </div>

      <!-- 等待验证状态 -->
      <div
        v-else
        class="text-center"
      >
        <div class="mx-auto h-12 w-12 flex items-center justify-center">
          <UIcon
            name="i-heroicons-envelope"
            class="w-12 h-12 text-blue-500"
          />
        </div>
        <h2
          class="mt-6 text-center text-3xl font-bold text-gray-900 dark:text-white"
        >
          验证邮箱
        </h2>
        <p
          class="mt-2 text-center text-sm text-gray-600 dark:text-gray-400"
        >
          我们已向您的邮箱发送了验证链接，请查看邮件并点击链接完成验证。
        </p>
        <div class="mt-8 space-y-4">
          <UButton
            variant="outline"
            size="lg"
            block
            :loading="resending"
            @click="resendVerification"
          >
            重新发送验证邮件
          </UButton>
          <div class="text-sm text-gray-600 dark:text-gray-400">
            没有收到邮件？请检查垃圾邮件文件夹，或
            <button
              class="text-primary-600 hover:text-primary-500 dark:text-primary-400"
              @click="resendVerification"
            >
              重新发送
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// SEO 配置
useHead({
  title: "邮箱验证",
  meta: [{ name: "description", content: "验证您的邮箱地址以完成注册" }],
});

// 状态
const verificationStatus = ref<"pending" | "verifying" | "success" | "error">(
  "pending",
);
const errorMessage = ref("");
const resending = ref(false);

const route = useRoute();
const router = useRouter();
const toast = useToast();

// 验证响应类型
interface VerifyResponse {
  success: boolean;
  error?: {
    message: string;
  };
}

// 检查是否有验证token
const token = route.query.token as string;

// 如果有token，自动进行验证
if (token) {
  verificationStatus.value = "verifying";
  verifyEmail(token);
}

// 验证邮箱
async function verifyEmail(verificationToken: string) {
  try {
    const runtimeConfig = useRuntimeConfig();
    const response = await $fetch<VerifyResponse>(
      `${runtimeConfig.public.apiBaseUrl}/auth/verify-email`,
      {
        method: "POST",
        body: { token: verificationToken },
      },
    );

    if (response.success) {
      verificationStatus.value = "success";
      toast.add({
        title: "验证成功",
        description: "您的邮箱已成功验证",
        color: "success",
      });
    }
    else {
      verificationStatus.value = "error";
      errorMessage.value = response.error?.message || "验证失败，请重试";
    }
  }
  catch {
    verificationStatus.value = "error";
    errorMessage.value = "网络错误，请稍后重试";
  }
}

// 重新发送验证邮件
async function resendVerification() {
  resending.value = true;

  try {
    // 这里需要后端提供重新发送验证邮件的API
    // const response = await api.resendVerificationEmail();

    // 模拟成功
    toast.add({
      title: "发送成功",
      description: "验证邮件已重新发送，请查看您的邮箱",
      color: "success",
    });
  }
  catch {
    toast.add({
      title: "发送失败",
      description: "无法发送验证邮件，请稍后重试",
      color: "error",
    });
  }
  finally {
    resending.value = false;
  }
}

// 如果用户已经登录，重定向到首页
onMounted(() => {
  const authStore = useAuthStore();
  if (authStore.isAuthenticated) {
    router.push("/");
  }
});
</script>
