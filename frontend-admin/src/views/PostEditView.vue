<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import AdminLayout from '../../layouts/AdminLayout.vue'
import apiClient from '@/api'
import type { Post, PostCreate, PostUpdate, Category, Tag, User, ShareDraftPayload } from '@/types'
import { useAuthStore } from '@/stores/auth'

const authStore = useAuthStore()

const route = useRoute()
const router = useRouter()

// 状态管理
const post = ref<Post | null>(null)
const categories = ref<Category[]>([])
const tags = ref<Tag[]>([])
const users = ref<User[]>([])
const loading = ref(false)
const saving = ref(false)
const error = ref<string | null>(null)

// 草稿分享相关状态
const shareDialogVisible = ref(false)
const shareForm = ref<ShareDraftPayload>({
  shared_with: [],
  is_public: false,
  message: '',
})

// 表单数据
const formData = ref<PostCreate>({
  title: '',
  content: '',
  published_at: undefined,
  category_ids: [],
  tag_ids: [],
})

// 判断是否为编辑模式
const isEdit = !!route.params.id
const postId = route.params.id as string

// 获取文章详情（编辑模式）
const fetchPost = async () => {
  if (!postId || !isEdit) return

  try {
    loading.value = true
    const response = await apiClient.get(`/posts/${postId}`)
    post.value = response.data

    // 填充表单数据 - 添加 null 检查
    if (post.value) {
      formData.value = {
        title: post.value.title,
        content: post.value.content_markdown || post.value.content || '', // 编辑时使用原始markdown内容
        published_at: post.value.published_at,
        category_ids: post.value.categories?.map((c) => c.id) || [],
        tag_ids: post.value.tags?.map((t) => t.id) || [],
      }
    }
  } catch (err) {
    console.error('获取文章失败:', err)
    error.value = '获取文章失败'
  } finally {
    loading.value = false
  }
}

// 获取分类和标签
const fetchMetadata = async () => {
  try {
    const [categoriesRes, tagsRes] = await Promise.all([
      apiClient.get('/categories'),
      apiClient.get('/tags'),
    ])
    categories.value = categoriesRes.data || []
    tags.value = tagsRes.data || []
  } catch (err) {
    console.error('获取分类标签失败:', err)
  }
}

// 获取用户列表（用于草稿分享）
const fetchUsers = async () => {
  try {
    const response = await apiClient.get('/users')
    users.value = response.data || []
  } catch (err) {
    console.error('获取用户列表失败:', err)
  }
}

// 保存文章
const savePost = async () => {
  if (!formData.value.title.trim()) {
    alert('请输入文章标题')
    return
  }

  if (!formData.value.content.trim()) {
    alert('请输入文章内容')
    return
  }

  try {
    saving.value = true

    if (isEdit && postId) {
      await apiClient.put(`/posts/${postId}`, formData.value)
    } else {
      await apiClient.post('/posts', formData.value)
    }

    router.push('/posts')
  } catch (err) {
    console.error('保存文章失败:', err)
    alert('保存文章失败')
  } finally {
    saving.value = false
  }
}

// 发布文章
const publishPost = async () => {
  formData.value.published_at = new Date().toISOString()
  await savePost()
}

// 保存为草稿
const saveDraft = async () => {
  if (!formData.value.title.trim()) {
    alert('请输入文章标题')
    return
  }

  if (!formData.value.content.trim()) {
    alert('请输入文章内容')
    return
  }

  try {
    saving.value = true

    // 构建保存草稿的payload，明确设置unpublish为true
    const draftPayload = {
      ...formData.value,
      published_at: null, // 显式设置为null
      unpublish: true, // 明确表示要撤回为草稿
    }

    if (isEdit && postId) {
      await apiClient.put(`/posts/${postId}`, draftPayload)
    } else {
      await apiClient.post('/posts', draftPayload)
    }

    router.push('/posts')
  } catch (err) {
    console.error('保存草稿失败:', err)
    alert('保存草稿失败')
  } finally {
    saving.value = false
  }
}

// 打开草稿分享对话框
const openShareDialog = () => {
  if (!post.value) return

  shareForm.value = {
    shared_with: (post.value.draft_shared_with || []).map((user) => user.id),
    is_public: post.value.is_draft_public || false,
    message: '',
  }
  shareDialogVisible.value = true

  // 获取用户列表
  if (users.value.length === 0) {
    fetchUsers()
  }
}

// 关闭分享对话框
const closeShareDialog = () => {
  shareDialogVisible.value = false
  shareForm.value = {
    shared_with: [],
    is_public: false,
    message: '',
  }
}

// 分享草稿
const shareDraft = async () => {
  if (!post.value) return

  try {
    await apiClient.put(`/posts/${post.value.id}/share`, shareForm.value)
    // 重新获取文章数据以更新分享状态
    await fetchPost()
    closeShareDialog()
    alert('草稿分享设置已更新！')
  } catch (err) {
    console.error('分享草稿失败:', err)
    alert('分享草稿失败')
  }
}

// 检查是否可以分享草稿
const canShareDraft = () => {
  return (
    post.value &&
    !post.value.published_at &&
    authStore.canShareDrafts() &&
    post.value.author_id === authStore.user?.id
  )
}

// 权限检查的计算属性
const canEditPost = () => {
  if (!post.value) return !isEdit // 新建文章时允许编辑
  return post.value.can_edit || false
}

const canPublishPost = () => {
  if (!post.value) return !isEdit // 新建文章时允许发布
  return post.value.can_publish || false
}

const canSaveDraft = () => {
  if (!post.value) return !isEdit // 新建文章时允许保存草稿
  return post.value.can_edit || false
}

// 是否为查看模式（没有编辑权限）
const isViewOnly = (): boolean => {
  return isEdit && post.value ? !post.value.can_edit : false
}

onMounted(() => {
  fetchMetadata()
  if (isEdit) {
    fetchPost()
  }
})
</script>

<template>
  <AdminLayout>
    <div class="post-edit-page">
      <!-- 页面头部 -->
      <div class="page-header">
        <div class="page-title">
          <h1>
            {{ isEdit ? (isViewOnly() ? '查看文章' : '编辑文章') : '写新文章' }}
          </h1>
          <p class="page-subtitle">
            {{
              isEdit
                ? isViewOnly()
                  ? '只读模式 - 您只能查看此文章内容'
                  : '修改并更新您的文章内容'
                : '创建一篇新的博客文章'
            }}
          </p>
        </div>
        <div class="page-actions">
          <router-link to="/posts" class="btn btn-secondary"> 取消 </router-link>
        </div>
      </div>

      <div v-if="loading" class="loading-state">
        <div class="loading-spinner"></div>
        <p>加载中...</p>
      </div>

      <div v-else-if="error" class="error-state">
        <p class="error-message">{{ error }}</p>
        <button @click="fetchPost" class="btn btn-primary">重试</button>
      </div>

      <div v-else class="post-edit-form">
        <div class="form-grid">
          <!-- 主要内容区域 -->
          <div class="main-content">
            <div class="card">
              <div class="card-body">
                <!-- 标题 -->
                <div class="form-group">
                  <label class="form-label">文章标题 *</label>
                  <input
                    v-model="formData.title"
                    type="text"
                    class="form-input"
                    :class="{ 'form-input-readonly': isViewOnly() }"
                    placeholder="输入文章标题..."
                    :readonly="isViewOnly()"
                    required
                  />
                </div>

                <!-- 内容 -->
                <div class="form-group">
                  <label class="form-label">文章内容 *</label>
                  <textarea
                    v-model="formData.content"
                    class="form-textarea content-editor"
                    :class="{ 'form-textarea-readonly': isViewOnly() }"
                    placeholder="开始写作..."
                    rows="20"
                    :readonly="isViewOnly()"
                    required
                  ></textarea>
                  <div class="form-help">
                    {{ isViewOnly() ? '只读模式' : '支持 Markdown 语法' }}
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- 侧边栏 -->
          <div class="sidebar-content">
            <!-- 发布选项 -->
            <div class="card">
              <div class="card-header">
                <h3>发布选项</h3>
              </div>
              <div class="card-body">
                <!-- 只读模式提示 -->
                <div v-if="isViewOnly()" class="readonly-notice">
                  <div class="alert alert-info">
                    <p><strong>只读模式</strong></p>
                    <p>您只能查看此文章内容，无法进行编辑、保存或发布操作。</p>
                    <p v-if="post?.is_accessing_others_draft" class="text-sm">
                      这是他人分享给您的草稿。
                    </p>
                  </div>
                </div>

                <!-- 编辑模式的操作按钮 -->
                <div v-else class="publish-actions">
                  <button
                    v-if="canSaveDraft()"
                    @click="saveDraft"
                    class="btn btn-secondary w-full"
                    :disabled="saving"
                  >
                    {{ saving ? '保存中...' : '保存草稿' }}
                  </button>

                  <!-- 草稿分享按钮 -->
                  <button
                    v-if="isEdit && canShareDraft()"
                    @click="openShareDialog"
                    class="btn btn-info w-full"
                  >
                    🔗 分享草稿
                  </button>

                  <button
                    v-if="canPublishPost()"
                    @click="publishPost"
                    class="btn btn-primary w-full"
                    :disabled="saving"
                  >
                    {{
                      saving ? '发布中...' : isEdit && post?.published_at ? '更新发布' : '立即发布'
                    }}
                  </button>
                </div>

                <!-- 文章状态信息 -->
                <div v-if="post" class="publish-status">
                  <!-- 封禁状态显示 -->
                  <div v-if="post.is_banned" class="status-section ban-status">
                    <div class="status-item">
                      <span class="status-label">状态:</span>
                      <span class="badge badge-danger">已封禁</span>
                    </div>
                    <div class="ban-notice">
                      <p class="text-sm text-red-700">
                        此文章已被管理员封禁，只有作者和管理员可以查看。
                      </p>
                    </div>
                  </div>

                  <div v-else-if="post.published_at" class="status-section">
                    <div class="status-item">
                      <span class="status-label">状态:</span>
                      <span class="badge badge-success">已发布</span>
                    </div>
                    <div class="status-item">
                      <span class="status-label">发布时间:</span>
                      <span class="status-value">
                        {{ new Date(post.published_at).toLocaleString('zh-CN') }}
                      </span>
                    </div>
                  </div>

                  <!-- 草稿分享状态 -->
                  <div v-else-if="!post.published_at" class="status-section">
                    <div class="status-item">
                      <span class="status-label">状态:</span>
                      <span class="badge badge-warning">草稿</span>
                    </div>

                    <!-- 草稿分享信息 -->
                    <div
                      v-if="post.draft_shared_with && post.draft_shared_with.length > 0"
                      class="status-item"
                    >
                      <span class="status-label">已分享给:</span>
                      <span class="status-value">{{ post.draft_shared_with.length }} 人</span>
                    </div>

                    <div v-if="post.is_draft_public" class="status-item">
                      <span class="status-label">分享设置:</span>
                      <span class="status-value">编辑可见</span>
                    </div>

                    <div v-if="post.is_accessing_others_draft" class="status-item">
                      <span class="status-label">访问类型:</span>
                      <span class="badge badge-info">他人草稿</span>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <!-- 分类选择 -->
            <div class="card">
              <div class="card-header">
                <h3>分类</h3>
              </div>
              <div class="card-body">
                <div class="checkbox-group">
                  <label
                    v-for="category in categories"
                    :key="category.id"
                    class="checkbox-item"
                    :class="{ 'checkbox-readonly': isViewOnly() }"
                  >
                    <input
                      v-model="formData.category_ids"
                      type="checkbox"
                      :value="category.id"
                      :disabled="isViewOnly()"
                    />
                    <span>{{ category.name }}</span>
                  </label>
                </div>
                <div v-if="categories.length === 0" class="empty-notice">
                  暂无分类，<router-link v-if="!isViewOnly()" to="/categories"
                    >去创建分类</router-link
                  >
                  <span v-else>暂无分类</span>
                </div>
              </div>
            </div>

            <!-- 标签选择 -->
            <div class="card">
              <div class="card-header">
                <h3>标签</h3>
              </div>
              <div class="card-body">
                <div class="checkbox-group">
                  <label
                    v-for="tag in tags"
                    :key="tag.id"
                    class="checkbox-item"
                    :class="{ 'checkbox-readonly': isViewOnly() }"
                  >
                    <input
                      v-model="formData.tag_ids"
                      type="checkbox"
                      :value="tag.id"
                      :disabled="isViewOnly()"
                    />
                    <span>{{ tag.name }}</span>
                  </label>
                </div>
                <div v-if="tags.length === 0" class="empty-notice">
                  暂无标签，<router-link v-if="!isViewOnly()" to="/tags">去创建标签</router-link>
                  <span v-else>暂无标签</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 草稿分享对话框 -->
    <div v-if="shareDialogVisible" class="modal-overlay" @click="closeShareDialog">
      <div class="modal-dialog" @click.stop>
        <div class="modal-header">
          <h3>分享草稿</h3>
          <button @click="closeShareDialog" class="modal-close">&times;</button>
        </div>

        <div class="modal-body">
          <div v-if="post" class="share-post-info">
            <h4>{{ post.title }}</h4>
            <p class="text-gray-600">选择要分享给的用户或设置分享权限</p>
          </div>

          <div class="form-group">
            <label class="form-label">
              <input v-model="shareForm.is_public" type="checkbox" class="form-checkbox" />
              允许有权限的编辑查看此草稿
            </label>
            <p class="form-help">勾选后，具有相应权限的编辑可以查看此草稿</p>
          </div>

          <div class="form-group">
            <label class="form-label">分享给特定用户</label>
            <div class="user-select-list">
              <div v-for="user in users" :key="user.id" class="user-item">
                <label class="user-label">
                  <input
                    :value="user.id"
                    v-model="shareForm.shared_with"
                    type="checkbox"
                    class="form-checkbox"
                  />
                  <div class="user-info">
                    <span class="user-name">{{ user.username }}</span>
                    <span class="user-email">{{ user.email }}</span>
                  </div>
                </label>
              </div>
            </div>
          </div>

          <div class="form-group">
            <label class="form-label">分享消息（可选）</label>
            <textarea
              v-model="shareForm.message"
              class="form-textarea"
              placeholder="为什么要分享这个草稿？"
              rows="3"
            ></textarea>
          </div>
        </div>

        <div class="modal-footer">
          <button @click="closeShareDialog" class="btn btn-secondary">取消</button>
          <button @click="shareDraft" class="btn btn-primary">确认分享</button>
        </div>
      </div>
    </div>
  </AdminLayout>
</template>

<style scoped>
.post-edit-page {
  max-width: 1200px;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: var(--space-6);
}

.page-title h1 {
  font-size: var(--text-3xl);
  font-weight: 700;
  color: var(--color-gray-900);
  margin: 0 0 var(--space-2);
}

.page-subtitle {
  color: var(--color-gray-600);
  margin: 0;
}

.loading-state,
.error-state {
  text-align: center;
  padding: var(--space-8);
}

.loading-spinner {
  width: 40px;
  height: 40px;
  border: 3px solid var(--color-gray-200);
  border-top: 3px solid var(--color-primary);
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto var(--space-4);
}

@keyframes spin {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}

.form-grid {
  display: grid;
  grid-template-columns: 1fr 300px;
  gap: var(--space-6);
}

.main-content {
  min-width: 0; /* 防止内容溢出 */
}

.content-editor {
  min-height: 500px;
  font-family: var(--font-family-mono);
  font-size: var(--text-sm);
  line-height: 1.6;
  resize: vertical;
}

.form-help {
  font-size: var(--text-sm);
  color: var(--color-gray-500);
  margin-top: var(--space-1);
}

.card-header h3 {
  font-size: var(--text-base);
  font-weight: 600;
  margin: 0;
  color: var(--color-gray-900);
}

.publish-actions {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
  margin-bottom: var(--space-4);
}

.publish-status {
  padding-top: var(--space-4);
  border-top: 1px solid var(--color-gray-200);
}

.status-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--space-2);
}

.status-label {
  font-size: var(--text-sm);
  color: var(--color-gray-600);
}

.status-value {
  font-size: var(--text-sm);
  color: var(--color-gray-900);
}

.checkbox-group {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}

.checkbox-item {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  cursor: pointer;
  padding: var(--space-2);
  border-radius: var(--radius-base);
  transition: background-color var(--transition-fast);
}

.checkbox-item:hover {
  background-color: var(--color-gray-50);
}

.checkbox-item input[type='checkbox'] {
  margin: 0;
  cursor: pointer;
}

.empty-notice {
  font-size: var(--text-sm);
  color: var(--color-gray-500);
  text-align: center;
  padding: var(--space-4);
}

.empty-notice a {
  color: var(--color-primary);
  text-decoration: none;
}

.empty-notice a:hover {
  text-decoration: underline;
}

/* 响应式设计 */
@media (max-width: 1024px) {
  .form-grid {
    grid-template-columns: 1fr;
  }

  .sidebar-content {
    order: -1;
  }

  .sidebar-content .card {
    margin-bottom: var(--space-4);
  }
}

@media (max-width: 768px) {
  .page-header {
    flex-direction: column;
    gap: var(--space-4);
  }

  .page-actions {
    width: 100%;
  }

  .content-editor {
    min-height: 300px;
  }

  .publish-actions {
    flex-direction: row;
  }
}

/* 草稿分享相关样式 */
.status-section {
  margin-bottom: var(--space-4);
}

.status-section:last-child {
  margin-bottom: 0;
}

/* 模态对话框样式 */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-dialog {
  background: white;
  border-radius: var(--border-radius-lg);
  box-shadow: var(--shadow-xl);
  max-width: 500px;
  width: 90%;
  max-height: 80vh;
  overflow-y: auto;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--space-6);
  border-bottom: 1px solid var(--color-gray-200);
}

.modal-header h3 {
  margin: 0;
  font-size: var(--text-xl);
  font-weight: 600;
}

.modal-close {
  background: none;
  border: none;
  font-size: 24px;
  cursor: pointer;
  color: var(--color-gray-500);
  padding: 0;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.modal-close:hover {
  color: var(--color-gray-700);
}

.modal-body {
  padding: var(--space-6);
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: var(--space-3);
  padding: var(--space-6);
  border-top: 1px solid var(--color-gray-200);
}

.share-post-info {
  margin-bottom: var(--space-6);
  padding: var(--space-4);
  background: var(--color-gray-50);
  border-radius: var(--border-radius-md);
}

.share-post-info h4 {
  margin: 0 0 var(--space-2);
  font-size: var(--text-lg);
  font-weight: 600;
}

.share-post-info p {
  margin: 0;
  font-size: var(--text-sm);
}

.user-select-list {
  max-height: 200px;
  overflow-y: auto;
  border: 1px solid var(--color-gray-300);
  border-radius: var(--border-radius-md);
  background: white;
}

.user-item {
  border-bottom: 1px solid var(--color-gray-200);
}

.user-item:last-child {
  border-bottom: none;
}

.user-label {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-3);
  cursor: pointer;
  transition: background-color 0.2s;
}

.user-label:hover {
  background-color: var(--color-gray-50);
}

.user-info {
  display: flex;
  flex-direction: column;
  flex: 1;
}

.user-name {
  font-weight: 500;
  color: var(--color-gray-900);
}

.user-email {
  font-size: var(--text-sm);
  color: var(--color-gray-600);
}

/* info按钮样式 */
.btn-info {
  background-color: var(--color-blue-500, #3b82f6);
  color: white;
  border-color: var(--color-blue-500, #3b82f6);
}

.btn-info:hover {
  background-color: var(--color-blue-600, #2563eb);
  border-color: var(--color-blue-600, #2563eb);
}

/* 只读模式样式 */
.form-input-readonly,
.form-textarea-readonly {
  background-color: #f8f9fa !important;
  border-color: #e9ecef !important;
  color: #6c757d !important;
  cursor: default !important;
}

.form-input-readonly:focus,
.form-textarea-readonly:focus {
  box-shadow: none !important;
  border-color: #e9ecef !important;
}

.checkbox-readonly {
  opacity: 0.7;
}

.checkbox-readonly input:disabled {
  cursor: default;
}

.readonly-notice {
  margin-bottom: 1rem;
}

.alert {
  padding: 1rem;
  border-radius: 0.375rem;
  border: 1px solid;
}

.alert-info {
  background-color: #e1f5fe;
  border-color: #81d4fa;
  color: #01579b;
}

.alert p {
  margin: 0 0 0.5rem 0;
}

.alert p:last-child {
  margin-bottom: 0;
}

.text-sm {
  font-size: 0.875rem;
}

/* 封禁状态样式 */
.ban-status {
  background-color: var(--color-red-50, #fef2f2);
  border: 1px solid var(--color-red-200, #fecaca);
  border-radius: var(--border-radius-md);
  padding: var(--space-4);
}

.ban-notice {
  margin-top: var(--space-2);
  padding: var(--space-2);
  background-color: var(--color-red-100, #fee2e2);
  border-radius: var(--border-radius-sm);
}

.badge-danger {
  background-color: var(--color-red-100, #fef2f2);
  color: var(--color-red-800, #991b1b);
  border: 1px solid var(--color-red-200, #fecaca);
}

.text-red-700 {
  color: var(--color-red-700, #b91c1c);
}
</style>
