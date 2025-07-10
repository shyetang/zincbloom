// ====================================
// 共享类型定义库
// 用于前端、管理后台和后端之间的类型一致性
// ====================================

// ===== 用户相关类型 =====
export interface User {
    id: string;
    username: string;
    email: string;
    created_at: string;
    updated_at?: string;
    email_verified_at?: string;
    is_banned?: boolean;
    roles?: string[];
}

export interface UserPublic {
    id: string;
    username: string;
    email: string;
    created_at: string;
    roles: string[];
}

export interface UserCreateData {
    username: string;
    email: string;
    password: string;
}

export interface UserUpdateData {
    username?: string;
    email?: string;
}

// ===== 认证相关类型 =====
export interface LoginCredentials {
    username: string;
    password: string;
}

export interface RegisterData {
    username: string;
    email: string;
    password: string;
}

export interface LoginResponse {
    access_token: string;
    refresh_token: string;
    user: User;
}

export interface RefreshTokenData {
    refresh_token: string;
}

export interface ForgotPasswordData {
    email: string;
}

export interface ResetPasswordData {
    token: string;
    password: string;
    confirm_password: string;
}

export interface VerifyEmailData {
    token: string;
}

// ===== 文章相关类型 =====
export interface Post {
    id: string;
    title: string;
    slug: string;
    content: string;
    excerpt?: string;
    thumbnail?: string;
    status: PostStatus;
    is_banned?: boolean;
    author_id: string;
    author: {
        id: string;
        username: string;
    };
    categories: Category[];
    tags: Tag[];
    created_at: string;
    updated_at: string;
    published_at?: string;
}

export type PostStatus = "draft" | "published";

export interface CreatePostData {
    title: string;
    content: string;
    excerpt?: string;
    thumbnail?: string;
    category_ids?: string[];
    tag_ids?: string[];
    status?: PostStatus;
}

export interface UpdatePostData extends Partial<CreatePostData> {
    id: string;
}

export interface PostQueryParams {
    page?: number;
    per_page?: number;
    category?: string;
    tag?: string;
    author?: string;
    search?: string;
    status?: PostStatus;
    sort?: "created_at" | "updated_at" | "published_at" | "title";
    order?: "asc" | "desc";
}

// ===== 分类相关类型 =====
export interface Category {
    id: string;
    name: string;
    description?: string;
    slug: string;
    created_at: string;
    updated_at: string;
    post_count?: number;
}

export interface CreateCategoryData {
    name: string;
    description?: string;
    slug?: string;
}

export interface UpdateCategoryData extends Partial<CreateCategoryData> {
    id: string;
}

export interface CategoryQueryParams {
    page?: number;
    per_page?: number;
    search?: string;
    sort?: "name" | "created_at" | "post_count";
    order?: "asc" | "desc";
}

// ===== 标签相关类型 =====
export interface Tag {
    id: string;
    name: string;
    slug: string;
    created_at: string;
    updated_at: string;
    post_count?: number;
}

export interface CreateTagData {
    name: string;
    slug?: string;
}

export interface UpdateTagData extends Partial<CreateTagData> {
    id: string;
}

export interface TagQueryParams {
    page?: number;
    per_page?: number;
    search?: string;
    sort?: "name" | "created_at" | "post_count";
    order?: "asc" | "desc";
}

// ===== 权限和角色相关类型 =====
export interface Permission {
    id: string;
    name: string;
    description?: string;
    resource: string;
    action: string;
    created_at: string;
    updated_at: string;
}

export interface Role {
    id: string;
    name: string;
    description?: string;
    created_at: string;
    updated_at: string;
    permissions?: Permission[];
}

export interface CreateRoleData {
    name: string;
    description?: string;
    permission_ids?: string[];
}

export interface UpdateRoleData extends Partial<CreateRoleData> {
    id: string;
}

// ===== API响应类型 =====
export interface ApiResponse<T = any> {
    data: T;
    message?: string;
    status?: number;
}

export interface PaginatedResponse<T> {
    data: T[];
    pagination: {
        page: number;
        per_page: number;
        total: number;
        total_pages: number;
        has_next_page: boolean;
        has_prev_page: boolean;
    };
}

export interface ApiError {
    message: string;
    code?: string;
    details?: any;
    field?: string;
}

// ===== 表单和UI状态类型 =====
export interface FormErrors {
    [key: string]: string | string[];
}

export interface UIState {
    isLoading: boolean;
    error: string | null;
    success: string | null;
}

export interface LoadingState {
    [key: string]: boolean;
}

// ===== 搜索相关类型 =====
export interface SearchResult {
    type: "post" | "category" | "tag" | "user";
    id: string;
    title: string;
    excerpt?: string;
    url: string;
    score?: number;
}

export interface SearchQueryParams {
    q: string;
    type?: SearchResult["type"][];
    page?: number;
    per_page?: number;
}

// ===== 统计数据类型 =====
export interface DashboardStats {
    total_posts: number;
    published_posts: number;
    draft_posts: number;
    banned_posts: number;
    total_categories: number;
    total_tags: number;
    total_users: number;
    verified_users: number;
    unverified_users: number;
    banned_users: number;
    recent_posts: Post[];
    popular_posts: Post[];
    recent_registrations: UserPublic[];
}

export interface PostStats {
    views: number;
    likes: number;
    comments: number;
    shares: number;
}

// ===== 文件上传相关类型 =====
export interface FileUploadData {
    file: File;
    type: "image" | "document" | "video";
    max_size?: number; // MB
}

export interface UploadedFile {
    id: string;
    filename: string;
    original_name: string;
    url: string;
    size: number;
    mime_type: string;
    created_at: string;
}

// ===== 通知相关类型 =====
export interface Notification {
    id: string;
    title: string;
    message: string;
    type: "info" | "success" | "warning" | "error";
    read: boolean;
    created_at: string;
    data?: any;
}

// ===== 设置相关类型 =====
export interface SiteSettings {
    site_name: string;
    site_description: string;
    site_keywords: string[];
    site_logo?: string;
    site_favicon?: string;
    allow_registration: boolean;
    require_email_verification: boolean;
    default_user_role: string;
    posts_per_page: number;
    max_upload_size: number; // MB
    allowed_file_types: string[];
}

// ===== 导入/导出类型 =====
export interface ImportData {
    posts?: CreatePostData[];
    categories?: CreateCategoryData[];
    tags?: CreateTagData[];
    users?: UserCreateData[];
}

export interface ExportOptions {
    include_posts: boolean;
    include_categories: boolean;
    include_tags: boolean;
    include_users: boolean;
    date_range?: {
        start: string;
        end: string;
    };
    format: "json" | "csv" | "xml";
}

// ===== 常用工具类型 =====
export type Maybe<T> = T | null | undefined;
export type ID = string;
export type Timestamp = string;
export type Email = string;
export type URL = string;
export type Slug = string;

// 分页参数基础接口
export interface BasePaginationParams {
    page?: number;
    per_page?: number;
}

// 排序参数基础接口
export interface BaseSortParams {
    sort?: string;
    order?: "asc" | "desc";
}

// 搜索参数基础接口
export interface BaseSearchParams {
    search?: string;
}

// 时间范围参数
export interface DateRangeParams {
    start_date?: string;
    end_date?: string;
}

// 完整的查询参数接口
export interface BaseQueryParams
    extends BasePaginationParams,
        BaseSortParams,
        BaseSearchParams,
        DateRangeParams {}
