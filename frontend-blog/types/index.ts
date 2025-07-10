// ====================================
// 博客前端类型定义
// 重新导出共享类型，并添加前端特有的类型定义
// ====================================

// 重新导出所有共享类型
export * from "@shared/types";

// 前端特有的类型定义
export interface NavigationItem {
    name: string;
    href: string;
    icon?: string;
    children?: NavigationItem[];
}

export interface BreadcrumbItem {
    name: string;
    href?: string;
    current?: boolean;
}

export interface ToastMessage {
    id: string;
    type: "success" | "error" | "warning" | "info";
    title: string;
    message?: string;
    duration?: number;
    action?: {
        label: string;
        onClick: () => void;
    };
}

export interface ModalProps {
    isOpen: boolean;
    title?: string;
    size?: "sm" | "md" | "lg" | "xl";
    onClose: () => void;
}

export interface PageMeta {
    title: string;
    description?: string;
    keywords?: string[];
    image?: string;
    canonical?: string;
    noindex?: boolean;
}

export interface RouteParams {
    slug?: string;
    id?: string;
    page?: string;
    category?: string;
    tag?: string;
    search?: string;
}

// 主题相关类型
export interface ThemeConfig {
    primaryColor: string;
    secondaryColor: string;
    accentColor: string;
    backgroundColor: string;
    textColor: string;
    borderColor: string;
}

// 布局相关类型
export interface LayoutProps {
    showSidebar?: boolean;
    showHeader?: boolean;
    showFooter?: boolean;
    maxWidth?: "sm" | "md" | "lg" | "xl" | "2xl" | "full";
}

// 表单组件类型
export interface FormFieldProps {
    label?: string;
    placeholder?: string;
    required?: boolean;
    disabled?: boolean;
    error?: string;
    hint?: string;
}

export interface SelectOption {
    label: string;
    value: string | number;
    disabled?: boolean;
    icon?: string;
}

// 分页组件类型
export interface PaginationProps {
    current: number;
    total: number;
    pageSize: number;
    showSizeChanger?: boolean;
    showQuickJumper?: boolean;
    showTotal?: boolean;
    onChange: (page: number, pageSize: number) => void;
}

// 搜索组件类型
export interface SearchProps {
    placeholder?: string;
    value?: string;
    suggestions?: string[];
    onSearch: (query: string) => void;
    onSuggestionClick?: (suggestion: string) => void;
}

// 文章卡片组件类型
export interface PostCardProps {
    post: import("@shared/types").Post;
    variant?: "default" | "featured" | "minimal";
    showAuthor?: boolean;
    showDate?: boolean;
    showExcerpt?: boolean;
    showTags?: boolean;
}

// 用户头像组件类型
export interface AvatarProps {
    src?: string;
    name: string;
    size?: "sm" | "md" | "lg" | "xl";
    shape?: "circle" | "square";
    fallbackIcon?: string;
}

// 加载状态类型
export interface LoadingProps {
    size?: "sm" | "md" | "lg";
    text?: string;
    overlay?: boolean;
}

// 响应式配置
export interface ResponsiveConfig {
    mobile: boolean;
    tablet: boolean;
    desktop: boolean;
    largeDesktop: boolean;
}

// 编辑器相关类型
export interface EditorProps {
    value: string;
    onChange: (value: string) => void;
    placeholder?: string;
    readonly?: boolean;
    height?: number;
    toolbar?: boolean;
    preview?: boolean;
}

export interface EditorConfig {
    theme: "light" | "dark";
    fontSize: number;
    lineHeight: number;
    wordWrap: boolean;
    minimap: boolean;
    lineNumbers: boolean;
}
