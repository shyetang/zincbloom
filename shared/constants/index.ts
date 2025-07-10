// ====================================
// 共享常量库
// 用于前端、管理后台和后端之间的常量共享
// ====================================

// ===== API 端点常量 =====
export const API_ENDPOINTS = {
    // 认证相关
    AUTH: {
        LOGIN: "/auth/login",
        REGISTER: "/auth/register",
        LOGOUT: "/auth/logout",
        REFRESH: "/auth/refresh",
        FORGOT_PASSWORD: "/auth/forgot-password",
        RESET_PASSWORD: "/auth/reset-password",
        VERIFY_EMAIL: "/auth/verify-email",
        RESEND_VERIFICATION: "/auth/resend-verification",
    },

    // 用户相关
    USERS: {
        ME: "/me",
        PROFILE: "/me/profile",
        PASSWORD: "/me/password",
        PERMISSIONS: "/me/permissions",
        POSTS: "/me/posts",
        LIST: "/users",
        DETAIL: (id: string) => `/users/${id}`,
        CREATE: "/users",
        UPDATE: (id: string) => `/users/${id}`,
        DELETE: (id: string) => `/users/${id}`,
    },

    // 文章相关
    POSTS: {
        LIST: "/posts",
        DETAIL: (slug: string) => `/posts/${slug}`,
        CREATE: "/posts",
        UPDATE: (id: string) => `/posts/${id}`,
        DELETE: (id: string) => `/posts/${id}`,
        PUBLISH: (id: string) => `/posts/${id}/publish`,
        UNPUBLISH: (id: string) => `/posts/${id}/unpublish`,
        BAN: (id: string) => `/posts/${id}/ban`,
        UNBAN: (id: string) => `/posts/${id}/unban`,
    },

    // 分类相关
    CATEGORIES: {
        LIST: "/categories",
        DETAIL: (slug: string) => `/categories/${slug}`,
        CREATE: "/categories",
        UPDATE: (id: string) => `/categories/${id}`,
        DELETE: (id: string) => `/categories/${id}`,
        POSTS: (slug: string) => `/categories/${slug}/posts`,
    },

    // 标签相关
    TAGS: {
        LIST: "/tags",
        DETAIL: (slug: string) => `/tags/${slug}`,
        CREATE: "/tags",
        UPDATE: (id: string) => `/tags/${id}`,
        DELETE: (id: string) => `/tags/${id}`,
        POSTS: (slug: string) => `/tags/${slug}/posts`,
    },

    // 搜索相关
    SEARCH: {
        POSTS: "/search/posts",
        USERS: "/search/users",
        GLOBAL: "/search",
    },

    // 文件上传
    UPLOAD: {
        IMAGE: "/upload/image",
        FILE: "/upload/file",
    },

    // 统计数据
    STATS: {
        DASHBOARD: "/stats/dashboard",
        POSTS: "/stats/posts",
        USERS: "/stats/users",
    },

    // 管理员功能
    ADMIN: {
        USERS: "/admin/users",
        POSTS: "/admin/posts",
        CATEGORIES: "/admin/categories",
        TAGS: "/admin/tags",
        ROLES: "/admin/roles",
        PERMISSIONS: "/admin/permissions",
        SETTINGS: "/admin/settings",
    },
} as const;

// ===== 状态常量 =====
export const POST_STATUS = {
    DRAFT: "draft",
    PUBLISHED: "published",
} as const;

export const USER_ROLES = {
    ADMIN: "admin",
    MODERATOR: "moderator",
    AUTHOR: "author",
    USER: "user",
} as const;

export const PERMISSION_ACTIONS = {
    CREATE: "create",
    READ: "read",
    UPDATE: "update",
    DELETE: "delete",
    PUBLISH: "publish",
    BAN: "ban",
} as const;

export const PERMISSION_RESOURCES = {
    POST: "post",
    CATEGORY: "category",
    TAG: "tag",
    USER: "user",
    ROLE: "role",
    PERMISSION: "permission",
    SETTING: "setting",
} as const;

// ===== 通知类型常量 =====
export const NOTIFICATION_TYPES = {
    INFO: "info",
    SUCCESS: "success",
    WARNING: "warning",
    ERROR: "error",
} as const;

// ===== 文件类型常量 =====
export const FILE_TYPES = {
    IMAGE: "image",
    DOCUMENT: "document",
    VIDEO: "video",
    AUDIO: "audio",
} as const;

export const ALLOWED_IMAGE_TYPES = [
    "image/jpeg",
    "image/png",
    "image/gif",
    "image/webp",
    "image/svg+xml",
] as const;

export const ALLOWED_DOCUMENT_TYPES = [
    "application/pdf",
    "application/msword",
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
    "text/plain",
] as const;

// ===== 分页常量 =====
export const PAGINATION = {
    DEFAULT_PAGE: 1,
    DEFAULT_PER_PAGE: 10,
    MAX_PER_PAGE: 100,
    POSTS_PER_PAGE: 12,
    ADMIN_PER_PAGE: 20,
} as const;

// ===== 验证规则常量 =====
export const VALIDATION_RULES = {
    USERNAME: {
        MIN_LENGTH: 3,
        MAX_LENGTH: 20,
        PATTERN: /^[a-zA-Z0-9_]+$/,
    },
    PASSWORD: {
        MIN_LENGTH: 8,
        MAX_LENGTH: 128,
        REQUIRE_UPPERCASE: true,
        REQUIRE_LOWERCASE: true,
        REQUIRE_NUMBER: true,
        REQUIRE_SPECIAL_CHAR: false,
    },
    EMAIL: {
        PATTERN: /^[^\s@]+@[^\s@]+\.[^\s@]+$/,
    },
    POST: {
        TITLE_MIN_LENGTH: 1,
        TITLE_MAX_LENGTH: 200,
        CONTENT_MIN_LENGTH: 1,
        EXCERPT_MAX_LENGTH: 500,
    },
    CATEGORY: {
        NAME_MIN_LENGTH: 1,
        NAME_MAX_LENGTH: 50,
        DESCRIPTION_MAX_LENGTH: 200,
    },
    TAG: {
        NAME_MIN_LENGTH: 1,
        NAME_MAX_LENGTH: 30,
    },
} as const;

// ===== 文件大小限制 =====
export const FILE_SIZE_LIMITS = {
    IMAGE_MAX_SIZE: 5 * 1024 * 1024, // 5MB
    DOCUMENT_MAX_SIZE: 10 * 1024 * 1024, // 10MB
    VIDEO_MAX_SIZE: 100 * 1024 * 1024, // 100MB
} as const;

// ===== 缓存键常量 =====
export const CACHE_KEYS = {
    USER_PROFILE: "user_profile",
    USER_PERMISSIONS: "user_permissions",
    POSTS_LIST: "posts_list",
    CATEGORIES_LIST: "categories_list",
    TAGS_LIST: "tags_list",
    DASHBOARD_STATS: "dashboard_stats",
} as const;

// ===== 本地存储键常量 =====
export const STORAGE_KEYS = {
    ACCESS_TOKEN: "access_token",
    REFRESH_TOKEN: "refresh_token",
    USER_PREFERENCES: "user_preferences",
    THEME: "theme",
    LANGUAGE: "language",
    DRAFT_POST: "draft_post",
} as const;

// ===== 主题常量 =====
export const THEMES = {
    LIGHT: "light",
    DARK: "dark",
    AUTO: "auto",
} as const;

// ===== 语言常量 =====
export const LANGUAGES = {
    ZH_CN: "zh-CN",
    EN_US: "en-US",
} as const;

// ===== 排序选项常量 =====
export const SORT_OPTIONS = {
    POSTS: {
        CREATED_AT_DESC: { sort: "created_at", order: "desc" },
        CREATED_AT_ASC: { sort: "created_at", order: "asc" },
        UPDATED_AT_DESC: { sort: "updated_at", order: "desc" },
        TITLE_ASC: { sort: "title", order: "asc" },
        TITLE_DESC: { sort: "title", order: "desc" },
    },
    USERS: {
        CREATED_AT_DESC: { sort: "created_at", order: "desc" },
        USERNAME_ASC: { sort: "username", order: "asc" },
        EMAIL_ASC: { sort: "email", order: "asc" },
    },
    CATEGORIES: {
        NAME_ASC: { sort: "name", order: "asc" },
        CREATED_AT_DESC: { sort: "created_at", order: "desc" },
        POST_COUNT_DESC: { sort: "post_count", order: "desc" },
    },
} as const;

// ===== 错误代码常量 =====
export const ERROR_CODES = {
    // 认证错误
    UNAUTHORIZED: "UNAUTHORIZED",
    FORBIDDEN: "FORBIDDEN",
    TOKEN_EXPIRED: "TOKEN_EXPIRED",
    INVALID_TOKEN: "INVALID_TOKEN",

    // 验证错误
    VALIDATION_ERROR: "VALIDATION_ERROR",
    INVALID_INPUT: "INVALID_INPUT",
    REQUIRED_FIELD: "REQUIRED_FIELD",

    // 资源错误
    NOT_FOUND: "NOT_FOUND",
    ALREADY_EXISTS: "ALREADY_EXISTS",
    CONFLICT: "CONFLICT",

    // 权限错误
    INSUFFICIENT_PERMISSIONS: "INSUFFICIENT_PERMISSIONS",
    ACCESS_DENIED: "ACCESS_DENIED",

    // 系统错误
    INTERNAL_ERROR: "INTERNAL_ERROR",
    SERVICE_UNAVAILABLE: "SERVICE_UNAVAILABLE",
    NETWORK_ERROR: "NETWORK_ERROR",
} as const;

// ===== 默认配置常量 =====
export const DEFAULT_CONFIG = {
    SITE_NAME: "ZincBloom",
    SITE_DESCRIPTION: "现代化博客系统",
    POSTS_PER_PAGE: 12,
    RECENT_POSTS_COUNT: 5,
    POPULAR_POSTS_COUNT: 5,
    MAX_TAGS_DISPLAY: 10,
    READING_SPEED_WPM: 200, // 每分钟阅读字数
    DEBOUNCE_DELAY: 300, // 防抖延迟（毫秒）
    THROTTLE_DELAY: 1000, // 节流延迟（毫秒）
    API_TIMEOUT: 10000, // API超时时间（毫秒）
} as const;

// ===== 正则表达式常量 =====
export const REGEX_PATTERNS = {
    EMAIL: /^[^\s@]+@[^\s@]+\.[^\s@]+$/,
    USERNAME: /^[a-zA-Z0-9_]{3,20}$/,
    SLUG: /^[a-z0-9]+(?:-[a-z0-9]+)*$/,
    URL: /^https?:\/\/(?:www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b(?:[-a-zA-Z0-9()@:%_\+.~#?&=]*)$/,
    PHONE: /^1[3-9]\d{9}$/,
    PASSWORD_STRONG: /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)[a-zA-Z\d@$!%*?&]{8,}$/,
} as const;

// ===== HTTP 状态码常量 =====
export const HTTP_STATUS = {
    OK: 200,
    CREATED: 201,
    NO_CONTENT: 204,
    BAD_REQUEST: 400,
    UNAUTHORIZED: 401,
    FORBIDDEN: 403,
    NOT_FOUND: 404,
    CONFLICT: 409,
    UNPROCESSABLE_ENTITY: 422,
    INTERNAL_SERVER_ERROR: 500,
    SERVICE_UNAVAILABLE: 503,
} as const;
