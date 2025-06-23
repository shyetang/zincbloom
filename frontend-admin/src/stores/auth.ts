import {defineStore} from "pinia";
import type {User} from "@/types";
import apiClient from "@/api";
import router from "@/router";
import {computed, ref} from "vue";

// 定义登录凭据的类型，匹配后端的 UserLoginPayload DTO
interface LoginCredentials {
    username: string;
    password: string;
}

/*// 定义 state 的类型
interface AuthState {
    user: User | null;
    token: string | null;
}*/

/*
// 选项式api风格
export const useAuthStore = defineStore('auth', {
    // 状态： 定义 store 的数据
    state: (): AuthState => ({
        user: null,
        // 从 localStorage 初始化 token，防止刷新页面后丢失登录状态
        token: localStorage.getItem('accessToken') || null
    }),
    // Getters: 类似计算属性，可以派生出新的状态
    getters: {
        isAuthenticated: (state) => !!state.token,
        getUser: (state) => state.user,
    },
    // Actions: 定义可以修改状态的业务逻辑方法
    actions: {
        // 登录方法
        async login(credentials: LoginCredentials) {
            try {
                // 调用后端 /auth/login 接口
                const response = await apiClient.post('/auth/login', credentials);
                const {access_token, refresh_token, user} = response.data;
                // 更新状态
                this.token = access_token;
                this.user = user;
                // 将token存储到 localStorage 以实现持久化登录
                localStorage.setItem('accessToken', access_token);
                localStorage.setItem('refreshToken', refresh_token);
                // 登录成功后跳转到后台主页
                await (router as any).push({name: 'dashboard'});
            } catch (error) {
                // 处理登录失败
                console.error('Login failed:', error);
                throw error;
            }
        },
        // 登出方法
        async logout() {
            try {
                const refreshToken = localStorage.getItem('refreshToken');
                if (refreshToken) {
                    // 调用后端接口
                    await apiClient.post('/auth/logput', {refresh_token: refreshToken});
                }
            } catch (error) {
                console.error('Logout API call failed:', error);
            } finally {
                // 清理状态
                this.user = null;
                this.token = null;
                // 清理 localStorage
                localStorage.removeItem('accessToken');
                localStorage.removeItem('refreshToken');
                // 跳转到登录页
                await (router as any).push({name: 'login'});
            }
        },
        // 应用加载时，尝试获取用户信息
        async fetchUserProfile() {
            if (!this.isAuthenticated) return;
            try {
                const response = await apiClient.get('/me');
                // 后端返回的是 UserPublic
                this.user = response.data as User;
            } catch (error) {
                console.error('Failed to fetch user profile,logging out.', error);
                // 如果 token 失效，自动登出
                await this.logout();
            }
        },
    }
})*/

// 组合式api风格
export const useAuthStore = defineStore('auth', () => {
    const user = ref<User | null>(null);
    const token = ref<string | null>(localStorage.getItem('accessToken'));

    const isAuthenticated = computed(() => !!token.value);
    const getUser = computed(() => user.value);

    // 登录方法
    async function login(credentials: LoginCredentials) {
        try {
            // 调用后端 /auth/login 接口
            const response = await apiClient.post('/auth/login', credentials);
            const {access_token, refresh_token, user: userData} = response.data;
            // 更新状态
            token.value = access_token;
            user.value = userData;
            // 将token存储到 localStorage 以实现持久化登录
            localStorage.setItem('accessToken', access_token);
            localStorage.setItem('refreshToken', refresh_token);
            // 登录成功后跳转到后台主页
            await (router as any).push({name: 'dashboard'});
        } catch (error) {
            // 处理登录失败
            console.error('Login failed:', error);
            throw error;
        }
    }

    // 登出方法
    async function logout() {
        try {
            const refreshToken = localStorage.getItem('refreshToken');
            if (refreshToken) {
                // 调用后端接口
                await apiClient.post('/auth/logput', {refresh_token: refreshToken});
            }
        } catch (error) {
            console.error('Logout API call failed:', error);
        } finally {
            // 清理状态
            user.value = null;
            token.value = null;
            // 清理 localStorage
            localStorage.removeItem('accessToken');
            localStorage.removeItem('refreshToken');
            // 跳转到登录页
            await (router as any).push({name: 'login'});
        }
    }

    // 应用加载时，尝试获取用户信息
    async function fetchUserProfile() {
        if (!isAuthenticated.value) return;
        try {
            const response = await apiClient.get('/me');
            // 后端返回的是 UserPublic
            user.value = response.data as User;
        } catch (error) {
            console.error('Failed to fetch user profile,logging out.', error);
            // 如果 token 失效，自动登出
            await logout();
        }
    }

    return {
        user,
        token,
        isAuthenticated,
        getUser,
        login,
        logout,
        fetchUserProfile,
    };
});