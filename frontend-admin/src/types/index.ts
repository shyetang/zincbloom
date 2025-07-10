// 通用API响应类型
export interface ApiResponse<T> {
  success: boolean
  data: T
  message?: string
  errors?: Record<string, string>
}

// 分页响应类型
export interface PaginatedResponse<T> {
  items: T[]
  total_items: number
  page: number
  page_size: number
  total_pages: number
}

// 用户相关类型
export interface User {
  id: string
  username: string
  email: string
  created_at: string
  updated_at: string
  verified_at?: string
  roles?: Role[]
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
}

// 角色和权限相关类型
export interface Role {
  id: string
  name: string
  description?: string
  created_at: string
  updated_at: string
  permissions?: Permission[]
}

export interface RoleCreate {
  name: string
  description?: string
  permission_ids?: string[]
}

export interface RoleUpdate {
  name?: string
  description?: string
}

export interface Permission {
  id: string
  name: string
  description?: string
  created_at: string
  updated_at: string
}

// 文章相关类型
export interface Post {
  id: string
  title: string
  slug: string
  content: string
  content_markdown?: string
  summary?: string
  published: boolean
  created_at: string
  updated_at: string
  published_at?: string
  author_id: string
  author?: UserBasic
  categories?: CategoryBasic[]
  tags?: TagBasic[]
  is_shared?: boolean
  shared_with?: UserBasic[]
  share_message?: string
  is_public_draft?: boolean
  draft_shared_with?: UserBasic[]
  is_draft_public?: boolean
  is_accessing_others_draft?: boolean

  // 新增的权限字段
  can_edit: boolean
  can_delete: boolean
  can_publish: boolean
  can_view_detail: boolean
}

export interface UserBasic {
  id: string
  username: string
  email: string
}

export interface PostCreate {
  title: string
  content: string
  summary?: string
  category_ids?: string[]
  tag_ids?: string[]
  published_at?: string | null
  draft_shared_with?: string[]
  is_draft_public?: boolean
}

export interface PostUpdate {
  title?: string
  content?: string
  summary?: string
  category_ids?: string[]
  tag_ids?: string[]
  published_at?: string | null
  unpublish?: boolean
  draft_shared_with?: string[]
  is_draft_public?: boolean
}

export interface PostFilters {
  query: string
  status: 'all' | 'published' | 'draft'
  category_id: string
  tag_id: string
  sort_by: string
  sort_order: 'asc' | 'desc'
}

export interface ShareDraftPayload {
  shared_with: string[]
  is_public: boolean
  message: string
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

export interface CategoryBasic {
  id: string
  name: string
  slug: string
}

export interface CategoryCreate {
  name: string
  description?: string
}

export interface CategoryUpdate {
  name?: string
  description?: string
}

// 分类合并相关类型
export interface MergeCategoriesPayload {
  target_category_id: string
  source_category_ids: string[]
  new_target_name?: string
}

export interface MergeCategoriesResponse {
  target_category: Category
  merged_category_count: number
  affected_post_count: number
  duplicate_relations_removed: number
  operation_summary: string
}

export interface MergeCategoriesPreviewPayload {
  target_category_id: string
  source_category_ids: string[]
}

export interface CategoryPostCount {
  category: Category
  post_count: number
  sample_post_titles: string[]
}

export interface MergeCategoriesPreviewResponse {
  target_category: Category
  source_categories: Category[]
  total_posts_affected: number
  posts_with_duplicates: number
  posts_by_category: CategoryPostCount[]
  potential_issues: string[]
}

export interface CategoryUsageStats {
  category: Category
  post_count: number
}

export interface SimilarCategoryGroup {
  categories: Category[]
  similarity_reason: string
}

export interface BatchDeleteCategoriesPayload {
  category_ids: string[]
  handle_orphaned_posts?: OrphanedPostsHandling
}

export type OrphanedPostsHandling =
  | 'LeaveAsIs'
  | 'AddUncategorizedCategory'
  | 'AutoSuggestCategories'

// 标签相关类型
export interface Tag {
  id: string
  name: string
  slug: string
  created_at: string
  updated_at: string
}

export interface TagBasic {
  id: string
  name: string
  slug: string
}

export interface TagCreate {
  name: string
}

export interface TagUpdate {
  name?: string
}

// 标签合并相关类型（保持现有）
export interface MergeTagsPayload {
  target_tag_id: string
  source_tag_ids: string[]
  new_target_name?: string
}

export interface MergeTagsResponse {
  target_tag: Tag
  merged_tag_count: number
  affected_post_count: number
  duplicate_relations_removed: number
  operation_summary: string
}

export interface MergeTagsPreviewPayload {
  target_tag_id: string
  source_tag_ids: string[]
}

export interface TagPostCount {
  tag: Tag
  post_count: number
  sample_post_titles: string[]
}

export interface MergeTagsPreviewResponse {
  target_tag: Tag
  source_tags: Tag[]
  total_posts_affected: number
  posts_with_duplicates: number
  posts_by_tag: TagPostCount[]
  potential_issues: string[]
}

export interface TagUsageStats {
  tag: Tag
  post_count: number
}

export interface SimilarTagGroup {
  tags: Tag[]
  similarity_reason: string
}

export interface BatchDeleteTagsPayload {
  tag_ids: string[]
  handle_orphaned_posts?: OrphanedPostsHandling
}

// 管理员统计数据
export interface DashboardStats {
  total_users: number
  total_posts: number
  total_categories: number
  total_tags: number
  published_posts: number
  draft_posts: number
  recent_posts: Post[]
  verified_users: number
  unverified_users: number
}

export interface UserStats {
  new_users_this_month: number
  total_active_users: number
  users_by_role: Array<{
    role_name: string
    user_count: number
  }>
}
