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

      const data = await response.json();

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
            message: data.message || "请求失败",
            code: data.code || ERROR_CODES.SERVER_ERROR,
            details: data.details,
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

    const response = await this.request<PaginatedResponse<Category>>(
      `${API_ENDPOINTS.CATEGORIES.LIST}${queryString}`,
    );

    if (response.success && response.data) {
      const transformedData = transformPaginatedResponse(response.data);
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

  async getCategory(slug: string): Promise<ApiResponse<Category>> {
    return this.request<Category>(API_ENDPOINTS.CATEGORIES.DETAIL(slug));
  }

  // 标签相关 API
  async getTags(
    params?: TagQueryParams,
  ): Promise<ApiResponse<FrontendPaginatedResponse<Tag>>> {
    const queryString = params ? `?${buildQueryString(params)}` : "";

    const response = await this.request<PaginatedResponse<Tag>>(
      `${API_ENDPOINTS.TAGS.LIST}${queryString}`,
    );

    if (response.success && response.data) {
      const transformedData = transformPaginatedResponse(response.data);
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

  async getTag(slug: string): Promise<ApiResponse<Tag>> {
    return this.request<Tag>(API_ENDPOINTS.TAGS.DETAIL(slug));
  }
}

// 创建单例 API 客户端
let apiClient: ApiClient | null = null;

export function useApi() {
  if (!apiClient) {
    // 在浏览器环境中使用相对路径通过代理访问
    // 在服务器端直接使用后端 URL
    const baseURL = import.meta.client ? "" : "http://localhost:8080";

    apiClient = new ApiClient({
      baseURL,
    });
  }
  return apiClient;
}
