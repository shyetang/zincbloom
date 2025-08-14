<template>
  <div
    class="min-h-screen relative overflow-hidden editor-page-modern"
    style="
      background: linear-gradient(
        135deg,
        #f7fafc 0%,
        #e0f7fa 50%,
        #e8f5e8 100%
      ) !important;
      position: relative !important;
      overflow: hidden !important;
      min-height: 100vh !important;
    "
  >
    <!-- 背景装饰元素 -->
    <div class="absolute inset-0 overflow-hidden pointer-events-none">
      <!-- 动态渐变球体 -->
      <div
        class="absolute -top-40 -right-40 w-80 h-80 bg-gradient-to-r from-purple-400/20 via-pink-400/20 to-red-400/20 rounded-full blur-3xl animate-float"
      />
      <div
        class="absolute -bottom-40 -left-40 w-96 h-96 bg-gradient-to-r from-blue-400/15 via-cyan-400/15 to-teal-400/15 rounded-full blur-3xl animate-float"
        style="animation-delay: -2s"
      />
      <div
        class="absolute top-1/3 left-1/4 w-60 h-60 bg-gradient-to-r from-emerald-400/10 via-green-400/10 to-lime-400/10 rounded-full blur-3xl animate-float"
        style="animation-delay: -4s"
      />

      <!-- 网格背景 -->
      <div
        class="absolute inset-0 bg-grid-pattern opacity-[0.02] dark:opacity-[0.05]"
      />

      <!-- 光晕效果 -->
      <div
        class="absolute top-0 left-1/2 transform -translate-x-1/2 w-full h-px bg-gradient-to-r from-transparent via-purple-500/30 to-transparent"
      />
    </div>
    <!-- 加载状态 -->
    <div
      v-if="pending"
      class="flex items-center justify-center min-h-screen"
    >
      <div class="text-center">
        <div class="relative">
          <div
            class="w-20 h-20 mx-auto mb-6 bg-gradient-to-r from-blue-500 via-purple-600 to-pink-500 rounded-full flex items-center justify-center shadow-2xl animate-spin"
          >
            <div
              class="w-16 h-16 bg-white dark:bg-gray-900 rounded-full flex items-center justify-center"
            >
              <UIcon
                name="i-heroicons-pencil-square"
                class="w-8 h-8 text-blue-500"
              />
            </div>
          </div>
        </div>
        <p
          class="text-lg font-medium text-gray-600 dark:text-gray-400 animate-pulse"
        >
          正在加载编辑器...
        </p>
      </div>
    </div>

    <!-- 错误状态 -->
    <div
      v-else-if="error"
      class="flex items-center justify-center min-h-screen p-6"
    >
      <div class="text-center max-w-md">
        <div
          class="bg-gradient-to-br from-red-50 to-rose-100 dark:from-red-950/20 dark:to-rose-950/20 rounded-3xl p-12 shadow-2xl border border-red-100 dark:border-red-800/30"
        >
          <UIcon
            name="i-heroicons-exclamation-triangle"
            class="w-20 h-20 text-red-500 mx-auto mb-6"
          />
          <h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-4">
            文章不存在
          </h1>
          <p
            class="text-gray-600 dark:text-gray-400 mb-8 leading-relaxed text-lg"
          >
            您要编辑的文章不存在或您没有权限访问。
          </p>
          <UButton
            to="/write"
            color="primary"
            size="xl"
            class="shadow-xl hover:shadow-2xl transition-all duration-300 transform hover:scale-105 px-8 py-4"
          >
            <UIcon
              name="i-heroicons-arrow-left"
              class="w-5 h-5 mr-3"
            />
            返回写作页面
          </UButton>
        </div>
      </div>
    </div>

    <!-- 主编辑界面 -->
    <div
      v-else
      class="flex flex-col h-screen"
    >
      <!-- 顶部导航栏 -->
      <div
        class="sticky top-0 z-50 backdrop-blur-3xl bg-white/80 dark:bg-slate-900/80 border-b border-white/20 dark:border-slate-700/50 shadow-2xl shadow-purple-500/10 dark:shadow-purple-500/20"
        style="
          backdrop-filter: blur(64px) saturate(200%) brightness(1.1) !important;
          background: rgba(255, 255, 255, 0.9) !important;
          border-bottom: 1px solid rgba(139, 92, 246, 0.2) !important;
          box-shadow: 0 25px 50px -12px rgba(139, 92, 246, 0.25) !important;
          position: sticky !important;
          top: 0 !important;
          z-index: 50 !important;
        "
      >
        <!-- 导航栏装饰 -->
        <div
          class="absolute inset-0 bg-gradient-to-r from-purple-500/5 via-blue-500/5 to-emerald-500/5 dark:from-purple-500/10 dark:via-blue-500/10 dark:to-emerald-500/10"
        />
        <div class="relative max-w-full px-8 py-6">
          <div class="flex items-center justify-between">
            <!-- 左侧导航区域 -->
            <div class="flex items-center space-x-8">
              <UButton
                variant="ghost"
                size="lg"
                class="glass-button text-slate-600 dark:text-slate-300 hover:text-white hover:bg-gradient-to-r hover:from-purple-500 hover:to-blue-500 border border-slate-200/50 dark:border-slate-700/50 hover:border-transparent shadow-lg hover:shadow-xl transition-all duration-300 transform hover:scale-105 px-6 py-3 rounded-xl"
                style="
                  backdrop-filter: blur(16px) saturate(180%) !important;
                  background: rgba(255, 255, 255, 0.8) !important;
                  border: 2px solid rgba(255, 255, 255, 0.3) !important;
                  box-shadow: 0 10px 15px -3px rgba(139, 92, 246, 0.1) !important;
                  transition: all 0.3s cubic-bezier(0.25, 0.46, 0.45, 0.94) !important;
                  border-radius: 0.75rem !important;
                  padding: 0.75rem 1.5rem !important;
                "
                @click="goBack"
              >
                <UIcon
                  name="i-heroicons-arrow-left"
                  class="w-5 h-5 mr-2"
                />
                <span class="font-semibold">返回</span>
              </UButton>

              <!-- 文档信息 -->
              <div class="hidden md:flex items-center space-x-6">
                <div class="flex items-center space-x-3">
                  <div class="flex items-center space-x-2">
                    <div class="relative">
                      <div
                        class="w-3 h-3 rounded-full bg-gradient-to-r from-emerald-400 via-green-400 to-teal-400 animate-pulse shadow-lg shadow-emerald-500/50"
                      />
                      <div
                        class="absolute inset-0 w-3 h-3 rounded-full bg-emerald-400 animate-ping opacity-75"
                      />
                      <div
                        class="absolute inset-0 w-3 h-3 rounded-full bg-gradient-to-r from-emerald-300 to-green-300 animate-pulse"
                        style="animation-delay: 0.5s"
                      />
                    </div>
                    <span
                      class="text-sm font-semibold bg-gradient-to-r from-emerald-600 to-green-600 dark:from-emerald-400 dark:to-green-400 bg-clip-text text-transparent"
                    >
                      {{
                        saving
                          ? '保存中...'
                          : lastSaved
                            ? `${formatDate(lastSaved)} 已保存`
                            : '实时保存'
                      }}
                    </span>
                  </div>
                </div>

                <!-- 文档统计概览 -->
                <div
                  class="flex items-center space-x-6 text-sm text-gray-500 dark:text-gray-400"
                >
                  <div
                    class="flex items-center space-x-2 bg-blue-500/10 dark:bg-blue-500/20 px-3 py-1.5 rounded-full border border-blue-200/50 dark:border-blue-400/30"
                  >
                    <UIcon
                      name="i-heroicons-document-text"
                      class="w-4 h-4 text-blue-600 dark:text-blue-400"
                    />
                    <span
                      class="font-semibold text-blue-700 dark:text-blue-300"
                    >{{
                      (post?.content_markdown || '').length.toLocaleString()
                    }}</span>
                    <span class="text-xs text-blue-600/80 dark:text-blue-400/80">字符</span>
                  </div>
                  <div
                    class="flex items-center space-x-2 bg-purple-500/10 dark:bg-purple-500/20 px-3 py-1.5 rounded-full border border-purple-200/50 dark:border-purple-400/30"
                  >
                    <UIcon
                      name="i-heroicons-clock"
                      class="w-4 h-4 text-purple-600 dark:text-purple-400"
                    />
                    <span
                      class="font-semibold text-purple-700 dark:text-purple-300"
                    >{{
                      Math.ceil((post?.content_markdown || '').length / 400)
                    }}</span>
                    <span
                      class="text-xs text-purple-600/80 dark:text-purple-400/80"
                    >分钟</span>
                  </div>
                </div>
              </div>
            </div>

            <!-- 右侧操作按钮组 -->
            <div class="flex items-center space-x-4">
              <!-- 保存草稿 -->
              <UButton
                variant="outline"
                size="lg"
                :loading="saving"
                class="glass-button border-2 border-slate-300/50 dark:border-slate-600/50 hover:border-slate-400/70 dark:hover:border-slate-500/70 text-slate-700 dark:text-slate-200 hover:text-slate-900 dark:hover:text-white px-6 py-3 font-semibold transition-all duration-300 transform hover:scale-105 shadow-lg hover:shadow-xl backdrop-blur-sm rounded-xl"
                @click="() => saveAsDraft()"
              >
                <UIcon
                  name="i-heroicons-document-arrow-down"
                  class="w-5 h-5 mr-2"
                />
                <span class="hidden sm:inline">保存草稿</span>
              </UButton>

              <!-- 并排预览 -->
              <UButton
                :variant="showSideBySide ? 'solid' : 'outline'"
                size="lg"
                class="hidden lg:inline-flex px-6 py-3 font-semibold transition-all duration-300 transform hover:scale-105 rounded-xl shadow-lg hover:shadow-xl"
                :class="
                  showSideBySide
                    ? 'bg-gradient-to-r from-blue-500 via-cyan-500 to-teal-500 hover:from-blue-600 hover:via-cyan-600 hover:to-teal-600 text-white shadow-blue-500/25 hover:shadow-blue-500/40'
                    : 'glass-button border-2 border-blue-300/50 dark:border-blue-600/50 text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300 hover:bg-blue-50/80 dark:hover:bg-blue-950/50 hover:border-blue-400/70 dark:hover:border-blue-500/70'
                "
                @click="showSideBySide = !showSideBySide"
              >
                <UIcon
                  name="i-heroicons-squares-2x2"
                  class="w-5 h-5 mr-2"
                />
                并排预览
              </UButton>

              <!-- 全屏预览 -->
              <UButton
                :variant="showPreview ? 'solid' : 'outline'"
                size="lg"
                class="px-6 py-3 font-semibold transition-all duration-300 transform hover:scale-105 rounded-xl shadow-lg hover:shadow-xl"
                :class="
                  showPreview
                    ? 'bg-gradient-to-r from-purple-500 via-pink-500 to-rose-500 hover:from-purple-600 hover:via-pink-600 hover:to-rose-600 text-white shadow-purple-500/25 hover:shadow-purple-500/40'
                    : 'glass-button border-2 border-purple-300/50 dark:border-purple-600/50 text-purple-600 dark:text-purple-400 hover:text-purple-700 dark:hover:text-purple-300 hover:bg-purple-50/80 dark:hover:bg-purple-950/50 hover:border-purple-400/70 dark:hover:border-purple-500/70'
                "
                @click="togglePreview"
              >
                <UIcon
                  :name="showPreview ? 'i-heroicons-pencil' : 'i-heroicons-eye'"
                  class="w-5 h-5 mr-2"
                />
                {{ showPreview ? '编辑' : '预览' }}
              </UButton>

              <!-- 发布按钮 -->
              <UButton
                size="lg"
                :loading="publishing"
                class="bg-gradient-to-r from-emerald-500 via-green-500 to-teal-500 hover:from-emerald-600 hover:via-green-600 hover:to-teal-600 text-white shadow-xl hover:shadow-2xl shadow-emerald-500/25 hover:shadow-emerald-500/40 transition-all duration-300 transform hover:scale-105 px-8 py-3 font-semibold rounded-xl relative overflow-hidden"
                @click="publishPost"
              >
                <div
                  class="absolute inset-0 bg-gradient-to-r from-white/20 via-transparent to-white/20 translate-x-[-100%] hover:translate-x-[100%] transition-transform duration-700"
                />
                <UIcon
                  :name="
                    post?.status === 'published'
                      ? 'i-heroicons-arrow-path'
                      : 'i-heroicons-rocket-launch'
                  "
                  class="w-5 h-5 mr-2 relative z-10"
                />
                <span class="relative z-10">{{
                  post?.status === 'published' ? '更新文章' : '发布文章'
                }}</span>
              </UButton>
            </div>
          </div>
        </div>
      </div>

      <!-- 主内容区域 -->
      <div class="flex flex-1 overflow-hidden">
        <!-- 编辑器主区域 -->
        <div class="flex-1 flex flex-col overflow-hidden">
          <!-- 编辑模式 -->
          <div
            v-if="!showPreview"
            class="flex-1 flex flex-col overflow-hidden"
          >
            <!-- 标题输入区域 -->
            <div
              class="border-b border-white/30 dark:border-slate-700/50 bg-gradient-to-r from-white/90 via-slate-50/90 to-white/90 dark:from-slate-900/90 dark:via-slate-800/90 dark:to-slate-900/90 backdrop-blur-2xl relative"
            >
              <!-- 标题区域装饰 -->
              <div
                class="absolute inset-0 bg-gradient-to-r from-purple-500/5 via-blue-500/5 to-emerald-500/5 dark:from-purple-500/10 dark:via-blue-500/10 dark:to-emerald-500/10"
              />
              <div
                class="absolute top-0 left-0 right-0 h-px bg-gradient-to-r from-transparent via-purple-400/30 to-transparent"
              />

              <div class="relative max-w-6xl mx-auto px-12 py-10">
                <!-- 标题输入框 -->
                <div class="relative group">
                  <UInput
                    v-model="post.title"
                    placeholder="在此输入您的标题，让它发光发热..."
                    size="xl"
                    variant="none"
                    class="title-input text-4xl md:text-5xl font-bold bg-gradient-to-r from-slate-900 via-purple-900 to-slate-900 dark:from-white dark:via-purple-100 dark:to-white bg-clip-text text-transparent placeholder-slate-400 dark:placeholder-slate-500 bg-transparent border-none focus:ring-0 px-0 leading-tight transition-all duration-500"
                    @input="handleTitleChange"
                  />
                  <!-- 标题下划线 -->
                  <div
                    class="absolute bottom-0 left-0 h-1 bg-gradient-to-r from-purple-500 via-blue-500 to-emerald-500 transform scale-x-0 transition-transform duration-500 origin-left group-focus-within:scale-x-100 rounded-full"
                  />
                </div>

                <!-- 副标题 -->
                <div
                  class="mt-4 flex items-center space-x-2 text-sm text-slate-500 dark:text-slate-400"
                >
                  <UIcon
                    name="i-heroicons-sparkles"
                    class="w-4 h-4"
                  />
                  <span>让您的标题成为读者的第一眼吸引</span>
                </div>
              </div>
            </div>

            <!-- 专业编辑器区域 -->
            <div
              class="flex-1 overflow-hidden relative bg-gradient-to-br from-white via-blue-50/20 to-indigo-50/30 dark:from-gray-900 dark:via-blue-950/10 dark:to-indigo-950/20"
            >
              <!-- 编辑器主体 -->
              <div class="relative h-full flex">
                <!-- 左侧编辑区域 -->
                <div class="flex-1 flex flex-col">
                  <!-- 增强版工具栏 -->
                  <div
                    class="flex-shrink-0 bg-gradient-to-r from-white/95 via-slate-50/95 to-white/95 dark:from-slate-900/95 dark:via-slate-800/95 dark:to-slate-900/95 backdrop-blur-2xl border-b border-white/30 dark:border-slate-700/50 px-8 py-6 shadow-lg shadow-purple-500/10 dark:shadow-purple-500/20 relative"
                  >
                    <!-- 工具栏装饰 -->
                    <div
                      class="absolute inset-0 bg-gradient-to-r from-purple-500/5 via-blue-500/5 to-emerald-500/5 dark:from-purple-500/10 dark:via-blue-500/10 dark:to-emerald-500/10"
                    />
                    <div class="relative flex items-center justify-between">
                      <!-- 格式化工具 -->
                      <div class="flex items-center space-x-3">
                        <div
                          class="flex items-center bg-gradient-to-r from-slate-100/90 via-white/90 to-slate-100/90 dark:from-slate-800/90 dark:via-slate-700/90 dark:to-slate-800/90 rounded-2xl p-2 space-x-1 border border-slate-200/50 dark:border-slate-600/50 shadow-lg shadow-slate-500/10 dark:shadow-slate-500/20 backdrop-blur-sm"
                        >
                          <button
                            class="tool-button flex items-center space-x-2 px-4 py-3 text-sm bg-transparent hover:bg-gradient-to-r hover:from-purple-500 hover:to-blue-500 hover:text-white text-slate-600 dark:text-slate-300 rounded-xl transition-all duration-300 transform hover:scale-105 shadow-sm hover:shadow-md group"
                            title="粗体 (Ctrl+B)"
                            @click="insertMarkdown('**', '**')"
                          >
                            <UIcon
                              name="i-heroicons-bold"
                              class="w-4 h-4 group-hover:scale-110 transition-transform"
                            />
                            <span class="hidden xl:inline font-semibold">粗体</span>
                          </button>
                          <button
                            class="tool-button flex items-center space-x-2 px-4 py-3 text-sm bg-transparent hover:bg-gradient-to-r hover:from-purple-500 hover:to-blue-500 hover:text-white text-slate-600 dark:text-slate-300 rounded-xl transition-all duration-300 transform hover:scale-105 shadow-sm hover:shadow-md group"
                            title="斜体 (Ctrl+I)"
                            @click="insertMarkdown('*', '*')"
                          >
                            <UIcon
                              name="i-heroicons-italic"
                              class="w-4 h-4 group-hover:scale-110 transition-transform"
                            />
                            <span class="hidden xl:inline font-semibold">斜体</span>
                          </button>
                          <button
                            class="tool-button flex items-center space-x-2 px-4 py-3 text-sm bg-transparent hover:bg-gradient-to-r hover:from-purple-500 hover:to-blue-500 hover:text-white text-slate-600 dark:text-slate-300 rounded-xl transition-all duration-300 transform hover:scale-105 shadow-sm hover:shadow-md group"
                            title="行内代码"
                            @click="insertMarkdown('`', '`')"
                          >
                            <UIcon
                              name="i-heroicons-code-bracket-square"
                              class="w-4 h-4 group-hover:scale-110 transition-transform"
                            />
                            <span class="hidden xl:inline font-semibold">代码</span>
                          </button>
                        </div>

                        <div
                          class="flex items-center bg-gradient-to-r from-slate-100/90 via-white/90 to-slate-100/90 dark:from-slate-800/90 dark:via-slate-700/90 dark:to-slate-800/90 rounded-2xl p-2 space-x-1 border border-slate-200/50 dark:border-slate-600/50 shadow-lg shadow-slate-500/10 dark:shadow-slate-500/20 backdrop-blur-sm"
                        >
                          <button
                            class="flex items-center space-x-2 px-4 py-2.5 text-sm bg-transparent hover:bg-white dark:hover:bg-gray-700 rounded-lg transition-all duration-200 group"
                            title="二级标题"
                            @click="insertMarkdown('\\n## ', '')"
                          >
                            <UIcon
                              name="i-heroicons-hashtag"
                              class="w-4 h-4 group-hover:scale-110 transition-transform"
                            />
                            <span class="hidden xl:inline font-medium">标题</span>
                          </button>
                          <button
                            class="flex items-center space-x-2 px-4 py-2.5 text-sm bg-transparent hover:bg-white dark:hover:bg-gray-700 rounded-lg transition-all duration-200 group"
                            title="无序列表"
                            @click="insertMarkdown('\\n- ', '')"
                          >
                            <UIcon
                              name="i-heroicons-list-bullet"
                              class="w-4 h-4 group-hover:scale-110 transition-transform"
                            />
                            <span class="hidden xl:inline font-medium">列表</span>
                          </button>
                          <button
                            class="flex items-center space-x-2 px-4 py-2.5 text-sm bg-transparent hover:bg-white dark:hover:bg-gray-700 rounded-lg transition-all duration-200 group"
                            title="引用块"
                            @click="insertMarkdown('\\n> ', '')"
                          >
                            <UIcon
                              name="i-heroicons-chat-bubble-left-ellipsis"
                              class="w-4 h-4 group-hover:scale-110 transition-transform"
                            />
                            <span class="hidden xl:inline font-medium">引用</span>
                          </button>
                        </div>

                        <div
                          class="flex items-center bg-gradient-to-r from-slate-100/90 via-white/90 to-slate-100/90 dark:from-slate-800/90 dark:via-slate-700/90 dark:to-slate-800/90 rounded-2xl p-2 space-x-1 border border-slate-200/50 dark:border-slate-600/50 shadow-lg shadow-slate-500/10 dark:shadow-slate-500/20 backdrop-blur-sm"
                        >
                          <button
                            class="flex items-center space-x-2 px-4 py-2.5 text-sm bg-transparent hover:bg-white dark:hover:bg-gray-700 rounded-lg transition-all duration-200 group"
                            title="插入链接"
                            @click="insertMarkdown('[', ']()')"
                          >
                            <UIcon
                              name="i-heroicons-link"
                              class="w-4 h-4 group-hover:scale-110 transition-transform"
                            />
                            <span class="hidden xl:inline font-medium">链接</span>
                          </button>
                          <button
                            class="flex items-center space-x-2 px-4 py-2.5 text-sm bg-transparent hover:bg-white dark:hover:bg-gray-700 rounded-lg transition-all duration-200 group"
                            title="插入图片"
                            @click="insertMarkdown('![', ']()')"
                          >
                            <UIcon
                              name="i-heroicons-photo"
                              class="w-4 h-4 group-hover:scale-110 transition-transform"
                            />
                            <span class="hidden xl:inline font-medium">图片</span>
                          </button>
                        </div>
                      </div>

                      <!-- 编辑器状态 -->
                      <div
                        class="flex items-center space-x-6 text-sm text-gray-600 dark:text-gray-400"
                      >
                        <div class="flex items-center space-x-3">
                          <div class="flex items-center space-x-2">
                            <div
                              class="w-2.5 h-2.5 bg-gradient-to-r from-green-400 to-emerald-500 rounded-full animate-pulse shadow-lg"
                            />
                            <span
                              class="font-medium text-emerald-600 dark:text-emerald-400"
                            >实时保存</span>
                          </div>
                          <div class="w-px h-4 bg-gray-300 dark:bg-gray-600" />
                          <div class="flex items-center space-x-2">
                            <UIcon
                              name="i-heroicons-document-text"
                              class="w-4 h-4 text-blue-500"
                            />
                            <span class="font-medium">Markdown</span>
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>

                  <!-- 编辑文本区域 -->
                  <div class="flex-1 relative overflow-hidden">
                    <!-- 背景装饰 -->
                    <div
                      class="absolute inset-0 bg-gradient-to-br from-white via-slate-50/50 to-blue-50/30 dark:from-gray-900 dark:via-gray-900/90 dark:to-blue-950/20"
                    />

                    <!-- 编辑器主体 -->
                    <div class="relative h-full">
                      <UTextarea
                        ref="editorRef"
                        v-model="post.content_markdown"
                        placeholder="# 开始您的创作之旅

欢迎来到您的专属写作空间！在这里，您可以自由地表达想法、分享见解、记录生活。

## ✨ 支持的 Markdown 语法

- **粗体文字** 和 *斜体文字*
- [链接文字](https://example.com)
- ![图片描述](图片链接)
- `行内代码` 和代码块
- > 优雅的引用块
- 有序和无序列表

## 🎯 写作小贴士

1. 保持段落简洁明了
2. 使用标题构建清晰的文章结构
3. 适当添加链接和图片丰富内容
4. 善用引用块突出重要观点

现在，让我们开始创作吧！每一个字都承载着您的思想，每一句话都可能触动读者的心灵。✨"
                        class="professional-editor h-full w-full resize-none border-none focus:ring-0 focus:outline-none bg-transparent text-gray-800 dark:text-gray-100"
                        :style="{
                          fontSize: '18px',
                          lineHeight: '1.8',
                          fontFamily:
                            '-apple-system, BlinkMacSystemFont, Segoe UI, Roboto, SF Pro Text, Inter, sans-serif',
                          padding: '4rem 5rem',
                          minHeight: '100%',
                          fontWeight: '400',
                          letterSpacing: '0.01em',
                        }"
                        autofocus
                        @input="handleContentChange"
                        @keydown="handleKeydown"
                      />

                      <!-- 编辑器装饰效果 -->
                      <div
                        class="absolute inset-0 pointer-events-none overflow-hidden"
                      >
                        <!-- 顶部装饰线 -->
                        <div
                          class="absolute top-0 left-20 right-20 h-px bg-gradient-to-r from-transparent via-blue-400/30 to-transparent animate-pulse"
                        />
                        <!-- 底部装饰线 -->
                        <div
                          class="absolute bottom-0 left-20 right-20 h-px bg-gradient-to-r from-transparent via-purple-400/30 to-transparent animate-pulse"
                          style="animation-delay: 1s"
                        />
                        <!-- 左侧装饰线 -->
                        <div
                          class="absolute top-16 left-20 bottom-16 w-px bg-gradient-to-b from-blue-400/20 via-purple-400/20 to-pink-400/20 opacity-60"
                        />
                        <!-- 右上角装饰 -->
                        <div
                          class="absolute top-8 right-8 w-32 h-32 bg-gradient-to-br from-blue-400/5 to-purple-400/5 rounded-full animate-pulse"
                          style="animation-duration: 4s"
                        />
                      </div>
                    </div>
                  </div>

                  <!-- 底部状态栏 -->
                  <div
                    class="flex-shrink-0 bg-white/95 dark:bg-gray-900/95 backdrop-blur-xl border-t border-gray-200/60 dark:border-gray-700/60 px-8 py-5 shadow-lg"
                  >
                    <div class="flex items-center justify-between">
                      <div class="flex items-center space-x-10">
                        <div
                          class="grid grid-cols-2 md:grid-cols-4 gap-8 text-sm"
                        >
                          <div class="flex items-center space-x-2">
                            <div
                              class="w-8 h-8 bg-gradient-to-br from-blue-500 to-blue-600 rounded-lg flex items-center justify-center shadow-lg"
                            >
                              <UIcon
                                name="i-heroicons-document-text"
                                class="w-4 h-4 text-white"
                              />
                            </div>
                            <div>
                              <div
                                class="font-semibold text-gray-900 dark:text-white"
                              >
                                {{
                                  (
                                    post?.content_markdown || ''
                                  ).length.toLocaleString()
                                }}
                              </div>
                              <div
                                class="text-xs text-gray-500 dark:text-gray-400"
                              >
                                字符
                              </div>
                            </div>
                          </div>

                          <div class="flex items-center space-x-2">
                            <div
                              class="w-8 h-8 bg-gradient-to-br from-emerald-500 to-emerald-600 rounded-lg flex items-center justify-center shadow-lg"
                            >
                              <UIcon
                                name="i-heroicons-bars-3-bottom-left"
                                class="w-4 h-4 text-white"
                              />
                            </div>
                            <div>
                              <div
                                class="font-semibold text-gray-900 dark:text-white"
                              >
                                {{
                                  (post?.content_markdown || '').split('\\n')
                                    .length
                                }}
                              </div>
                              <div
                                class="text-xs text-gray-500 dark:text-gray-400"
                              >
                                行数
                              </div>
                            </div>
                          </div>

                          <div class="flex items-center space-x-2">
                            <div
                              class="w-8 h-8 bg-gradient-to-br from-purple-500 to-purple-600 rounded-lg flex items-center justify-center shadow-lg"
                            >
                              <UIcon
                                name="i-heroicons-clock"
                                class="w-4 h-4 text-white"
                              />
                            </div>
                            <div>
                              <div
                                class="font-semibold text-gray-900 dark:text-white"
                              >
                                {{
                                  Math.ceil(
                                    (post?.content_markdown || '').length / 400,
                                  )
                                }}
                              </div>
                              <div
                                class="text-xs text-gray-500 dark:text-gray-400"
                              >
                                分钟阅读
                              </div>
                            </div>
                          </div>

                          <div class="flex items-center space-x-2">
                            <div
                              class="w-8 h-8 bg-gradient-to-br from-orange-500 to-orange-600 rounded-lg flex items-center justify-center shadow-lg"
                            >
                              <UIcon
                                name="i-heroicons-language"
                                class="w-4 h-4 text-white"
                              />
                            </div>
                            <div>
                              <div
                                class="font-semibold text-gray-900 dark:text-white"
                              >
                                {{ countWords(post?.content_markdown || '') }}
                              </div>
                              <div
                                class="text-xs text-gray-500 dark:text-gray-400"
                              >
                                词数
                              </div>
                            </div>
                          </div>
                        </div>
                      </div>

                      <!-- 快捷键提示 -->
                      <div
                        class="hidden lg:flex items-center space-x-6 text-xs text-gray-500 dark:text-gray-400"
                      >
                        <div
                          class="flex items-center space-x-4 bg-gray-100/80 dark:bg-gray-800/80 rounded-lg px-4 py-2"
                        >
                          <span class="flex items-center space-x-1">
                            <kbd
                              class="px-2 py-1 bg-white dark:bg-gray-700 rounded shadow text-xs font-mono"
                            >Ctrl</kbd>
                            <span>+</span>
                            <kbd
                              class="px-2 py-1 bg-white dark:bg-gray-700 rounded shadow text-xs font-mono"
                            >S</kbd>
                            <span class="ml-2">保存</span>
                          </span>
                          <div class="w-px h-4 bg-gray-300 dark:bg-gray-600" />
                          <span class="flex items-center space-x-1">
                            <kbd
                              class="px-2 py-1 bg-white dark:bg-gray-700 rounded shadow text-xs font-mono"
                            >Ctrl</kbd>
                            <span>+</span>
                            <kbd
                              class="px-2 py-1 bg-white dark:bg-gray-700 rounded shadow text-xs font-mono"
                            >B</kbd>
                            <span class="ml-2">粗体</span>
                          </span>
                          <div class="w-px h-4 bg-gray-300 dark:bg-gray-600" />
                          <span class="flex items-center space-x-1">
                            <kbd
                              class="px-2 py-1 bg-white dark:bg-gray-700 rounded shadow text-xs font-mono"
                            >Tab</kbd>
                            <span class="ml-2">缩进</span>
                          </span>
                        </div>
                      </div>
                    </div>
                  </div>
                </div>

                <!-- 实时预览侧边栏 -->
                <div
                  v-if="showSideBySide"
                  class="w-1/2 border-l border-gray-200/60 dark:border-gray-700/60 bg-white/95 dark:bg-gray-900/95 backdrop-blur-xl overflow-hidden"
                >
                  <div class="h-full flex flex-col">
                    <!-- 预览头部 -->
                    <div
                      class="flex-shrink-0 bg-gradient-to-r from-blue-50 to-indigo-50 dark:from-blue-950/50 dark:to-indigo-950/50 border-b border-gray-200/60 dark:border-gray-700/60 px-8 py-6"
                    >
                      <div class="flex items-center justify-between">
                        <div class="flex items-center space-x-3">
                          <div
                            class="w-10 h-10 bg-gradient-to-br from-blue-500 to-indigo-600 rounded-xl flex items-center justify-center shadow-lg"
                          >
                            <UIcon
                              name="i-heroicons-eye"
                              class="w-5 h-5 text-white"
                            />
                          </div>
                          <div>
                            <h3
                              class="text-lg font-semibold text-gray-900 dark:text-white"
                            >
                              实时预览
                            </h3>
                            <p class="text-sm text-gray-600 dark:text-gray-400">
                              所见即所得
                            </p>
                          </div>
                        </div>
                        <button
                          class="p-2 hover:bg-white/80 dark:hover:bg-gray-800/80 rounded-lg transition-colors"
                          @click="showSideBySide = false"
                        >
                          <UIcon
                            name="i-heroicons-x-mark"
                            class="w-5 h-5 text-gray-500"
                          />
                        </button>
                      </div>
                    </div>

                    <!-- 预览内容 -->
                    <div class="flex-1 overflow-y-auto p-8">
                      <article
                        class="prose prose-lg prose-gray dark:prose-invert max-w-none"
                      >
                        <h1
                          v-if="post?.title"
                          class="text-3xl font-bold mb-8 leading-tight"
                        >
                          {{ post.title }}
                        </h1>
                        <MarkdownRenderer
                          :content="post?.content_markdown || ''"
                          :show-toc="false"
                        />
                      </article>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- 全屏预览模式 -->
          <div
            v-else
            class="flex-1 overflow-y-auto bg-white dark:bg-gray-900"
          >
            <div class="max-w-5xl mx-auto px-12 py-16">
              <article
                class="prose prose-xl prose-gray dark:prose-invert max-w-none"
              >
                <header
                  class="mb-16 pb-8 border-b border-gray-200 dark:border-gray-700"
                >
                  <h1
                    class="text-5xl font-bold bg-gradient-to-r from-gray-900 to-gray-600 dark:from-white dark:to-gray-300 bg-clip-text text-transparent mb-6 leading-tight"
                  >
                    {{ post?.title || '无标题文章' }}
                  </h1>
                  <div
                    class="flex items-center space-x-8 text-gray-500 dark:text-gray-400"
                  >
                    <div class="flex items-center space-x-2">
                      <UIcon
                        name="i-heroicons-calendar"
                        class="w-5 h-5"
                      />
                      <span class="text-lg">{{
                        new Date().toLocaleDateString('zh-CN')
                      }}</span>
                    </div>
                    <div class="flex items-center space-x-2">
                      <UIcon
                        name="i-heroicons-clock"
                        class="w-5 h-5"
                      />
                      <span class="text-lg">{{
                        Math.ceil((post?.content_markdown || '').length / 400)
                      }}
                        分钟阅读</span>
                    </div>
                    <div class="flex items-center space-x-2">
                      <UIcon
                        name="i-heroicons-document-text"
                        class="w-5 h-5"
                      />
                      <span class="text-lg">{{ countWords(post?.content_markdown || '') }} 词</span>
                    </div>
                  </div>
                </header>

                <div class="prose-content">
                  <MarkdownRenderer
                    :content="post?.content_markdown || ''"
                    :show-toc="false"
                  />
                </div>
              </article>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Post } from "~/types";
import MarkdownRenderer from "~/components/common/MarkdownRenderer.vue";

// 路由守卫
definePageMeta({
  middleware: "auth",
});

// 获取路由参数
const route = useRoute();
const router = useRouter();
const toast = useToast();

const postId = route.params.id as string;

// 状态
const saving = ref(false);
const publishing = ref(false);
const showPreview = ref(false);
const showSideBySide = ref(false);
const lastSaved = ref<string | null>(null);
const editorRef = ref();

const apiClient = useApi();

// 获取文章数据
const post = ref({
  id: "",
  slug: "",
  title: "",
  content_markdown: "",
  excerpt: "",
  thumbnail: "",
  status: "draft" as const,
  tags: [],
  categories: [],
  author: { id: "", username: "" },
  author_id: "",
  created_at: "",
  updated_at: "",
  published_at: undefined,
} as Post);

const pending = ref(true);
const error = ref(false);

// SEO
useHead({
  title: computed(() =>
    post.value?.title ? `编辑: ${post.value.title}` : "编辑文章",
  ),
});

// 自动保存功能
let autoSaveTimer: NodeJS.Timeout | null = null;

const scheduleAutoSave = () => {
  if (autoSaveTimer) {
    clearTimeout(autoSaveTimer);
  }
  autoSaveTimer = setTimeout(() => {
    saveAsDraft(true);
  }, 3000);
};

// 方法
const goBack = () => {
  router.back();
};

const handleTitleChange = () => {
  scheduleAutoSave();
};

const handleContentChange = () => {
  scheduleAutoSave();
};

// Markdown 插入功能
const insertMarkdown = (prefix: string, suffix: string) => {
  const textarea = editorRef.value?.$el?.querySelector("textarea");
  if (!textarea) return;

  const start = textarea.selectionStart;
  const end = textarea.selectionEnd;
  const selectedText = post.value.content_markdown.substring(start, end);

  const newText = prefix + selectedText + suffix;
  const before = post.value.content_markdown.substring(0, start);
  const after = post.value.content_markdown.substring(end);

  post.value.content_markdown = before + newText + after;

  nextTick(() => {
    textarea.focus();
    textarea.setSelectionRange(start + prefix.length, end + prefix.length);
  });

  scheduleAutoSave();
};

// 快捷键处理
const handleKeydown = (event: KeyboardEvent) => {
  if (event.ctrlKey || event.metaKey) {
    switch (event.key) {
      case "s":
        event.preventDefault();
        saveAsDraft();
        break;
      case "b":
        event.preventDefault();
        insertMarkdown("**", "**");
        break;
      case "i":
        event.preventDefault();
        insertMarkdown("*", "*");
        break;
    }
  }
};

// 字数统计功能
const countWords = (text: string): number => {
  if (!text) return 0;
  const cleaned = text.replace(/[#*`[\]()_~]/g, "").trim();
  const chineseWords = (cleaned.match(/[\u4e00-\u9fa5]/g) || []).length;
  const englishWords = (cleaned.match(/[a-zA-Z]+/g) || []).length;
  return chineseWords + englishWords;
};

const saveAsDraft = async (silent = false) => {
  saving.value = true;

  try {
    const response = await apiClient.updatePost(postId, {
      title: post.value.title,
      content_markdown: post.value.content_markdown,
      excerpt: post.value.excerpt,
      thumbnail: post.value.thumbnail,
      status: "draft",
    });

    if (response.success) {
      lastSaved.value = new Date().toISOString();
      if (!silent) {
        toast.add({
          title: "保存成功",
          description: "草稿已保存",
          color: "success",
        });
      }
    }
  }
  catch {
    if (!silent) {
      toast.add({
        title: "保存失败",
        description: "保存草稿时发生错误",
        color: "error",
      });
    }
  }
  finally {
    saving.value = false;
  }
};

const publishPost = async () => {
  if (!post.value.title.trim()) {
    toast.add({
      title: "发布失败",
      description: "请先输入文章标题",
      color: "error",
    });
    return;
  }

  publishing.value = true;

  try {
    const response = await apiClient.updatePost(postId, {
      title: post.value.title,
      content_markdown: post.value.content_markdown,
      excerpt: post.value.excerpt,
      thumbnail: post.value.thumbnail,
      status: "published",
      published_at: post.value.published_at || new Date().toISOString(),
    });

    if (response.success) {
      post.value.status = "published";
      toast.add({
        title: "发布成功",
        description: "文章已发布",
        color: "success",
      });

      router.push(`/posts/${response.data?.slug || postId}`);
    }
  }
  catch {
    toast.add({
      title: "发布失败",
      description: "发布文章时发生错误",
      color: "error",
    });
  }
  finally {
    publishing.value = false;
  }
};

const togglePreview = () => {
  showPreview.value = !showPreview.value;
};

const formatDate = (dateString: string) => {
  return new Date(dateString).toLocaleTimeString("zh-CN", {
    hour: "2-digit",
    minute: "2-digit",
  });
};

// 加载文章数据
onMounted(async () => {
  try {
    const response = await $fetch(`http://localhost:8080/posts/${postId}`, {
      headers: {
        Authorization: `Bearer ${localStorage.getItem("access_token")}`,
      },
    });
    post.value = response as Post;
  }
  catch (err) {
    console.error("Failed to load post:", err);
    error.value = true;
  }
  finally {
    pending.value = false;
  }
});

onUnmounted(() => {
  if (autoSaveTimer) {
    clearTimeout(autoSaveTimer);
  }
});

// 监听路由离开，提醒保存
onBeforeRouteLeave((to, from, next) => {
  if (lastSaved.value) {
    next();
  }
  else {
    const confirmed = confirm("您有未保存的更改，确定要离开吗？");
    if (confirmed) {
      next();
    }
    else {
      next(false);
    }
  }
});
</script>

<style>
/* 全局强制样式覆盖 - 不使用scoped以确保优先级 */

/* 页面背景强制覆盖 */
.editor-page-modern {
  background: linear-gradient(
    135deg,
    #f7fafc 0%,
    #e0f7fa 50%,
    #e8f5e8 100%
  ) !important;
  position: relative !important;
  overflow: hidden !important;
  min-height: 100vh !important;
}

html.dark .editor-page-modern {
  background: linear-gradient(
    135deg,
    #0f172a 0%,
    #312e81 30%,
    #1e3a8a 100%
  ) !important;
}

/* 浮动动画强制生效 */
@keyframes float-force {
  0%,
  100% {
    transform: translateY(0px) translateX(0px);
  }
  33% {
    transform: translateY(-15px) translateX(8px);
  }
  66% {
    transform: translateY(8px) translateX(-5px);
  }
}

.animate-float {
  animation: float-force 8s ease-in-out infinite !important;
}

/* 网格背景强制生效 */
.bg-grid-pattern {
  background-image: linear-gradient(
      rgba(139, 92, 246, 0.08) 1px,
      transparent 1px
    ),
    linear-gradient(90deg, rgba(139, 92, 246, 0.08) 1px, transparent 1px) !important;
  background-size: 25px 25px !important;
  opacity: 0.6 !important;
}

/* 毛玻璃效果强制生效 */
.backdrop-blur-3xl {
  backdrop-filter: blur(64px) saturate(200%) brightness(1.15) !important;
}

/* 按钮强制样式 */
button:hover {
  transform: translateY(-2px) scale(1.02) !important;
  box-shadow: 0 12px 40px rgba(139, 92, 246, 0.25) !important;
  transition: all 0.3s ease !important;
}

/* 工具按钮强制样式 */
.tool-button:hover {
  background: linear-gradient(135deg, #8b5cf6, #06b6d4) !important;
  color: white !important;
  transform: translateY(-1px) scale(1.05) !important;
  box-shadow: 0 8px 20px rgba(139, 92, 246, 0.3) !important;
}

/* 状态指示器强制动画 */
@keyframes pulse-force {
  0%,
  100% {
    opacity: 1;
  }
  50% {
    opacity: 0.5;
  }
}

.animate-pulse {
  animation: pulse-force 2s ease-in-out infinite !important;
}

@keyframes ping-force {
  75%,
  100% {
    transform: scale(2);
    opacity: 0;
  }
}

.animate-ping {
  animation: ping-force 1s cubic-bezier(0, 0, 0.2, 1) infinite !important;
}

/* 编辑器文本区域强制样式 */
.professional-editor,
textarea[class*='professional-editor'] {
  background: transparent !important;
  border: none !important;
  outline: none !important;
  font-family: 'SF Pro Text', -apple-system, BlinkMacSystemFont, 'Segoe UI',
    Roboto, Inter, sans-serif !important;
  font-size: 18px !important;
  line-height: 1.8 !important;
  color: #1e293b !important;
  padding: 4rem 5rem !important;
  font-weight: 400 !important;
  letter-spacing: 0.01em !important;
}

html.dark .professional-editor,
html.dark textarea[class*='professional-editor'] {
  color: #f8fafc !important;
}

.professional-editor::placeholder,
textarea[class*='professional-editor']::placeholder {
  color: #94a3b8 !important;
  font-weight: 300 !important;
}

.dark .min-h-screen {
  background: linear-gradient(
    135deg,
    #0f172a 0%,
    #312e81 30%,
    #1e3a8a 100%
  ) !important;
}
/* 专业编辑器样式 */
.professional-editor {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'SF Pro Text',
    Roboto, Inter, 'Helvetica Neue', Arial, sans-serif;
  font-size: 18px;
  line-height: 1.8;
  color: #1f2937;
  background: transparent;
  font-weight: 400;
  letter-spacing: 0.01em;
  font-feature-settings: 'kern' 1, 'liga' 1, 'calt' 1;
}

.dark .professional-editor {
  color: #f8fafc;
}

.professional-editor::placeholder {
  color: #94a3b8;
  font-style: normal;
  font-weight: 300;
  line-height: 1.7;
}

.dark .professional-editor::placeholder {
  color: #64748b;
}

.professional-editor:focus {
  outline: none;
  box-shadow: none;
  transform: translateZ(0);
}

/* 优雅滚动条 */
.overflow-y-auto::-webkit-scrollbar {
  width: 10px;
}

.overflow-y-auto::-webkit-scrollbar-track {
  background: rgba(0, 0, 0, 0.02);
  border-radius: 5px;
  margin: 10px 0;
}

.overflow-y-auto::-webkit-scrollbar-thumb {
  background: linear-gradient(180deg, #e2e8f0 0%, #cbd5e1 100%);
  border-radius: 5px;
  border: 2px solid rgba(255, 255, 255, 0.7);
  box-shadow: inset 0 1px 3px rgba(0, 0, 0, 0.1);
}

.overflow-y-auto::-webkit-scrollbar-thumb:hover {
  background: linear-gradient(180deg, #cbd5e1 0%, #94a3b8 100%);
}

.dark .overflow-y-auto::-webkit-scrollbar-track {
  background: rgba(255, 255, 255, 0.02);
}

.dark .overflow-y-auto::-webkit-scrollbar-thumb {
  background: linear-gradient(180deg, #4b5563 0%, #374151 100%);
  border: 2px solid rgba(0, 0, 0, 0.3);
}

.dark .overflow-y-auto::-webkit-scrollbar-thumb:hover {
  background: linear-gradient(180deg, #6b7280 0%, #4b5563 100%);
}

/* 平滑过渡动画 */
* {
  transition: all 0.2s cubic-bezier(0.25, 0.46, 0.45, 0.94);
}

/* 毛玻璃效果 */
.backdrop-blur-2xl {
  backdrop-filter: blur(40px) saturate(180%) brightness(1.05);
}

.backdrop-blur-xl {
  backdrop-filter: blur(24px) saturate(180%) brightness(1.02);
}

/* 按钮悬停效果 */
button {
  transition: all 0.2s cubic-bezier(0.25, 0.46, 0.45, 0.94);
}

button:hover {
  transform: translateY(-1px) scale(1.02);
}

button:active {
  transform: translateY(0) scale(0.98);
}

/* 工具栏按钮组样式 */
.group:hover .group-hover\\:scale-110 {
  transform: scale(1.1);
}

/* 装饰动画 */
@keyframes float {
  0%,
  100% {
    transform: translateY(0px);
  }
  50% {
    transform: translateY(-6px);
  }
}

@keyframes glow {
  0%,
  100% {
    opacity: 0.5;
    filter: blur(1px);
  }
  50% {
    opacity: 0.8;
    filter: blur(0px);
  }
}

/* 渐变背景动画 */
@keyframes gradient {
  0%,
  100% {
    background-position: 0% 50%;
  }
  50% {
    background-position: 100% 50%;
  }
}

/* 响应式设计 */
@media (max-width: 1024px) {
  .professional-editor {
    padding: 3rem 2rem;
    font-size: 17px;
  }
}

@media (max-width: 768px) {
  .professional-editor {
    padding: 2rem 1.5rem;
    font-size: 16px;
    line-height: 1.7;
  }
}

@media (max-width: 640px) {
  .professional-editor {
    padding: 1.5rem 1rem;
    font-size: 15px;
  }
}

/* 键盘快捷键样式 */
kbd {
  font-family: ui-monospace, SFMono-Regular, 'SF Mono', 'Monaco', 'Inconsolata',
    'Roboto Mono', monospace;
  font-size: 0.75rem;
  font-weight: 500;
  line-height: 1;
}

/* 状态栏卡片样式 */
.grid > div {
  transition: transform 0.2s ease, box-shadow 0.2s ease;
}

.grid > div:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

/* 侧边预览滑入动画 */
.w-1\/2 {
  animation: slideInRight 0.3s cubic-bezier(0.25, 0.46, 0.45, 0.94);
}

@keyframes slideInRight {
  from {
    opacity: 0;
    transform: translateX(100%);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

/* 深色模式文本优化 */
.dark .text-gray-800 {
  color: #f1f5f9;
}

.dark .text-gray-600 {
  color: #cbd5e1;
}

.dark .text-gray-500 {
  color: #94a3b8;
}

/* Prose 内容样式优化 */
.prose-content {
  font-size: 18px;
  line-height: 1.8;
}

.prose-content h1 {
  font-size: 2.5em;
  font-weight: 700;
  line-height: 1.2;
  margin-bottom: 1em;
}

.prose-content h2 {
  font-size: 2em;
  font-weight: 600;
  line-height: 1.3;
  margin-top: 2em;
  margin-bottom: 1em;
}

.prose-content p {
  margin-bottom: 1.5em;
  text-align: justify;
}

.prose-content blockquote {
  border-left: 4px solid #3b82f6;
  padding-left: 1.5em;
  margin: 1.5em 0;
  font-style: italic;
  color: #64748b;
}

.dark .prose-content blockquote {
  color: #94a3b8;
  border-left-color: #6366f1;
}

/* 编辑器聚焦状态 */
.professional-editor:focus {
  background: radial-gradient(
    circle at 50% 50%,
    rgba(59, 130, 246, 0.03) 0%,
    transparent 50%
  );
}

.dark .professional-editor:focus {
  background: radial-gradient(
    circle at 50% 50%,
    rgba(99, 102, 241, 0.05) 0%,
    transparent 50%
  );
}

/* 新增的现代化样式 */
/* 标题输入样式 */
.title-input:focus {
  transform: scale(1.002) !important;
}

/* 网格背景 */
.bg-grid-pattern {
  background-image: linear-gradient(
      rgba(139, 92, 246, 0.1) 1px,
      transparent 1px
    ),
    linear-gradient(90deg, rgba(139, 92, 246, 0.1) 1px, transparent 1px) !important;
  background-size: 20px 20px !important;
}

/* 玻璃效果按钮 */
.glass-button {
  backdrop-filter: blur(16px) saturate(180%) !important;
  background: rgba(255, 255, 255, 0.8) !important;
  border: 1px solid rgba(255, 255, 255, 0.2) !important;
}

.dark .glass-button {
  background: rgba(15, 23, 42, 0.8) !important;
  border: 1px solid rgba(148, 163, 184, 0.2) !important;
}

/* 工具按钮样式 */
.tool-button {
  position: relative;
  overflow: hidden;
}

.tool-button::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(
    90deg,
    transparent,
    rgba(255, 255, 255, 0.2),
    transparent
  );
  transition: left 0.6s;
}

.tool-button:hover::before {
  left: 100%;
}

/* 浮动动画 */
@keyframes float {
  0%,
  100% {
    transform: translateY(0px);
  }
  50% {
    transform: translateY(-8px);
  }
}

.animate-float {
  animation: float 6s ease-in-out infinite !important;
}

/* 增强滚动条 */
.overflow-y-auto::-webkit-scrollbar {
  width: 12px;
}

.overflow-y-auto::-webkit-scrollbar-track {
  background: rgba(148, 163, 184, 0.1);
  border-radius: 6px;
  margin: 12px 0;
}

.overflow-y-auto::-webkit-scrollbar-thumb {
  background: linear-gradient(180deg, #a78bfa 0%, #8b5cf6 50%, #7c3aed 100%);
  border-radius: 6px;
  border: 2px solid rgba(255, 255, 255, 0.2);
  box-shadow: 0 2px 4px rgba(139, 92, 246, 0.2);
}

.overflow-y-auto::-webkit-scrollbar-thumb:hover {
  background: linear-gradient(180deg, #8b5cf6 0%, #7c3aed 50%, #6d28d9 100%);
  box-shadow: 0 4px 8px rgba(139, 92, 246, 0.3);
}

/* 按钮悬停效果增强 */
button:hover {
  transform: translateY(-2px) scale(1.02) !important;
  box-shadow: 0 8px 25px rgba(139, 92, 246, 0.15) !important;
}

button:active {
  transform: translateY(0) scale(0.98) !important;
  transition: all 0.1s ease !important;
}

/* 工具栏按钮组样式增强 */
.group:hover .group-hover\:scale-110 {
  transform: scale(1.15) rotate(3deg);
}

/* 渐变背景动画 */
@keyframes gradient-shift {
  0%,
  100% {
    background-position: 0% 50%;
  }
  50% {
    background-position: 100% 50%;
  }
}

/* 光晕效果 */
@keyframes glow-pulse {
  0%,
  100% {
    opacity: 0.4;
    filter: blur(2px);
  }
  50% {
    opacity: 0.8;
    filter: blur(0px);
  }
}

/* 特殊效果 */
.shimmer {
  background: linear-gradient(
    110deg,
    transparent 40%,
    rgba(255, 255, 255, 0.5) 50%,
    transparent 60%
  );
  background-size: 200% 100%;
  animation: shimmer 2s infinite;
}

@keyframes shimmer {
  0% {
    background-position: -200% 0;
  }
  100% {
    background-position: 200% 0;
  }
}

/* 编辑器增强效果 */
.editor-glow {
  position: relative;
}

.editor-glow::after {
  content: '';
  position: absolute;
  top: -2px;
  left: -2px;
  right: -2px;
  bottom: -2px;
  background: linear-gradient(45deg, #8b5cf6, #06b6d4, #10b981);
  border-radius: 1rem;
  z-index: -1;
  opacity: 0;
  transition: opacity 0.3s ease;
  background-size: 300%;
  animation: gradient-shift 4s ease infinite;
}

.editor-glow:focus-within::after {
  opacity: 0.3;
  filter: blur(4px);
}

/* 强制样式应用 */
.backdrop-blur-3xl {
  backdrop-filter: blur(64px) saturate(200%) brightness(1.1) !important;
}

.backdrop-blur-2xl {
  backdrop-filter: blur(40px) saturate(180%) brightness(1.05) !important;
}

.shadow-2xl {
  box-shadow: 0 25px 50px -12px rgba(139, 92, 246, 0.25) !important;
}

.shadow-xl {
  box-shadow: 0 20px 25px -5px rgba(139, 92, 246, 0.1),
    0 10px 10px -5px rgba(139, 92, 246, 0.04) !important;
}

.shadow-lg {
  box-shadow: 0 10px 15px -3px rgba(139, 92, 246, 0.1),
    0 4px 6px -2px rgba(139, 92, 246, 0.05) !important;
}

/* Nuxt UI 覆盖 */
.bg-gradient-to-r {
  background-image: linear-gradient(
    to right,
    var(--tw-gradient-stops)
  ) !important;
}

.bg-gradient-to-br {
  background-image: linear-gradient(
    to bottom right,
    var(--tw-gradient-stops)
  ) !important;
}
</style>
/* Force CSS reload */
