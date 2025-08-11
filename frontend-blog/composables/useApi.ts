// API 客户端 Composable
import type {
  ApiResponse,
  LoginCredentials,
  RegisterData,
  LoginResponse,
  Post,
  PostWithContent,
  Category,
  Tag,
  User,
  BackendPostsResponse,
  PostsResponse,
  PaginatedResponse,
  FrontendPaginatedResponse,
  PostQueryParams,
  CategoryQueryParams,
  TagQueryParams,
} from "~/types";

import { API_ENDPOINTS, ERROR_CODES, STORAGE_KEYS } from "~/types";
import {
  transformPost,
  transformPostsResponse,
  transformPaginatedResponse,
  buildQueryString,
} from "~/utils/dataTransform";

interface ApiClientConfig {
  baseURL: string;
  timeout?: number;
}

class ApiClient {
  private baseURL: string;
  private timeout: number;

  constructor(config: ApiClientConfig) {
    this.baseURL = config.baseURL;
    this.timeout = config.timeout || 10000;
  }

  private getAuthToken(): string | null {
    if (typeof window !== "undefined") {
      return localStorage.getItem(STORAGE_KEYS.ACCESS_TOKEN);
    }
    return null;
  }

  private async request<T>(
    endpoint: string,
    options: RequestInit = {},
  ): Promise<ApiResponse<T>> {
    const url = `${this.baseURL}${endpoint}`;
    const token = this.getAuthToken();

    const headers: Record<string, string> = {
      "Content-Type": "application/json",
      ...(options.headers as Record<string, string>),
    };

    if (token) {
      headers.Authorization = `Bearer ${token}`;
    }

    try {
      const response = await fetch(url, {
        ...options,
        headers,
        signal: AbortSignal.timeout(this.timeout),
      });

      let data;
      const contentType = response.headers.get("content-type");

      if (contentType && contentType.includes("application/json")) {
        data = await response.json();
      }
      else {
        // 如果不是JSON，获取文本内容
        const text = await response.text();
        console.error(`非JSON响应 (${response.status}):`, text);
        data = { message: text, error: `HTTP ${response.status}` };
      }

      if (response.ok) {
        return {
          success: true,
          data,
          status: response.status,
        };
      }
      else {
        return {
          success: false,
          error: {
            message: data.message || data.error || "请求失败",
            code: data.code || ERROR_CODES.SERVER_ERROR,
            details: data.details || data,
          },
          status: response.status,
        };
      }
    }
    catch (error) {
      console.error("API Error:", error);
      return {
        success: false,
        error: {
          message:
                        error instanceof Error ? error.message : "网络错误",
          code: ERROR_CODES.SERVER_ERROR,
        },
        status: 500,
      };
    }
  }

  // 认证相关 API
  async login(
    credentials: LoginCredentials,
  ): Promise<ApiResponse<LoginResponse>> {
    return this.request<LoginResponse>(API_ENDPOINTS.AUTH.LOGIN, {
      method: "POST",
      body: JSON.stringify(credentials),
    });
  }

  async register(data: RegisterData): Promise<ApiResponse<User>> {
    return this.request<User>(API_ENDPOINTS.AUTH.REGISTER, {
      method: "POST",
      body: JSON.stringify(data),
    });
  }

  async logout(): Promise<ApiResponse<null>> {
    return this.request<null>(API_ENDPOINTS.AUTH.LOGOUT, {
      method: "POST",
    });
  }

  async refreshToken(data: {
    refresh_token: string;
  }): Promise<ApiResponse<LoginResponse>> {
    return this.request<LoginResponse>(API_ENDPOINTS.AUTH.REFRESH, {
      method: "POST",
      body: JSON.stringify(data),
    });
  }

  // 用户相关 API
  async getCurrentUser(): Promise<ApiResponse<User>> {
    return this.request<User>("/me");
  }

  async getUserPermissions(): Promise<ApiResponse<string[]>> {
    return this.request<string[]>("/me/permissions");
  }

  // 更新用户资料
  async updateProfile(data: {
    username?: string;
    email?: string;
    bio?: string;
    website?: string;
  }): Promise<ApiResponse<User>> {
    return this.request<User>("/me/profile", {
      method: "PUT",
      body: JSON.stringify(data),
    });
  }

  // 修改密码
  async changePassword(data: {
    current_password: string;
    new_password: string;
  }): Promise<ApiResponse<null>> {
    return this.request<null>("/me/password", {
      method: "PUT",
      body: JSON.stringify(data),
    });
  }

  // 更新用户偏好设置
  async updatePreferences(data: {
    email_notifications?: boolean;
    public_profile?: boolean;
    theme?: string;
  }): Promise<ApiResponse<null>> {
    return this.request<null>("/me/preferences", {
      method: "PUT",
      body: JSON.stringify(data),
    });
  }

  // 文章相关 API - 使用公开接口
  async getPosts(
    params?: PostQueryParams,
  ): Promise<ApiResponse<PostsResponse>> {
    const queryString = params ? `?${buildQueryString(params)}` : "";

    const response = await this.request<BackendPostsResponse>(
      `${API_ENDPOINTS.POSTS.LIST}${queryString}`,
    );

    if (response.success && response.data) {
      const transformedData = transformPostsResponse(response.data);
      return {
        success: true,
        data: transformedData,
        status: response.status,
      };
    }

    return {
      success: false,
      error: response.error,
      status: response.status,
    };
  }

  async getPost(slug: string): Promise<ApiResponse<PostWithContent>> {
    const response = await this.request<Post>(API_ENDPOINTS.POSTS.DETAIL(slug));

    if (response.success && response.data) {
      const transformedData = transformPost(response.data);
      return {
        success: true,
        data: transformedData,
        status: response.status,
      };
    }

    return {
      success: false,
      error: response.error,
      status: response.status,
    };
  }

  // 分类相关 API
  async getCategories(
    params?: CategoryQueryParams,
  ): Promise<ApiResponse<FrontendPaginatedResponse<Category>>> {
    const queryString = params ? `?${buildQueryString(params)}` : "";

    const response = await this.request<Category[]>(
      `${API_ENDPOINTS.CATEGORIES.LIST}${queryString}`,
    );

    if (response.success && response.data) {
      // 后端返回简单数组，转换为前端期望的分页格式
      const transformedData: FrontendPaginatedResponse<Category> = {
        data: response.data,
        pagination: {
          page: 1,
          per_page: response.data.length,
          total: response.data.length,
          total_pages: 1,
        },
      };
      return {
        success: true,
        data: transformedData,
        status: response.status,
      };
    }

    return {
      success: false,
      error: response.error,
      status: response.status,
    };
  }

  // 获取所有分类（简单数组格式）
  async getCategoriesList(): Promise<ApiResponse<Category[]>> {
    return this.request<Category[]>(API_ENDPOINTS.CATEGORIES.LIST);
  }

  async getCategory(slug: string): Promise<ApiResponse<Category>> {
    return this.request<Category>(API_ENDPOINTS.CATEGORIES.DETAIL(slug));
  }

  // 标签相关 API
  async getTags(
    params?: TagQueryParams,
  ): Promise<ApiResponse<FrontendPaginatedResponse<Tag>>> {
    const queryString = params ? `?${buildQueryString(params)}` : "";

    const response = await this.request<Tag[]>(
      `${API_ENDPOINTS.TAGS.LIST}${queryString}`,
    );

    if (response.success && response.data) {
      // 后端返回简单数组，转换为前端期望的分页格式
      const transformedData: FrontendPaginatedResponse<Tag> = {
        data: response.data,
        pagination: {
          page: 1,
          per_page: response.data.length,
          total: response.data.length,
          total_pages: 1,
        },
      };
      return {
        success: true,
        data: transformedData,
        status: response.status,
      };
    }

    return {
      success: false,
      error: response.error,
      status: response.status,
    };
  }

  // 获取所有标签（简单数组格式）
  async getTagsList(): Promise<ApiResponse<Tag[]>> {
    return this.request<Tag[]>(API_ENDPOINTS.TAGS.LIST);
  }

  async getTag(slug: string): Promise<ApiResponse<Tag>> {
    return this.request<Tag>(API_ENDPOINTS.TAGS.DETAIL(slug));
  }

  // 文章管理 API (需要认证)
  async createPost(data: {
    title: string;
    content_markdown: string;
    excerpt?: string;
    thumbnail?: string;
    status?: "draft" | "published";
    categories?: string[];
    tags?: string[];
  }): Promise<ApiResponse<Post>> {
    // 转换前端字段到后端字段
    const payload = {
      title: data.title,
      content: data.content_markdown, // 后端期望 content 而不是 content_markdown
      category_ids: data.categories, // 转换为 category_ids
      tag_ids: data.tags, // 转换为 tag_ids
    };

    return this.request<Post>("/posts", {
      method: "POST",
      body: JSON.stringify(payload),
    });
  }

  async updatePost(id: string, data: {
    title?: string;
    content_markdown?: string;
    excerpt?: string;
    thumbnail?: string;
    status?: "draft" | "published";
    categories?: string[];
    tags?: string[];
    published_at?: string;
  }): Promise<ApiResponse<Post>> {
    // 转换前端字段到后端字段
    const payload: any = {};

    if (data.title !== undefined) payload.title = data.title;
    if (data.content_markdown !== undefined) payload.content = data.content_markdown;
    if (data.excerpt !== undefined) payload.excerpt = data.excerpt;
    if (data.thumbnail !== undefined) payload.thumbnail = data.thumbnail;
    if (data.categories !== undefined) payload.category_ids = data.categories;
    if (data.tags !== undefined) payload.tag_ids = data.tags;
    if (data.published_at !== undefined) payload.published_at = data.published_at;

    // 根据status决定是否发布
    if (data.status === "published") {
      payload.published_at = data.published_at || new Date().toISOString();
    }
    else if (data.status === "draft") {
      payload.unpublish = true;
    }

    return this.request<Post>(`/posts/${id}`, {
      method: "PUT",
      body: JSON.stringify(payload),
    });
  }

  async deletePost(id: string): Promise<ApiResponse<null>> {
    return this.request<null>(`/posts/${id}`, {
      method: "DELETE",
    });
  }

  // 批量操作
  async bulkUpdatePosts(data: {
    post_ids: string[];
    action: "publish" | "draft" | "delete";
  }): Promise<ApiResponse<null>> {
    return this.request<null>("/posts/bulk", {
      method: "POST",
      body: JSON.stringify(data),
    });
  }

  // 根据分类获取文章
  async getPostsByCategory(
    categorySlug: string,
    params?: PostQueryParams,
  ): Promise<ApiResponse<PostsResponse>> {
    const mergedParams = {
      category: categorySlug,
      ...params,
    };
    return this.getPosts(mergedParams);
  }

  // 根据标签获取文章
  async getPostsByTag(
    tagName: string,
    params?: PostQueryParams,
  ): Promise<ApiResponse<PostsResponse>> {
    const mergedParams = {
      tag: tagName,
      ...params,
    };
    return this.getPosts(mergedParams);
  }

  // 根据作者获取文章
  async getPostsByAuthor(
    authorUsername: string,
    params?: PostQueryParams,
  ): Promise<ApiResponse<PostsResponse>> {
    const mergedParams = {
      author: authorUsername,
      ...params,
    };
    return this.getPosts(mergedParams);
  }

  // 获取作者统计信息（基于文章数据推导）
  async getAuthorStats(
    authorUsername: string,
  ): Promise<ApiResponse<{
    totalPosts: number;
    publishedPosts: number;
    draftPosts: number;
    totalViews: number;
    categories: string[];
    tags: string[];
  }>> {
    try {
      // 获取该作者的所有已发布文章
      const publishedResponse = await this.getPostsByAuthor(authorUsername, {
        status: "published",
        per_page: 1000, // 获取所有文章用于统计
      });

      if (!publishedResponse.success || !publishedResponse.data) {
        return {
          success: false,
          error: { message: "获取作者文章失败" },
        };
      }

      const posts = publishedResponse.data.data;
      const categoriesSet = new Set<string>();
      const tagsSet = new Set<string>();

      posts.forEach((post) => {
        post.categories?.forEach(cat => categoriesSet.add(cat.name));
        post.tags?.forEach(tag => tagsSet.add(tag.name));
      });

      return {
        success: true,
        data: {
          totalPosts: posts.length,
          publishedPosts: posts.length,
          draftPosts: 0, // 无法获取草稿数据
          totalViews: 0, // 暂无浏览量数据
          categories: Array.from(categoriesSet),
          tags: Array.from(tagsSet),
        },
      };
    }
    catch (error) {
      return {
        success: false,
        error: {
          message: error instanceof Error ? error.message : "获取作者统计失败",
        },
      };
    }
  }
}

// 创建单例 API 客户端
let apiClient: ApiClient | null = null;

export function useApi() {
  if (!apiClient) {
    // 直接使用后端 URL，无论在客户端还是服务端
    const baseURL = "http://localhost:8080";

    apiClient = new ApiClient({
      baseURL,
    });
  }
  return apiClient;
}
