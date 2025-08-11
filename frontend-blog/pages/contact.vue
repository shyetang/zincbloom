<template>
  <div class="container mx-auto px-4 py-8">
    <div class="max-w-4xl mx-auto">
      <h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-8">
        联系我们
      </h1>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
        <!-- 联系信息 -->
        <div>
          <h2 class="text-xl font-semibold mb-4">
            联系方式
          </h2>
          <div class="space-y-4">
            <div class="flex items-center">
              <UIcon
                name="i-heroicons-envelope"
                class="w-5 h-5 mr-3 text-primary-600"
              />
              <span>contact@zincbloom.com</span>
            </div>
            <div class="flex items-center">
              <UIcon
                name="i-heroicons-map-pin"
                class="w-5 h-5 mr-3 text-primary-600"
              />
              <span>北京市朝阳区</span>
            </div>
          </div>
        </div>

        <!-- 联系表单 -->
        <div>
          <h2 class="text-xl font-semibold mb-4">
            发送消息
          </h2>
          <form
            class="space-y-4"
            @submit.prevent="handleSubmit"
          >
            <UInput
              v-model="form.name"
              placeholder="您的姓名"
            />
            <UInput
              v-model="form.email"
              placeholder="邮箱地址"
              type="email"
            />
            <UTextarea
              v-model="form.message"
              placeholder="消息内容"
              :rows="4"
            />
            <UButton
              color="primary"
              block
              type="submit"
            >
              发送消息
            </UButton>
          </form>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const toast = useToast();

// 表单状态
const form = reactive({
  name: "",
  email: "",
  message: "",
});

// 表单提交处理
const handleSubmit = async () => {
  try {
    // TODO: 实现实际的表单提交逻辑
    toast.add({
      title: "发送成功",
      description: "我们已收到您的消息，会尽快回复您",
      color: "success",
    });

    // 重置表单
    form.name = "";
    form.email = "";
    form.message = "";
  }
  catch (error) {
    toast.add({
      title: "发送失败",
      description: "消息发送失败，请稍后重试",
      color: "error",
    });
  }
};

useHead({
  title: "联系我们",
  meta: [
    {
      name: "description",
      content: "联系ZincBloom团队，我们很乐意听取您的建议和反馈",
    },
  ],
});
</script>
