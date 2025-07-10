<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import AdminLayout from '../../layouts/AdminLayout.vue'
import apiClient from '@/api'
import type {
  Category,
  CategoryCreate,
  CategoryUpdate,
  CategoryUsageStats,
  MergeCategoriesPreviewResponse,
  MergeCategoriesResponse,
  SimilarCategoryGroup,
} from '@/types'
import { useAuthStore } from '@/stores/auth'

// 分类统计接口
interface CategoryUsageStatsLocal {
  category: Category
  post_count: number
}

// 合并预览接口
interface MergePreview {
  target_category: Category
  source_categories: Category[]
  total_posts_affected: number
  posts_with_duplicates: number
  posts_by_category: Array<{
    category: Category
    post_count: number
    sample_post_titles: string[]
  }>
  potential_issues: string[]
}

// 合并结果接口
interface MergeResult {
  target_category: Category
  merged_category_count: number
  affected_post_count: number
  duplicate_relations_removed: number
  operation_summary: string
}

// 移除本地SimilarCategoryGroup声明，使用从types导入的版本

// 状态管理
const categories = ref<Category[]>([])
const categoryStats = ref<CategoryUsageStatsLocal[]>([])
const loading = ref(false)
const error = ref<string | null>(null)

// 搜索和筛选
const searchQuery = ref('')
const sortBy = ref<'name' | 'created_at' | 'post_count'>('name')
const sortOrder = ref<'asc' | 'desc'>('asc')
const viewMode = ref<'simple' | 'stats'>('simple')

// 弹窗状态
const showModal = ref(false)
const modalMode = ref<'create' | 'edit'>('create')
const editingCategory = ref<Category | null>(null)

// 合并功能状态
const mergeMode = ref(false)
const selectedCategories = ref<Set<string>>(new Set())
const targetCategoryId = ref<string | null>(null)
const showMergePreview = ref(false)
const mergePreview = ref<MergePreview | null>(null)
const mergeLoading = ref(false)
const newTargetName = ref('')

// 相似分类
const similarCategories = ref<SimilarCategoryGroup[]>([])
const showSimilarCategories = ref(false)

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

// 获取分类使用统计
const fetchCategoryStats = async () => {
  if (!canManageCategories.value) return

  try {
    loading.value = true
    error.value = null
    const response = await apiClient.get('/admin/categories/usage-stats')
    categoryStats.value = response.data || []
  } catch (err) {
    console.error('获取分类统计失败:', err)
    error.value = '获取分类统计失败'
  } finally {
    loading.value = false
  }
}

// 切换视图模式
const switchViewMode = async (mode: 'simple' | 'stats') => {
  viewMode.value = mode
  if (mode === 'stats' && categoryStats.value.length === 0) {
    await fetchCategoryStats()
  } else if (mode === 'simple' && categories.value.length === 0) {
    await fetchCategories()
  }
}

// 计算属性：当前显示的数据
const currentData = computed(() => {
  if (viewMode.value === 'stats') {
    return categoryStats.value
  } else {
    return categories.value.map((category) => ({
      category,
      post_count: 0, // 简单视图不显示计数
    }))
  }
})

// 计算属性：过滤和排序后的分类
const filteredCategories = computed(() => {
  let filtered = currentData.value

  // 搜索过滤
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    filtered = filtered.filter((item) => item.category.name.toLowerCase().includes(query))
  }

  // 排序
  filtered.sort((a, b) => {
    let valueA: string | Date | number
    let valueB: string | Date | number

    if (sortBy.value === 'created_at') {
      valueA = new Date(a.category.created_at)
      valueB = new Date(b.category.created_at)
    } else if (sortBy.value === 'post_count') {
      valueA = a.post_count
      valueB = b.post_count
    } else {
      valueA = a.category[sortBy.value].toLowerCase()
      valueB = b.category[sortBy.value].toLowerCase()
    }

    if (valueA < valueB) return sortOrder.value === 'asc' ? -1 : 1
    if (valueA > valueB) return sortOrder.value === 'asc' ? 1 : -1
    return 0
  })

  return filtered
})

// 合并模式相关函数
const toggleMergeMode = () => {
  mergeMode.value = !mergeMode.value
  if (!mergeMode.value) {
    clearMergeSelection()
  }
}

// 清空合并选择
const clearMergeSelection = () => {
  selectedCategories.value.clear()
  targetCategoryId.value = null
  showMergePreview.value = false
  mergePreview.value = null
  newTargetName.value = ''
}

const selectCategory = (categoryId: string) => {
  if (targetCategoryId.value === categoryId) {
    targetCategoryId.value = null
  } else if (selectedCategories.value.has(categoryId)) {
    selectedCategories.value.delete(categoryId)
  } else if (!targetCategoryId.value) {
    targetCategoryId.value = categoryId
  } else {
    selectedCategories.value.add(categoryId)
  }
}

const canMerge = computed(() => {
  return mergeMode.value && targetCategoryId.value && selectedCategories.value.size > 0
})

// 获取合并预览
const getMergePreview = async () => {
  if (!canMerge.value) return

  try {
    mergeLoading.value = true
    const response = await apiClient.post('/admin/categories/merge-preview', {
      target_category_id: targetCategoryId.value,
      source_category_ids: Array.from(selectedCategories.value),
    })
    mergePreview.value = response.data
    showMergePreview.value = true
  } catch (err) {
    console.error('获取合并预览失败:', err)
    alert('获取合并预览失败')
  } finally {
    mergeLoading.value = false
  }
}

// 执行合并（带确认）
const executeMergeWithConfirm = async () => {
  if (!targetCategoryId.value || selectedCategories.value.size === 0) {
    alert('请先选择目标分类和至少一个源分类')
    return
  }

  const sourceCategoryNames = Array.from(selectedCategories.value)
    .map((id) => filteredCategories.value.find((item) => item.category.id === id)?.category.name)
    .filter(Boolean)

  const targetCategoryName = filteredCategories.value.find(
    (item) => item.category.id === targetCategoryId.value,
  )?.category.name

  if (
    !confirm(
      `确认要将分类 "${sourceCategoryNames.join(', ')}" 合并到 "${targetCategoryName}" 吗？此操作不可撤销！`,
    )
  ) {
    return
  }

  await doExecuteMerge()
}

// 执行合并（跳过确认）
const executeMerge = async () => {
  if (!mergePreview.value) return
  await doExecuteMerge()
}

// 实际执行合并的内部函数
const doExecuteMerge = async () => {
  try {
    mergeLoading.value = true
    const response = await apiClient.post('/admin/categories/merge-enhanced', {
      target_category_id: targetCategoryId.value,
      source_category_ids: Array.from(selectedCategories.value),
      new_target_name: newTargetName.value.trim() || null,
    })

    const result: MergeResult = response.data
    alert(`合并成功！\n${result.operation_summary}`)

    // 重新加载数据
    clearMergeSelection()
    toggleMergeMode()
    if (viewMode.value === 'stats') {
      await fetchCategoryStats()
    } else {
      await fetchCategories()
    }

    showMergePreview.value = false
  } catch (err: any) {
    console.error('分类合并失败:', err)
    alert('分类合并失败: ' + (err.response?.data?.message || err.message))
  } finally {
    mergeLoading.value = false
  }
}

// 查找相似分类
const findSimilarCategories = async () => {
  try {
    loading.value = true
    const response = await apiClient.get('/admin/categories/similar')
    similarCategories.value = response.data || []
    showSimilarCategories.value = true
  } catch (err) {
    console.error('查找相似分类失败:', err)
    alert('查找相似分类失败')
  } finally {
    loading.value = false
  }
}

// 应用相似分类建议
const applySimilarSuggestion = (group: SimilarCategoryGroup) => {
  if (group.categories.length < 2) return

  // 设置第一个为目标分类，其余为源分类
  targetCategoryId.value = group.categories[0].id
  selectedCategories.value.clear()

  for (let i = 1; i < group.categories.length; i++) {
    selectedCategories.value.add(group.categories[i].id)
  }

  showSimilarCategories.value = false
  mergeMode.value = true
}

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
const openEditModal = (item: CategoryUsageStatsLocal) => {
  const category = item.category
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
const deleteCategory = async (item: CategoryUsageStatsLocal) => {
  const category = item.category
  if (!confirm(`确定要删除分类 "${category.name}" 吗？删除后关联的文章将变为未分类状态。`)) {
    return
  }

  try {
    await apiClient.delete(`/categories/${category.id}`)
    if (viewMode.value === 'stats') {
      await fetchCategoryStats()
    } else {
      await fetchCategories()
    }
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

      <!-- 管理员工具栏 -->
      <div v-if="canManageCategories" class="admin-toolbar card mb-4">
        <div class="card-body">
          <div class="toolbar-section">
            <h3>🛠️ 管理工具</h3>
            <div class="toolbar-actions">
              <!-- 视图切换 -->
              <div class="view-switcher">
                <button
                  @click="switchViewMode('simple')"
                  :class="['btn', viewMode === 'simple' ? 'btn-primary' : 'btn-secondary']"
                >
                  📋 简单视图
                </button>
                <button
                  @click="switchViewMode('stats')"
                  :class="['btn', viewMode === 'stats' ? 'btn-primary' : 'btn-secondary']"
                >
                  📊 统计视图
                </button>
              </div>

              <!-- 合并工具 -->
              <div class="merge-tools">
                <button
                  @click="toggleMergeMode"
                  :class="['btn', mergeMode ? 'btn-warning' : 'btn-secondary']"
                >
                  {{ mergeMode ? '🚫 退出合并' : '🔄 分类合并' }}
                </button>
                <button @click="findSimilarCategories" class="btn btn-info">🔍 查找相似</button>
              </div>
            </div>
          </div>

          <!-- 合并模式状态栏 -->
          <div v-if="mergeMode" class="merge-status">
            <div class="merge-info">
              <div class="selection-status">
                <span class="target-status">
                  🎯 目标分类:
                  {{
                    targetCategoryId
                      ? filteredCategories.find((item) => item.category.id === targetCategoryId)
                          ?.category.name
                      : '未选择'
                  }}
                </span>
                <span class="source-status">
                  📦 源分类:
                  {{
                    selectedCategories.size > 0
                      ? Array.from(selectedCategories)
                          .map(
                            (id) =>
                              filteredCategories.find((item) => item.category.id === id)?.category
                                .name,
                          )
                          .join(', ')
                      : '未选择'
                  }}
                </span>
              </div>

              <div class="merge-actions">
                <input
                  v-model="newTargetName"
                  type="text"
                  placeholder="新分类名称（可选）"
                  class="form-input"
                />
                <button
                  @click="getMergePreview"
                  :disabled="!canMerge || mergeLoading"
                  class="btn btn-info btn-sm"
                >
                  {{ mergeLoading ? '分析中...' : '📋 预览合并' }}
                </button>
                <button
                  @click="executeMergeWithConfirm"
                  :disabled="!canMerge || mergeLoading"
                  class="btn btn-danger btn-sm"
                >
                  {{ mergeLoading ? '执行中...' : '⚡ 执行合并' }}
                </button>
                <button @click="clearMergeSelection" class="btn btn-secondary btn-sm">
                  🧹 清空选择
                </button>
              </div>
            </div>

            <div class="merge-help">
              <p><strong>📖 操作说明：</strong></p>
              <ul>
                <li>🎯 <strong>绿色边框</strong>：目标分类（将保留）</li>
                <li>🔵 <strong>蓝色边框</strong>：源分类（将被合并删除）</li>
                <li>📝 点击分类卡片进行选择，先选择目标分类，再选择源分类</li>
                <li>⚠️ 合并操作不可撤销，建议先使用预览功能</li>
              </ul>
            </div>
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
                <option value="post_count" v-if="viewMode === 'stats'">按文章数量排序</option>
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
            <button
              @click="viewMode === 'stats' ? fetchCategoryStats() : fetchCategories()"
              class="btn btn-primary btn-sm"
            >
              重试
            </button>
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
            <div
              v-for="item in filteredCategories"
              :key="item.category.id"
              :class="[
                'category-card',
                {
                  'target-category': mergeMode && targetCategoryId === item.category.id,
                  'source-category': mergeMode && selectedCategories.has(item.category.id),
                  selectable: mergeMode,
                  clickable: mergeMode,
                },
              ]"
              @click="mergeMode ? selectCategory(item.category.id) : null"
            >
              <div class="category-header">
                <h3 class="category-name">
                  {{ item.category.name }}
                  <span v-if="viewMode === 'stats'" class="post-count-badge">
                    {{ item.post_count }}篇
                  </span>
                </h3>
                <div class="category-actions">
                  <button
                    v-if="canManageCategories && !mergeMode"
                    @click="openEditModal(item)"
                    class="btn btn-secondary btn-sm"
                    title="编辑"
                  >
                    ✏️
                  </button>
                  <button
                    v-if="canManageCategories && !mergeMode"
                    @click="deleteCategory(item)"
                    class="btn btn-danger btn-sm"
                    title="删除"
                  >
                    🗑️
                  </button>
                  <div v-if="!canManageCategories" class="admin-only-notice">
                    <span title="仅管理员可编辑">🔒</span>
                  </div>

                  <!-- 合并模式选择指示器 -->
                  <div v-if="mergeMode" class="merge-indicator">
                    <span v-if="targetCategoryId === item.category.id" class="target-indicator"
                      >🎯</span
                    >
                    <span
                      v-else-if="selectedCategories.has(item.category.id)"
                      class="source-indicator"
                      >📦</span
                    >
                    <span v-else class="selectable-indicator">👆</span>
                  </div>
                </div>
              </div>

              <div class="category-content">
                <div class="category-slug">
                  <span class="slug-label">URL:</span>
                  <code>/{{ item.category.slug }}</code>
                </div>

                <div class="category-meta">
                  <div class="meta-item">
                    <span class="meta-label">创建时间:</span>
                    <span class="meta-value">{{ formatDate(item.category.created_at) }}</span>
                  </div>
                  <div class="meta-item">
                    <span class="meta-label">更新时间:</span>
                    <span class="meta-value">{{ formatDate(item.category.updated_at) }}</span>
                  </div>
                  <div v-if="viewMode === 'stats'" class="meta-item">
                    <span class="meta-label">关联文章:</span>
                    <span class="meta-value">{{ item.post_count }}篇</span>
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

      <!-- 合并预览弹窗 -->
      <div v-if="showMergePreview" class="modal-overlay" @click="showMergePreview = false">
        <div class="modal-content large" @click.stop>
          <div class="modal-header">
            <h2>🔄 分类合并预览</h2>
            <button @click="showMergePreview = false" class="modal-close">✕</button>
          </div>

          <div v-if="mergePreview" class="modal-body">
            <!-- 合并概览 -->
            <div class="merge-overview">
              <h3>📊 合并概览</h3>
              <div class="stats-grid">
                <div class="stat-item">
                  <span class="stat-number">{{ mergePreview.total_posts_affected }}</span>
                  <span class="stat-label">篇文章将受影响</span>
                </div>
                <div class="stat-item">
                  <span class="stat-number">{{ mergePreview.posts_with_duplicates }}</span>
                  <span class="stat-label">重复关联将被清理</span>
                </div>
                <div class="stat-item">
                  <span class="stat-number">{{ mergePreview.source_categories.length }}</span>
                  <span class="stat-label">个分类将被合并</span>
                </div>
              </div>
            </div>

            <!-- 目标分类 -->
            <div class="target-section">
              <h3>🎯 目标分类（保留）</h3>
              <div class="category-preview target">
                <h4>{{ mergePreview.target_category.name }}</h4>
                <p>
                  URL: <code>/{{ mergePreview.target_category.slug }}</code>
                </p>
              </div>

              <!-- 可选的重命名 -->
              <div class="rename-section">
                <label class="form-label">
                  <input type="checkbox" v-model="newTargetName" value="rename" />
                  重命名目标分类（可选）
                </label>
                <input
                  v-if="newTargetName"
                  v-model="newTargetName"
                  type="text"
                  class="form-input"
                  :placeholder="mergePreview.target_category.name"
                />
              </div>
            </div>

            <!-- 源分类 -->
            <div class="source-section">
              <h3>📝 源分类（将被删除）</h3>
              <div class="categories-list">
                <div
                  v-for="category in mergePreview.source_categories"
                  :key="category.id"
                  class="category-preview source"
                >
                  <h4>{{ category.name }}</h4>
                  <p>
                    URL: <code>/{{ category.slug }}</code>
                  </p>
                </div>
              </div>
            </div>

            <!-- 每个分类的详细信息 -->
            <div class="details-section">
              <h3>📋 详细影响分析</h3>
              <div class="category-details">
                <div
                  v-for="item in mergePreview.posts_by_category"
                  :key="item.category.id"
                  class="detail-item"
                >
                  <div class="detail-header">
                    <h4>{{ item.category.name }}</h4>
                    <span class="post-count">{{ item.post_count }} 篇文章</span>
                  </div>
                  <div v-if="item.sample_post_titles.length > 0" class="sample-posts">
                    <p><strong>示例文章：</strong></p>
                    <ul>
                      <li v-for="title in item.sample_post_titles.slice(0, 3)" :key="title">
                        {{ title }}
                      </li>
                    </ul>
                  </div>
                </div>
              </div>
            </div>

            <!-- 潜在问题警告 -->
            <div class="issues-section">
              <h3>⚠️ 注意事项</h3>
              <div class="issues-list">
                <div
                  v-for="issue in mergePreview.potential_issues"
                  :key="issue"
                  :class="[
                    'issue-item',
                    {
                      warning: issue.startsWith('⚠️'),
                      info: issue.startsWith('ℹ️'),
                      success: issue.startsWith('✅'),
                    },
                  ]"
                >
                  {{ issue }}
                </div>
              </div>
            </div>

            <div class="modal-actions">
              <button @click="showMergePreview = false" class="btn btn-secondary">取消</button>
              <button @click="executeMerge" :disabled="mergeLoading" class="btn btn-danger">
                {{ mergeLoading ? '合并中...' : '✅ 确认合并' }}
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- 相似分类弹窗 -->
      <div
        v-if="showSimilarCategories"
        class="modal-overlay"
        @click="showSimilarCategories = false"
      >
        <div class="modal-content" @click.stop>
          <div class="modal-header">
            <h2>🔍 相似分类检测</h2>
            <button @click="showSimilarCategories = false" class="modal-close">✕</button>
          </div>

          <div class="modal-body">
            <div v-if="similarCategories.length === 0" class="empty-state">
              <p>🎉 没有发现相似的分类</p>
              <p>您的分类命名很规范！</p>
            </div>

            <div v-else>
              <p class="mb-4">发现 {{ similarCategories.length }} 组相似分类，建议进行合并：</p>

              <div class="similar-groups">
                <div v-for="(group, index) in similarCategories" :key="index" class="similar-group">
                  <div class="group-header">
                    <h4>分组 {{ index + 1 }}</h4>
                    <span class="reason">{{ group.similarity_reason }}</span>
                    <button @click="applySimilarSuggestion(group)" class="btn btn-primary btn-sm">
                      📝 应用建议
                    </button>
                  </div>

                  <div class="group-categories">
                    <div
                      v-for="category in group.categories"
                      :key="category.id"
                      class="category-item"
                    >
                      <span class="category-name">{{ category.name }}</span>
                      <code class="category-slug">/{{ category.slug }}</code>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <div class="modal-actions">
              <button @click="showSimilarCategories = false" class="btn btn-secondary">关闭</button>
            </div>
          </div>
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

/* 管理员工具栏样式 */
.admin-toolbar {
  background: linear-gradient(135deg, #f8fafc 0%, #e2e8f0 100%);
  border: 1px solid var(--color-gray-300);
}

.toolbar-section h3 {
  font-size: var(--text-lg);
  font-weight: 600;
  color: var(--color-gray-800);
  margin: 0 0 var(--space-4);
  display: flex;
  align-items: center;
  gap: var(--space-2);
}

.toolbar-actions {
  display: flex;
  gap: var(--space-6);
  flex-wrap: wrap;
  align-items: center;
}

.view-switcher,
.merge-tools {
  display: flex;
  gap: var(--space-2);
}

/* 合并模式样式 */
.merge-guide {
  background: var(--color-blue-50);
  border: 1px solid var(--color-blue-200);
  border-radius: var(--radius-lg);
  padding: var(--space-4);
  margin-top: var(--space-4);
}

.guide-step {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  margin-bottom: var(--space-2);
}

.guide-step:last-child {
  margin-bottom: 0;
}

.step-number {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  background: var(--color-blue-500);
  color: white;
  border-radius: 50%;
  font-size: var(--text-sm);
  font-weight: 600;
  flex-shrink: 0;
}

.step-text {
  color: var(--color-blue-800);
  font-size: var(--text-sm);
}

.merge-actions {
  display: flex;
  gap: var(--space-3);
  align-items: center;
  margin-top: var(--space-4);
}

.help-text {
  color: var(--color-gray-600);
  font-size: var(--text-sm);
  font-style: italic;
}

/* 合并模式分类样式 */
.category-card.selectable {
  cursor: pointer;
  border-width: 2px;
}

.category-card.target-category {
  border-color: var(--color-green-500);
  background: var(--color-green-50);
}

.category-card.source-category {
  border-color: var(--color-blue-500);
  background: var(--color-blue-50);
}

.category-card.selectable:hover {
  border-color: var(--color-gray-400);
  transform: translateY(-2px);
}

.merge-status {
  display: flex;
  align-items: center;
  margin-top: var(--space-2);
}

.status-badge {
  padding: var(--space-1) var(--space-2);
  border-radius: var(--radius-base);
  font-size: var(--text-xs);
  font-weight: 500;
}

.status-badge.target {
  background: var(--color-green-100);
  color: var(--color-green-800);
}

.status-badge.source {
  background: var(--color-blue-100);
  color: var(--color-blue-800);
}

.status-badge.selectable {
  background: var(--color-gray-100);
  color: var(--color-gray-600);
}

/* 统计视图样式 */
.post-count-badge {
  background: var(--color-primary);
  color: white;
  padding: var(--space-1) var(--space-2);
  border-radius: var(--radius-full);
  font-size: var(--text-xs);
  font-weight: 500;
}

/* 合并预览弹窗样式 */
.modal-content.large {
  max-width: 800px;
}

.merge-overview {
  background: var(--color-blue-50);
  border: 1px solid var(--color-blue-200);
  border-radius: var(--radius-lg);
  padding: var(--space-4);
  margin-bottom: var(--space-6);
}

.merge-overview h3 {
  margin: 0 0 var(--space-4);
  color: var(--color-blue-800);
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: var(--space-4);
}

.stat-item {
  text-align: center;
}

.stat-number {
  display: block;
  font-size: var(--text-2xl);
  font-weight: 700;
  color: var(--color-blue-600);
}

.stat-label {
  font-size: var(--text-sm);
  color: var(--color-blue-700);
}

.target-section,
.source-section,
.details-section,
.issues-section {
  margin-bottom: var(--space-6);
}

.target-section h3,
.source-section h3,
.details-section h3,
.issues-section h3 {
  margin: 0 0 var(--space-3);
  color: var(--color-gray-800);
}

.category-preview {
  background: var(--color-gray-50);
  border-radius: var(--radius-lg);
  padding: var(--space-3);
  margin-bottom: var(--space-2);
}

.category-preview.target {
  background: var(--color-green-50);
  border: 1px solid var(--color-green-200);
}

.category-preview.source {
  background: var(--color-red-50);
  border: 1px solid var(--color-red-200);
}

.category-preview h4 {
  margin: 0 0 var(--space-1);
  color: var(--color-gray-800);
}

.rename-section {
  margin-top: var(--space-4);
  padding: var(--space-3);
  background: var(--color-gray-50);
  border-radius: var(--radius-lg);
}

.categories-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}

.detail-item {
  background: var(--color-white);
  border: 1px solid var(--color-gray-200);
  border-radius: var(--radius-lg);
  padding: var(--space-3);
  margin-bottom: var(--space-2);
}

.detail-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--space-2);
}

.detail-header h4 {
  margin: 0;
  color: var(--color-gray-800);
}

.post-count {
  background: var(--color-gray-100);
  color: var(--color-gray-700);
  padding: var(--space-1) var(--space-2);
  border-radius: var(--radius-base);
  font-size: var(--text-xs);
}

.sample-posts ul {
  margin: var(--space-2) 0 0;
  padding-left: var(--space-4);
  color: var(--color-gray-600);
}

.issues-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}

.issue-item {
  padding: var(--space-3);
  border-radius: var(--radius-lg);
  border-left: 4px solid;
}

.issue-item.warning {
  background: var(--color-yellow-50);
  border-color: var(--color-yellow-400);
  color: var(--color-yellow-800);
}

.issue-item.info {
  background: var(--color-blue-50);
  border-color: var(--color-blue-400);
  color: var(--color-blue-800);
}

.issue-item.success {
  background: var(--color-green-50);
  border-color: var(--color-green-400);
  color: var(--color-green-800);
}

/* 相似分类样式 */
.similar-groups {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
}

.similar-group {
  background: var(--color-gray-50);
  border: 1px solid var(--color-gray-200);
  border-radius: var(--radius-lg);
  padding: var(--space-4);
}

.group-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--space-3);
  flex-wrap: wrap;
  gap: var(--space-2);
}

.group-header h4 {
  margin: 0;
  color: var(--color-gray-800);
}

.reason {
  color: var(--color-gray-600);
  font-size: var(--text-sm);
  font-style: italic;
}

.group-categories {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}

.category-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background: var(--color-white);
  padding: var(--space-2) var(--space-3);
  border-radius: var(--radius-base);
  border: 1px solid var(--color-gray-200);
}

.category-name {
  font-weight: 500;
  color: var(--color-gray-800);
}

.category-slug {
  font-family: var(--font-family-mono);
  font-size: var(--text-sm);
  color: var(--color-gray-600);
  background: var(--color-gray-100);
  padding: var(--space-1) var(--space-2);
  border-radius: var(--radius-base);
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

  .toolbar-actions {
    flex-direction: column;
    align-items: stretch;
  }

  .view-switcher,
  .merge-tools {
    flex-wrap: wrap;
  }

  .stats-grid {
    grid-template-columns: 1fr;
  }

  .detail-header {
    flex-direction: column;
    align-items: flex-start;
    gap: var(--space-2);
  }

  .group-header {
    flex-direction: column;
    align-items: flex-start;
  }

  .category-item {
    flex-direction: column;
    align-items: flex-start;
    gap: var(--space-1);
  }
}
</style>
