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
} from "@shared/types";
import {
    API_ENDPOINTS,
    HTTP_STATUS,
    ERROR_CODES,
    STORAGE_KEYS,
} from "@shared/constants";

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

    private async request<T>(
        endpoint: string,
        options: RequestInit = {}
    ): Promise<ApiResponse<T>> {
        const url = `${this.baseURL}${endpoint}`;

        // 获取访问令牌
        const token = this.getAccessToken();

        const config: RequestInit = {
            ...options,
            headers: {
                "Content-Type": "application/json",
                ...options.headers,
                ...(token && { Authorization: `Bearer ${token}` }),
            },
        };

        try {
            const controller = new AbortController();
            const timeoutId = setTimeout(
                () => controller.abort(),
                this.timeout
            );

            const response = await fetch(url, {
                ...config,
                signal: controller.signal,
            });

            clearTimeout(timeoutId);

            // 处理 token 过期
            if (response.status === HTTP_STATUS.UNAUTHORIZED) {
                await this.handleTokenRefresh();
                // 重试请求
                const retryToken = this.getAccessToken();
                if (retryToken && retryToken !== token) {
                    const retryResponse = await fetch(url, {
                        ...config,
                        headers: {
                            ...config.headers,
                            Authorization: `Bearer ${retryToken}`,
                        },
                    });
                    return await this.handleResponse<T>(retryResponse);
                }
            }

            return await this.handleResponse<T>(response);
        } catch (error) {
            if (error instanceof Error && error.name === "AbortError") {
                throw new Error("请求超时");
            }
            throw error;
        }
    }

    private async handleResponse<T>(
        response: Response
    ): Promise<ApiResponse<T>> {
        const contentType = response.headers.get("content-type");
        const isJson = contentType?.includes("application/json");

        let data: any;
        if (isJson) {
            data = await response.json();
        } else {
            data = await response.text();
        }

        if (!response.ok) {
            const error: ApiError = {
                message: data?.message || `HTTP Error: ${response.status}`,
                code: data?.code || ERROR_CODES.NETWORK_ERROR,
                details: data,
            };
            throw error;
        }

        return {
            data,
            message: data?.message,
            status: response.status,
        };
    }

    private getAccessToken(): string | null {
        if (import.meta.client) {
            return localStorage.getItem(STORAGE_KEYS.ACCESS_TOKEN);
        }
        return null;
    }

    private getRefreshToken(): string | null {
        if (import.meta.client) {
            return localStorage.getItem(STORAGE_KEYS.REFRESH_TOKEN);
        }
        return null;
    }

    private async handleTokenRefresh(): Promise<void> {
        const refreshToken = this.getRefreshToken();
        if (!refreshToken) {
            this.clearTokens();
            await navigateTo("/auth/login");
            return;
        }

        try {
            const response = await fetch(
                `${this.baseURL}${API_ENDPOINTS.AUTH.REFRESH}`,
                {
                    method: "POST",
                    headers: { "Content-Type": "application/json" },
                    body: JSON.stringify({ refresh_token: refreshToken }),
                }
            );

            if (response.ok) {
                const data = await response.json();
                if (import.meta.client) {
                    localStorage.setItem(
                        STORAGE_KEYS.ACCESS_TOKEN,
                        data.access_token
                    );
                    if (data.refresh_token) {
                        localStorage.setItem(
                            STORAGE_KEYS.REFRESH_TOKEN,
                            data.refresh_token
                        );
                    }
                }
            } else {
                this.clearTokens();
                await navigateTo("/auth/login");
            }
        } catch (error) {
            this.clearTokens();
            await navigateTo("/auth/login");
        }
    }

    private clearTokens(): void {
        if (import.meta.client) {
            localStorage.removeItem(STORAGE_KEYS.ACCESS_TOKEN);
            localStorage.removeItem(STORAGE_KEYS.REFRESH_TOKEN);
        }
    }

    // ===== 认证相关 API =====
    async login(
        credentials: LoginCredentials
    ): Promise<ApiResponse<LoginResponse>> {
        return this.request<LoginResponse>(API_ENDPOINTS.AUTH.LOGIN, {
            method: "POST",
            body: JSON.stringify(credentials),
        });
    }

    async register(data: RegisterData): Promise<ApiResponse<LoginResponse>> {
        return this.request<LoginResponse>(API_ENDPOINTS.AUTH.REGISTER, {
            method: "POST",
            body: JSON.stringify(data),
        });
    }

    async logout(): Promise<ApiResponse<void>> {
        const response = await this.request<void>(API_ENDPOINTS.AUTH.LOGOUT, {
            method: "POST",
        });
        this.clearTokens();
        return response;
    }

    async getMe(): Promise<ApiResponse<User>> {
        return this.request<User>(API_ENDPOINTS.USERS.ME);
    }

    async verifyEmail(token: string): Promise<ApiResponse<void>> {
        return this.request<void>(API_ENDPOINTS.AUTH.VERIFY_EMAIL, {
            method: "POST",
            body: JSON.stringify({ token }),
        });
    }

    async forgotPassword(email: string): Promise<ApiResponse<void>> {
        return this.request<void>(API_ENDPOINTS.AUTH.FORGOT_PASSWORD, {
            method: "POST",
            body: JSON.stringify({ email }),
        });
    }

    // ===== 文章相关 API =====
    async getPosts(
        params?: PostQueryParams
    ): Promise<ApiResponse<PaginatedResponse<Post>>> {
        const queryString = params
            ? `?${new URLSearchParams(params as any).toString()}`
            : "";
        return this.request<PaginatedResponse<Post>>(
            `${API_ENDPOINTS.POSTS.LIST}${queryString}`
        );
    }

    async getPost(slug: string): Promise<ApiResponse<Post>> {
        return this.request<Post>(API_ENDPOINTS.POSTS.DETAIL(slug));
    }

    async createPost(data: any): Promise<ApiResponse<Post>> {
        return this.request<Post>(API_ENDPOINTS.POSTS.CREATE, {
            method: "POST",
            body: JSON.stringify(data),
        });
    }

    async updatePost(id: string, data: any): Promise<ApiResponse<Post>> {
        return this.request<Post>(API_ENDPOINTS.POSTS.UPDATE(id), {
            method: "PUT",
            body: JSON.stringify(data),
        });
    }

    async deletePost(id: string): Promise<ApiResponse<void>> {
        return this.request<void>(API_ENDPOINTS.POSTS.DELETE(id), {
            method: "DELETE",
        });
    }

    // ===== 分类相关 API =====
    async getCategories(
        params?: CategoryQueryParams
    ): Promise<ApiResponse<PaginatedResponse<Category>>> {
        const queryString = params
            ? `?${new URLSearchParams(params as any).toString()}`
            : "";
        return this.request<PaginatedResponse<Category>>(
            `${API_ENDPOINTS.CATEGORIES.LIST}${queryString}`
        );
    }

    async getCategory(slug: string): Promise<ApiResponse<Category>> {
        return this.request<Category>(API_ENDPOINTS.CATEGORIES.DETAIL(slug));
    }

    // ===== 标签相关 API =====
    async getTags(
        params?: TagQueryParams
    ): Promise<ApiResponse<PaginatedResponse<Tag>>> {
        const queryString = params
            ? `?${new URLSearchParams(params as any).toString()}`
            : "";
        return this.request<PaginatedResponse<Tag>>(
            `${API_ENDPOINTS.TAGS.LIST}${queryString}`
        );
    }

    async getTag(slug: string): Promise<ApiResponse<Tag>> {
        return this.request<Tag>(API_ENDPOINTS.TAGS.DETAIL(slug));
    }

    // ===== 搜索相关 API =====
    async searchPosts(
        query: string,
        params?: any
    ): Promise<ApiResponse<PaginatedResponse<Post>>> {
        const searchParams = new URLSearchParams({ q: query, ...params });
        return this.request<PaginatedResponse<Post>>(
            `${API_ENDPOINTS.SEARCH.POSTS}?${searchParams}`
        );
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

// 导出类型
export type { ApiClient, ApiClientConfig };
