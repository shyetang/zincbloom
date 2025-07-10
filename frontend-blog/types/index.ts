// 用户相关类型
export interface User {
    id: string;
    username: string;
    email: string;
    created_at: string;
    updated_at?: string;
    email_verified_at?: string;
    roles?: string[];
}

export interface UserPublic {
    id: string;
    username: string;
    email: string;
    created_at: string;
    roles: string[];
}

// 认证相关类型
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
    user: UserPublic;
}

// 文章相关类型
export interface Post {
    id: string;
    title: string;
    slug: string;
    content: string;
    excerpt?: string;
    thumbnail?: string;
    status: "draft" | "published";
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

export interface CreatePostData {
    title: string;
    content: string;
    excerpt?: string;
    thumbnail?: string;
    category_ids?: string[];
    tag_ids?: string[];
    status?: "draft" | "published";
}

export interface UpdatePostData extends Partial<CreatePostData> {
    id: string;
}

// 分类相关类型
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

// 标签相关类型
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

// API响应类型
export interface ApiResponse<T = any> {
    data: T;
    message?: string;
}

export interface PaginatedResponse<T> {
    data: T[];
    pagination: {
        page: number;
        per_page: number;
        total: number;
        total_pages: number;
    };
}

// 查询参数类型
export interface PostQueryParams {
    page?: number;
    per_page?: number;
    category?: string;
    tag?: string;
    author?: string;
    search?: string;
    status?: "draft" | "published";
}

export interface CategoryQueryParams {
    page?: number;
    per_page?: number;
    search?: string;
}

export interface TagQueryParams {
    page?: number;
    per_page?: number;
    search?: string;
}

// 表单状态类型
export interface FormErrors {
    [key: string]: string;
}

// UI状态类型
export interface UIState {
    isLoading: boolean;
    error: string | null;
    success: string | null;
}

// 权限相关类型
export interface Permission {
    id: string;
    name: string;
    description?: string;
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

// 搜索相关类型
export interface SearchResult {
    type: "post" | "category" | "tag" | "user";
    id: string;
    title: string;
    excerpt?: string;
    url: string;
}

// 统计数据类型
export interface DashboardStats {
    total_posts: number;
    published_posts: number;
    draft_posts: number;
    total_categories: number;
    total_tags: number;
    total_users: number;
    verified_users: number;
    unverified_users: number;
    recent_posts: Post[];
}

// 错误类型
export interface ApiError {
    message: string;
    code?: string;
    details?: any;
}
