<template>
  <div class="markdown-editor">
    <!-- 工具栏 -->
    <div class="editor-toolbar bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 p-3">
      <div class="flex items-center space-x-2 overflow-x-auto">
        <!-- 格式化按钮组 -->
        <div class="flex items-center space-x-1 border-r border-gray-200 dark:border-gray-600 pr-3">
          <UButton
            variant="ghost"
            size="sm"
            icon="i-heroicons-bold"
            title="粗体"
            @click="insertFormat('**', '**', '粗体文字')"
          />
          <UButton
            variant="ghost"
            size="sm"
            icon="i-heroicons-italic"
            title="斜体"
            @click="insertFormat('*', '*', '斜体文字')"
          />
          <UButton
            variant="ghost"
            size="sm"
            icon="i-heroicons-underline"
            title="删除线"
            @click="insertFormat('~~', '~~', '删除线文字')"
          />
          <UButton
            variant="ghost"
            size="sm"
            icon="i-heroicons-code-bracket"
            title="行内代码"
            @click="insertFormat('`', '`', '代码')"
          />
        </div>

        <!-- 标题按钮组 -->
        <div class="flex items-center space-x-1 border-r border-gray-200 dark:border-gray-600 pr-3">
          <UButton
            variant="ghost"
            size="sm"
            title="标题1"
            @click="insertHeading(1)"
          >
            H1
          </UButton>
          <UButton
            variant="ghost"
            size="sm"
            title="标题2"
            @click="insertHeading(2)"
          >
            H2
          </UButton>
          <UButton
            variant="ghost"
            size="sm"
            title="标题3"
            @click="insertHeading(3)"
          >
            H3
          </UButton>
        </div>

        <!-- 列表和引用 -->
        <div class="flex items-center space-x-1 border-r border-gray-200 dark:border-gray-600 pr-3">
          <UButton
            variant="ghost"
            size="sm"
            icon="i-heroicons-list-bullet"
            title="无序列表"
            @click="insertList('-')"
          />
          <UButton
            variant="ghost"
            size="sm"
            icon="i-heroicons-numbered-list"
            title="有序列表"
            @click="insertList('1.')"
          />
          <UButton
            variant="ghost"
            size="sm"
            icon="i-heroicons-chat-bubble-left-right"
            title="引用"
            @click="insertQuote()"
          />
        </div>

        <!-- 插入内容 -->
        <div class="flex items-center space-x-1 border-r border-gray-200 dark:border-gray-600 pr-3">
          <UButton
            variant="ghost"
            size="sm"
            icon="i-heroicons-link"
            title="链接"
            @click="insertLink()"
          />
          <UButton
            variant="ghost"
            size="sm"
            icon="i-heroicons-photo"
            title="图片"
            @click="insertImage()"
          />
          <UButton
            variant="ghost"
            size="sm"
            icon="i-heroicons-table-cells"
            title="表格"
            @click="insertTable()"
          />
          <UButton
            variant="ghost"
            size="sm"
            icon="i-heroicons-code-bracket-square"
            title="代码块"
            @click="insertCodeBlock()"
          />
        </div>

        <!-- 视图切换 -->
        <div class="flex items-center space-x-1">
          <UButton
            :variant="viewMode === 'edit' ? 'solid' : 'ghost'"
            size="sm"
            title="编辑模式"
            @click="viewMode = 'edit'"
          >
            编辑
          </UButton>
          <UButton
            :variant="viewMode === 'split' ? 'solid' : 'ghost'"
            size="sm"
            title="分屏模式"
            @click="viewMode = 'split'"
          >
            分屏
          </UButton>
          <UButton
            :variant="viewMode === 'preview' ? 'solid' : 'ghost'"
            size="sm"
            title="预览模式"
            @click="viewMode = 'preview'"
          >
            预览
          </UButton>
        </div>
      </div>
    </div>

    <!-- 编辑器主体 -->
    <div
      class="editor-body flex"
      :class="bodyClasses"
    >
      <!-- 编辑器区域 -->
      <div
        v-if="viewMode !== 'preview'"
        class="editor-pane flex-1 flex flex-col"
      >
        <!-- 编辑器文本框 -->
        <textarea
          ref="textareaRef"
          v-model="content"
          class="editor-textarea flex-1 w-full p-4 resize-none border-0 focus:ring-0 focus:outline-none bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100"
          :placeholder="placeholder"
          @input="handleInput"
          @keydown="handleKeydown"
          @select="handleSelection"
        />

        <!-- 状态栏 -->
        <div class="editor-status bg-gray-50 dark:bg-gray-800 px-4 py-2 text-sm text-gray-600 dark:text-gray-400 border-t border-gray-200 dark:border-gray-700">
          <div class="flex items-center justify-between">
            <div class="flex items-center space-x-4">
              <span>行数: {{ lineCount }}</span>
              <span>字数: {{ wordCount }}</span>
              <span>字符: {{ charCount }}</span>
            </div>
            <div class="flex items-center space-x-4">
              <span v-if="selectedText">已选择: {{ selectedText.length }} 字符</span>
              <span>光标位置: {{ cursorPosition.line }}:{{ cursorPosition.column }}</span>
            </div>
          </div>
        </div>
      </div>

      <!-- 分隔线 -->
      <div
        v-if="viewMode === 'split'"
        class="w-px bg-gray-200 dark:bg-gray-700"
      />

      <!-- 预览区域 -->
      <div
        v-if="viewMode !== 'edit'"
        class="preview-pane flex-1 overflow-y-auto bg-white dark:bg-gray-900"
      >
        <div class="p-4">
          <MarkdownRenderer
            :content="content"
            :show-toc="false"
            class="preview-content"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import MarkdownRenderer from "~/components/common/MarkdownRenderer.vue";

interface Props {
  modelValue?: string;
  placeholder?: string;
  height?: string;
  disabled?: boolean;
  autofocus?: boolean;
}

interface Emits {
  (e: "update:modelValue", value: string): void;
  (e: "change", value: string): void;
  (e: "input", value: string): void;
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: "",
  placeholder: "开始写作...",
  height: "500px",
  disabled: false,
  autofocus: false,
});

const emit = defineEmits<Emits>();

// 响应式数据
const textareaRef = ref<HTMLTextAreaElement>();
const viewMode = ref<"edit" | "split" | "preview">("split");
const selectedText = ref("");
const cursorPosition = ref({ line: 1, column: 1 });

// 双向绑定
const content = computed({
  get: () => props.modelValue,
  set: (value: string) => {
    emit("update:modelValue", value);
    emit("change", value);
  },
});

// 计算属性
const lineCount = computed(() => {
  return content.value.split("\n").length;
});

const wordCount = computed(() => {
  const text = content.value.replace(/\s+/g, " ").trim();
  return text ? text.length : 0;
});

const charCount = computed(() => {
  return content.value.length;
});

const bodyClasses = computed(() => {
  return {
    "min-h-[500px]": true,
    [`min-h-[${props.height}]`]: props.height !== "500px",
  };
});

// 方法
const handleInput = (event: Event) => {
  const target = event.target as HTMLTextAreaElement;
  emit("input", target.value);
  updateCursorPosition();
};

const handleSelection = () => {
  if (textareaRef.value) {
    const start = textareaRef.value.selectionStart;
    const end = textareaRef.value.selectionEnd;
    selectedText.value = content.value.substring(start, end);
    updateCursorPosition();
  }
};

const handleKeydown = (event: KeyboardEvent) => {
  // Tab 键处理
  if (event.key === "Tab") {
    event.preventDefault();
    insertText("  "); // 插入两个空格
  }

  // Ctrl/Cmd + 快捷键
  if (event.ctrlKey || event.metaKey) {
    switch (event.key) {
      case "b":
        event.preventDefault();
        insertFormat("**", "**", "粗体文字");
        break;
      case "i":
        event.preventDefault();
        insertFormat("*", "*", "斜体文字");
        break;
      case "k":
        event.preventDefault();
        insertLink();
        break;
    }
  }
};

const updateCursorPosition = () => {
  if (!textareaRef.value) return;

  const textarea = textareaRef.value;
  const cursorPos = textarea.selectionStart;
  const textBeforeCursor = content.value.substring(0, cursorPos);
  const lines = textBeforeCursor.split("\n");

  cursorPosition.value = {
    line: lines.length,
    column: lines[lines.length - 1].length + 1,
  };
};

const insertText = (text: string) => {
  if (!textareaRef.value) return;

  const textarea = textareaRef.value;
  const start = textarea.selectionStart;
  const end = textarea.selectionEnd;

  const newContent = content.value.substring(0, start) + text + content.value.substring(end);
  content.value = newContent;

  nextTick(() => {
    textarea.focus();
    textarea.setSelectionRange(start + text.length, start + text.length);
  });
};

const insertFormat = (prefix: string, suffix: string, placeholder: string) => {
  if (!textareaRef.value) return;

  const textarea = textareaRef.value;
  const start = textarea.selectionStart;
  const end = textarea.selectionEnd;
  const selectedText = content.value.substring(start, end);

  const text = selectedText || placeholder;
  const formatted = prefix + text + suffix;

  const newContent = content.value.substring(0, start) + formatted + content.value.substring(end);
  content.value = newContent;

  nextTick(() => {
    textarea.focus();
    if (!selectedText) {
      // 如果没有选中文字，选中占位符
      textarea.setSelectionRange(start + prefix.length, start + prefix.length + placeholder.length);
    }
    else {
      // 如果有选中文字，光标移到格式化文本后面
      textarea.setSelectionRange(start + formatted.length, start + formatted.length);
    }
  });
};

const insertHeading = (level: number) => {
  const prefix = "#".repeat(level) + " ";

  if (!textareaRef.value) return;

  const textarea = textareaRef.value;
  const start = textarea.selectionStart;
  const lineStart = content.value.lastIndexOf("\n", start - 1) + 1;

  const newContent
    = content.value.substring(0, lineStart)
      + prefix
      + content.value.substring(lineStart);

  content.value = newContent;

  nextTick(() => {
    textarea.focus();
    textarea.setSelectionRange(start + prefix.length, start + prefix.length);
  });
};

const insertList = (marker: string) => {
  if (!textareaRef.value) return;

  const textarea = textareaRef.value;
  const start = textarea.selectionStart;
  const lineStart = content.value.lastIndexOf("\n", start - 1) + 1;

  const prefix = marker === "1." ? "1. " : "- ";
  const newContent
    = content.value.substring(0, lineStart)
      + prefix
      + content.value.substring(lineStart);

  content.value = newContent;

  nextTick(() => {
    textarea.focus();
    textarea.setSelectionRange(start + prefix.length, start + prefix.length);
  });
};

const insertQuote = () => {
  if (!textareaRef.value) return;

  const textarea = textareaRef.value;
  const start = textarea.selectionStart;
  const lineStart = content.value.lastIndexOf("\n", start - 1) + 1;

  const prefix = "> ";
  const newContent
    = content.value.substring(0, lineStart)
      + prefix
      + content.value.substring(lineStart);

  content.value = newContent;

  nextTick(() => {
    textarea.focus();
    textarea.setSelectionRange(start + prefix.length, start + prefix.length);
  });
};

const insertLink = () => {
  const selectedText = getSelectedText();
  const linkText = selectedText || "链接文字";
  const linkUrl = "https://example.com";

  insertFormat(`[${linkText}](`, ")", linkUrl);
};

const insertImage = () => {
  const altText = "图片描述";
  const imageUrl = "https://example.com/image.jpg";

  insertFormat(`![${altText}](`, ")", imageUrl);
};

const insertTable = () => {
  const table = `
| 列1 | 列2 | 列3 |
|-----|-----|-----|
| 内容1 | 内容2 | 内容3 |
| 内容4 | 内容5 | 内容6 |
`;
  insertText(table.trim());
};

const insertCodeBlock = () => {
  const codeBlock = "\n```javascript\n// 在这里输入代码\nconsole.log('Hello World!');\n```\n";
  insertText(codeBlock);
};

const getSelectedText = (): string => {
  if (!textareaRef.value) return "";

  const start = textareaRef.value.selectionStart;
  const end = textareaRef.value.selectionEnd;
  return content.value.substring(start, end);
};

// 生命周期
onMounted(() => {
  if (props.autofocus && textareaRef.value) {
    textareaRef.value.focus();
  }

  // 监听文本区域的变化
  if (textareaRef.value) {
    textareaRef.value.addEventListener("scroll", updateCursorPosition);
  }
});

onUnmounted(() => {
  if (textareaRef.value) {
    textareaRef.value.removeEventListener("scroll", updateCursorPosition);
  }
});

// 暴露方法给父组件
defineExpose({
  focus: () => textareaRef.value?.focus(),
  blur: () => textareaRef.value?.blur(),
  insertText,
  insertFormat,
  getSelectedText,
});
</script>

<style scoped>
.markdown-editor {
  border: 1px solid rgb(229 231 235);
  border-radius: 0.5rem;
  overflow: hidden;
  background: white;
}

.dark .markdown-editor {
  border-color: rgb(55 65 81);
  background: rgb(17 24 39);
}

.editor-toolbar {
  border-bottom: 1px solid rgb(229 231 235);
}

.dark .editor-toolbar {
  border-bottom-color: rgb(55 65 81);
}

.editor-textarea {
  font-family: 'SFMono-Regular', 'Consolas', 'Liberation Mono', 'Menlo', monospace;
  font-size: 14px;
  line-height: 1.5;
  tab-size: 2;
}

.editor-textarea:focus {
  outline: none;
}

.preview-content {
  min-height: 100%;
}

/* 滚动条样式 */
.editor-pane::-webkit-scrollbar,
.preview-pane::-webkit-scrollbar {
  width: 8px;
}

.editor-pane::-webkit-scrollbar-track,
.preview-pane::-webkit-scrollbar-track {
  background: rgb(243 244 246);
}

.editor-pane::-webkit-scrollbar-thumb,
.preview-pane::-webkit-scrollbar-thumb {
  background: rgb(156 163 175);
  border-radius: 4px;
}

.editor-pane::-webkit-scrollbar-thumb:hover,
.preview-pane::-webkit-scrollbar-thumb:hover {
  background: rgb(107 114 128);
}

.dark .editor-pane::-webkit-scrollbar-track,
.dark .preview-pane::-webkit-scrollbar-track {
  background: rgb(31 41 55);
}

.dark .editor-pane::-webkit-scrollbar-thumb,
.dark .preview-pane::-webkit-scrollbar-thumb {
  background: rgb(75 85 99);
}

.dark .editor-pane::-webkit-scrollbar-thumb:hover,
.dark .preview-pane::-webkit-scrollbar-thumb:hover {
  background: rgb(107 114 128);
}
</style>
