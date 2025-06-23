import axios, {type InternalAxiosRequestConfig} from "axios";
import {useAuthStore} from "@/stores/auth.ts";

const apiClient = axios.create({
    baseURL: import.meta.env.VITE_API_BASE_URL,
    headers: {
        'Content-Type': 'application/json'
    }
});

// 请求拦截器
// 在每个请求被发送前，拦截它并附加 Authorization 头
apiClient.interceptors.request.use(
    (config: InternalAxiosRequestConfig) => {
        // 在 Pinia 初始化之前，路由守卫可能会先运行，
        // 所以需要在函数内部获取 store 实例。
        const authStore = useAuthStore();
        const token = authStore.token;

        if (token) {
            config.headers.Authorization = `Bearer ${token}`;
        }
        return config;
    },
    (error) => {
        return Promise.reject(error);
    }
)
export default apiClient;