<template>
    <div class="container mx-auto px-4 py-8 max-w-4xl">
        <!-- 页面标题 -->
        <div class="mb-8">
            <h1 class="text-3xl font-bold text-gray-900 dark:text-white">
                个人资料
            </h1>
            <p class="mt-2 text-gray-600 dark:text-gray-400">
                管理您的账户信息和偏好设置
            </p>
        </div>

        <div class="grid lg:grid-cols-3 gap-8">
            <!-- 侧边栏导航 -->
            <div class="lg:col-span-1">
                <div
                    class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6"
                >
                    <div class="flex items-center space-x-4 mb-6">
                        <div
                            class="w-16 h-16 bg-primary-100 dark:bg-primary-900 rounded-full flex items-center justify-center"
                        >
                            <UIcon
                                name="i-heroicons-user"
                                class="w-8 h-8 text-primary-600 dark:text-primary-400"
                            />
                        </div>
                        <div>
                            <h3
                                class="font-semibold text-gray-900 dark:text-white"
                            >
                                {{ user?.username }}
                            </h3>
                            <p class="text-sm text-gray-600 dark:text-gray-400">
                                {{ user?.email }}
                            </p>
                        </div>
                    </div>

                    <nav class="space-y-2">
                        <button
                            v-for="tab in tabs"
                            :key="tab.id"
                            @click="activeTab = tab.id"
                            :class="[
                                'w-full text-left px-3 py-2 rounded-md text-sm font-medium transition-colors',
                                activeTab === tab.id
                                    ? 'bg-primary-100 text-primary-700 dark:bg-primary-900 dark:text-primary-300'
                                    : 'text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800',
                            ]"
                        >
                            <UIcon
                                :name="tab.icon"
                                class="inline-block w-4 h-4 mr-2"
                            />
                            {{ tab.name }}
                        </button>
                    </nav>
                </div>
            </div>

            <!-- 主内容区域 -->
            <div class="lg:col-span-2">
                <!-- 基本信息 -->
                <div
                    v-if="activeTab === 'basic'"
                    class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6"
                >
                    <h2
                        class="text-xl font-semibold text-gray-900 dark:text-white mb-6"
                    >
                        基本信息
                    </h2>

                    <UForm
                        :state="profileForm"
                        class="space-y-6"
                        @submit="updateProfile"
                    >
                        <UFormGroup label="用户名" name="username">
                            <UInput v-model="profileForm.username" />
                        </UFormGroup>

                        <UFormGroup label="邮箱地址" name="email">
                            <UInput v-model="profileForm.email" type="email" />
                        </UFormGroup>

                        <UFormGroup label="个人简介" name="bio">
                            <UTextarea
                                v-model="profileForm.bio"
                                :rows="4"
                                placeholder="简单介绍一下自己..."
                            />
                        </UFormGroup>

                        <UFormGroup label="网站" name="website">
                            <UInput
                                v-model="profileForm.website"
                                placeholder="https://example.com"
                            />
                        </UFormGroup>

                        <div class="flex justify-end">
                            <UButton
                                type="submit"
                                color="primary"
                                :loading="saving"
                            >
                                保存更改
                            </UButton>
                        </div>
                    </UForm>
                </div>

                <!-- 安全设置 -->
                <div
                    v-if="activeTab === 'security'"
                    class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6"
                >
                    <h2
                        class="text-xl font-semibold text-gray-900 dark:text-white mb-6"
                    >
                        安全设置
                    </h2>

                    <UForm
                        :state="passwordForm"
                        class="space-y-6"
                        @submit="changePassword"
                    >
                        <UFormGroup label="当前密码" name="currentPassword">
                            <UInput
                                v-model="passwordForm.currentPassword"
                                type="password"
                            />
                        </UFormGroup>

                        <UFormGroup label="新密码" name="newPassword">
                            <UInput
                                v-model="passwordForm.newPassword"
                                type="password"
                            />
                        </UFormGroup>

                        <UFormGroup label="确认新密码" name="confirmPassword">
                            <UInput
                                v-model="passwordForm.confirmPassword"
                                type="password"
                            />
                        </UFormGroup>

                        <div class="flex justify-end">
                            <UButton
                                type="submit"
                                color="primary"
                                :loading="changingPassword"
                            >
                                更改密码
                            </UButton>
                        </div>
                    </UForm>
                </div>

                <!-- 偏好设置 -->
                <div
                    v-if="activeTab === 'preferences'"
                    class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6"
                >
                    <h2
                        class="text-xl font-semibold text-gray-900 dark:text-white mb-6"
                    >
                        偏好设置
                    </h2>

                    <div class="space-y-6">
                        <div class="flex items-center justify-between">
                            <div>
                                <h3
                                    class="font-medium text-gray-900 dark:text-white"
                                >
                                    深色模式
                                </h3>
                                <p
                                    class="text-sm text-gray-600 dark:text-gray-400"
                                >
                                    切换界面主题
                                </p>
                            </div>
                            <UToggle
                                v-model="isDarkMode"
                                @change="toggleTheme"
                            />
                        </div>

                        <div class="flex items-center justify-between">
                            <div>
                                <h3
                                    class="font-medium text-gray-900 dark:text-white"
                                >
                                    邮件通知
                                </h3>
                                <p
                                    class="text-sm text-gray-600 dark:text-gray-400"
                                >
                                    接收文章评论和系统通知
                                </p>
                            </div>
                            <UToggle
                                v-model="preferences.emailNotifications"
                                @change="updatePreferences"
                            />
                        </div>

                        <div class="flex items-center justify-between">
                            <div>
                                <h3
                                    class="font-medium text-gray-900 dark:text-white"
                                >
                                    公开个人资料
                                </h3>
                                <p
                                    class="text-sm text-gray-600 dark:text-gray-400"
                                >
                                    允许其他用户查看您的个人资料
                                </p>
                            </div>
                            <UToggle
                                v-model="preferences.publicProfile"
                                @change="updatePreferences"
                            />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
// 路由守卫
definePageMeta({
    middleware: ["auth"],
});

// SEO
useHead({
    title: "个人资料",
});

// 状态
const activeTab = ref("basic");
const saving = ref(false);
const changingPassword = ref(false);

const authStore = useAuthStore();
const colorMode = useColorMode();
const toast = useToast();

// 用户信息
const user = computed(() => authStore.user);

// 标签页配置
const tabs = [
    { id: "basic", name: "基本信息", icon: "i-heroicons-user" },
    { id: "security", name: "安全设置", icon: "i-heroicons-lock-closed" },
    { id: "preferences", name: "偏好设置", icon: "i-heroicons-cog-6-tooth" },
];

// 表单数据
const profileForm = reactive({
    username: user.value?.username || "",
    email: user.value?.email || "",
    bio: "",
    website: "",
});

const passwordForm = reactive({
    currentPassword: "",
    newPassword: "",
    confirmPassword: "",
});

const preferences = reactive({
    emailNotifications: true,
    publicProfile: true,
});

// 计算属性
const isDarkMode = computed({
    get: () => colorMode.value === "dark",
    set: (value) => {
        colorMode.preference = value ? "dark" : "light";
    },
});

// 方法
const updateProfile = async () => {
    saving.value = true;

    try {
        // TODO: 调用API更新用户资料
        await new Promise((resolve) => setTimeout(resolve, 1000)); // 模拟API调用

        toast.add({
            title: "保存成功",
            description: "个人资料已更新",
            color: "green",
        });
    } catch (error) {
        toast.add({
            title: "保存失败",
            description: "更新个人资料时发生错误",
            color: "red",
        });
    } finally {
        saving.value = false;
    }
};

const changePassword = async () => {
    if (passwordForm.newPassword !== passwordForm.confirmPassword) {
        toast.add({
            title: "密码不匹配",
            description: "新密码和确认密码不一致",
            color: "red",
        });
        return;
    }

    changingPassword.value = true;

    try {
        // TODO: 调用API更改密码
        await new Promise((resolve) => setTimeout(resolve, 1000)); // 模拟API调用

        // 清空表单
        passwordForm.currentPassword = "";
        passwordForm.newPassword = "";
        passwordForm.confirmPassword = "";

        toast.add({
            title: "密码更改成功",
            description: "您的密码已更新",
            color: "green",
        });
    } catch (error) {
        toast.add({
            title: "更改失败",
            description: "更改密码时发生错误",
            color: "red",
        });
    } finally {
        changingPassword.value = false;
    }
};

const toggleTheme = () => {
    // 主题切换已通过计算属性处理
};

const updatePreferences = async () => {
    try {
        // TODO: 调用API更新偏好设置
        await new Promise((resolve) => setTimeout(resolve, 500)); // 模拟API调用

        toast.add({
            title: "设置已保存",
            color: "green",
        });
    } catch (error) {
        toast.add({
            title: "保存失败",
            description: "更新设置时发生错误",
            color: "red",
        });
    }
};

// 初始化数据
onMounted(() => {
    if (user.value) {
        profileForm.username = user.value.username;
        profileForm.email = user.value.email;
    }
});
</script>
