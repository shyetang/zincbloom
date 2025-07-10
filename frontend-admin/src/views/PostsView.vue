<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useRouter } from 'vue-router'
import AdminLayout from '../../layouts/AdminLayout.vue'
import apiClient from '@/api'
import type { Post, PostFilters, Category, Tag, User, ShareDraftPayload } from '@/types'
import { useAuthStore } from '@/stores/auth'

const authStore = useAuthStore()

const router = useRouter()

// 状态管理
const posts = ref<Post[]>([])
const categories = ref<Category[]>([])
const tags = ref<Tag[]>([])
const users = ref<User[]>([]) // 用于草稿分享的用户列表
const loading = ref(false)
const error = ref<string | null>(null)

// 草稿分享相关状态
const shareDialogVisible = ref(false)
const selectedPost = ref<Post | null>(null)
const shareForm = ref<ShareDraftPayload>({
  shared_with: [],
  is_public: false,
  message: '',
})

// 移除了管理员紧急访问功能，保护用户隐私

// 搜索和筛选
const filters = ref<PostFilters>({
  query: '',
  status: 'all',
  category_id: '',
  tag_id: '',
  sort_by: 'created_at',
  sort_order: 'desc',
})

// 分页
const currentPage = ref(1)
const perPage = ref(10)
const total = ref(0)

// 获取文章列表
const fetchPosts = async () => {
  try {
    loading.value = true
    error.value = null

    const params = new URLSearchParams()
    if (filters.value.query) params.append('q', filters.value.query)
    if (filters.value.category_id) params.append('category_id', filters.value.category_id)
    if (filters.value.tag_id) params.append('tag_id', filters.value.tag_id)
    if (filters.value.status !== 'all') {
      if (filters.value.status === 'published') {
        params.append('published', 'true')
      } else if (filters.value.status === 'draft') {
        params.append('published', 'false')
      }
    }
    params.append('page', currentPage.value.toString())
    params.append('per_page', perPage.value.toString())
    params.append('sort_by', filters.value.sort_by || 'created_at')
    params.append('sort_order', filters.value.sort_order || 'desc')

    const response = await apiClient.get(`/posts?${params.toString()}`)

    // 后端返回的是 PaginatedResponse<PostDetailDto> 结构，字段名为 items
    if (response.data && response.data.items) {
      posts.value = response.data.items || []
      total.value = response.data.total_items || 0
    } else {
      posts.value = []
      total.value = 0
    }
  } catch (err) {
    console.error('获取文章列表失败:', err)
    error.value = '获取文章列表失败'
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

// 删除文章
const deletePost = async (post: Post) => {
  if (!confirm(`确定要删除文章 "${post.title}" 吗？`)) {
    return
  }

  try {
    await apiClient.delete(`/posts/${post.id}`)
    await fetchPosts()
  } catch (err) {
    console.error('删除文章失败:', err)
    alert('删除文章失败')
  }
}

// 封禁文章
const banPost = async (post: Post) => {
  if (!confirm(`确定要封禁文章 "${post.title}" 吗？\n封禁后只有作者和管理员可以查看此文章。`)) {
    return
  }

  try {
    await apiClient.put(`/posts/${post.id}/ban`, { reason: '违规内容' })
    await fetchPosts()
    alert('文章已被封禁')
  } catch (err) {
    console.error('封禁文章失败:', err)
    alert('封禁文章失败')
  }
}

// 解封文章
const unbanPost = async (post: Post) => {
  if (!confirm(`确定要解封文章 "${post.title}" 吗？`)) {
    return
  }

  try {
    await apiClient.put(`/posts/${post.id}/unban`)
    await fetchPosts()
    alert('文章已被解封')
  } catch (err) {
    console.error('解封文章失败:', err)
    alert('解封文章失败')
  }
}

// 切换发布状态
const togglePublish = async (post: Post) => {
  try {
    const endpoint = post.published_at ? `/posts/${post.id}/unpublish` : `/posts/${post.id}/publish`
    await apiClient.put(endpoint)
    await fetchPosts()
  } catch (err) {
    console.error('更新发布状态失败:', err)
    alert('更新发布状态失败')
  }
}

// 打开草稿分享对话框
const openShareDialog = (post: Post) => {
  selectedPost.value = post
  shareForm.value = {
    shared_with: (post.draft_shared_with || []).map((user) => user.id),
    is_public: post.is_draft_public || false,
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
  selectedPost.value = null
  shareForm.value = {
    shared_with: [],
    is_public: false,
    message: '',
  }
}

// 分享草稿
const shareDraft = async () => {
  if (!selectedPost.value) return

  try {
    await apiClient.put(`/posts/${selectedPost.value.id}/share`, shareForm.value)
    await fetchPosts() // 刷新列表
    closeShareDialog()
    alert('草稿分享设置已更新！')
  } catch (err) {
    console.error('分享草稿失败:', err)
    alert('分享草稿失败')
  }
}

// 重置筛选
const resetFilters = () => {
  filters.value = {
    query: '',
    status: 'all',
    category_id: '',
    tag_id: '',
    sort_by: 'created_at',
    sort_order: 'desc',
  }
  currentPage.value = 1
  fetchPosts()
}

// 计算属性
const filteredPosts = computed(() => {
  // 管理员可以看到所有文章列表（包括他人私人草稿），但不能查看详情
  if (authStore.hasAnyPermission(['post:read_any', 'post:manage_any'])) {
    return posts.value
  }

  // 普通用户只能看到有权访问的文章
  return posts.value.filter((post) => {
    // 对于已发布的文章，所有人都可以看到
    if (post.published_at) return true

    // 对于草稿，只显示用户有权访问的
    return authStore.canAccessDraft(post)
  })
})

// 注意：移除了受限草稿列表功能
// 根据草稿隐私保护原则，管理员不应该能够看到他人草稿的任何信息
// 如果需要紧急访问，应该通过其他途径（如用户举报、系统监控等）获得具体的文章ID

const formatDate = (date: string) => {
  return new Date(date).toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  })
}

const getStatusBadge = (post: Post) => {
  if (post.is_banned) return 'danger'
  return post.published_at ? 'success' : 'warning'
}

const getStatusText = (post: Post) => {
  if (post.is_banned) return '已封禁'
  if (post.published_at) return '已发布'

  // 草稿状态的细分显示
  if (post.is_accessing_others_draft) return '他人草稿'
  if (post.draft_shared_with && post.draft_shared_with.length > 0) return '已分享草稿'
  if (post.is_draft_public) return '公开草稿'
  return '私人草稿'
}

// 获取草稿分享信息显示文本
const getDraftShareInfo = (post: Post) => {
  if (post.published_at) return ''

  const parts = []
  if (post.draft_shared_with && post.draft_shared_with.length > 0) {
    parts.push(`分享给${post.draft_shared_with.length}人`)
  }
  if (post.is_draft_public) {
    parts.push('编辑可见')
  }
  if (post.is_accessing_others_draft) {
    parts.push('访问他人草稿')
  }

  return parts.join(' • ')
}

// 简化的权限检查函数 - 直接使用后端返回的权限字段
const canEditPost = (post: Post) => {
  return post.can_edit
}

const canDeletePost = (post: Post) => {
  return post.can_delete
}

const canSharePost = (post: Post) => {
  return (
    !post.published_at && // 只有草稿可以分享
    authStore.canShareDrafts() &&
    post.author_id === authStore.user?.id
  ) // 分享权限仍然需要前端检查，因为这是业务逻辑
}

const canPublishPost = (post: Post) => {
  return post.can_publish
}

const canViewPostDetail = (post: Post) => {
  return post.can_view_detail
}

// 移除了紧急访问功能，用户可以通过分享草稿的方式让管理员查看

onMounted(() => {
  fetchPosts()
  fetchMetadata()
})
</script>

<template>
  <AdminLayout>
    <div class="posts-page">
      <!-- 页面头部 -->
      <div class="page-header">
        <div class="page-title">
          <h1>文章管理</h1>
          <p class="page-subtitle">管理您的博客文章内容</p>
        </div>
        <div class="page-actions">
          <router-link to="/posts/new" class="btn btn-primary">
            <span>✍️</span>
            <span>写新文章</span>
          </router-link>
        </div>
      </div>

      <!-- 搜索和筛选 -->
      <div class="card mb-4">
        <div class="card-body">
          <div class="filters-grid">
            <div class="filter-group">
              <label class="form-label">搜索文章</label>
              <input
                v-model="filters.query"
                type="text"
                class="form-input"
                placeholder="输入标题关键词..."
                @input="fetchPosts"
              />
            </div>

            <div class="filter-group">
              <label class="form-label">状态</label>
              <select v-model="filters.status" class="form-select" @change="fetchPosts">
                <option value="all">全部</option>
                <option value="published">已发布</option>
                <option value="draft">草稿</option>
              </select>
            </div>

            <div class="filter-group">
              <label class="form-label">分类</label>
              <select v-model="filters.category_id" class="form-select" @change="fetchPosts">
                <option value="">全部分类</option>
                <option v-for="category in categories" :key="category.id" :value="category.id">
                  {{ category.name }}
                </option>
              </select>
            </div>

            <div class="filter-group">
              <label class="form-label">标签</label>
              <select v-model="filters.tag_id" class="form-select" @change="fetchPosts">
                <option value="">全部标签</option>
                <option v-for="tag in tags" :key="tag.id" :value="tag.id">
                  {{ tag.name }}
                </option>
              </select>
            </div>

            <div class="filter-actions">
              <button @click="resetFilters" class="btn btn-secondary btn-sm">重置筛选</button>
            </div>
          </div>
        </div>
      </div>

      <!-- 文章列表 -->
      <div class="card">
        <div class="card-body">
          <div v-if="loading" class="loading-state">
            <div class="loading-spinner"></div>
            <p>加载中...</p>
          </div>

          <div v-else-if="error" class="error-state">
            <p class="error-message">{{ error }}</p>
            <button @click="fetchPosts" class="btn btn-primary btn-sm">重试</button>
          </div>

          <div v-else-if="filteredPosts.length === 0" class="empty-state">
            <div class="empty-icon">📝</div>
            <h3>暂无文章</h3>
            <p>还没有创建任何文章，点击上方按钮开始写作吧！</p>
            <router-link to="/posts/new" class="btn btn-primary"> 写第一篇文章 </router-link>
          </div>

          <div v-else class="posts-table-container">
            <table class="table">
              <thead>
                <tr>
                  <th>标题</th>
                  <th>作者</th>
                  <th>状态</th>
                  <th>分类</th>
                  <th>标签</th>
                  <th>创建时间</th>
                  <th>更新时间</th>
                  <th>操作</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="post in filteredPosts" :key="post.id">
                  <td>
                    <div class="post-title-cell">
                      <router-link
                        v-if="canViewPostDetail(post)"
                        :to="`/posts/${post.id}/edit`"
                        class="post-title-link"
                      >
                        {{ post.title }}
                      </router-link>
                      <span
                        v-else
                        class="post-title-text"
                        :title="
                          post.is_accessing_others_draft
                            ? '管理员无法查看他人私有草稿详情'
                            : '无查看权限'
                        "
                      >
                        {{ post.title }}
                      </span>
                      <div class="post-slug">{{ post.slug }}</div>
                    </div>
                  </td>
                  <td>
                    <div class="author-cell">
                      <span v-if="post.author" class="author-name">
                        {{ post.author.username }}
                      </span>
                      <span v-else class="text-gray-500">未知作者</span>
                    </div>
                  </td>
                  <td>
                    <div class="status-cell">
                      <span class="badge" :class="`badge-${getStatusBadge(post)}`">
                        {{ getStatusText(post) }}
                      </span>
                      <div v-if="getDraftShareInfo(post)" class="draft-share-info">
                        {{ getDraftShareInfo(post) }}
                      </div>
                    </div>
                  </td>
                  <td>
                    <span v-if="post.categories && post.categories.length">
                      {{ post.categories.map((c) => c.name).join(', ') }}
                    </span>
                    <span v-else class="text-gray-500">未分类</span>
                  </td>
                  <td>
                    <span v-if="post.tags && post.tags.length">
                      {{ post.tags.map((t) => t.name).join(', ') }}
                    </span>
                    <span v-else class="text-gray-500">无标签</span>
                  </td>
                  <td>{{ formatDate(post.created_at) }}</td>
                  <td>{{ formatDate(post.updated_at) }}</td>
                  <td>
                    <div class="action-buttons">
                      <!-- 编辑按钮 -->
                      <router-link
                        v-if="canEditPost(post)"
                        :to="`/posts/${post.id}/edit`"
                        class="btn btn-secondary btn-sm"
                        title="编辑"
                      >
                        ✏️
                      </router-link>

                      <!-- 草稿分享按钮 -->
                      <button
                        v-if="canSharePost(post)"
                        @click="openShareDialog(post)"
                        class="btn btn-info btn-sm"
                        title="分享草稿"
                      >
                        🔗
                      </button>

                      <!-- 发布/撤回按钮 -->
                      <button
                        v-if="canPublishPost(post)"
                        @click="togglePublish(post)"
                        class="btn btn-sm"
                        :class="post.published_at ? 'btn-warning' : 'btn-primary'"
                        :title="post.published_at ? '撤回发布' : '发布文章'"
                      >
                        {{ post.published_at ? '📥' : '📤' }}
                      </button>

                      <!-- 封禁/解封按钮 -->
                      <button
                        v-if="post.can_ban && !post.is_banned"
                        @click="banPost(post)"
                        class="btn btn-warning btn-sm"
                        title="封禁文章"
                      >
                        🚫
                      </button>

                      <button
                        v-if="post.can_unban && post.is_banned"
                        @click="unbanPost(post)"
                        class="btn btn-success btn-sm"
                        title="解封文章"
                      >
                        ✅
                      </button>

                      <!-- 删除按钮 -->
                      <button
                        v-if="canDeletePost(post)"
                        @click="deletePost(post)"
                        class="btn btn-danger btn-sm"
                        title="删除"
                      >
                        🗑️
                      </button>
                    </div>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>

          <!-- 分页 -->
          <div v-if="filteredPosts.length > 0" class="pagination">
            <div class="pagination-info">
              显示 {{ (currentPage - 1) * perPage + 1 }}-{{
                Math.min(currentPage * perPage, total)
              }}
              条， 共 {{ total }} 条记录
            </div>
            <!-- 这里可以添加分页组件 -->
          </div>
        </div>
      </div>

      <!-- 提示：移除了紧急访问功能，保护用户隐私 -->
    </div>

    <!-- 草稿分享对话框 -->
    <div v-if="shareDialogVisible" class="modal-overlay" @click="closeShareDialog">
      <div class="modal-dialog" @click.stop>
        <div class="modal-header">
          <h3>分享草稿</h3>
          <button @click="closeShareDialog" class="modal-close">&times;</button>
        </div>

        <div class="modal-body">
          <div v-if="selectedPost" class="share-post-info">
            <h4>{{ selectedPost.title }}</h4>
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

    <!-- 移除了紧急访问对话框，保护用户隐私 -->
  </AdminLayout>
</template>

<style scoped>
.posts-page {
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

.page-actions .btn {
  display: flex;
  align-items: center;
  gap: var(--space-2);
}

.filters-grid {
  display: grid;
  grid-template-columns: 2fr 1fr 1fr 1fr auto;
  gap: var(--space-4);
  align-items: end;
}

.filter-group {
  display: flex;
  flex-direction: column;
}

.filter-actions {
  display: flex;
  align-items: end;
}

.loading-state,
.error-state,
.empty-state {
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

.empty-icon {
  font-size: 4rem;
  margin-bottom: var(--space-4);
}

.empty-state h3 {
  margin: 0 0 var(--space-2);
  color: var(--color-gray-700);
}

.empty-state p {
  color: var(--color-gray-600);
  margin: 0 0 var(--space-6);
}

.posts-table-container {
  overflow-x: auto;
}

.post-title-cell {
  min-width: 200px;
}

.post-title-link {
  font-weight: 600;
  color: var(--color-gray-900);
  text-decoration: none;
  display: block;
  margin-bottom: var(--space-1);
}

.post-title-link:hover {
  color: var(--color-primary);
}

.post-slug {
  font-size: var(--text-sm);
  color: var(--color-gray-500);
  font-family: var(--font-family-mono);
}

.author-cell {
  min-width: 120px;
}

.author-name {
  font-weight: 500;
  color: var(--color-gray-800);
}

.action-buttons {
  display: flex;
  gap: var(--space-2);
}

.action-buttons .btn {
  min-width: 32px;
  padding: var(--space-2);
}

.pagination {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: var(--space-6);
  padding-top: var(--space-4);
  border-top: 1px solid var(--color-gray-200);
}

.pagination-info {
  font-size: var(--text-sm);
  color: var(--color-gray-600);
}

/* 响应式设计 */
@media (max-width: 1024px) {
  .filters-grid {
    grid-template-columns: 1fr;
    gap: var(--space-3);
  }

  .filter-actions {
    justify-content: center;
  }

  .posts-table-container {
    font-size: var(--text-sm);
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

  .page-actions .btn {
    width: 100%;
    justify-content: center;
  }

  .action-buttons {
    flex-direction: column;
  }

  .action-buttons .btn {
    width: 100%;
    min-width: auto;
  }
}

/* 草稿状态和分享相关样式 */
.status-cell {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
}

.draft-share-info {
  font-size: var(--text-xs);
  color: var(--color-gray-500);
  font-style: italic;
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

.form-help {
  font-size: var(--text-sm);
  color: var(--color-gray-600);
  margin-top: var(--space-1);
  margin-bottom: 0;
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

/* 移除了紧急访问相关样式，简化界面设计 */

/* warning按钮样式 */
.btn-warning {
  background-color: var(--color-yellow-500, #eab308);
  color: var(--color-gray-900);
  border-color: var(--color-yellow-500, #eab308);
}

.btn-warning:hover {
  background-color: var(--color-yellow-600, #ca8a04);
  border-color: var(--color-yellow-600, #ca8a04);
}

/* danger按钮样式 */
.btn-danger:disabled {
  background-color: var(--color-gray-300);
  color: var(--color-gray-500);
  border-color: var(--color-gray-300);
  cursor: not-allowed;
}

/* 不可点击的标题样式 */
.post-title-text {
  font-weight: 600;
  color: #9ca3af; /* 灰色表示不可点击 */
  cursor: not-allowed;
}

/* 状态徽章样式 */
.badge-danger {
  background-color: var(--color-red-100, #fef2f2);
  color: var(--color-red-800, #991b1b);
  border: 1px solid var(--color-red-200, #fecaca);
}

/* 封禁文章的行样式 */
tr:has(.badge-danger) {
  background-color: var(--color-red-50, #fef2f2);
}
</style>
