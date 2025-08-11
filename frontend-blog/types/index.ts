// ====================================
// 博客前端类型定义
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

// 作者信息类型（基于文章数据推导）
export interface Author {
  id: string;
  username: string;
  email?: string;
  bio?: string;
  website?: string;
  avatar?: string;
  created_at: string;
  postCount: number;
  categories: string[];
  tags: string[];
  lastActiveAt: string;
}

export interface AuthorStats {
  totalPosts: number;
  publishedPosts: number;
  draftPosts: number;
  totalViews: number;
  categories: string[];
  tags: string[];
}

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

// ===== 文章相关类型 =====
export interface Post {
  id: string;
  title: string;
  slug: string;
  content_markdown: string; // 后端返回的是 content_markdown
  content_html?: string; // 后端提供的 HTML 内容
  excerpt?: string;
  thumbnail?: string;
  status?: "draft" | "published";
  is_banned?: boolean;
  author_id: string;
  author: {
    id: string;
    username: string;
    email?: string;
  };
  categories: Category[];
  tags: Tag[];
  created_at: string;
  updated_at: string;
  published_at?: string;
  // 后端权限字段
  can_edit?: boolean;
  can_delete?: boolean;
  can_publish?: boolean;
  can_view_detail?: boolean;
  can_ban?: boolean;
  can_unban?: boolean;
}

// 为了向后兼容，添加一个 content 的 getter
export interface PostWithContent extends Omit<Post, "content_markdown"> {
  content: string; // 映射到 content_markdown
}

export interface PostQueryParams {
  page?: number;
  per_page?: number;
  category?: string;
  tag?: string;
  author?: string;
  search?: string;
  status?: "draft" | "published";
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

export interface TagQueryParams {
  page?: number;
  per_page?: number;
  search?: string;
  sort?: "name" | "created_at" | "post_count";
  order?: "asc" | "desc";
}

// ===== API 响应类型 =====
export interface ApiResponse<T = unknown> {
  success: boolean;
  data?: T;
  error?: {
    message: string;
    code?: string;
    details?: Record<string, unknown>;
    field?: string;
  };
  message?: string;
  status?: number;
}

export interface PaginatedResponse<T> {
  items: T[]; // 后端返回的是 items 不是 data
  total_items: number; // 后端返回的是 total_items
  page: number;
  page_size: number; // 后端返回的是 page_size 不是 per_page
  total_pages: number;
}

// 为了向后兼容，提供一个转换接口
export interface FrontendPaginatedResponse<T> {
  data: T[];
  pagination: {
    page: number;
    per_page: number;
    total: number;
    total_pages: number;
  };
}

export interface ApiError {
  message: string;
  code?: string;
  details?: Record<string, unknown>;
  field?: string;
}

// ===== UI 组件类型 =====
export interface DropdownItem {
  label: string;
  icon?: string;
  to?: string;
  click?: () => void;
}

export interface FormSubmitEvent<T = Record<string, unknown>> {
  data: T;
}

// ===== 具体 API 响应类型 =====
export type PostsResponse = FrontendPaginatedResponse<PostWithContent>;
export type CategoriesResponse = FrontendPaginatedResponse<Category>;
export type TagsResponse = FrontendPaginatedResponse<Tag>;

// 后端原始响应类型
export type BackendPostsResponse = PaginatedResponse<Post>;
export type BackendCategoriesResponse = PaginatedResponse<Category>;
export type BackendTagsResponse = PaginatedResponse<Tag>;

// ===== 常量 =====
export const API_ENDPOINTS = {
  AUTH: {
    LOGIN: "/auth/login",
    REGISTER: "/auth/register",
    LOGOUT: "/auth/logout",
    REFRESH: "/auth/refresh",
  },
  POSTS: {
    LIST: "/blog/posts", // 使用公开的博客接口
    DETAIL: (slug: string) => `/blog/posts/${slug}`,
  },
  CATEGORIES: {
    LIST: "/categories",
    DETAIL: (slug: string) => `/categories/${slug}`,
  },
  TAGS: {
    LIST: "/tags",
    DETAIL: (slug: string) => `/tags/${slug}`,
  },
} as const;

export const STORAGE_KEYS = {
  ACCESS_TOKEN: "access_token",
  REFRESH_TOKEN: "refresh_token",
  USER: "user",
  THEME: "theme",
} as const;

export const USER_ROLES = {
  ADMIN: "admin",
  MODERATOR: "moderator",
  AUTHOR: "author",
  USER: "user",
} as const;

export const PERMISSION_ACTIONS = {
  CREATE: "create",
  READ: "read",
  UPDATE: "update",
  DELETE: "delete",
  PUBLISH: "publish",
  BAN: "ban",
} as const;

export const PERMISSION_RESOURCES = {
  POST: "post",
  CATEGORY: "category",
  TAG: "tag",
  USER: "user",
} as const;

export const HTTP_STATUS = {
  OK: 200,
  CREATED: 201,
  BAD_REQUEST: 400,
  UNAUTHORIZED: 401,
  FORBIDDEN: 403,
  NOT_FOUND: 404,
  INTERNAL_SERVER_ERROR: 500,
} as const;

export const ERROR_CODES = {
  VALIDATION_ERROR: "VALIDATION_ERROR",
  AUTHENTICATION_ERROR: "AUTHENTICATION_ERROR",
  AUTHORIZATION_ERROR: "AUTHORIZATION_ERROR",
  NOT_FOUND_ERROR: "NOT_FOUND_ERROR",
  SERVER_ERROR: "SERVER_ERROR",
} as const;
