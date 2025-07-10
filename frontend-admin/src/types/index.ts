// 这个类型与后端 UserPublic DTO 的结构保持一致
export interface User {
  id: string // UUID 在前端通常作为字符串处理
  username: string
  email: string
  created_at: string // TIMESTAMPTZ 转换为字符串
  updated_at: string
  verified_at?: string
  roles: string[]
}

export interface UserCreate {
  username: string
  email: string
  password: string
  confirm_password: string
}

export interface UserUpdate {
  username?: string
  email?: string
  password?: string
  confirm_password?: string
}

// 文章相关类型
export interface Post {
  id: string
  slug: string
  title: string
  content_markdown: string // 原始 markdown 内容
  content_html: string // 渲染后的 HTML
  author_id?: string
  created_at: string
  updated_at: string
  published_at?: string
  categories?: Category[]
  tags?: Tag[]
  // 草稿分享相关字段
  draft_shared_with?: string[] // 分享给哪些用户（UUID数组）
  is_draft_public?: boolean // 是否允许有权限的编辑查看
  is_accessing_others_draft?: boolean // 标识是否在访问他人的草稿
}

export interface PostCreate {
  title: string
  content: string // 创建时使用简单的content字段
  published_at?: string
  category_ids?: string[]
  tag_ids?: string[]
}

export interface PostUpdate {
  title?: string
  content?: string // 更新时也使用简单的content字段
  published_at?: string
  category_ids?: string[]
  tag_ids?: string[]
  // 草稿分享相关字段
  draft_shared_with?: string[]
  is_draft_public?: boolean
}

// 草稿分享操作类型
export interface ShareDraftPayload {
  shared_with: string[] // 要分享给的用户ID列表
  is_public: boolean // 是否设为公开（允许有权限的编辑查看）
  message?: string // 分享时的消息
}

// 草稿访问日志类型
export interface DraftAccessLog {
  id: string
  post_id: string
  post_title: string
  accessed_by: string
  accessed_by_username: string
  access_type: string // 'view', 'edit', 'delete'
  access_reason?: string
  created_at: string
}

// 分类相关类型
export interface Category {
  id: string
  name: string
  slug: string
  description?: string
  created_at: string
  updated_at: string
}

export interface CategoryCreate {
  name: string
  description?: string
}

export interface CategoryUpdate {
  name?: string
  description?: string
}

// 标签相关类型
export interface Tag {
  id: string
  name: string
  slug: string
  created_at: string
  updated_at: string
}

export interface TagCreate {
  name: string
}

export interface TagUpdate {
  name?: string
}

// 角色权限相关类型
export interface Role {
  id: string
  name: string
  description?: string
  created_at: string
  updated_at: string
  permissions?: Permission[]
}

export interface Permission {
  id: string
  name: string
  description?: string
  created_at: string
  updated_at: string
}

export interface RoleCreate {
  name: string
  description?: string
  permission_ids?: string[]
}

export interface RoleUpdate {
  name?: string
  description?: string
  permission_ids?: string[]
}

// API 响应类型
export interface ApiResponse<T> {
  data: T
  message?: string
  status: number
}

export interface PaginationMeta {
  current_page: number
  per_page: number
  total: number
  total_pages: number
  has_next: boolean
  has_prev: boolean
}

export interface PaginatedResponse<T> {
  items: T[]
  total_items: number
  page: number
  page_size: number
  total_pages: number
}

// 表单相关类型
export interface FormError {
  field: string
  message: string
}

export interface ValidationErrors {
  [key: string]: string[]
}

// 通用列表项类型
export interface ListItem {
  id: string
  name: string
  slug?: string
}

// 搜索和过滤类型
export interface SearchParams {
  query?: string
  page?: number
  per_page?: number
  sort_by?: string
  sort_order?: 'asc' | 'desc'
}

export interface PostFilters extends SearchParams {
  category_id?: string
  tag_id?: string
  status?: 'published' | 'draft' | 'all'
  author_id?: string
}

// 文件上传相关类型
export interface FileUpload {
  file: File
  progress: number
  status: 'pending' | 'uploading' | 'success' | 'error'
  url?: string
  error?: string
}

// 组件属性类型
export interface TableColumn {
  key: string
  label: string
  sortable?: boolean
  width?: string
  align?: 'left' | 'center' | 'right'
  render?: (value: any, row: any) => string
}

export interface ActionButton {
  label: string
  icon?: string
  variant?: 'primary' | 'secondary' | 'danger' | 'warning'
  disabled?: boolean
  onClick: () => void
}

// 通知类型
export interface Notification {
  id: string
  type: 'success' | 'error' | 'warning' | 'info'
  title: string
  message?: string
  duration?: number
  persistent?: boolean
}

// 统计数据类型
export interface DashboardStats {
  total_posts: number
  published_posts: number
  draft_posts: number
  total_categories: number
  total_tags: number
  total_users: number
  verified_users: number
  unverified_users: number
}

// 用户统计数据（管理员专用）
export interface UserStats {
  total: number
  verified: number
  unverified: number
  by_role: RoleUserCount[]
}

// 角色用户数量统计
export interface RoleUserCount {
  role_name: string
  user_count: number
}

// 主题和界面类型
export type Theme = 'light' | 'dark' | 'auto'

export interface UISettings {
  theme: Theme
  sidebarCollapsed: boolean
  language: string
}

// 路由元信息类型
declare module 'vue-router' {
  interface RouteMeta {
    requiresAuth?: boolean
    roles?: string[]
    title?: string
    icon?: string
  }
}
