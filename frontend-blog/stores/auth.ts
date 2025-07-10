import type {
    User,
    UserPublic,
    LoginCredentials,
    RegisterData,
    LoginResponse,
    Permission,
} from "~/types";

export const useAuthStore = defineStore("auth", () => {
    // 状态
    const user = ref<UserPublic | null>(null);
    const permissions = ref<Permission[]>([]);
    const isLoading = ref(false);
    const error = ref<string | null>(null);

    // 计算属性
    const isAuthenticated = computed(() => !!user.value);
    const userRoles = computed(() => user.value?.roles || []);

    // API客户端
    const api = useApi();

    // 权限检查函数
    const hasPermission = (permission: string): boolean => {
        return permissions.value.some((p) => p.name === permission);
    };

    // 批量权限检查
    const hasAnyPermission = (permissionList: string[]): boolean => {
        return permissionList.some((permission) => hasPermission(permission));
    };

    const hasAllPermissions = (permissionList: string[]): boolean => {
        return permissionList.every((permission) => hasPermission(permission));
    };

    // 角色检查函数
    const hasRole = (role: string): boolean => {
        return userRoles.value.includes(role);
    };

    const hasAnyRole = (roles: string[]): boolean => {
        return roles.some((role) => hasRole(role));
    };

    // 登录方法
    const login = async (credentials: LoginCredentials): Promise<boolean> => {
        try {
            isLoading.value = true;
            error.value = null;

            const response = await api.post<LoginResponse>(
                "/auth/login",
                credentials
            );

            // 存储tokens
            api.setToken(response.access_token);
            if (process.client) {
                localStorage.setItem("refresh_token", response.refresh_token);
            }

            // 更新用户信息
            user.value = response.user;

            // 获取用户权限
            await fetchUserPermissions();

            return true;
        } catch (err: any) {
            error.value = err.message || "登录失败";
            return false;
        } finally {
            isLoading.value = false;
        }
    };

    // 注册方法
    const register = async (registerData: RegisterData): Promise<boolean> => {
        try {
            isLoading.value = true;
            error.value = null;

            const response = await api.post<UserPublic>(
                "/auth/register",
                registerData
            );

            // 注册成功后不自动登录，需要用户验证邮箱
            return true;
        } catch (err: any) {
            error.value = err.message || "注册失败";
            return false;
        } finally {
            isLoading.value = false;
        }
    };

    // 登出方法
    const logout = async (): Promise<void> => {
        try {
            const refreshToken = process.client
                ? localStorage.getItem("refresh_token")
                : null;

            if (refreshToken) {
                // 调用后端登出接口
                await api.post("/auth/logout", { refresh_token: refreshToken });
            }
        } catch (err) {
            // 登出失败也继续清理本地状态
            console.error("Logout API call failed:", err);
        } finally {
            // 清理状态
            user.value = null;
            permissions.value = [];
            api.clearToken();

            // 跳转到首页
            await navigateTo("/");
        }
    };

    // 获取当前用户信息
    const fetchUserProfile = async (): Promise<void> => {
        if (!isAuthenticated.value) return;

        try {
            const response = await api.get<UserPublic>("/me");
            user.value = response;
        } catch (err: any) {
            console.error("Failed to fetch user profile:", err);
            // 如果获取用户信息失败，可能token已失效，执行登出
            await logout();
        }
    };

    // 获取用户权限
    const fetchUserPermissions = async (): Promise<void> => {
        if (!isAuthenticated.value) return;

        try {
            const response = await api.get<Permission[]>("/me/permissions");
            permissions.value = response || [];
        } catch (err: any) {
            console.error("Failed to fetch user permissions:", err);
            permissions.value = [];
        }
    };

    // 初始化认证状态
    const initAuth = async (): Promise<void> => {
        if (process.client) {
            const token = localStorage.getItem("access_token");
            if (token) {
                api.setToken(token);
                await fetchUserProfile();
                await fetchUserPermissions();
            }
        }
    };

    // 更新用户资料
    const updateProfile = async (
        profileData: Partial<UserPublic>
    ): Promise<boolean> => {
        try {
            isLoading.value = true;
            error.value = null;

            const response = await api.put<UserPublic>("/me", profileData);
            user.value = response;

            return true;
        } catch (err: any) {
            error.value = err.message || "更新资料失败";
            return false;
        } finally {
            isLoading.value = false;
        }
    };

    // 修改密码
    const changePassword = async (passwordData: {
        current_password: string;
        new_password: string;
        confirm_password: string;
    }): Promise<boolean> => {
        try {
            isLoading.value = true;
            error.value = null;

            await api.put("/me/password", passwordData);
            return true;
        } catch (err: any) {
            error.value = err.message || "修改密码失败";
            return false;
        } finally {
            isLoading.value = false;
        }
    };

    // 忘记密码
    const forgotPassword = async (email: string): Promise<boolean> => {
        try {
            isLoading.value = true;
            error.value = null;

            await api.post("/auth/forgot-password", { email });
            return true;
        } catch (err: any) {
            error.value = err.message || "发送重置邮件失败";
            return false;
        } finally {
            isLoading.value = false;
        }
    };

    // 重置密码
    const resetPassword = async (resetData: {
        token: string;
        password: string;
        confirm_password: string;
    }): Promise<boolean> => {
        try {
            isLoading.value = true;
            error.value = null;

            await api.post("/auth/reset-password", resetData);
            return true;
        } catch (err: any) {
            error.value = err.message || "重置密码失败";
            return false;
        } finally {
            isLoading.value = false;
        }
    };

    // 验证邮箱
    const verifyEmail = async (token: string): Promise<boolean> => {
        try {
            isLoading.value = true;
            error.value = null;

            await api.post("/auth/verify-email", { token });
            return true;
        } catch (err: any) {
            error.value = err.message || "邮箱验证失败";
            return false;
        } finally {
            isLoading.value = false;
        }
    };

    // 清除错误
    const clearError = () => {
        error.value = null;
    };

    return {
        // 状态
        user: readonly(user),
        permissions: readonly(permissions),
        isLoading: readonly(isLoading),
        error: readonly(error),

        // 计算属性
        isAuthenticated,
        userRoles,

        // 权限检查方法
        hasPermission,
        hasAnyPermission,
        hasAllPermissions,
        hasRole,
        hasAnyRole,

        // 操作方法
        login,
        register,
        logout,
        fetchUserProfile,
        fetchUserPermissions,
        initAuth,
        updateProfile,
        changePassword,
        forgotPassword,
        resetPassword,
        verifyEmail,
        clearError,
    };
});

// 非setup中使用
export const useAuthStoreRefs = () => storeToRefs(useAuthStore());
