<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import AdminLayout from '../../layouts/AdminLayout.vue'
import apiClient from '@/api'
import type { User, UserCreate, UserUpdate, Role } from '@/types'

// 状态管理
const users = ref<User[]>([])
const roles = ref<Role[]>([])
const loading = ref(false)
const error = ref<string | null>(null)

// 搜索和筛选
const searchQuery = ref('')
const sortBy = ref<'username' | 'email' | 'created_at'>('username')
const sortOrder = ref<'asc' | 'desc'>('asc')

// 弹窗状态
const showModal = ref(false)
const modalMode = ref<'create' | 'edit'>('create')
const editingUser = ref<User | null>(null)

// 表单状态
const formData = ref<UserCreate>({
  username: '',
  email: '',
  password: '',
  confirm_password: '',
})
const formErrors = ref<Record<string, string>>({})
const saving = ref(false)

// 角色分配弹窗状态
const showRoleModal = ref(false)
const editingUserForRoles = ref<User | null>(null)
const selectedRoles = ref<string[]>([])
const savingRoles = ref(false)

// 获取用户列表
const fetchUsers = async () => {
  try {
    loading.value = true
    error.value = null
    const response = await apiClient.get('/admin/users')
    users.value = response.data || []
  } catch (err) {
    console.error('获取用户列表失败:', err)
    error.value = '获取用户列表失败'
  } finally {
    loading.value = false
  }
}

// 获取角色列表
const fetchRoles = async () => {
  try {
    const response = await apiClient.get('/admin/roles')
    roles.value = response.data || []
  } catch (err) {
    console.error('获取角色列表失败:', err)
  }
}

// 计算属性：过滤和排序后的用户
const filteredUsers = computed(() => {
  let filtered = users.value

  // 搜索过滤
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    filtered = filtered.filter(
      (user) =>
        user.username.toLowerCase().includes(query) || user.email.toLowerCase().includes(query),
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
  editingUser.value = null
  formData.value = {
    username: '',
    email: '',
    password: '',
    confirm_password: '',
  }
  formErrors.value = {}
  showModal.value = true
}

// 打开编辑弹窗
const openEditModal = (user: User) => {
  modalMode.value = 'edit'
  editingUser.value = user
  formData.value = {
    username: user.username,
    email: user.email,
    password: '', // 编辑时密码留空，表示不修改
    confirm_password: '',
  }
  formErrors.value = {}
  showModal.value = true
}

// 关闭弹窗
const closeModal = () => {
  showModal.value = false
  editingUser.value = null
  formData.value = {
    username: '',
    email: '',
    password: '',
    confirm_password: '',
  }
  formErrors.value = {}
}

// 验证表单
const validateForm = () => {
  formErrors.value = {}
  let isValid = true

  if (!formData.value.username.trim()) {
    formErrors.value.username = '用户名不能为空'
    isValid = false
  }

  if (!formData.value.email.trim()) {
    formErrors.value.email = '邮箱不能为空'
    isValid = false
  } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(formData.value.email)) {
    formErrors.value.email = '邮箱格式不正确'
    isValid = false
  }

  if (modalMode.value === 'create' && !formData.value.password.trim()) {
    formErrors.value.password = '密码不能为空'
    isValid = false
  } else if (formData.value.password) {
    // 验证密码强度
    const password = formData.value.password
    if (password.length < 8) {
      formErrors.value.password = '密码长度不能少于8个字符'
      isValid = false
    } else if (!/[A-Z]/.test(password)) {
      formErrors.value.password = '密码必须包含至少一个大写字母'
      isValid = false
    } else if (!/[a-z]/.test(password)) {
      formErrors.value.password = '密码必须包含至少一个小写字母'
      isValid = false
    } else if (!/[0-9]/.test(password)) {
      formErrors.value.password = '密码必须包含至少一个数字'
      isValid = false
    } else if (!/[^a-zA-Z0-9]/.test(password)) {
      formErrors.value.password = '密码必须包含至少一个特殊字符'
      isValid = false
    }
  }

  // 验证确认密码
  if (modalMode.value === 'create' || formData.value.password) {
    if (!formData.value.confirm_password.trim()) {
      formErrors.value.confirmPassword = '请确认密码'
      isValid = false
    } else if (formData.value.password !== formData.value.confirm_password) {
      formErrors.value.confirmPassword = '两次输入的密码不一致'
      isValid = false
    }
  }

  return isValid
}

// 保存用户
const saveUser = async () => {
  if (!validateForm()) return

  try {
    saving.value = true

    // 准备提交数据
    const submitData: any = {
      username: formData.value.username,
      email: formData.value.email,
    }

    // 只有在创建或者填写了密码时才包含password字段
    if (modalMode.value === 'create' || formData.value.password) {
      submitData.password = formData.value.password
      submitData.confirm_password = formData.value.confirm_password
    }

    if (modalMode.value === 'create') {
      await apiClient.post('/admin/users', submitData)
    } else if (editingUser.value) {
      await apiClient.put(`/admin/users/${editingUser.value.id}`, submitData)
    }

    await fetchUsers()
    closeModal()
  } catch (err: any) {
    console.error('保存用户失败:', err)

    // 处理服务器验证错误
    if (err.response?.status === 400 && err.response?.data?.errors) {
      formErrors.value = err.response.data.errors
    } else {
      alert('保存用户失败')
    }
  } finally {
    saving.value = false
  }
}

// 删除用户
const deleteUser = async (user: User) => {
  if (!confirm(`确定要删除用户 "${user.username}" 吗？删除后该用户的所有数据将无法恢复。`)) {
    return
  }

  try {
    await apiClient.delete(`/admin/users/${user.id}`)
    await fetchUsers()
  } catch (err) {
    console.error('删除用户失败:', err)
    alert('删除用户失败')
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

// 获取用户状态显示
const getUserStatus = (user: User) => {
  if (user.verified_at) {
    return { text: '已验证', class: 'status-verified' }
  } else {
    return { text: '未验证', class: 'status-unverified' }
  }
}

// 打开角色分配弹窗
const openRoleModal = (user: User) => {
  editingUserForRoles.value = user
  selectedRoles.value = (user.roles || []).map(role => typeof role === 'string' ? role : role.name) // 复制当前用户的角色名称
  showRoleModal.value = true
}

// 关闭角色分配弹窗
const closeRoleModal = () => {
  showRoleModal.value = false
  editingUserForRoles.value = null
  selectedRoles.value = []
}

// 保存角色分配
const saveUserRoles = async () => {
  if (!editingUserForRoles.value) return

  try {
    savingRoles.value = true

    // 获取选中角色的ID
    const roleIds = roles.value
      .filter((role) => selectedRoles.value.includes(role.name))
      .map((role) => role.id)

    await apiClient.put(`/admin/users/${editingUserForRoles.value.id}/roles`, {
      role_ids: roleIds,
    })

    await fetchUsers()
    closeRoleModal()
  } catch (err) {
    console.error('保存角色分配失败:', err)
    alert('保存角色分配失败')
  } finally {
    savingRoles.value = false
  }
}

onMounted(() => {
  fetchUsers()
  fetchRoles()
})
</script>

<template>
  <AdminLayout>
    <div class="users-page">
      <!-- 页面头部 -->
      <div class="page-header">
        <div class="page-title">
          <h1>用户管理</h1>
          <p class="page-subtitle">管理系统用户账户</p>
        </div>
        <div class="page-actions">
          <button @click="openCreateModal" class="btn btn-primary">
            <span>➕</span>
            <span>添加用户</span>
          </button>
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
                placeholder="搜索用户名或邮箱..."
              />
            </div>
            <div class="sort-group">
              <select v-model="sortBy" class="form-select">
                <option value="username">按用户名排序</option>
                <option value="email">按邮箱排序</option>
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

      <!-- 用户列表 -->
      <div class="card">
        <div class="card-body">
          <div v-if="loading" class="loading-state">
            <div class="loading-spinner"></div>
            <p>加载中...</p>
          </div>

          <div v-else-if="error" class="error-state">
            <p class="error-message">{{ error }}</p>
            <button @click="fetchUsers" class="btn btn-primary btn-sm">重试</button>
          </div>

          <div v-else-if="filteredUsers.length === 0" class="empty-state">
            <div class="empty-icon">👥</div>
            <h3>{{ searchQuery ? '没有找到匹配的用户' : '暂无用户' }}</h3>
            <p>
              {{
                searchQuery
                  ? '请尝试其他搜索关键词'
                  : '还没有创建任何用户，点击上方按钮开始创建吧！'
              }}
            </p>
            <button v-if="!searchQuery" @click="openCreateModal" class="btn btn-primary">
              创建第一个用户
            </button>
          </div>

          <div v-else class="users-grid">
            <div v-for="user in filteredUsers" :key="user.id" class="user-card">
              <div class="user-header">
                <div class="user-avatar">
                  {{ user.username.charAt(0).toUpperCase() }}
                </div>
                <div class="user-info">
                  <h3 class="user-name">{{ user.username }}</h3>
                  <p class="user-email">{{ user.email }}</p>
                </div>
                <div class="user-actions">
                  <button
                    @click="openRoleModal(user)"
                    class="btn btn-primary btn-sm"
                    title="管理角色"
                  >
                    👤
                  </button>
                  <button
                    @click="openEditModal(user)"
                    class="btn btn-secondary btn-sm"
                    title="编辑"
                  >
                    ✏️
                  </button>
                  <button @click="deleteUser(user)" class="btn btn-danger btn-sm" title="删除">
                    🗑️
                  </button>
                </div>
              </div>

              <div class="user-content">
                <div class="user-status">
                  <span class="status-label">状态:</span>
                  <span :class="['status-badge', getUserStatus(user).class]">
                    {{ getUserStatus(user).text }}
                  </span>
                </div>

                <div class="user-roles">
                  <span class="status-label">角色:</span>
                  <div class="roles-list">
                    <span v-for="role in (user.roles || [])" :key="role.id || role.name" class="role-badge">
                      {{ role.name || role }}
                    </span>
                    <span v-if="!user.roles || user.roles.length === 0" class="no-roles"> 未分配角色 </span>
                  </div>
                </div>

                <div class="user-meta">
                  <div class="meta-item">
                    <span class="meta-label">创建时间:</span>
                    <span class="meta-value">{{ formatDate(user.created_at) }}</span>
                  </div>
                  <div class="meta-item">
                    <span class="meta-label">更新时间:</span>
                    <span class="meta-value">{{ formatDate(user.updated_at) }}</span>
                  </div>
                  <div v-if="user.verified_at" class="meta-item">
                    <span class="meta-label">验证时间:</span>
                    <span class="meta-value">{{ formatDate(user.verified_at) }}</span>
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
            <h2>{{ modalMode === 'create' ? '添加用户' : '编辑用户' }}</h2>
            <button @click="closeModal" class="modal-close">✕</button>
          </div>

          <form @submit.prevent="saveUser" class="modal-body">
            <div class="form-group">
              <label class="form-label">用户名 *</label>
              <input
                v-model="formData.username"
                type="text"
                class="form-input"
                :class="{ error: formErrors.username }"
                placeholder="输入用户名..."
                required
              />
              <div v-if="formErrors.username" class="form-error">
                {{ formErrors.username }}
              </div>
            </div>

            <div class="form-group">
              <label class="form-label">邮箱 *</label>
              <input
                v-model="formData.email"
                type="email"
                class="form-input"
                :class="{ error: formErrors.email }"
                placeholder="输入邮箱地址..."
                required
              />
              <div v-if="formErrors.email" class="form-error">
                {{ formErrors.email }}
              </div>
            </div>

            <div class="form-group">
              <label class="form-label">
                密码 {{ modalMode === 'create' ? '*' : '(留空表示不修改)' }}
              </label>
              <input
                v-model="formData.password"
                type="password"
                class="form-input"
                :class="{ error: formErrors.password }"
                :placeholder="modalMode === 'create' ? '输入密码...' : '留空表示不修改密码...'"
                :required="modalMode === 'create'"
              />
              <div v-if="formErrors.password" class="form-error">
                {{ formErrors.password }}
              </div>
              <div class="form-help">密码要求：至少8位，包含大写字母、小写字母、数字和特殊字符</div>
            </div>

            <div v-if="modalMode === 'create' || formData.password" class="form-group">
              <label class="form-label">确认密码 *</label>
              <input
                v-model="formData.confirm_password"
                type="password"
                class="form-input"
                :class="{ error: formErrors.confirmPassword }"
                placeholder="请再次输入密码..."
                required
              />
              <div v-if="formErrors.confirmPassword" class="form-error">
                {{ formErrors.confirmPassword }}
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

      <!-- 角色分配弹窗 -->
      <div v-if="showRoleModal" class="modal-overlay" @click="closeRoleModal">
        <div class="modal-content" @click.stop>
          <div class="modal-header">
            <h2>管理用户角色</h2>
            <button @click="closeRoleModal" class="modal-close">✕</button>
          </div>

          <div class="modal-body">
            <div v-if="editingUserForRoles" class="user-info-section">
              <h3>{{ editingUserForRoles.username }}</h3>
              <p class="user-email">{{ editingUserForRoles.email }}</p>
            </div>

            <div class="roles-section">
              <h4>选择角色:</h4>
              <div class="roles-grid">
                <label v-for="role in roles" :key="role.id" class="role-checkbox">
                  <input type="checkbox" :value="role.name" v-model="selectedRoles" />
                  <div class="role-info">
                    <span class="role-name">{{ role.name }}</span>
                    <span class="role-description">{{ role.description || '无描述' }}</span>
                  </div>
                </label>
              </div>
            </div>

            <div class="modal-actions">
              <button type="button" @click="closeRoleModal" class="btn btn-secondary">取消</button>
              <button
                type="button"
                @click="saveUserRoles"
                class="btn btn-primary"
                :disabled="savingRoles"
              >
                {{ savingRoles ? '保存中...' : '保存' }}
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </AdminLayout>
</template>

<style scoped>
.users-page {
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

.users-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
  gap: var(--space-4);
}

.user-card {
  background: var(--color-white);
  border: 1px solid var(--color-gray-200);
  border-radius: var(--radius-lg);
  padding: var(--space-4);
  transition: all var(--transition-fast);
  position: relative;
  overflow: hidden;
}

.user-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 4px;
  background: linear-gradient(90deg, #10b981, #3b82f6, #8b5cf6);
}

.user-card:hover {
  border-color: var(--color-gray-300);
  box-shadow: var(--shadow-md);
  transform: translateY(-2px);
}

.user-header {
  display: flex;
  align-items: flex-start;
  gap: var(--space-3);
  margin-bottom: var(--space-3);
}

.user-avatar {
  width: 48px;
  height: 48px;
  border-radius: var(--radius-full);
  background: linear-gradient(45deg, var(--color-primary), var(--color-success));
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--color-white);
  font-weight: 600;
  font-size: var(--text-lg);
  flex-shrink: 0;
}

.user-info {
  flex: 1;
  min-width: 0;
}

.user-name {
  font-size: var(--text-lg);
  font-weight: 600;
  color: var(--color-gray-900);
  margin: 0 0 var(--space-1);
}

.user-email {
  font-size: var(--text-sm);
  color: var(--color-gray-600);
  margin: 0;
  word-break: break-all;
}

.user-actions {
  display: flex;
  gap: var(--space-2);
  flex-shrink: 0;
}

.user-actions .btn {
  padding: var(--space-1) var(--space-2);
  min-width: 28px;
}

.user-status {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  margin-bottom: var(--space-3);
}

.status-label {
  font-size: var(--text-sm);
  color: var(--color-gray-600);
}

.status-badge {
  padding: var(--space-1) var(--space-2);
  border-radius: var(--radius-full);
  font-size: var(--text-xs);
  font-weight: 500;
}

.status-verified {
  background-color: var(--color-success-light);
  color: var(--color-success-dark);
}

.status-unverified {
  background-color: var(--color-warning-light);
  color: var(--color-warning-dark);
}

.user-meta {
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

.modal-body {
  padding: var(--space-6);
}

/* 角色相关样式 */
.user-roles {
  margin-bottom: var(--space-3);
}

.roles-list {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-2);
  margin-top: var(--space-1);
}

.role-badge {
  display: inline-block;
  padding: var(--space-1) var(--space-2);
  background-color: var(--color-primary-light);
  color: var(--color-primary-dark);
  border-radius: var(--radius-full);
  font-size: var(--text-xs);
  font-weight: 500;
  border: 1px solid var(--color-primary);
}

.no-roles {
  font-size: var(--text-xs);
  color: var(--color-gray-500);
  font-style: italic;
}

/* 角色分配弹窗样式 */
.user-info-section {
  background: var(--color-gray-50);
  padding: var(--space-4);
  border-radius: var(--radius-md);
  margin-bottom: var(--space-4);
}

.user-info-section h3 {
  margin: 0 0 var(--space-1);
  color: var(--color-gray-900);
  font-size: var(--text-lg);
}

.user-info-section .user-email {
  margin: 0;
  color: var(--color-gray-600);
  font-size: var(--text-sm);
}

.roles-section h4 {
  margin: 0 0 var(--space-3);
  color: var(--color-gray-900);
  font-size: var(--text-base);
}

.roles-grid {
  display: grid;
  gap: var(--space-2);
  max-height: 300px;
  overflow-y: auto;
}

.role-checkbox {
  display: flex;
  align-items: flex-start;
  gap: var(--space-2);
  padding: var(--space-3);
  border: 1px solid var(--color-gray-200);
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all var(--transition-fast);
}

.role-checkbox:hover {
  border-color: var(--color-primary);
  background-color: var(--color-primary-light);
}

.role-checkbox input[type='checkbox'] {
  margin: 0;
  flex-shrink: 0;
}

.role-info {
  flex: 1;
  min-width: 0;
}

.role-name {
  display: block;
  font-weight: 600;
  color: var(--color-gray-900);
  margin-bottom: var(--space-1);
}

.role-description {
  display: block;
  font-size: var(--text-sm);
  color: var(--color-gray-600);
  word-wrap: break-word;
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

  .users-grid {
    grid-template-columns: 1fr;
  }

  .user-header {
    flex-direction: column;
    gap: var(--space-2);
    align-items: flex-start;
  }

  .user-actions {
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
