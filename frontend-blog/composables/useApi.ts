// API 客户端 Composable
import type {
    ApiResponse,
    ApiError,
    LoginCredentials,
    RegisterData,
    LoginResponse,
    Post,
    Category,
    Tag,
    User,
    PaginatedResponse,
    PostQueryParams,
    CategoryQueryParams,
    TagQueryParams,
    HTTP_STATUS,
} from "~/types";

import { API_ENDPOINTS, ERROR_CODES, STORAGE_KEYS } from "~/types";

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
        options: RequestInit = {}
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
            } else {
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
        } catch (error) {
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
        credentials: LoginCredentials
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

    async logout(): Promise<ApiResponse<void>> {
        return this.request<void>(API_ENDPOINTS.AUTH.LOGOUT, {
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

    // 文章相关 API
    async getPosts(
        params?: PostQueryParams
    ): Promise<ApiResponse<PaginatedResponse<Post>>> {
        const queryString = params
            ? `?${new URLSearchParams(
                  params as Record<string, string>
              ).toString()}`
            : "";
        return this.request<PaginatedResponse<Post>>(
            `${API_ENDPOINTS.POSTS.LIST}${queryString}`
        );
    }

    async getPost(slug: string): Promise<ApiResponse<Post>> {
        return this.request<Post>(API_ENDPOINTS.POSTS.DETAIL(slug));
    }

    // 分类相关 API
    async getCategories(
        params?: CategoryQueryParams
    ): Promise<ApiResponse<PaginatedResponse<Category>>> {
        const queryString = params
            ? `?${new URLSearchParams(
                  params as Record<string, string>
              ).toString()}`
            : "";
        return this.request<PaginatedResponse<Category>>(
            `${API_ENDPOINTS.CATEGORIES.LIST}${queryString}`
        );
    }

    async getCategory(slug: string): Promise<ApiResponse<Category>> {
        return this.request<Category>(API_ENDPOINTS.CATEGORIES.DETAIL(slug));
    }

    // 标签相关 API
    async getTags(
        params?: TagQueryParams
    ): Promise<ApiResponse<PaginatedResponse<Tag>>> {
        const queryString = params
            ? `?${new URLSearchParams(
                  params as Record<string, string>
              ).toString()}`
            : "";
        return this.request<PaginatedResponse<Tag>>(
            `${API_ENDPOINTS.TAGS.LIST}${queryString}`
        );
    }

    async getTag(slug: string): Promise<ApiResponse<Tag>> {
        return this.request<Tag>(API_ENDPOINTS.TAGS.DETAIL(slug));
    }
}

// 创建单例 API 客户端
let apiClient: ApiClient | null = null;

export function useApi() {
    if (!apiClient) {
        const config = useRuntimeConfig();
        apiClient = new ApiClient({
            baseURL: config.public.apiBaseUrl as string,
        });
    }
    return apiClient;
}
