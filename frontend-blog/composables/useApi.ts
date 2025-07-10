import type { ApiResponse, ApiError } from "~/types";

// API客户端类
class ApiClient {
    private baseURL: string;
    private token: string | null = null;

    constructor(baseURL: string) {
        this.baseURL = baseURL;
        this.loadTokenFromStorage();
    }

    // 从localStorage加载token
    private loadTokenFromStorage() {
        if (process.client) {
            this.token = localStorage.getItem("access_token");
        }
    }

    // 设置token
    setToken(token: string) {
        this.token = token;
        if (process.client) {
            localStorage.setItem("access_token", token);
        }
    }

    // 清除token
    clearToken() {
        this.token = null;
        if (process.client) {
            localStorage.removeItem("access_token");
            localStorage.removeItem("refresh_token");
        }
    }

    // 获取请求头
    private getHeaders(customHeaders: Record<string, string> = {}) {
        const headers: Record<string, string> = {
            "Content-Type": "application/json",
            ...customHeaders,
        };

        if (this.token) {
            headers.Authorization = `Bearer ${this.token}`;
        }

        return headers;
    }

    // 处理响应错误
    private handleError(error: any): never {
        if (error.response) {
            // 服务器返回错误响应
            const apiError: ApiError = {
                message:
                    error.response._data?.message ||
                    error.response.statusText ||
                    "请求失败",
                code: error.response.status?.toString(),
                details: error.response._data,
            };
            throw apiError;
        } else if (error.request) {
            // 网络错误
            const networkError: ApiError = {
                message: "网络连接失败，请检查网络设置",
                code: "NETWORK_ERROR",
            };
            throw networkError;
        } else {
            // 其他错误
            const unknownError: ApiError = {
                message: error.message || "未知错误",
                code: "UNKNOWN_ERROR",
            };
            throw unknownError;
        }
    }

    // 刷新token
    private async refreshToken(): Promise<string> {
        const refreshToken = process.client
            ? localStorage.getItem("refresh_token")
            : null;
        if (!refreshToken) {
            throw new Error("No refresh token available");
        }

        try {
            const response = await $fetch<{
                access_token: string;
                refresh_token: string;
            }>("/auth/refresh", {
                method: "POST",
                baseURL: this.baseURL,
                body: { refresh_token: refreshToken },
            });

            // 更新token
            this.setToken(response.access_token);
            if (process.client) {
                localStorage.setItem("refresh_token", response.refresh_token);
            }

            return response.access_token;
        } catch (error) {
            // 刷新失败，清除所有token
            this.clearToken();
            throw error;
        }
    }

    // 通用请求方法
    private async request<T>(
        url: string,
        options: {
            method?: "GET" | "POST" | "PUT" | "DELETE" | "PATCH";
            body?: any;
            headers?: Record<string, string>;
            retry?: boolean;
        } = {}
    ): Promise<T> {
        const { method = "GET", body, headers = {}, retry = true } = options;

        try {
            const response = await $fetch<T>(url, {
                method,
                baseURL: this.baseURL,
                headers: this.getHeaders(headers),
                body: body ? JSON.stringify(body) : undefined,
            });

            return response;
        } catch (error: any) {
            // 如果是401错误且还没重试过，尝试刷新token
            if (error.response?.status === 401 && retry && this.token) {
                try {
                    await this.refreshToken();
                    // 重新发送请求（设置retry为false避免无限循环）
                    return this.request<T>(url, { ...options, retry: false });
                } catch (refreshError) {
                    // 刷新失败，抛出原始错误
                    this.handleError(error);
                }
            }

            this.handleError(error);
        }
    }

    // GET请求
    async get<T>(url: string, headers?: Record<string, string>): Promise<T> {
        return this.request<T>(url, { method: "GET", headers });
    }

    // POST请求
    async post<T>(
        url: string,
        data?: any,
        headers?: Record<string, string>
    ): Promise<T> {
        return this.request<T>(url, { method: "POST", body: data, headers });
    }

    // PUT请求
    async put<T>(
        url: string,
        data?: any,
        headers?: Record<string, string>
    ): Promise<T> {
        return this.request<T>(url, { method: "PUT", body: data, headers });
    }

    // DELETE请求
    async delete<T>(url: string, headers?: Record<string, string>): Promise<T> {
        return this.request<T>(url, { method: "DELETE", headers });
    }

    // PATCH请求
    async patch<T>(
        url: string,
        data?: any,
        headers?: Record<string, string>
    ): Promise<T> {
        return this.request<T>(url, { method: "PATCH", body: data, headers });
    }
}

// 创建API客户端实例
let apiClient: ApiClient | null = null;

export const useApi = () => {
    if (!apiClient) {
        const config = useRuntimeConfig();
        apiClient = new ApiClient(config.public.apiBaseUrl as string);
    }

    return apiClient;
};

// 便捷的API调用composable
export const useApiCall = () => {
    const api = useApi();
    const isLoading = ref(false);
    const error = ref<ApiError | null>(null);

    const execute = async <T>(
        apiCall: () => Promise<T>,
        options: {
            onSuccess?: (data: T) => void;
            onError?: (error: ApiError) => void;
            showErrorToast?: boolean;
        } = {}
    ): Promise<T | null> => {
        const { onSuccess, onError, showErrorToast = true } = options;

        try {
            isLoading.value = true;
            error.value = null;

            const result = await apiCall();

            if (onSuccess) {
                onSuccess(result);
            }

            return result;
        } catch (err: any) {
            error.value = err as ApiError;

            if (onError) {
                onError(err as ApiError);
            }

            if (showErrorToast) {
                // 这里可以集成toast通知
                console.error("API Error:", err.message);
            }

            return null;
        } finally {
            isLoading.value = false;
        }
    };

    return {
        api,
        isLoading: readonly(isLoading),
        error: readonly(error),
        execute,
    };
};
