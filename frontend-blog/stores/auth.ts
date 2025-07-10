// 认证状态管理
import type {
    User,
    LoginCredentials,
    RegisterData,
    LoginResponse,
    ApiError,
} from "@shared/types";
import {
    STORAGE_KEYS,
    USER_ROLES,
    PERMISSION_ACTIONS,
    PERMISSION_RESOURCES,
} from "@shared/constants";

interface AuthState {
    user: User | null;
    isAuthenticated: boolean;
    isLoading: boolean;
    error: string | null;
    permissions: string[];
}

// 创建角色类型
type UserRole = (typeof USER_ROLES)[keyof typeof USER_ROLES];

export const useAuthStore = defineStore("auth", {
    state: (): AuthState => ({
        user: null,
        isAuthenticated: false,
        isLoading: false,
        error: null,
        permissions: [],
    }),

    getters: {
        // 检查是否为管理员
        isAdmin: (state: AuthState): boolean => {
            return state.user?.roles?.includes(USER_ROLES.ADMIN) || false;
        },

        // 检查是否为作者
        isAuthor: (state: AuthState): boolean => {
            return state.user?.roles?.includes(USER_ROLES.AUTHOR) || false;
        },

        // 检查是否为版主
        isModerator: (state: AuthState): boolean => {
            return state.user?.roles?.includes(USER_ROLES.MODERATOR) || false;
        },

        // 获取用户角色
        userRoles: (state: AuthState): string[] => {
            return state.user?.roles || [];
        },

        // 检查是否有指定权限
        hasPermission: (state: AuthState) => {
            return (resource: string, action: string): boolean => {
                if (state.user?.roles?.includes(USER_ROLES.ADMIN)) return true;
                const permission = `${resource}:${action}`;
                return state.permissions.includes(permission);
            };
        },

        // 检查是否可以创建文章
        canCreatePost: (state: AuthState): boolean => {
            const allowedRoles: UserRole[] = [
                USER_ROLES.ADMIN,
                USER_ROLES.AUTHOR,
                USER_ROLES.MODERATOR,
            ];
            return (
                state.user?.roles?.some((role: string) =>
                    allowedRoles.includes(role as UserRole)
                ) || false
            );
        },

        // 检查是否可以编辑指定文章
        canEditPost: (state: AuthState) => {
            return (post: { author_id: string }): boolean => {
                if (!state.user) return false;
                if (
                    state.user.roles?.includes(USER_ROLES.ADMIN) ||
                    state.user.roles?.includes(USER_ROLES.MODERATOR)
                )
                    return true;
                return post.author_id === state.user.id;
            };
        },

        // 检查是否可以删除指定文章
        canDeletePost: (state: AuthState) => {
            return (post: { author_id: string }): boolean => {
                if (!state.user) return false;
                if (state.user.roles?.includes(USER_ROLES.ADMIN)) return true;
                return post.author_id === state.user.id;
            };
        },
    },

    actions: {
        // 初始化认证状态
        async initialize() {
            this.isLoading = true;
            this.error = null;

            try {
                const token = this.getStoredToken();
                if (!token) {
                    this.isLoading = false;
                    return;
                }

                // 验证当前用户信息
                await this.getCurrentUser();
            } catch (error) {
                console.error("认证初始化失败:", error);
                this.clearAuth();
            } finally {
                this.isLoading = false;
            }
        },

        // 用户登录
        async login(
            credentials: LoginCredentials
        ): Promise<{ success: boolean; message?: string }> {
            this.isLoading = true;
            this.error = null;

            try {
                const api = useApi();
                const response = await api.login(credentials);

                if (response.data) {
                    this.setAuth(response.data);

                    // 跳转到仪表板或之前的页面
                    const redirectTo = this.getRedirectPath();
                    await navigateTo(redirectTo);

                    return { success: true };
                }

                return { success: false, message: "登录失败" };
            } catch (error) {
                const apiError = error as ApiError;
                this.error = apiError.message || "登录失败";
                console.error("登录错误:", error);
                return { success: false, message: this.error };
            } finally {
                this.isLoading = false;
            }
        },

        // 用户注册
        async register(
            data: RegisterData
        ): Promise<{ success: boolean; message?: string }> {
            this.isLoading = true;
            this.error = null;

            try {
                const api = useApi();
                const response = await api.register(data);

                if (response.data) {
                    // 注册成功后自动登录
                    this.setAuth(response.data);

                    // 跳转到邮箱验证页面或仪表板
                    await navigateTo("/auth/verify-email");

                    return { success: true };
                }

                return { success: false, message: "注册失败" };
            } catch (error) {
                const apiError = error as ApiError;
                this.error = apiError.message || "注册失败";
                console.error("注册错误:", error);
                return { success: false, message: this.error };
            } finally {
                this.isLoading = false;
            }
        },

        // 用户登出
        async logout(): Promise<void> {
            this.isLoading = true;

            try {
                const api = useApi();
                await api.logout();
            } catch (error) {
                console.error("登出API调用失败:", error);
                // 即使API调用失败，也要清除本地状态
            } finally {
                this.clearAuth();
                this.isLoading = false;
                await navigateTo("/auth/login");
            }
        },

        // 获取当前用户信息
        async getCurrentUser(): Promise<void> {
            try {
                const api = useApi();
                const response = await api.getMe();

                if (response.data) {
                    this.user = response.data;
                    this.isAuthenticated = true;

                    // 获取用户权限
                    await this.fetchUserPermissions();
                }
            } catch (error) {
                console.error("获取用户信息失败:", error);
                this.clearAuth();
                throw error;
            }
        },

        // 获取用户权限
        async fetchUserPermissions(): Promise<void> {
            try {
                // 这里应该调用获取用户权限的API
                // 暂时根据角色设置基本权限
                if (this.user?.roles && this.user.roles.length > 0) {
                    this.permissions = this.generatePermissionsFromRoles(
                        this.user.roles
                    );
                }
            } catch (error) {
                console.error("获取用户权限失败:", error);
            }
        },

        // 根据角色生成权限列表
        generatePermissionsFromRoles(roles: string[]): string[] {
            const permissions: string[] = [];

            roles.forEach((role) => {
                switch (role) {
                    case USER_ROLES.ADMIN:
                        // 管理员拥有所有权限
                        permissions.push(
                            `${PERMISSION_RESOURCES.POST}:${PERMISSION_ACTIONS.CREATE}`,
                            `${PERMISSION_RESOURCES.POST}:${PERMISSION_ACTIONS.READ}`,
                            `${PERMISSION_RESOURCES.POST}:${PERMISSION_ACTIONS.UPDATE}`,
                            `${PERMISSION_RESOURCES.POST}:${PERMISSION_ACTIONS.DELETE}`,
                            `${PERMISSION_RESOURCES.POST}:${PERMISSION_ACTIONS.PUBLISH}`,
                            `${PERMISSION_RESOURCES.POST}:${PERMISSION_ACTIONS.BAN}`,
                            `${PERMISSION_RESOURCES.CATEGORY}:${PERMISSION_ACTIONS.CREATE}`,
                            `${PERMISSION_RESOURCES.CATEGORY}:${PERMISSION_ACTIONS.UPDATE}`,
                            `${PERMISSION_RESOURCES.CATEGORY}:${PERMISSION_ACTIONS.DELETE}`,
                            `${PERMISSION_RESOURCES.TAG}:${PERMISSION_ACTIONS.CREATE}`,
                            `${PERMISSION_RESOURCES.TAG}:${PERMISSION_ACTIONS.UPDATE}`,
                            `${PERMISSION_RESOURCES.TAG}:${PERMISSION_ACTIONS.DELETE}`,
                            `${PERMISSION_RESOURCES.USER}:${PERMISSION_ACTIONS.READ}`,
                            `${PERMISSION_RESOURCES.USER}:${PERMISSION_ACTIONS.UPDATE}`,
                            `${PERMISSION_RESOURCES.USER}:${PERMISSION_ACTIONS.DELETE}`
                        );
                        break;

                    case USER_ROLES.MODERATOR:
                        // 版主权限
                        permissions.push(
                            `${PERMISSION_RESOURCES.POST}:${PERMISSION_ACTIONS.CREATE}`,
                            `${PERMISSION_RESOURCES.POST}:${PERMISSION_ACTIONS.READ}`,
                            `${PERMISSION_RESOURCES.POST}:${PERMISSION_ACTIONS.UPDATE}`,
                            `${PERMISSION_RESOURCES.POST}:${PERMISSION_ACTIONS.DELETE}`,
                            `${PERMISSION_RESOURCES.POST}:${PERMISSION_ACTIONS.PUBLISH}`,
                            `${PERMISSION_RESOURCES.POST}:${PERMISSION_ACTIONS.BAN}`,
                            `${PERMISSION_RESOURCES.CATEGORY}:${PERMISSION_ACTIONS.CREATE}`,
                            `${PERMISSION_RESOURCES.TAG}:${PERMISSION_ACTIONS.CREATE}`
                        );
                        break;

                    case USER_ROLES.AUTHOR:
                        // 作者权限
                        permissions.push(
                            `${PERMISSION_RESOURCES.POST}:${PERMISSION_ACTIONS.CREATE}`,
                            `${PERMISSION_RESOURCES.POST}:${PERMISSION_ACTIONS.READ}`,
                            `${PERMISSION_RESOURCES.POST}:${PERMISSION_ACTIONS.UPDATE}`,
                            `${PERMISSION_RESOURCES.POST}:${PERMISSION_ACTIONS.DELETE}`,
                            `${PERMISSION_RESOURCES.POST}:${PERMISSION_ACTIONS.PUBLISH}`
                        );
                        break;

                    case USER_ROLES.USER:
                        // 普通用户权限
                        permissions.push(
                            `${PERMISSION_RESOURCES.POST}:${PERMISSION_ACTIONS.READ}`
                        );
                        break;
                }
            });

            return [...new Set(permissions)]; // 去重
        },

        // 邮箱验证
        async verifyEmail(
            token: string
        ): Promise<{ success: boolean; message?: string }> {
            this.isLoading = true;
            this.error = null;

            try {
                const api = useApi();
                await api.verifyEmail(token);

                // 重新获取用户信息以更新验证状态
                await this.getCurrentUser();

                return { success: true, message: "邮箱验证成功" };
            } catch (error) {
                const apiError = error as ApiError;
                this.error = apiError.message || "邮箱验证失败";
                return { success: false, message: this.error };
            } finally {
                this.isLoading = false;
            }
        },

        // 忘记密码
        async forgotPassword(
            email: string
        ): Promise<{ success: boolean; message?: string }> {
            this.isLoading = true;
            this.error = null;

            try {
                const api = useApi();
                await api.forgotPassword(email);
                return { success: true, message: "密码重置邮件已发送" };
            } catch (error) {
                const apiError = error as ApiError;
                this.error = apiError.message || "发送重置邮件失败";
                return { success: false, message: this.error };
            } finally {
                this.isLoading = false;
            }
        },

        // 设置认证状态
        setAuth(loginResponse: LoginResponse): void {
            this.user = loginResponse.user;
            this.isAuthenticated = true;
            this.error = null;

            // 存储令牌
            if (import.meta.client) {
                localStorage.setItem(
                    STORAGE_KEYS.ACCESS_TOKEN,
                    loginResponse.access_token
                );
                localStorage.setItem(
                    STORAGE_KEYS.REFRESH_TOKEN,
                    loginResponse.refresh_token
                );
            }

            // 获取用户权限
            this.fetchUserPermissions();
        },

        // 清除认证状态
        clearAuth(): void {
            this.user = null;
            this.isAuthenticated = false;
            this.error = null;
            this.permissions = [];

            // 清除存储的令牌
            if (import.meta.client) {
                localStorage.removeItem(STORAGE_KEYS.ACCESS_TOKEN);
                localStorage.removeItem(STORAGE_KEYS.REFRESH_TOKEN);
            }
        },

        // 获取存储的令牌
        getStoredToken(): string | null {
            if (import.meta.client) {
                return localStorage.getItem(STORAGE_KEYS.ACCESS_TOKEN);
            }
            return null;
        },

        // 获取重定向路径
        getRedirectPath(): string {
            if (import.meta.client) {
                const redirectTo = sessionStorage.getItem(
                    "redirect_after_login"
                );
                sessionStorage.removeItem("redirect_after_login");
                return redirectTo || "/user/profile";
            }
            return "/user/profile";
        },

        // 设置重定向路径
        setRedirectPath(path: string): void {
            if (import.meta.client) {
                sessionStorage.setItem("redirect_after_login", path);
            }
        },

        // 清除错误状态
        clearError(): void {
            this.error = null;
        },
    },
});

// 非setup中使用的引用
export const useAuthStoreRefs = () => storeToRefs(useAuthStore());
