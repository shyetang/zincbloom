<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import AdminLayout from '../../layouts/AdminLayout.vue'
import apiClient from '@/api'
import type { Category, CategoryCreate, CategoryUpdate } from '@/types'
import { useAuthStore } from '@/stores/auth'

// 状态管理
const categories = ref<Category[]>([])
const loading = ref(false)
const error = ref<string | null>(null)

// 搜索和筛选
const searchQuery = ref('')
const sortBy = ref<'name' | 'created_at'>('name')
const sortOrder = ref<'asc' | 'desc'>('asc')

// 弹窗状态
const showModal = ref(false)
const modalMode = ref<'create' | 'edit'>('create')
const editingCategory = ref<Category | null>(null)

// 表单状态
const formData = ref<CategoryCreate>({
  name: '',
})
const formErrors = ref<Record<string, string>>({})
const saving = ref(false)

// 权限检查
const authStore = useAuthStore()
const canManageCategories = computed(() => authStore.hasPermission('category:manage'))
const canCreateCategories = computed(() => authStore.hasPermission('category:create'))

// 获取分类列表
const fetchCategories = async () => {
  try {
    loading.value = true
    error.value = null
    const response = await apiClient.get('/categories')
    categories.value = response.data || []
  } catch (err) {
    console.error('获取分类列表失败:', err)
    error.value = '获取分类列表失败'
  } finally {
    loading.value = false
  }
}

// 计算属性：过滤和排序后的分类
const filteredCategories = computed(() => {
  let filtered = categories.value

  // 搜索过滤
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    filtered = filtered.filter((category) => category.name.toLowerCase().includes(query))
  }

  // 排序
  filtered.sort((a, b) => {
    let valueA: string | Date
    let valueB: string | Date

    if (sortBy.value === 'created_at') {
      valueA = new Date(a.created_at)
      valueB = new Date(b.created_at)
    } else {
      valueA = a[sortBy.value].toLowerCase()
      valueB = b[sortBy.value].toLowerCase()
    }

    if (valueA < valueB) return sortOrder.value === 'asc' ? -1 : 1
    if (valueA > valueB) return sortOrder.value === 'asc' ? 1 : -1
    return 0
  })

  return filtered
})

// 打开创建弹窗
const openCreateModal = () => {
  modalMode.value = 'create'
  editingCategory.value = null
  formData.value = {
    name: '',
  }
  formErrors.value = {}
  showModal.value = true
}

// 打开编辑弹窗
const openEditModal = (category: Category) => {
  modalMode.value = 'edit'
  editingCategory.value = category
  formData.value = {
    name: category.name,
  }
  formErrors.value = {}
  showModal.value = true
}

// 关闭弹窗
const closeModal = () => {
  showModal.value = false
  editingCategory.value = null
  formData.value = {
    name: '',
  }
  formErrors.value = {}
}

// 验证表单
const validateForm = () => {
  formErrors.value = {}
  let isValid = true

  if (!formData.value.name.trim()) {
    formErrors.value.name = '分类名称不能为空'
    isValid = false
  }

  return isValid
}

// 保存分类
const saveCategory = async () => {
  if (!validateForm()) return

  try {
    saving.value = true

    if (modalMode.value === 'create') {
      await apiClient.post('/categories', formData.value)
    } else if (editingCategory.value) {
      await apiClient.put(`/categories/${editingCategory.value.id}`, formData.value)
    }

    await fetchCategories()
    closeModal()
  } catch (err: any) {
    console.error('保存分类失败:', err)

    // 处理服务器验证错误
    if (err.response?.status === 400 && err.response?.data?.errors) {
      formErrors.value = err.response.data.errors
    } else {
      alert('保存分类失败')
    }
  } finally {
    saving.value = false
  }
}

// 删除分类
const deleteCategory = async (category: Category) => {
  if (!confirm(`确定要删除分类 "${category.name}" 吗？删除后关联的文章将变为未分类状态。`)) {
    return
  }

  try {
    await apiClient.delete(`/categories/${category.id}`)
    await fetchCategories()
  } catch (err) {
    console.error('删除分类失败:', err)
    alert('删除分类失败')
  }
}

// 格式化日期
const formatDate = (date: string) => {
  return new Date(date).toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
  })
}

onMounted(() => {
  fetchCategories()
})
</script>

<template>
  <AdminLayout>
    <div class="categories-page">
      <!-- 页面头部 -->
      <div class="page-header">
        <div class="page-title">
          <h1>分类管理</h1>
          <p class="page-subtitle">管理您的博客文章分类</p>
        </div>
        <div class="page-actions">
          <button v-if="canCreateCategories" @click="openCreateModal" class="btn btn-primary">
            <span>➕</span>
            <span>添加分类</span>
          </button>
          <div v-else class="permission-notice">
            <span>🔒 您没有创建分类的权限</span>
          </div>
        </div>
      </div>

      <!-- 搜索和筛选 -->
      <div class="card mb-4">
        <div class="card-body">
          <div class="filters-row">
            <div class="search-group">
              <input
                v-model="searchQuery"
                type="text"
                class="form-input"
                placeholder="搜索分类名称..."
              />
            </div>
            <div class="sort-group">
              <select v-model="sortBy" class="form-select">
                <option value="name">按名称排序</option>
                <option value="created_at">按创建时间排序</option>
              </select>
              <button
                @click="sortOrder = sortOrder === 'asc' ? 'desc' : 'asc'"
                class="btn btn-secondary btn-sm"
                :title="sortOrder === 'asc' ? '升序' : '降序'"
              >
                {{ sortOrder === 'asc' ? '↑' : '↓' }}
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- 分类列表 -->
      <div class="card">
        <div class="card-body">
          <div v-if="loading" class="loading-state">
            <div class="loading-spinner"></div>
            <p>加载中...</p>
          </div>

          <div v-else-if="error" class="error-state">
            <p class="error-message">{{ error }}</p>
            <button @click="fetchCategories" class="btn btn-primary btn-sm">重试</button>
          </div>

          <div v-else-if="filteredCategories.length === 0" class="empty-state">
            <div class="empty-icon">📁</div>
            <h3>{{ searchQuery ? '没有找到匹配的分类' : '暂无分类' }}</h3>
            <p>
              {{
                searchQuery
                  ? '请尝试其他搜索关键词'
                  : '还没有创建任何分类，点击上方按钮开始创建吧！'
              }}
            </p>
            <button
              v-if="!searchQuery && canCreateCategories"
              @click="openCreateModal"
              class="btn btn-primary"
            >
              创建第一个分类
            </button>
          </div>

          <div v-else class="categories-grid">
            <div v-for="category in filteredCategories" :key="category.id" class="category-card">
              <div class="category-header">
                <h3 class="category-name">{{ category.name }}</h3>
                <div class="category-actions">
                  <button
                    v-if="canManageCategories"
                    @click="openEditModal(category)"
                    class="btn btn-secondary btn-sm"
                    title="编辑"
                  >
                    ✏️
                  </button>
                  <button
                    v-if="canManageCategories"
                    @click="deleteCategory(category)"
                    class="btn btn-danger btn-sm"
                    title="删除"
                  >
                    🗑️
                  </button>
                  <div v-if="!canManageCategories" class="admin-only-notice">
                    <span title="仅管理员可编辑">🔒</span>
                  </div>
                </div>
              </div>

              <div class="category-content">
                <div class="category-slug">
                  <span class="slug-label">URL:</span>
                  <code>/{{ category.slug }}</code>
                </div>

                <div class="category-meta">
                  <div class="meta-item">
                    <span class="meta-label">创建时间:</span>
                    <span class="meta-value">{{ formatDate(category.created_at) }}</span>
                  </div>
                  <div class="meta-item">
                    <span class="meta-label">更新时间:</span>
                    <span class="meta-value">{{ formatDate(category.updated_at) }}</span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 创建/编辑弹窗 -->
      <div v-if="showModal" class="modal-overlay" @click="closeModal">
        <div class="modal-content" @click.stop>
          <div class="modal-header">
            <h2>{{ modalMode === 'create' ? '添加分类' : '编辑分类' }}</h2>
            <button @click="closeModal" class="modal-close">✕</button>
          </div>

          <form @submit.prevent="saveCategory" class="modal-body">
            <div class="form-group">
              <label class="form-label">分类名称 *</label>
              <input
                v-model="formData.name"
                type="text"
                class="form-input"
                :class="{ error: formErrors.name }"
                placeholder="输入分类名称..."
                required
              />
              <div v-if="formErrors.name" class="form-error">
                {{ formErrors.name }}
              </div>
            </div>

            <div class="modal-actions">
              <button type="button" @click="closeModal" class="btn btn-secondary">取消</button>
              <button type="submit" class="btn btn-primary" :disabled="saving">
                {{ saving ? '保存中...' : '保存' }}
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  </AdminLayout>
</template>

<style scoped>
.categories-page {
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

.filters-row {
  display: grid;
  grid-template-columns: 1fr auto;
  gap: var(--space-4);
  align-items: center;
}

.search-group {
  display: flex;
  align-items: center;
}

.sort-group {
  display: flex;
  gap: var(--space-2);
  align-items: center;
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

.categories-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  gap: var(--space-4);
}

.category-card {
  background: var(--color-white);
  border: 1px solid var(--color-gray-200);
  border-radius: var(--radius-lg);
  padding: var(--space-4);
  transition: all var(--transition-fast);
}

.category-card:hover {
  border-color: var(--color-gray-300);
  box-shadow: var(--shadow-md);
}

.category-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: var(--space-3);
}

.category-name {
  font-size: var(--text-lg);
  font-weight: 600;
  color: var(--color-gray-900);
  margin: 0;
  flex: 1;
}

.category-actions {
  display: flex;
  gap: var(--space-2);
}

.category-actions .btn {
  padding: var(--space-1) var(--space-2);
  min-width: 28px;
}

.category-slug {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  margin-bottom: var(--space-3);
}

.slug-label {
  font-size: var(--text-sm);
  color: var(--color-gray-600);
}

.category-slug code {
  background: var(--color-gray-100);
  padding: var(--space-1) var(--space-2);
  border-radius: var(--radius-base);
  font-size: var(--text-sm);
  font-family: var(--font-family-mono);
}

.category-meta {
  border-top: 1px solid var(--color-gray-200);
  padding-top: var(--space-3);
}

.meta-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--space-1);
}

.meta-item:last-child {
  margin-bottom: 0;
}

.meta-label {
  font-size: var(--text-xs);
  color: var(--color-gray-500);
}

.meta-value {
  font-size: var(--text-xs);
  color: var(--color-gray-700);
}

/* 弹窗样式 */
.modal-overlay {
  position: fixed;
  inset: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  padding: var(--space-4);
}

.modal-content {
  background: var(--color-white);
  border-radius: var(--radius-lg);
  max-width: 500px;
  width: 100%;
  max-height: 90vh;
  overflow-y: auto;
  box-shadow: var(--shadow-xl);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--space-6);
  border-bottom: 1px solid var(--color-gray-200);
}

.modal-header h2 {
  font-size: var(--text-xl);
  font-weight: 600;
  margin: 0;
  color: var(--color-gray-900);
}

.modal-close {
  background: none;
  border: none;
  font-size: var(--text-xl);
  cursor: pointer;
  color: var(--color-gray-500);
  padding: var(--space-1);
}

.modal-close:hover {
  color: var(--color-gray-700);
}

/* 权限相关样式 */
.permission-notice {
  color: var(--color-gray-600);
  font-size: var(--text-sm);
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-3);
  background: var(--color-gray-100);
  border-radius: var(--radius-base);
  border: 1px solid var(--color-gray-200);
}

.admin-only-notice {
  display: flex;
  align-items: center;
  color: var(--color-gray-500);
  font-size: var(--text-sm);
}

.admin-only-notice span {
  cursor: help;
}

.modal-body {
  padding: var(--space-6);
}

.form-input.error {
  border-color: var(--color-error);
}

.form-help {
  font-size: var(--text-sm);
  color: var(--color-gray-500);
  margin-top: var(--space-1);
}

.modal-actions {
  display: flex;
  gap: var(--space-3);
  justify-content: flex-end;
  margin-top: var(--space-6);
}

/* 响应式设计 */
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

  .filters-row {
    grid-template-columns: 1fr;
    gap: var(--space-3);
  }

  .categories-grid {
    grid-template-columns: 1fr;
  }

  .category-header {
    flex-direction: column;
    gap: var(--space-2);
    align-items: flex-start;
  }

  .category-actions {
    align-self: stretch;
    justify-content: space-between;
  }

  .modal-overlay {
    padding: var(--space-2);
  }

  .modal-actions {
    flex-direction: column;
  }

  .modal-actions .btn {
    width: 100%;
  }
}
</style>
