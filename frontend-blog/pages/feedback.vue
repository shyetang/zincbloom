<template>
  <div class="container mx-auto px-4 py-8">
    <div class="max-w-2xl mx-auto">
      <h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-8">
        意见反馈
      </h1>

      <div class="bg-white dark:bg-gray-800 rounded-lg p-6 shadow-sm">
        <p class="text-gray-600 dark:text-gray-400 mb-6">
          您的反馈对我们非常重要！请告诉我们您的使用体验、建议或遇到的问题。
        </p>

        <form
          class="space-y-6"
          @submit.prevent="handleSubmit"
        >
          <!-- 反馈类型 -->
          <div>
            <label
              class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
            >
              反馈类型
            </label>
            <USelect
              v-model="feedbackForm.type"
              :options="feedbackTypes"
              placeholder="请选择反馈类型"
            />
          </div>

          <!-- 联系信息 -->
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <UInput
              v-model="feedbackForm.name"
              placeholder="您的姓名（可选）"
            />
            <UInput
              v-model="feedbackForm.email"
              placeholder="邮箱地址（可选）"
              type="email"
            />
          </div>

          <!-- 反馈内容 -->
          <div>
            <label
              class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
            >
              详细描述
            </label>
            <UTextarea
              v-model="feedbackForm.message"
              placeholder="请详细描述您的问题、建议或意见..."
              :rows="6"
            />
          </div>

          <!-- 提交按钮 -->
          <UButton
            color="primary"
            size="lg"
            block
            type="submit"
            :loading="submitting"
          >
            提交反馈
          </UButton>
        </form>
      </div>

      <!-- 其他反馈方式 -->
      <div class="mt-8 bg-gray-50 dark:bg-gray-800/50 rounded-lg p-6">
        <h2 class="text-lg font-semibold mb-4">
          其他反馈方式
        </h2>
        <div class="space-y-3">
          <div class="flex items-center">
            <UIcon
              name="i-heroicons-envelope"
              class="w-5 h-5 mr-3 text-primary-600"
            />
            <span>邮箱：feedback@zincbloom.com</span>
          </div>
          <div class="flex items-center">
            <UIcon
              name="i-heroicons-chat-bubble-left-right"
              class="w-5 h-5 mr-3 text-primary-600"
            />
            <span>在线客服：工作日 9:00-18:00</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const toast = useToast();

const feedbackTypes = [
  { label: "功能建议", value: "suggestion" },
  { label: "错误报告", value: "bug" },
  { label: "使用问题", value: "question" },
  { label: "内容投诉", value: "complaint" },
  { label: "其他", value: "other" },
];

// 表单状态
const feedbackForm = reactive({
  type: "",
  name: "",
  email: "",
  message: "",
});

const submitting = ref(false);

// 表单提交处理
const handleSubmit = async () => {
  if (!feedbackForm.message.trim()) {
    toast.add({
      title: "提交失败",
      description: "请填写反馈内容",
      color: "error",
    });
    return;
  }

  submitting.value = true;

  try {
    // TODO: 实现实际的反馈提交逻辑
    await new Promise(resolve => setTimeout(resolve, 1000)); // 模拟网络请求

    toast.add({
      title: "提交成功",
      description: "感谢您的反馈，我们会认真考虑您的建议",
      color: "success",
    });

    // 重置表单
    feedbackForm.type = "";
    feedbackForm.name = "";
    feedbackForm.email = "";
    feedbackForm.message = "";
  }
  catch (error) {
    toast.add({
      title: "提交失败",
      description: "反馈提交失败，请稍后重试",
      color: "error",
    });
  }
  finally {
    submitting.value = false;
  }
};

useHead({
  title: "意见反馈",
  meta: [
    {
      name: "description",
      content: "向ZincBloom团队提交您的意见和建议，帮助我们改进产品",
    },
  ],
});
</script>
