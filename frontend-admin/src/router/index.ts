import {createRouter, createWebHistory} from 'vue-router'
import LoginView from "@/views/LoginView.vue";
import DashboardView from "@/views/DashboardView.vue";
import {useAuthStore} from "@/stores/auth.ts";

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: '/login',
            name: 'login',
            component: LoginView
        },
        {
            path: '/dashboard',
            name: 'dashboard',
            component: DashboardView
        },
        {
            path: '/',
            redirect: '/dashboard'
        }
    ],
})

// 全局前置导航守卫
// 在每次路由跳转之前执行
router.beforeEach((to, from, next) => {
    const authStore = useAuthStore();
    const requireAuth = to.meta.requiresAuth;
    // 如果路由需要认证，但用户未登录
    if (requireAuth && !authStore.isAuthenticated) {
        // 将用户重定向到登录页
        next({name: 'login'});
    } else if (to.name === 'login' && authStore.isAuthenticated) {
        // 如果用户已登录，但想访问登录页，就转到dashboard
        next({name: 'dashboard'});
    } else {
        // 否则直接放行
        next();
    }
})

export default router
