// 认证状态管理
import type { User, LoginCredentials, RegisterData } from "~/types";

interface AuthState {
  user: User | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  error: string | null;
  permissions: string[];
}

// 使用常量
const storageKeys = {
  ACCESS_TOKEN: "access_token",
  REFRESH_TOKEN: "refresh_token",
  USER: "user",
} as const;

const userRoles = {
  ADMIN: "admin",
  MODERATOR: "moderator",
  AUTHOR: "author",
  USER: "user",
} as const;

const permissionActions = {
  CREATE: "create",
  READ: "read",
  UPDATE: "update",
  DELETE: "delete",
  PUBLISH: "publish",
  BAN: "ban",
} as const;

const permissionResources = {
  POST: "post",
  CATEGORY: "category",
  TAG: "tag",
  USER: "user",
} as const;

export const useAuthStore = defineStore("auth", {
  state: (): AuthState => ({
    user: null,
    isAuthenticated: false,
    isLoading: false,
    error: null,
    permissions: [],
  }),

  getters: {
    // 检查用户是否有特定角色
    hasRole: state => (role: keyof typeof userRoles) => {
      return state.user?.roles?.includes(userRoles[role]) ?? false;
    },

    // 检查用户是否为管理员
    isAdmin: (state) => {
      return state.user?.roles?.includes(userRoles.ADMIN) ?? false;
    },

    // 检查用户是否为作者
    isAuthor: (state) => {
      return (
        state.user?.roles?.includes(userRoles.AUTHOR)
        || state.user?.roles?.includes(userRoles.ADMIN)
        || state.user?.roles?.includes(userRoles.MODERATOR)
      );
    },

    // 检查用户是否有特定权限
    hasPermission:
            state =>
              (
                resource: keyof typeof permissionResources,
                action: keyof typeof permissionActions,
              ) => {
                const permission = `${permissionResources[resource]}:${permissionActions[action]}`;
                return state.permissions.includes(permission);
              },

    // 获取用户显示名称
    displayName: (state) => {
      return state.user?.username || "未知用户";
    },
  },

  actions: {
    // 初始化认证状态（从本地存储恢复）
    async initialize() {
      // 只在客户端执行
      if (typeof window === "undefined") {
        return;
      }

      this.isLoading = true;
      this.error = null;

      try {
        const token = localStorage.getItem(storageKeys.ACCESS_TOKEN);
        const userStr = localStorage.getItem(storageKeys.USER);

        if (token && userStr) {
          const user = JSON.parse(userStr);
          await this.setUser(user, token);
          // 验证 token 是否仍然有效
          await this.validateToken();
        }
      }
      catch (error) {
        console.error("初始化认证状态失败:", error);
        this.clearAuth();
      }
      finally {
        this.isLoading = false;
      }
    },

    // 登录
    async login(
      credentials: LoginCredentials,
    ): Promise<{ success: boolean; message?: string }> {
      this.isLoading = true;
      this.error = null;

      try {
        const api = useApi();
        const response = await api.login(credentials);

        if (response.success && response.data) {
          const { access_token, refresh_token, user } = response.data;
          await this.setUser(user, access_token, refresh_token);
          return { success: true };
        }
        else {
          this.error = response.error?.message || "登录失败";
          return {
            success: false,
            message: this.error,
          };
        }
      }
      catch (error) {
        let message = "网络错误，请稍后重试";
        if (
          error
          && typeof error === "object"
          && "message" in error
          && typeof (error as { message?: unknown }).message === "string"
        ) {
          message = (error as { message: string }).message;
        }
        this.error = message;
        return { success: false, message };
      }
      finally {
        this.isLoading = false;
      }
    },

    // 注册
    async register(
      data: RegisterData,
    ): Promise<{ success: boolean; message?: string }> {
      this.isLoading = true;
      this.error = null;

      try {
        const api = useApi();
        const response = await api.register(data);

        if (response.success) {
          return {
            success: true,
            message: "注册成功，请检查邮箱验证邮件",
          };
        }
        else {
          this.error = response.error?.message || "注册失败";
          return {
            success: false,
            message: this.error,
          };
        }
      }
      catch (error) {
        let message = "网络错误，请稍后重试";
        if (
          error
          && typeof error === "object"
          && "message" in error
          && typeof (error as { message?: unknown }).message === "string"
        ) {
          message = (error as { message: string }).message;
        }
        this.error = message;
        return { success: false, message };
      }
      finally {
        this.isLoading = false;
      }
    },

    // 登出
    async logout() {
      this.isLoading = true;

      try {
        const api = useApi();
        await api.logout();
      }
      catch (error) {
        console.error("登出请求失败:", error);
      }
      finally {
        this.clearAuth();
        this.isLoading = false;
      }
    },

    // 设置用户信息
    async setUser(user: User, accessToken: string, refreshToken?: string) {
      this.user = user;
      this.isAuthenticated = true;

      // 保存到本地存储（仅在客户端）
      if (typeof window !== "undefined") {
        localStorage.setItem(storageKeys.ACCESS_TOKEN, accessToken);
        localStorage.setItem(storageKeys.USER, JSON.stringify(user));

        if (refreshToken) {
          localStorage.setItem(storageKeys.REFRESH_TOKEN, refreshToken);
        }
      }

      // 获取用户权限
      await this.fetchPermissions();
    },

    // 清除认证状态
    clearAuth() {
      this.user = null;
      this.isAuthenticated = false;
      this.permissions = [];
      this.error = null;

      // 清除本地存储（仅在客户端）
      if (typeof window !== "undefined") {
        localStorage.removeItem(storageKeys.ACCESS_TOKEN);
        localStorage.removeItem(storageKeys.REFRESH_TOKEN);
        localStorage.removeItem(storageKeys.USER);

        // 检查是否需要保留记住的凭据
        const remembered = localStorage.getItem("rememberedCredentials");
        if (remembered) {
          try {
            const credentials = JSON.parse(remembered);
            // 如果用户之前没有选择记住我，则清除凭据
            if (!credentials.remember) {
              localStorage.removeItem("rememberedCredentials");
            }
          }
          catch {
            // 如果解析失败，直接清除
            localStorage.removeItem("rememberedCredentials");
          }
        }
      }
    },

    // 验证 token 有效性
    async validateToken(): Promise<boolean> {
      try {
        const api = useApi();
        const response = await api.getCurrentUser();

        if (response.success && response.data) {
          this.user = response.data;
          return true;
        }
        else {
          this.clearAuth();
          return false;
        }
      }
      catch (error) {
        console.error("验证 token 失败:", error);
        this.clearAuth();
        return false;
      }
    },

    // 获取用户权限
    async fetchPermissions() {
      try {
        const api = useApi();
        const response = await api.getUserPermissions();

        if (response.success && response.data) {
          this.permissions = response.data;
        }
      }
      catch (error) {
        console.error("获取用户权限失败:", error);
        this.permissions = [];
      }
    },

    // 刷新 token
    async refreshToken(): Promise<boolean> {
      if (typeof window === "undefined") {
        return false;
      }

      try {
        const refreshToken = localStorage.getItem(
          storageKeys.REFRESH_TOKEN,
        );
        if (!refreshToken) {
          throw new Error("没有刷新令牌");
        }

        const api = useApi();
        const response = await api.refreshToken({
          refresh_token: refreshToken,
        });

        if (response.success && response.data) {
          const { access_token, user } = response.data;
          await this.setUser(user, access_token);
          return true;
        }
        else {
          this.clearAuth();
          return false;
        }
      }
      catch (error) {
        console.error("刷新令牌失败:", error);
        this.clearAuth();
        return false;
      }
    },

    // 清除错误状态
    clearError() {
      this.error = null;
    },
  },
});

// 非setup中使用的引用
export const useAuthStoreRefs = () => storeToRefs(useAuthStore());
