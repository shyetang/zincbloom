<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import AdminLayout from '../../layouts/AdminLayout.vue'
import apiClient from '@/api'
import type { Tag, TagCreate, TagUpdate } from '@/types'
import { useAuthStore } from '@/stores/auth'

// 标签统计接口
interface TagUsageStats {
  tag: Tag
  post_count: number
}

// 合并预览接口
interface MergePreview {
  target_tag: Tag
  source_tags: Tag[]
  total_posts_affected: number
  posts_with_duplicates: number
  posts_by_tag: Array<{
    tag: Tag
    post_count: number
    sample_post_titles: string[]
  }>
  potential_issues: string[]
}

// 合并结果接口
interface MergeResult {
  target_tag: Tag
  merged_tag_count: number
  affected_post_count: number
  duplicate_relations_removed: number
  operation_summary: string
}

// 相似标签组接口
interface SimilarTagGroup {
  tags: Tag[]
  similarity_reason: string
}

// 状态管理
const tags = ref<Tag[]>([])
const tagStats = ref<TagUsageStats[]>([])
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
const editingTag = ref<Tag | null>(null)

// 合并功能状态
const mergeMode = ref(false)
const selectedTags = ref<Set<string>>(new Set())
const targetTagId = ref<string | null>(null)
const showMergePreview = ref(false)
const mergePreview = ref<MergePreview | null>(null)
const mergeLoading = ref(false)
const newTargetName = ref('')

// 相似标签
const similarTags = ref<SimilarTagGroup[]>([])
const showSimilarTags = ref(false)

// 表单状态
const formData = ref<TagCreate>({
  name: '',
})
const formErrors = ref<Record<string, string>>({})
const saving = ref(false)

// 权限检查
const authStore = useAuthStore()
const canManageTags = computed(() => authStore.hasPermission('tag:manage'))
const canCreateTags = computed(() => authStore.hasPermission('tag:create'))

// 获取标签列表
const fetchTags = async () => {
  try {
    loading.value = true
    error.value = null
    const response = await apiClient.get('/tags')
    tags.value = response.data || []
  } catch (err) {
    console.error('获取标签列表失败:', err)
    error.value = '获取标签列表失败'
  } finally {
    loading.value = false
  }
}

// 获取标签使用统计
const fetchTagStats = async () => {
  if (!canManageTags.value) return

  try {
    loading.value = true
    error.value = null
    const response = await apiClient.get('/admin/tags/usage-stats')
    tagStats.value = response.data || []
  } catch (err) {
    console.error('获取标签统计失败:', err)
    error.value = '获取标签统计失败'
  } finally {
    loading.value = false
  }
}

// 切换视图模式
const switchViewMode = async (mode: 'simple' | 'stats') => {
  viewMode.value = mode
  if (mode === 'stats' && tagStats.value.length === 0) {
    await fetchTagStats()
  } else if (mode === 'simple' && tags.value.length === 0) {
    await fetchTags()
  }
}

// 计算属性：当前显示的数据
const currentData = computed(() => {
  const source =
    viewMode.value === 'stats' ? tagStats.value : tags.value.map((tag) => ({ tag, post_count: 0 }))
  let filtered = source

  // 搜索过滤
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    filtered = filtered.filter((item) => item.tag.name.toLowerCase().includes(query))
  }

  // 排序
  filtered.sort((a, b) => {
    let valueA: string | Date | number
    let valueB: string | Date | number

    if (sortBy.value === 'post_count') {
      valueA = a.post_count
      valueB = b.post_count
    } else if (sortBy.value === 'created_at') {
      valueA = new Date(a.tag.created_at)
      valueB = new Date(b.tag.created_at)
    } else {
      valueA = a.tag[sortBy.value].toLowerCase()
      valueB = b.tag[sortBy.value].toLowerCase()
    }

    if (valueA < valueB) return sortOrder.value === 'asc' ? -1 : 1
    if (valueA > valueB) return sortOrder.value === 'asc' ? 1 : -1
    return 0
  })

  return filtered
})

// 合并功能：切换合并模式
const toggleMergeMode = () => {
  mergeMode.value = !mergeMode.value
  if (!mergeMode.value) {
    clearMergeSelection()
  }
}

// 合并功能：清空选择
const clearMergeSelection = () => {
  selectedTags.value.clear()
  targetTagId.value = null
  showMergePreview.value = false
  mergePreview.value = null
  newTargetName.value = ''
}

// 合并功能：选择标签
const selectTag = (tagId: string) => {
  if (!mergeMode.value) return

  if (targetTagId.value === tagId) {
    // 取消选择目标标签
    targetTagId.value = null
    selectedTags.value.delete(tagId)
  } else if (selectedTags.value.has(tagId)) {
    // 取消选择源标签
    selectedTags.value.delete(tagId)
  } else if (!targetTagId.value) {
    // 设置为目标标签
    targetTagId.value = tagId
  } else {
    // 添加为源标签
    selectedTags.value.add(tagId)
  }
}

// 合并功能：获取预览
const getMergePreview = async () => {
  if (!targetTagId.value || selectedTags.value.size === 0) {
    alert('请先选择目标标签和至少一个源标签')
    return
  }

  try {
    mergeLoading.value = true
    const response = await apiClient.post('/admin/tags/merge-preview', {
      target_tag_id: targetTagId.value,
      source_tag_ids: Array.from(selectedTags.value),
    })
    mergePreview.value = response.data
    showMergePreview.value = true
  } catch (err: any) {
    console.error('获取合并预览失败:', err)
    alert('获取合并预览失败: ' + (err.response?.data?.message || err.message))
  } finally {
    mergeLoading.value = false
  }
}

// 合并功能：执行合并
const executeMerge = async () => {
  if (!targetTagId.value || selectedTags.value.size === 0) {
    alert('请先选择目标标签和至少一个源标签')
    return
  }

  const sourceTagNames = Array.from(selectedTags.value)
    .map((id) => currentData.value.find((item) => item.tag.id === id)?.tag.name)
    .filter(Boolean)

  const targetTagName = currentData.value.find((item) => item.tag.id === targetTagId.value)?.tag
    .name

  if (
    !confirm(
      `确认要将标签 "${sourceTagNames.join(', ')}" 合并到 "${targetTagName}" 吗？此操作不可撤销！`,
    )
  ) {
    return
  }

  try {
    mergeLoading.value = true
    const response = await apiClient.post('/admin/tags/merge-enhanced', {
      target_tag_id: targetTagId.value,
      source_tag_ids: Array.from(selectedTags.value),
      new_target_name: newTargetName.value.trim() || null,
    })

    const result: MergeResult = response.data
    alert(`合并成功！\n${result.operation_summary}`)

    // 重新加载数据
    clearMergeSelection()
    toggleMergeMode()
    if (viewMode.value === 'stats') {
      await fetchTagStats()
    } else {
      await fetchTags()
    }
  } catch (err: any) {
    console.error('标签合并失败:', err)
    alert('标签合并失败: ' + (err.response?.data?.message || err.message))
  } finally {
    mergeLoading.value = false
  }
}

// 查找相似标签
const findSimilarTags = async () => {
  if (!canManageTags.value) return

  try {
    loading.value = true
    const response = await apiClient.get('/admin/tags/similar')
    similarTags.value = response.data || []
    showSimilarTags.value = true
  } catch (err: any) {
    console.error('查找相似标签失败:', err)
    alert('查找相似标签失败: ' + (err.response?.data?.message || err.message))
  } finally {
    loading.value = false
  }
}

// 应用相似标签建议
const applySimilarTagSuggestion = (group: SimilarTagGroup) => {
  if (!mergeMode.value) {
    toggleMergeMode()
  }

  clearMergeSelection()

  // 选择第一个标签作为目标标签
  if (group.tags.length > 0) {
    targetTagId.value = group.tags[0].id
    // 其余标签作为源标签
    for (let i = 1; i < group.tags.length; i++) {
      selectedTags.value.add(group.tags[i].id)
    }
  }

  showSimilarTags.value = false
}

// 打开创建弹窗
const openCreateModal = () => {
  modalMode.value = 'create'
  editingTag.value = null
  formData.value = {
    name: '',
  }
  formErrors.value = {}
  showModal.value = true
}

// 打开编辑弹窗
const openEditModal = (tag: Tag) => {
  modalMode.value = 'edit'
  editingTag.value = tag
  formData.value = {
    name: tag.name,
  }
  formErrors.value = {}
  showModal.value = true
}

// 关闭弹窗
const closeModal = () => {
  showModal.value = false
  editingTag.value = null
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
    formErrors.value.name = '标签名称不能为空'
    isValid = false
  }

  return isValid
}

// 保存标签
const saveTag = async () => {
  if (!validateForm()) return

  try {
    saving.value = true

    if (modalMode.value === 'create') {
      await apiClient.post('/tags', formData.value)
    } else if (editingTag.value) {
      await apiClient.put(`/tags/${editingTag.value.id}`, formData.value)
    }

    if (viewMode.value === 'stats') {
      await fetchTagStats()
    } else {
      await fetchTags()
    }
    closeModal()
  } catch (err: any) {
    console.error('保存标签失败:', err)

    // 处理服务器验证错误
    if (err.response?.status === 400 && err.response?.data?.errors) {
      formErrors.value = err.response.data.errors
    } else {
      alert('保存标签失败')
    }
  } finally {
    saving.value = false
  }
}

// 删除标签
const deleteTag = async (tag: Tag) => {
  if (!confirm(`确定要删除标签 "${tag.name}" 吗？删除后关联的文章将失去该标签。`)) {
    return
  }

  try {
    await apiClient.delete(`/tags/${tag.id}`)
    if (viewMode.value === 'stats') {
      await fetchTagStats()
    } else {
      await fetchTags()
    }
  } catch (err) {
    console.error('删除标签失败:', err)
    alert('删除标签失败')
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
  fetchTags()
})
</script>

<template>
  <AdminLayout>
    <div class="tags-page">
      <!-- 页面头部 -->
      <div class="page-header">
        <div class="page-title">
          <h1>标签管理</h1>
          <p class="page-subtitle">管理您的博客文章标签</p>
        </div>
        <div class="page-actions">
          <button v-if="canCreateTags" @click="openCreateModal" class="btn btn-primary">
            <span>➕</span>
            <span>添加标签</span>
          </button>
          <div v-else class="permission-notice">
            <span>🔒 您没有创建标签的权限</span>
          </div>
        </div>
      </div>

      <!-- 管理员工具栏 -->
      <div v-if="canManageTags" class="admin-toolbar card mb-4">
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
                  {{ mergeMode ? '🚫 退出合并' : '🔄 标签合并' }}
                </button>
                <button @click="findSimilarTags" class="btn btn-info">🔍 查找相似</button>
              </div>
            </div>
          </div>

          <!-- 合并模式状态栏 -->
          <div v-if="mergeMode" class="merge-status">
            <div class="merge-info">
              <div class="selection-status">
                <span class="target-status">
                  🎯 目标标签:
                  {{
                    targetTagId
                      ? currentData.find((item) => item.tag.id === targetTagId)?.tag.name
                      : '未选择'
                  }}
                </span>
                <span class="source-status">
                  📦 源标签:
                  {{
                    selectedTags.size > 0
                      ? Array.from(selectedTags)
                          .map((id) => currentData.find((item) => item.tag.id === id)?.tag.name)
                          .join(', ')
                      : '未选择'
                  }}
                </span>
              </div>

              <div class="merge-actions">
                <input
                  v-model="newTargetName"
                  type="text"
                  placeholder="新标签名称（可选）"
                  class="form-input"
                />
                <button
                  @click="getMergePreview"
                  :disabled="!targetTagId || selectedTags.size === 0 || mergeLoading"
                  class="btn btn-info btn-sm"
                >
                  {{ mergeLoading ? '分析中...' : '📋 预览合并' }}
                </button>
                <button
                  @click="executeMerge"
                  :disabled="!targetTagId || selectedTags.size === 0 || mergeLoading"
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
                <li>🎯 <strong>绿色边框</strong>：目标标签（将保留）</li>
                <li>🔵 <strong>蓝色边框</strong>：源标签（将被合并删除）</li>
                <li>📝 点击标签卡片进行选择，先选择目标标签，再选择源标签</li>
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
                placeholder="搜索标签名称..."
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

      <!-- 标签列表 -->
      <div class="card">
        <div class="card-body">
          <div v-if="loading" class="loading-state">
            <div class="loading-spinner"></div>
            <p>加载中...</p>
          </div>

          <div v-else-if="error" class="error-state">
            <p class="error-message">{{ error }}</p>
            <button
              @click="viewMode === 'stats' ? fetchTagStats() : fetchTags()"
              class="btn btn-primary btn-sm"
            >
              重试
            </button>
          </div>

          <div v-else-if="currentData.length === 0" class="empty-state">
            <div class="empty-icon">🏷️</div>
            <h3>{{ searchQuery ? '没有找到匹配的标签' : '暂无标签' }}</h3>
            <p>
              {{
                searchQuery
                  ? '请尝试其他搜索关键词'
                  : '还没有创建任何标签，点击上方按钮开始创建吧！'
              }}
            </p>
            <button
              v-if="!searchQuery && canCreateTags"
              @click="openCreateModal"
              class="btn btn-primary"
            >
              创建第一个标签
            </button>
          </div>

          <div v-else class="tags-grid">
            <div
              v-for="item in currentData"
              :key="item.tag.id"
              :class="[
                'tag-card',
                {
                  'merge-target': mergeMode && targetTagId === item.tag.id,
                  'merge-source': mergeMode && selectedTags.has(item.tag.id),
                  'merge-selectable': mergeMode,
                },
              ]"
              @click="mergeMode ? selectTag(item.tag.id) : null"
            >
              <div class="tag-header">
                <h3 class="tag-name">
                  {{ item.tag.name }}
                  <span v-if="viewMode === 'stats'" class="post-count-badge">
                    {{ item.post_count }}篇
                  </span>
                </h3>
                <div class="tag-actions">
                  <button
                    v-if="canManageTags && !mergeMode"
                    @click="openEditModal(item.tag)"
                    class="btn btn-secondary btn-sm"
                    title="编辑"
                  >
                    ✏️
                  </button>
                  <button
                    v-if="canManageTags && !mergeMode"
                    @click="deleteTag(item.tag)"
                    class="btn btn-danger btn-sm"
                    title="删除"
                  >
                    🗑️
                  </button>
                  <div v-if="!canManageTags" class="admin-only-notice">
                    <span title="仅管理员可编辑">🔒</span>
                  </div>

                  <!-- 合并模式选择指示器 -->
                  <div v-if="mergeMode" class="merge-indicator">
                    <span v-if="targetTagId === item.tag.id" class="target-indicator">🎯</span>
                    <span v-else-if="selectedTags.has(item.tag.id)" class="source-indicator"
                      >📦</span
                    >
                    <span v-else class="selectable-indicator">👆</span>
                  </div>
                </div>
              </div>

              <div class="tag-content">
                <div class="tag-slug">
                  <span class="slug-label">URL:</span>
                  <code>/{{ item.tag.slug }}</code>
                </div>

                <div class="tag-meta">
                  <div class="meta-item">
                    <span class="meta-label">创建时间:</span>
                    <span class="meta-value">{{ formatDate(item.tag.created_at) }}</span>
                  </div>
                  <div class="meta-item">
                    <span class="meta-label">更新时间:</span>
                    <span class="meta-value">{{ formatDate(item.tag.updated_at) }}</span>
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

      <!-- 合并预览弹窗 -->
      <div
        v-if="showMergePreview && mergePreview"
        class="modal-overlay"
        @click="showMergePreview = false"
      >
        <div class="modal-content merge-preview-modal" @click.stop>
          <div class="modal-header">
            <h2>🔍 合并预览</h2>
            <button @click="showMergePreview = false" class="modal-close">✕</button>
          </div>

          <div class="modal-body">
            <div class="preview-summary">
              <div class="summary-grid">
                <div class="summary-item">
                  <div class="summary-number">{{ mergePreview.total_posts_affected }}</div>
                  <div class="summary-label">总影响文章</div>
                </div>
                <div class="summary-item">
                  <div class="summary-number">{{ mergePreview.posts_with_duplicates }}</div>
                  <div class="summary-label">重复关联</div>
                </div>
                <div class="summary-item">
                  <div class="summary-number">{{ mergePreview.source_tags.length }}</div>
                  <div class="summary-label">被合并标签</div>
                </div>
              </div>
            </div>

            <div class="preview-details">
              <h4>📋 详细信息</h4>
              <div class="detail-section">
                <p><strong>🎯 目标标签：</strong>{{ mergePreview.target_tag.name }}</p>
                <p>
                  <strong>📦 源标签：</strong
                  >{{ mergePreview.source_tags.map((t) => t.name).join(', ') }}
                </p>
              </div>

              <div v-if="mergePreview.potential_issues.length > 0" class="issues-section">
                <h5>⚠️ 潜在问题</h5>
                <ul>
                  <li v-for="issue in mergePreview.potential_issues" :key="issue">{{ issue }}</li>
                </ul>
              </div>

              <div class="tags-stats-section">
                <h5>📊 各标签统计</h5>
                <div class="stats-list">
                  <div
                    v-for="stat in mergePreview.posts_by_tag"
                    :key="stat.tag.id"
                    class="stat-item"
                  >
                    <div class="stat-header">
                      <strong>{{ stat.tag.name }}</strong>
                      <span class="post-count">{{ stat.post_count }}篇文章</span>
                    </div>
                    <div v-if="stat.sample_post_titles.length > 0" class="sample-posts">
                      <span class="sample-label">示例文章：</span>
                      <span>{{ stat.sample_post_titles.slice(0, 3).join(', ') }}</span>
                      <span v-if="stat.sample_post_titles.length > 3">...</span>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <div class="modal-actions">
              <button @click="showMergePreview = false" class="btn btn-secondary">关闭预览</button>
              <button @click="executeMerge" class="btn btn-danger">确认合并</button>
            </div>
          </div>
        </div>
      </div>

      <!-- 相似标签弹窗 -->
      <div v-if="showSimilarTags" class="modal-overlay" @click="showSimilarTags = false">
        <div class="modal-content similar-tags-modal" @click.stop>
          <div class="modal-header">
            <h2>🔍 相似标签建议</h2>
            <button @click="showSimilarTags = false" class="modal-close">✕</button>
          </div>

          <div class="modal-body">
            <div v-if="similarTags.length === 0" class="empty-state">
              <p>🎉 没有发现相似的标签，您的标签管理很规范！</p>
            </div>
            <div v-else>
              <p class="similar-intro">发现以下相似标签组，建议考虑合并：</p>
              <div class="similar-groups">
                <div v-for="(group, index) in similarTags" :key="index" class="similar-group">
                  <div class="group-header">
                    <h4>组 {{ index + 1 }}</h4>
                    <span class="similarity-reason">{{ group.similarity_reason }}</span>
                  </div>
                  <div class="group-tags">
                    <span v-for="tag in group.tags" :key="tag.id" class="similar-tag">
                      {{ tag.name }}
                    </span>
                  </div>
                  <div class="group-actions">
                    <button
                      @click="applySimilarTagSuggestion(group)"
                      class="btn btn-primary btn-sm"
                    >
                      🔄 应用建议
                    </button>
                  </div>
                </div>
              </div>
            </div>

            <div class="modal-actions">
              <button @click="showSimilarTags = false" class="btn btn-secondary">关闭</button>
            </div>
          </div>
        </div>
      </div>

      <!-- 创建/编辑弹窗 -->
      <div v-if="showModal" class="modal-overlay" @click="closeModal">
        <div class="modal-content" @click.stop>
          <div class="modal-header">
            <h2>{{ modalMode === 'create' ? '添加标签' : '编辑标签' }}</h2>
            <button @click="closeModal" class="modal-close">✕</button>
          </div>

          <form @submit.prevent="saveTag" class="modal-body">
            <div class="form-group">
              <label class="form-label">标签名称 *</label>
              <input
                v-model="formData.name"
                type="text"
                class="form-input"
                :class="{ error: formErrors.name }"
                placeholder="输入标签名称..."
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
.tags-page {
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

.tags-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: var(--space-4);
}

.tag-card {
  background: var(--color-white);
  border: 1px solid var(--color-gray-200);
  border-radius: var(--radius-lg);
  padding: var(--space-4);
  transition: all var(--transition-fast);
  position: relative;
  overflow: hidden;
}

.tag-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 4px;
  background: linear-gradient(90deg, #3b82f6, #8b5cf6, #06b6d4);
}

.tag-card:hover {
  border-color: var(--color-gray-300);
  box-shadow: var(--shadow-md);
  transform: translateY(-2px);
}

.tag-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: var(--space-3);
}

.tag-name {
  font-size: var(--text-lg);
  font-weight: 600;
  color: var(--color-gray-900);
  margin: 0;
  flex: 1;
}

.tag-actions {
  display: flex;
  gap: var(--space-2);
}

.tag-actions .btn {
  padding: var(--space-1) var(--space-2);
  min-width: 28px;
}

.tag-slug {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  margin-bottom: var(--space-3);
}

.slug-label {
  font-size: var(--text-sm);
  color: var(--color-gray-600);
}

.tag-slug code {
  background: var(--color-gray-100);
  padding: var(--space-1) var(--space-2);
  border-radius: var(--radius-base);
  font-size: var(--text-sm);
  font-family: var(--font-family-mono);
}

.tag-meta {
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
  max-width: 450px;
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

  .tags-grid {
    grid-template-columns: 1fr;
  }

  .tag-header {
    flex-direction: column;
    gap: var(--space-2);
    align-items: flex-start;
  }

  .tag-actions {
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
    gap: var(--space-3);
  }

  .merge-info {
    flex-direction: column;
    gap: var(--space-3);
  }

  .merge-actions {
    flex-direction: column;
    gap: var(--space-2);
  }

  .merge-actions .form-input {
    width: 100%;
  }
}

/* 管理员工具栏样式 */
.admin-toolbar {
  background: linear-gradient(135deg, #f8f9fa 0%, #e9ecef 100%);
  border: 1px solid var(--color-gray-300);
}

.toolbar-section h3 {
  margin: 0 0 var(--space-4);
  color: var(--color-gray-800);
  font-size: var(--text-lg);
}

.toolbar-actions {
  display: flex;
  gap: var(--space-6);
  align-items: flex-start;
  flex-wrap: wrap;
}

.view-switcher,
.merge-tools {
  display: flex;
  gap: var(--space-2);
  align-items: center;
}

.view-switcher .btn,
.merge-tools .btn {
  padding: var(--space-2) var(--space-4);
  font-size: var(--text-sm);
  font-weight: 500;
}

/* 合并模式状态栏样式 */
.merge-status {
  margin-top: var(--space-6);
  padding-top: var(--space-4);
  border-top: 2px solid var(--color-primary);
  background: var(--color-white);
  border-radius: var(--radius-lg);
  padding: var(--space-4);
}

.merge-info {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: var(--space-4);
  margin-bottom: var(--space-4);
}

.selection-status {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}

.target-status,
.source-status {
  padding: var(--space-2) var(--space-3);
  border-radius: var(--radius-base);
  font-size: var(--text-sm);
  font-weight: 500;
}

.target-status {
  background: rgba(34, 197, 94, 0.1);
  color: #059669;
  border: 1px solid rgba(34, 197, 94, 0.3);
}

.source-status {
  background: rgba(59, 130, 246, 0.1);
  color: #2563eb;
  border: 1px solid rgba(59, 130, 246, 0.3);
}

.merge-actions {
  display: flex;
  gap: var(--space-2);
  align-items: center;
  flex-wrap: wrap;
}

.merge-actions .form-input {
  min-width: 200px;
  padding: var(--space-1) var(--space-2);
  font-size: var(--text-sm);
}

.merge-help {
  background: rgba(59, 130, 246, 0.05);
  border: 1px solid rgba(59, 130, 246, 0.2);
  border-radius: var(--radius-base);
  padding: var(--space-3);
}

.merge-help p {
  margin: 0 0 var(--space-2);
  font-size: var(--text-sm);
  color: var(--color-gray-700);
}

.merge-help ul {
  margin: 0;
  padding-left: var(--space-4);
  font-size: var(--text-sm);
  color: var(--color-gray-600);
}

.merge-help li {
  margin-bottom: var(--space-1);
}

/* 合并模式标签卡片样式 */
.tag-card.merge-selectable {
  cursor: pointer;
  transition: all var(--transition-fast);
}

.tag-card.merge-selectable:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-lg);
}

.tag-card.merge-target {
  border: 3px solid #22c55e;
  background: rgba(34, 197, 94, 0.05);
}

.tag-card.merge-source {
  border: 3px solid #3b82f6;
  background: rgba(59, 130, 246, 0.05);
}

.merge-indicator {
  display: flex;
  align-items: center;
  font-size: var(--text-lg);
}

.target-indicator {
  color: #22c55e;
}

.source-indicator {
  color: #3b82f6;
}

.selectable-indicator {
  color: var(--color-gray-400);
}

.post-count-badge {
  background: var(--color-primary);
  color: white;
  padding: var(--space-1) var(--space-2);
  border-radius: var(--radius-full);
  font-size: var(--text-xs);
  font-weight: 600;
  margin-left: var(--space-2);
}

/* 合并预览弹窗样式 */
.merge-preview-modal {
  max-width: 600px;
  max-height: 80vh;
}

.preview-summary {
  margin-bottom: var(--space-6);
}

.summary-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: var(--space-4);
}

.summary-item {
  text-align: center;
  padding: var(--space-3);
  background: var(--color-gray-50);
  border-radius: var(--radius-lg);
  border: 1px solid var(--color-gray-200);
}

.summary-number {
  font-size: var(--text-2xl);
  font-weight: 700;
  color: var(--color-primary);
  margin-bottom: var(--space-1);
}

.summary-label {
  font-size: var(--text-sm);
  color: var(--color-gray-600);
}

.preview-details h4 {
  margin: 0 0 var(--space-3);
  color: var(--color-gray-800);
}

.detail-section {
  margin-bottom: var(--space-4);
  padding: var(--space-3);
  background: var(--color-gray-50);
  border-radius: var(--radius-base);
}

.detail-section p {
  margin: 0 0 var(--space-2);
  font-size: var(--text-sm);
}

.detail-section p:last-child {
  margin-bottom: 0;
}

.issues-section {
  margin-bottom: var(--space-4);
  padding: var(--space-3);
  background: rgba(245, 158, 11, 0.1);
  border: 1px solid rgba(245, 158, 11, 0.3);
  border-radius: var(--radius-base);
}

.issues-section h5 {
  margin: 0 0 var(--space-2);
  color: #d97706;
  font-size: var(--text-sm);
  font-weight: 600;
}

.issues-section ul {
  margin: 0;
  padding-left: var(--space-4);
  font-size: var(--text-sm);
  color: #92400e;
}

.tags-stats-section h5 {
  margin: 0 0 var(--space-3);
  color: var(--color-gray-800);
  font-size: var(--text-sm);
  font-weight: 600;
}

.stats-list {
  max-height: 200px;
  overflow-y: auto;
}

.stat-item {
  padding: var(--space-2) var(--space-3);
  margin-bottom: var(--space-2);
  background: var(--color-white);
  border: 1px solid var(--color-gray-200);
  border-radius: var(--radius-base);
}

.stat-item:last-child {
  margin-bottom: 0;
}

.stat-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--space-1);
}

.post-count {
  font-size: var(--text-xs);
  color: var(--color-gray-600);
  background: var(--color-gray-100);
  padding: var(--space-1) var(--space-2);
  border-radius: var(--radius-base);
}

.sample-posts {
  font-size: var(--text-xs);
  color: var(--color-gray-500);
}

.sample-label {
  font-weight: 500;
  margin-right: var(--space-1);
}

/* 相似标签弹窗样式 */
.similar-tags-modal {
  max-width: 500px;
  max-height: 70vh;
}

.similar-intro {
  margin-bottom: var(--space-4);
  color: var(--color-gray-700);
  font-size: var(--text-sm);
}

.similar-groups {
  max-height: 400px;
  overflow-y: auto;
}

.similar-group {
  margin-bottom: var(--space-4);
  padding: var(--space-3);
  background: var(--color-gray-50);
  border: 1px solid var(--color-gray-200);
  border-radius: var(--radius-lg);
}

.similar-group:last-child {
  margin-bottom: 0;
}

.group-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--space-2);
}

.group-header h4 {
  margin: 0;
  font-size: var(--text-base);
  color: var(--color-gray-800);
}

.similarity-reason {
  font-size: var(--text-xs);
  color: var(--color-gray-500);
  background: var(--color-white);
  padding: var(--space-1) var(--space-2);
  border-radius: var(--radius-base);
  border: 1px solid var(--color-gray-300);
}

.group-tags {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-2);
  margin-bottom: var(--space-3);
}

.similar-tag {
  background: var(--color-primary);
  color: white;
  padding: var(--space-1) var(--space-3);
  border-radius: var(--radius-full);
  font-size: var(--text-sm);
  font-weight: 500;
}

.group-actions {
  text-align: right;
}
</style>
