<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import AdminLayout from '../../layouts/AdminLayout.vue'
import apiClient from '@/api'
import type { Role, RoleCreate, RoleUpdate, Permission } from '@/types'

// 状态管理
const roles = ref<Role[]>([])
const permissions = ref<Permission[]>([])
const userPermissions = ref<Permission[]>([])
const loading = ref(false)
const error = ref<string | null>(null)

// 搜索和筛选
const searchQuery = ref('')
const sortBy = ref<'name' | 'created_at'>('name')
const sortOrder = ref<'asc' | 'desc'>('asc')

// 弹窗状态
const showModal = ref(false)
const modalMode = ref<'create' | 'edit'>('create')
const editingRole = ref<Role | null>(null)

// 表单状态
const formData = ref<RoleCreate>({
  name: '',
  description: '',
  permission_ids: [],
})
const formErrors = ref<Record<string, string>>({})
const saving = ref(false)

// 权限分配弹窗状态
const showPermissionModal = ref(false)
const permissionRole = ref<Role | null>(null)
const selectedPermissions = ref<string[]>([])
const savingPermissions = ref(false)

// 获取角色列表
const fetchRoles = async () => {
  try {
    loading.value = true
    error.value = null
    const response = await apiClient.get('/admin/roles/with-permissions')
    roles.value = response.data || []
  } catch (err) {
    console.error('获取角色列表失败:', err)
    error.value = '获取角色列表失败'
  } finally {
    loading.value = false
  }
}

// 获取权限列表
const fetchPermissions = async () => {
  try {
    const response = await apiClient.get('/admin/permissions')
    permissions.value = response.data || []
  } catch (err) {
    console.error('获取权限列表失败:', err)
  }
}

// 获取当前用户权限
const fetchUserPermissions = async () => {
  try {
    const response = await apiClient.get('/me/permissions')
    userPermissions.value = response.data || []
  } catch (err) {
    console.error('获取用户权限失败:', err)
  }
}

// 计算属性：过滤和排序后的角色
const filteredRoles = computed(() => {
  let filtered = roles.value

  // 搜索过滤
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    filtered = filtered.filter(
      (role) =>
        role.name.toLowerCase().includes(query) ||
        (role.description && role.description.toLowerCase().includes(query)),
    )
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
  editingRole.value = null
  formData.value = {
    name: '',
    description: '',
    permission_ids: [],
  }
  formErrors.value = {}
  showModal.value = true
}

// 打开编辑弹窗
const openEditModal = (role: Role) => {
  modalMode.value = 'edit'
  editingRole.value = role
  formData.value = {
    name: role.name,
    description: role.description || '',
    permission_ids: role.permissions?.map((p) => p.id) || [],
  }
  formErrors.value = {}
  showModal.value = true
}

// 关闭弹窗
const closeModal = () => {
  showModal.value = false
  editingRole.value = null
  formData.value = {
    name: '',
    description: '',
    permission_ids: [],
  }
  formErrors.value = {}
}

// 验证表单
const validateForm = () => {
  formErrors.value = {}
  let isValid = true

  if (!formData.value.name.trim()) {
    formErrors.value.name = '角色名称不能为空'
    isValid = false
  }

  return isValid
}

// 保存角色
const saveRole = async () => {
  if (!validateForm()) return

  try {
    saving.value = true

    if (modalMode.value === 'create') {
      await apiClient.post('/admin/roles', formData.value)
    } else if (editingRole.value) {
      await apiClient.put(`/admin/roles/${editingRole.value.id}`, formData.value)
    }

    await fetchRoles()
    closeModal()
  } catch (err: any) {
    console.error('保存角色失败:', err)

    // 处理服务器验证错误
    if (err.response?.status === 400 && err.response?.data?.errors) {
      formErrors.value = err.response.data.errors
    } else {
      alert('保存角色失败')
    }
  } finally {
    saving.value = false
  }
}

// 删除角色
const deleteRole = async (role: Role) => {
  if (!confirm(`确定要删除角色 "${role.name}" 吗？删除后拥有该角色的用户将失去相应权限。`)) {
    return
  }

  try {
    await apiClient.delete(`/admin/roles/${role.id}`)
    await fetchRoles()
  } catch (err) {
    console.error('删除角色失败:', err)
    alert('删除角色失败')
  }
}

// 打开权限分配弹窗
const openPermissionModal = (role: Role) => {
  permissionRole.value = role
  selectedPermissions.value = role.permissions?.map((p) => p.id) || []
  showPermissionModal.value = true
}

// 关闭权限分配弹窗
const closePermissionModal = () => {
  showPermissionModal.value = false
  permissionRole.value = null
  selectedPermissions.value = []
}

// 保存权限分配
const savePermissions = async () => {
  if (!permissionRole.value) return

  try {
    savingPermissions.value = true
    await apiClient.put(`/admin/roles/${permissionRole.value.id}/permissions`, {
      permission_ids: selectedPermissions.value,
    })
    await fetchRoles()
    closePermissionModal()
  } catch (err) {
    console.error('保存权限分配失败:', err)
    alert('保存权限分配失败')
  } finally {
    savingPermissions.value = false
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

// 获取权限名称列表
const getPermissionNames = (rolePermissions: Permission[] | undefined) => {
  if (!rolePermissions || rolePermissions.length === 0) {
    return '无权限'
  }
  return rolePermissions.map((p) => p.name).join(', ')
}

onMounted(() => {
  fetchRoles()
  fetchPermissions()
  fetchUserPermissions()
})
</script>

<template>
  <AdminLayout>
    <div class="roles-page">
      <!-- 页面头部 -->
      <div class="page-header">
        <div class="page-title">
          <h1>角色权限管理</h1>
          <p class="page-subtitle">管理系统角色和权限分配</p>
        </div>
        <div class="page-actions">
          <button @click="openCreateModal" class="btn btn-primary">
            <span>➕</span>
            <span>添加角色</span>
          </button>
        </div>
      </div>

      <!-- 当前用户权限显示 -->
      <div class="card mb-4 user-permissions-card">
        <div class="card-body">
          <h3 class="permissions-title">
            <span class="permissions-icon">🔑</span>
            我的权限
          </h3>
          <div v-if="userPermissions.length === 0" class="no-permissions">暂无权限</div>
          <div v-else class="permissions-grid">
            <div
              v-for="permission in userPermissions"
              :key="permission.id"
              class="permission-badge"
              :title="permission.description"
            >
              {{ permission.name }}
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
                placeholder="搜索角色名称或描述..."
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

      <!-- 角色列表 -->
      <div class="card">
        <div class="card-body">
          <div v-if="loading" class="loading-state">
            <div class="loading-spinner"></div>
            <p>加载中...</p>
          </div>

          <div v-else-if="error" class="error-state">
            <p class="error-message">{{ error }}</p>
            <button @click="fetchRoles" class="btn btn-primary btn-sm">重试</button>
          </div>

          <div v-else-if="filteredRoles.length === 0" class="empty-state">
            <div class="empty-icon">🔐</div>
            <h3>{{ searchQuery ? '没有找到匹配的角色' : '暂无角色' }}</h3>
            <p>
              {{
                searchQuery
                  ? '请尝试其他搜索关键词'
                  : '还没有创建任何角色，点击上方按钮开始创建吧！'
              }}
            </p>
            <button v-if="!searchQuery" @click="openCreateModal" class="btn btn-primary">
              创建第一个角色
            </button>
          </div>

          <div v-else class="roles-grid">
            <div v-for="role in filteredRoles" :key="role.id" class="role-card">
              <div class="role-header">
                <h3 class="role-name">{{ role.name }}</h3>
                <div class="role-actions">
                  <button
                    @click="openPermissionModal(role)"
                    class="btn btn-info btn-sm"
                    title="分配权限"
                  >
                    🔑
                  </button>
                  <button
                    @click="openEditModal(role)"
                    class="btn btn-secondary btn-sm"
                    title="编辑"
                  >
                    ✏️
                  </button>
                  <button @click="deleteRole(role)" class="btn btn-danger btn-sm" title="删除">
                    🗑️
                  </button>
                </div>
              </div>

              <div class="role-content">
                <div v-if="role.description" class="role-description">
                  {{ role.description }}
                </div>
                <div v-else class="role-description empty">暂无描述</div>

                <div class="role-permissions">
                  <span class="permissions-label">权限:</span>
                  <div class="permissions-list">
                    {{ getPermissionNames(role.permissions) }}
                  </div>
                </div>

                <div class="role-meta">
                  <div class="meta-item">
                    <span class="meta-label">创建时间:</span>
                    <span class="meta-value">{{ formatDate(role.created_at) }}</span>
                  </div>
                  <div class="meta-item">
                    <span class="meta-label">更新时间:</span>
                    <span class="meta-value">{{ formatDate(role.updated_at) }}</span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 创建/编辑角色弹窗 -->
      <div v-if="showModal" class="modal-overlay" @click="closeModal">
        <div class="modal-content" @click.stop>
          <div class="modal-header">
            <h2>{{ modalMode === 'create' ? '添加角色' : '编辑角色' }}</h2>
            <button @click="closeModal" class="modal-close">✕</button>
          </div>

          <form @submit.prevent="saveRole" class="modal-body">
            <div class="form-group">
              <label class="form-label">角色名称 *</label>
              <input
                v-model="formData.name"
                type="text"
                class="form-input"
                :class="{ error: formErrors.name }"
                placeholder="输入角色名称..."
                required
              />
              <div v-if="formErrors.name" class="form-error">
                {{ formErrors.name }}
              </div>
            </div>

            <div class="form-group">
              <label class="form-label">描述</label>
              <textarea
                v-model="formData.description"
                class="form-textarea"
                placeholder="输入角色描述（可选）..."
                rows="3"
              ></textarea>
            </div>

            <div class="form-group">
              <label class="form-label">权限分配</label>
              <div class="permissions-checkboxes">
                <div v-for="permission in permissions" :key="permission.id" class="permission-item">
                  <label class="checkbox-label">
                    <input
                      type="checkbox"
                      :value="permission.id"
                      v-model="formData.permission_ids"
                      class="checkbox-input"
                    />
                    <span class="checkbox-text">{{ permission.name }}</span>
                    <span v-if="permission.description" class="permission-desc">
                      {{ permission.description }}
                    </span>
                  </label>
                </div>
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

      <!-- 权限分配弹窗 -->
      <div v-if="showPermissionModal" class="modal-overlay" @click="closePermissionModal">
        <div class="modal-content" @click.stop>
          <div class="modal-header">
            <h2>为角色 "{{ permissionRole?.name }}" 分配权限</h2>
            <button @click="closePermissionModal" class="modal-close">✕</button>
          </div>

          <div class="modal-body">
            <div class="permissions-checkboxes">
              <div v-for="permission in permissions" :key="permission.id" class="permission-item">
                <label class="checkbox-label">
                  <input
                    type="checkbox"
                    :value="permission.id"
                    v-model="selectedPermissions"
                    class="checkbox-input"
                  />
                  <span class="checkbox-text">{{ permission.name }}</span>
                  <span v-if="permission.description" class="permission-desc">
                    {{ permission.description }}
                  </span>
                </label>
              </div>
            </div>

            <div class="modal-actions">
              <button type="button" @click="closePermissionModal" class="btn btn-secondary">
                取消
              </button>
              <button
                @click="savePermissions"
                class="btn btn-primary"
                :disabled="savingPermissions"
              >
                {{ savingPermissions ? '保存中...' : '保存权限' }}
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </AdminLayout>
</template>

<style scoped>
.roles-page {
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

.roles-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(380px, 1fr));
  gap: var(--space-4);
}

.role-card {
  background: var(--color-white);
  border: 1px solid var(--color-gray-200);
  border-radius: var(--radius-lg);
  padding: var(--space-4);
  transition: all var(--transition-fast);
  position: relative;
  overflow: hidden;
}

.role-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 4px;
  background: linear-gradient(90deg, #f59e0b, #ef4444, #8b5cf6);
}

.role-card:hover {
  border-color: var(--color-gray-300);
  box-shadow: var(--shadow-md);
  transform: translateY(-2px);
}

.role-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: var(--space-3);
}

.role-name {
  font-size: var(--text-lg);
  font-weight: 600;
  color: var(--color-gray-900);
  margin: 0;
  flex: 1;
}

.role-actions {
  display: flex;
  gap: var(--space-2);
}

.role-actions .btn {
  padding: var(--space-1) var(--space-2);
  min-width: 28px;
}

.role-description {
  font-size: var(--text-sm);
  color: var(--color-gray-700);
  line-height: 1.5;
  margin-bottom: var(--space-3);
}

.role-description.empty {
  color: var(--color-gray-500);
  font-style: italic;
}

.role-permissions {
  margin-bottom: var(--space-3);
}

.permissions-label {
  font-size: var(--text-sm);
  color: var(--color-gray-600);
  font-weight: 500;
}

.permissions-list {
  font-size: var(--text-sm);
  color: var(--color-gray-700);
  margin-top: var(--space-1);
  line-height: 1.4;
}

.role-meta {
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
  max-width: 600px;
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

.modal-body {
  padding: var(--space-6);
}

.form-input.error,
.form-textarea.error {
  border-color: var(--color-error);
}

.permissions-checkboxes {
  max-height: 300px;
  overflow-y: auto;
  border: 1px solid var(--color-gray-200);
  border-radius: var(--radius-base);
  padding: var(--space-3);
}

.permission-item {
  margin-bottom: var(--space-2);
}

.permission-item:last-child {
  margin-bottom: 0;
}

.checkbox-label {
  display: flex;
  align-items: flex-start;
  gap: var(--space-2);
  cursor: pointer;
  padding: var(--space-2);
  border-radius: var(--radius-base);
  transition: background-color var(--transition-fast);
}

.checkbox-label:hover {
  background-color: var(--color-gray-50);
}

.checkbox-input {
  margin: 0;
  flex-shrink: 0;
}

.checkbox-text {
  font-weight: 500;
  color: var(--color-gray-900);
}

.permission-desc {
  font-size: var(--text-sm);
  color: var(--color-gray-600);
  margin-left: auto;
  max-width: 200px;
  text-align: right;
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

  .roles-grid {
    grid-template-columns: 1fr;
  }

  .role-header {
    flex-direction: column;
    gap: var(--space-2);
    align-items: flex-start;
  }

  .role-actions {
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

  .permission-desc {
    max-width: none;
    text-align: left;
    margin-left: 0;
    margin-top: var(--space-1);
  }

  .checkbox-label {
    flex-direction: column;
    align-items: flex-start;
  }

  .permissions-grid {
    grid-template-columns: 1fr;
    gap: var(--space-2);
  }

  .permission-badge {
    text-align: center;
  }
}

/* 用户权限显示样式 */
.user-permissions-card {
  background: linear-gradient(135deg, #f8fafc 0%, #e2e8f0 100%);
  border: 1px solid #cbd5e1;
}

.permissions-title {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  margin: 0 0 var(--space-4);
  font-size: var(--text-lg);
  font-weight: 600;
  color: var(--color-gray-800);
}

.permissions-icon {
  font-size: 1.2em;
}

.no-permissions {
  color: var(--color-gray-500);
  font-style: italic;
  text-align: center;
  padding: var(--space-4);
}

.permissions-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: var(--space-2);
}

.permission-badge {
  background: linear-gradient(135deg, #3b82f6 0%, #1d4ed8 100%);
  color: white;
  padding: var(--space-2) var(--space-3);
  border-radius: var(--radius-full);
  font-size: var(--text-sm);
  font-weight: 500;
  text-align: center;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  transition: all var(--transition-fast);
  cursor: default;
}

.permission-badge:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);
}
</style>
