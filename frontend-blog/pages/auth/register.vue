<template>
  <div
    class="min-h-screen flex items-center justify-center bg-gray-50 dark:bg-gray-900 py-12 px-4 sm:px-6 lg:px-8"
  >
    <div class="max-w-md w-full space-y-8">
      <!-- 头部 -->
      <div>
        <div class="mx-auto h-12 w-12 flex items-center justify-center">
          <UIcon
            name="i-heroicons-sparkles"
            class="w-12 h-12 text-primary-500"
          />
        </div>
        <h2
          class="mt-6 text-center text-3xl font-bold text-gray-900 dark:text-white"
        >
          创建新账户
        </h2>
        <p
          class="mt-2 text-center text-sm text-gray-600 dark:text-gray-400"
        >
          或者
          <NuxtLink
            to="/auth/login"
            class="font-medium text-primary-600 hover:text-primary-500 dark:text-primary-400"
          >
            登录已有账户
          </NuxtLink>
        </p>
      </div>

      <!-- 注册表单 -->
      <UForm
        :schema="registerSchema"
        :state="formData"
        class="mt-8 space-y-6"
        @submit="handleSubmit"
      >
        <div class="space-y-4">
          <UFormGroup
            label="用户名"
            name="username"
            required
          >
            <UInput
              v-model="formData.username"
              placeholder="请输入用户名"
              icon="i-heroicons-user"
              size="lg"
            />
          </UFormGroup>

          <UFormGroup
            label="邮箱地址"
            name="email"
            required
          >
            <UInput
              v-model="formData.email"
              type="email"
              placeholder="请输入邮箱地址"
              icon="i-heroicons-envelope"
              size="lg"
            />
          </UFormGroup>

          <UFormGroup
            label="密码"
            name="password"
            required
          >
            <UInput
              v-model="formData.password"
              type="password"
              placeholder="请输入密码"
              icon="i-heroicons-lock-closed"
              size="lg"
            />
          </UFormGroup>

          <UFormGroup
            label="确认密码"
            name="confirmPassword"
            required
          >
            <UInput
              v-model="formData.confirmPassword"
              type="password"
              placeholder="请再次输入密码"
              icon="i-heroicons-lock-closed"
              size="lg"
            />
          </UFormGroup>
        </div>

        <div class="flex items-center">
          <UCheckbox
            v-model="formData.acceptTerms"
            name="acceptTerms"
          />
          <label
            class="ml-2 block text-sm text-gray-900 dark:text-gray-300"
          >
            我同意
            <NuxtLink
              to="/terms"
              class="text-primary-600 hover:text-primary-500 dark:text-primary-400"
            >
              服务条款
            </NuxtLink>
            和
            <NuxtLink
              to="/privacy"
              class="text-primary-600 hover:text-primary-500 dark:text-primary-400"
            >
              隐私政策
            </NuxtLink>
          </label>
        </div>

        <div>
          <UButton
            type="submit"
            color="primary"
            size="lg"
            block
            :loading="pending"
            :disabled="!formData.acceptTerms"
          >
            创建账户
          </UButton>
        </div>
      </UForm>
    </div>
  </div>
</template>

<script setup lang="ts">
import { z } from "zod";

// SEO 配置
useHead({
  title: "用户注册",
  meta: [
    {
      name: "description",
      content: "注册 ZincBloom 账户，开始您的博客之旅",
    },
  ],
});

// 表单验证架构
const registerSchema = z
  .object({
    username: z
      .string()
      .min(3, "用户名至少3个字符")
      .max(20, "用户名不能超过20个字符"),
    email: z.string().email("请输入有效的邮箱地址"),
    password: z
      .string()
      .min(8, "密码至少8个字符")
      .regex(
        /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)/,
        "密码必须包含大小写字母和数字",
      ),
    confirmPassword: z.string(),
    acceptTerms: z
      .boolean()
      .refine(val => val === true, "请同意服务条款和隐私政策"),
  })
  .refine(data => data.password === data.confirmPassword, {
    message: "两次输入的密码不一致",
    path: ["confirmPassword"],
  });

// 表单数据
const formData = reactive({
  username: "",
  email: "",
  password: "",
  confirmPassword: "",
  acceptTerms: false,
});

// 状态
const pending = ref(false);
const authStore = useAuthStore();
const router = useRouter();
const toast = useToast();

// 提交处理
const handleSubmit = async () => {
  pending.value = true;

  try {
    const result = await authStore.register({
      username: formData.username,
      email: formData.email,
      password: formData.password,
    });

    if (result.success) {
      toast.add({
        title: "注册成功",
        description: "请查看您的邮箱并点击验证链接",
        color: "success",
      });

      router.push("/auth/verify");
    }
    else {
      toast.add({
        title: "注册失败",
        description:
                    (result as any).error?.message || "注册过程中发生错误",
        color: "error",
      });
    }
  }
  catch (error) {
    toast.add({
      title: "注册失败",
      description: "网络错误，请稍后重试",
      color: "error",
    });
  }
  finally {
    pending.value = false;
  }
};
</script>
