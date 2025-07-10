// ====================================
// 共享工具函数库
// 用于前端、管理后台和后端之间的工具函数共享
// ====================================

import type { PostStatus, Slug } from "../types";

// ===== 日期时间处理工具 =====
export const dateUtils = {
    /**
     * 格式化日期为可读字符串
     */
    formatDate(
        date: string | Date,
        format: "short" | "long" | "relative" = "short"
    ): string {
        const d = typeof date === "string" ? new Date(date) : date;

        if (format === "relative") {
            return this.getRelativeTime(d);
        }

        const options: Intl.DateTimeFormatOptions =
            format === "long"
                ? {
                      year: "numeric",
                      month: "long",
                      day: "numeric",
                      hour: "2-digit",
                      minute: "2-digit",
                  }
                : { year: "numeric", month: "short", day: "numeric" };

        return d.toLocaleDateString("zh-CN", options);
    },

    /**
     * 获取相对时间（如：2小时前、3天前）
     */
    getRelativeTime(date: Date): string {
        const now = new Date();
        const diff = now.getTime() - date.getTime();
        const minute = 60 * 1000;
        const hour = minute * 60;
        const day = hour * 24;
        const week = day * 7;
        const month = day * 30;
        const year = day * 365;

        if (diff < minute) return "刚刚";
        if (diff < hour) return `${Math.floor(diff / minute)}分钟前`;
        if (diff < day) return `${Math.floor(diff / hour)}小时前`;
        if (diff < week) return `${Math.floor(diff / day)}天前`;
        if (diff < month) return `${Math.floor(diff / week)}周前`;
        if (diff < year) return `${Math.floor(diff / month)}个月前`;
        return `${Math.floor(diff / year)}年前`;
    },

    /**
     * 检查日期是否为今天
     */
    isToday(date: string | Date): boolean {
        const d = typeof date === "string" ? new Date(date) : date;
        const today = new Date();
        return d.toDateString() === today.toDateString();
    },
};

// ===== 字符串处理工具 =====
export const stringUtils = {
    /**
     * 生成URL slug（用于文章URL）
     */
    generateSlug(text: string): Slug {
        return text
            .toLowerCase()
            .trim()
            .replace(/[\s\W-]+/g, "-")
            .replace(/^-+|-+$/g, "");
    },

    /**
     * 截断文本并添加省略号
     */
    truncate(text: string, maxLength: number, suffix: string = "..."): string {
        if (text.length <= maxLength) return text;
        return text.slice(0, maxLength - suffix.length) + suffix;
    },

    /**
     * 生成摘要（从Markdown内容中提取纯文本）
     */
    generateExcerpt(markdownContent: string, maxLength: number = 160): string {
        // 移除Markdown标记
        const plainText = markdownContent
            .replace(/#{1,6}\s+/g, "") // 移除标题标记
            .replace(/\*\*(.*?)\*\*/g, "$1") // 移除粗体标记
            .replace(/\*(.*?)\*/g, "$1") // 移除斜体标记
            .replace(/\[(.*?)\]\(.*?\)/g, "$1") // 移除链接，保留文本
            .replace(/```[\s\S]*?```/g, "") // 移除代码块
            .replace(/`(.*?)`/g, "$1") // 移除行内代码标记
            .replace(/\n+/g, " ") // 将换行替换为空格
            .trim();

        return this.truncate(plainText, maxLength);
    },

    /**
     * 验证邮箱格式
     */
    isValidEmail(email: string): boolean {
        const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
        return emailRegex.test(email);
    },

    /**
     * 验证用户名格式（只允许字母、数字、下划线）
     */
    isValidUsername(username: string): boolean {
        const usernameRegex = /^[a-zA-Z0-9_]{3,20}$/;
        return usernameRegex.test(username);
    },

    /**
     * 计算文本阅读时间（基于平均阅读速度）
     */
    calculateReadingTime(text: string, wordsPerMinute: number = 200): number {
        const words = text.trim().split(/\s+/).length;
        return Math.ceil(words / wordsPerMinute);
    },
};

// ===== URL和路由工具 =====
export const urlUtils = {
    /**
     * 生成文章详情页URL
     */
    generatePostUrl(slug: string): string {
        return `/posts/${slug}`;
    },

    /**
     * 生成分类页面URL
     */
    generateCategoryUrl(slug: string): string {
        return `/posts?category=${slug}`;
    },

    /**
     * 生成标签页面URL
     */
    generateTagUrl(slug: string): string {
        return `/posts?tag=${slug}`;
    },

    /**
     * 生成用户资料页URL
     */
    generateUserUrl(username: string): string {
        return `/user/${username}`;
    },

    /**
     * 解析查询参数
     */
    parseQuery(query: string): Record<string, string> {
        const params = new URLSearchParams(query);
        const result: Record<string, string> = {};
        params.forEach((value, key) => {
            result[key] = value;
        });
        return result;
    },

    /**
     * 构建查询字符串
     */
    buildQuery(params: Record<string, any>): string {
        const searchParams = new URLSearchParams();
        Object.entries(params).forEach(([key, value]) => {
            if (value !== undefined && value !== null && value !== "") {
                searchParams.append(key, String(value));
            }
        });
        return searchParams.toString();
    },
};

// ===== 数据处理工具 =====
export const dataUtils = {
    /**
     * 深拷贝对象
     */
    deepClone<T>(obj: T): T {
        if (obj === null || typeof obj !== "object") return obj;
        if (obj instanceof Date) return new Date(obj.getTime()) as T;
        if (obj instanceof Array)
            return obj.map((item) => this.deepClone(item)) as T;
        if (typeof obj === "object") {
            const clonedObj = {} as T;
            for (const key in obj) {
                if (obj.hasOwnProperty(key)) {
                    clonedObj[key] = this.deepClone(obj[key]);
                }
            }
            return clonedObj;
        }
        return obj;
    },

    /**
     * 数组去重
     */
    unique<T>(array: T[], key?: keyof T): T[] {
        if (!key) {
            return [...new Set(array)];
        }
        const seen = new Set();
        return array.filter((item) => {
            const val = item[key];
            if (seen.has(val)) return false;
            seen.add(val);
            return true;
        });
    },

    /**
     * 分页数据
     */
    paginate<T>(array: T[], page: number, perPage: number) {
        const start = (page - 1) * perPage;
        const end = start + perPage;
        return {
            data: array.slice(start, end),
            pagination: {
                page,
                per_page: perPage,
                total: array.length,
                total_pages: Math.ceil(array.length / perPage),
                has_next_page: end < array.length,
                has_prev_page: page > 1,
            },
        };
    },

    /**
     * 按键分组
     */
    groupBy<T>(array: T[], key: keyof T): Record<string, T[]> {
        return array.reduce((groups, item) => {
            const groupKey = String(item[key]);
            if (!groups[groupKey]) {
                groups[groupKey] = [];
            }
            groups[groupKey].push(item);
            return groups;
        }, {} as Record<string, T[]>);
    },
};

// ===== 文件处理工具 =====
export const fileUtils = {
    /**
     * 格式化文件大小
     */
    formatFileSize(bytes: number): string {
        if (bytes === 0) return "0 B";
        const k = 1024;
        const sizes = ["B", "KB", "MB", "GB", "TB"];
        const i = Math.floor(Math.log(bytes) / Math.log(k));
        return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
    },

    /**
     * 检查文件类型
     */
    isImageFile(filename: string): boolean {
        const imageExtensions = ["jpg", "jpeg", "png", "gif", "webp", "svg"];
        const extension = filename.split(".").pop()?.toLowerCase();
        return imageExtensions.includes(extension || "");
    },

    /**
     * 获取文件扩展名
     */
    getFileExtension(filename: string): string {
        return filename.split(".").pop()?.toLowerCase() || "";
    },
};

// ===== 状态处理工具 =====
export const statusUtils = {
    /**
     * 获取文章状态的显示文本
     */
    getPostStatusText(status: PostStatus): string {
        const statusMap = {
            draft: "草稿",
            published: "已发布",
        };
        return statusMap[status] || status;
    },

    /**
     * 获取状态颜色类
     */
    getStatusColor(status: PostStatus): string {
        const colorMap = {
            draft: "text-yellow-600 bg-yellow-100",
            published: "text-green-600 bg-green-100",
        };
        return colorMap[status] || "text-gray-600 bg-gray-100";
    },
};

// ===== 表单验证工具 =====
export const validationUtils = {
    /**
     * 验证密码强度
     */
    validatePassword(password: string): { isValid: boolean; message: string } {
        if (password.length < 8) {
            return { isValid: false, message: "密码长度至少8位" };
        }
        if (!/(?=.*[a-z])/.test(password)) {
            return { isValid: false, message: "密码必须包含小写字母" };
        }
        if (!/(?=.*[A-Z])/.test(password)) {
            return { isValid: false, message: "密码必须包含大写字母" };
        }
        if (!/(?=.*\d)/.test(password)) {
            return { isValid: false, message: "密码必须包含数字" };
        }
        return { isValid: true, message: "密码强度符合要求" };
    },

    /**
     * 验证必填字段
     */
    required(value: any, fieldName: string): string | null {
        if (value === null || value === undefined || value === "") {
            return `${fieldName}是必填项`;
        }
        return null;
    },

    /**
     * 验证字符串长度
     */
    length(
        value: string,
        min: number,
        max: number,
        fieldName: string
    ): string | null {
        if (value.length < min) {
            return `${fieldName}长度不能少于${min}个字符`;
        }
        if (value.length > max) {
            return `${fieldName}长度不能超过${max}个字符`;
        }
        return null;
    },
};

// ===== 防抖和节流工具 =====
export const performanceUtils = {
    /**
     * 防抖函数
     */
    debounce<T extends (...args: any[]) => any>(
        func: T,
        wait: number
    ): (...args: Parameters<T>) => void {
        let timeout: NodeJS.Timeout;
        return (...args: Parameters<T>) => {
            clearTimeout(timeout);
            timeout = setTimeout(() => func(...args), wait);
        };
    },

    /**
     * 节流函数
     */
    throttle<T extends (...args: any[]) => any>(
        func: T,
        limit: number
    ): (...args: Parameters<T>) => void {
        let inThrottle: boolean;
        return (...args: Parameters<T>) => {
            if (!inThrottle) {
                func(...args);
                inThrottle = true;
                setTimeout(() => (inThrottle = false), limit);
            }
        };
    },
};

// 导出所有工具
export {
    dateUtils,
    stringUtils,
    urlUtils,
    dataUtils,
    fileUtils,
    statusUtils,
    validationUtils,
    performanceUtils,
};
