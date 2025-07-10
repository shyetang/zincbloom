<template>
    <div
        class="min-h-screen flex items-center justify-center bg-gray-50 dark:bg-gray-900 py-12 px-4 sm:px-6 lg:px-8"
    >
        <div class="max-w-md w-full space-y-8">
            <!-- 标题区域 -->
            <div class="text-center">
                <NuxtLink
                    to="/"
                    class="flex items-center justify-center space-x-2 mb-6"
                >
                    <UIcon
                        name="i-heroicons-sparkles"
                        class="w-10 h-10 text-primary-600"
                    />
                    <span
                        class="text-2xl font-bold text-gray-900 dark:text-white"
                        >ZincBloom</span
                    >
                </NuxtLink>
                <h2 class="text-3xl font-bold text-gray-900 dark:text-white">
                    登录您的账户
                </h2>
                <p class="mt-2 text-sm text-gray-600 dark:text-gray-400">
                    还没有账户？
                    <NuxtLink
                        to="/auth/register"
                        class="font-medium text-primary-600 hover:text-primary-500"
                    >
                        立即注册
                    </NuxtLink>
                </p>
            </div>

            <!-- 登录表单 -->
            <UCard class="mt-8">
                <UForm
                    ref="formRef"
                    :schema="loginSchema"
                    :state="formData"
                    class="space-y-6"
                    @submit="handleSubmit"
                >
                    <!-- 用户名 -->
                    <UFormGroup
                        label="用户名"
                        name="username"
                        help="请输入您的用户名或邮箱"
                    >
                        <UInput
                            v-model="formData.username"
                            icon="i-heroicons-user"
                            placeholder="用户名或邮箱"
                            size="lg"
                            :disabled="loading"
                        />
                    </UFormGroup>

                    <!-- 密码 -->
                    <UFormGroup label="密码" name="password">
                        <UInput
                            v-model="formData.password"
                            type="password"
                            icon="i-heroicons-lock-closed"
                            placeholder="请输入密码"
                            size="lg"
                            :disabled="loading"
                        />
                    </UFormGroup>

                    <!-- 记住我和忘记密码 -->
                    <div class="flex items-center justify-between">
                        <UCheckbox
                            v-model="formData.remember"
                            label="记住我"
                            :disabled="loading"
                        />
                        <NuxtLink
                            to="/auth/forgot-password"
                            class="text-sm text-primary-600 hover:text-primary-500"
                        >
                            忘记密码？
                        </NuxtLink>
                    </div>

                    <!-- 提交按钮 -->
                    <UButton
                        type="submit"
                        color="primary"
                        size="lg"
                        block
                        :loading="loading"
                        :disabled="loading"
                    >
                        {{ loading ? "登录中..." : "登录" }}
                    </UButton>
                </UForm>
            </UCard>

            <!-- 其他登录方式（未来扩展） -->
            <!-- 
            <div class="mt-6">
                <div class="relative">
                    <div class="absolute inset-0 flex items-center">
                        <div class="w-full border-t border-gray-300 dark:border-gray-600" />
                    </div>
                    <div class="relative flex justify-center text-sm">
                        <span class="px-2 bg-gray-50 dark:bg-gray-900 text-gray-500">或者</span>
                    </div>
                </div>

                <div class="mt-6 grid grid-cols-1 gap-3">
                    <UButton
                        variant="outline"
                        color="gray"
                        size="lg"
                        block
                        icon="i-simple-icons-github"
                    >
                        使用 GitHub 登录
                    </UButton>
                </div>
            </div>
            -->
        </div>
    </div>
</template>

<script setup lang="ts">
import { z } from "zod";
import type { FormSubmitEvent } from '~/types/ui'

// 禁用默认布局
definePageMeta({
    layout: false,
});

// SEO 配置
useHead({
    title: "登录",
    meta: [{ name: "description", content: "登录您的 ZincBloom 账户" }],
});

// 状态管理
const authStore = useAuthStore();
const toast = useToast();
const router = useRouter();
const route = useRoute();

// 响应式数据
const loading = ref(false);
const formRef = ref();

// 表单数据
const formData = reactive({
    username: "",
    password: "",
    remember: false,
});

// 表单验证规则
const loginSchema = z.object({
    username: z
        .string()
        .min(1, "请输入用户名或邮箱")
        .max(100, "用户名长度不能超过100个字符"),
    password: z
        .string()
        .min(1, "请输入密码")
        .min(6, "密码长度至少6个字符")
        .max(100, "密码长度不能超过100个字符"),
});

// 处理表单提交
const handleSubmit = async (event: FormSubmitEvent<any>) => {
    if (loading.value) return;

    loading.value = true;

    try {
        const result = await authStore.login({
            username: formData.username,
            password: formData.password,
        });

        if (result.success) {
            toast.add({
                title: "登录成功",
                description: "欢迎回来！",
                icon: "i-heroicons-check-circle",
                color: "success",
            });

            // 跳转到目标页面或首页
            const redirectTo = (route.query.redirect as string) || "/";
            await router.push(redirectTo);
        } else {
            toast.add({
                title: "登录失败",
                description: result.message || "用户名或密码错误",
                icon: "i-heroicons-x-circle",
                color: "error",
            });
        }
    } catch (error: any) {
        console.error("登录错误:", error);
        toast.add({
            title: "登录失败",
            description: "网络错误，请稍后重试",
            icon: "i-heroicons-x-circle",
            color: "error",
        });
    } finally {
        loading.value = false;
    }
};

// 如果已经登录，重定向到首页
onMounted(() => {
    if (authStore.isAuthenticated) {
        router.push("/");
    }
});
</script>
