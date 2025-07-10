// ====================================
// 共享验证规则库
// 用于前端、管理后台和后端之间的验证逻辑共享
// ====================================

import { VALIDATION_RULES, REGEX_PATTERNS } from "../constants";
import type {
    LoginCredentials,
    RegisterData,
    CreatePostData,
    CreateCategoryData,
    CreateTagData,
    UserCreateData,
    FormErrors,
} from "../types";

// ===== 基础验证函数 =====
export const baseValidators = {
    /**
     * 验证必填字段
     */
    required(value: any): string | null {
        if (
            value === null ||
            value === undefined ||
            (typeof value === "string" && value.trim() === "") ||
            (Array.isArray(value) && value.length === 0)
        ) {
            return "此字段为必填项";
        }
        return null;
    },

    /**
     * 验证字符串长度
     */
    length(value: string, min: number, max: number): string | null {
        const length = value?.trim().length || 0;
        if (length < min) {
            return `长度不能少于${min}个字符`;
        }
        if (length > max) {
            return `长度不能超过${max}个字符`;
        }
        return null;
    },

    /**
     * 验证邮箱格式
     */
    email(value: string): string | null {
        if (!value) return null;
        if (!REGEX_PATTERNS.EMAIL.test(value)) {
            return "请输入有效的邮箱地址";
        }
        return null;
    },

    /**
     * 验证URL格式
     */
    url(value: string): string | null {
        if (!value) return null;
        if (!REGEX_PATTERNS.URL.test(value)) {
            return "请输入有效的URL";
        }
        return null;
    },

    /**
     * 验证正则表达式
     */
    pattern(value: string, pattern: RegExp, message: string): string | null {
        if (!value) return null;
        if (!pattern.test(value)) {
            return message;
        }
        return null;
    },

    /**
     * 验证数字范围
     */
    range(value: number, min: number, max: number): string | null {
        if (value < min || value > max) {
            return `数值必须在${min}到${max}之间`;
        }
        return null;
    },

    /**
     * 验证数组长度
     */
    arrayLength(value: any[], min: number, max: number): string | null {
        const length = value?.length || 0;
        if (length < min) {
            return `至少选择${min}项`;
        }
        if (length > max) {
            return `最多选择${max}项`;
        }
        return null;
    },

    /**
     * 验证文件大小
     */
    fileSize(file: File, maxSize: number): string | null {
        if (file.size > maxSize) {
            const maxSizeMB = Math.round(maxSize / (1024 * 1024));
            return `文件大小不能超过${maxSizeMB}MB`;
        }
        return null;
    },

    /**
     * 验证文件类型
     */
    fileType(file: File, allowedTypes: string[]): string | null {
        if (!allowedTypes.includes(file.type)) {
            return `不支持的文件类型，仅支持：${allowedTypes.join(", ")}`;
        }
        return null;
    },
};

// ===== 用户相关验证 =====
export const userValidators = {
    /**
     * 验证用户名
     */
    username(value: string): string | null {
        if (!value) return baseValidators.required(value);

        const lengthError = baseValidators.length(
            value,
            VALIDATION_RULES.USERNAME.MIN_LENGTH,
            VALIDATION_RULES.USERNAME.MAX_LENGTH
        );
        if (lengthError) return lengthError;

        const patternError = baseValidators.pattern(
            value,
            VALIDATION_RULES.USERNAME.PATTERN,
            "用户名只能包含字母、数字和下划线"
        );
        if (patternError) return patternError;

        return null;
    },

    /**
     * 验证邮箱
     */
    email(value: string): string | null {
        if (!value) return baseValidators.required(value);
        return baseValidators.email(value);
    },

    /**
     * 验证密码
     */
    password(value: string): string | null {
        if (!value) return baseValidators.required(value);

        const rules = VALIDATION_RULES.PASSWORD;
        const lengthError = baseValidators.length(
            value,
            rules.MIN_LENGTH,
            rules.MAX_LENGTH
        );
        if (lengthError) return lengthError;

        if (rules.REQUIRE_LOWERCASE && !/[a-z]/.test(value)) {
            return "密码必须包含小写字母";
        }

        if (rules.REQUIRE_UPPERCASE && !/[A-Z]/.test(value)) {
            return "密码必须包含大写字母";
        }

        if (rules.REQUIRE_NUMBER && !/\d/.test(value)) {
            return "密码必须包含数字";
        }

        if (
            rules.REQUIRE_SPECIAL_CHAR &&
            !/[!@#$%^&*(),.?":{}|<>]/.test(value)
        ) {
            return "密码必须包含特殊字符";
        }

        return null;
    },

    /**
     * 验证确认密码
     */
    confirmPassword(value: string, originalPassword: string): string | null {
        if (!value) return baseValidators.required(value);
        if (value !== originalPassword) {
            return "两次密码输入不一致";
        }
        return null;
    },
};

// ===== 文章相关验证 =====
export const postValidators = {
    /**
     * 验证文章标题
     */
    title(value: string): string | null {
        if (!value) return baseValidators.required(value);
        return baseValidators.length(
            value,
            VALIDATION_RULES.POST.TITLE_MIN_LENGTH,
            VALIDATION_RULES.POST.TITLE_MAX_LENGTH
        );
    },

    /**
     * 验证文章内容
     */
    content(value: string): string | null {
        if (!value) return baseValidators.required(value);
        if (value.trim().length < VALIDATION_RULES.POST.CONTENT_MIN_LENGTH) {
            return "文章内容不能为空";
        }
        return null;
    },

    /**
     * 验证文章摘要
     */
    excerpt(value: string): string | null {
        if (!value) return null; // 摘要是可选的
        return baseValidators.length(
            value,
            0,
            VALIDATION_RULES.POST.EXCERPT_MAX_LENGTH
        );
    },

    /**
     * 验证缩略图URL
     */
    thumbnail(value: string): string | null {
        if (!value) return null; // 缩略图是可选的
        return baseValidators.url(value);
    },

    /**
     * 验证文章slug
     */
    slug(value: string): string | null {
        if (!value) return baseValidators.required(value);
        const patternError = baseValidators.pattern(
            value,
            REGEX_PATTERNS.SLUG,
            "URL别名只能包含小写字母、数字和连字符"
        );
        return patternError;
    },
};

// ===== 分类相关验证 =====
export const categoryValidators = {
    /**
     * 验证分类名称
     */
    name(value: string): string | null {
        if (!value) return baseValidators.required(value);
        return baseValidators.length(
            value,
            VALIDATION_RULES.CATEGORY.NAME_MIN_LENGTH,
            VALIDATION_RULES.CATEGORY.NAME_MAX_LENGTH
        );
    },

    /**
     * 验证分类描述
     */
    description(value: string): string | null {
        if (!value) return null; // 描述是可选的
        return baseValidators.length(
            value,
            0,
            VALIDATION_RULES.CATEGORY.DESCRIPTION_MAX_LENGTH
        );
    },

    /**
     * 验证分类slug
     */
    slug(value: string): string | null {
        if (!value) return baseValidators.required(value);
        return baseValidators.pattern(
            value,
            REGEX_PATTERNS.SLUG,
            "URL别名只能包含小写字母、数字和连字符"
        );
    },
};

// ===== 标签相关验证 =====
export const tagValidators = {
    /**
     * 验证标签名称
     */
    name(value: string): string | null {
        if (!value) return baseValidators.required(value);
        return baseValidators.length(
            value,
            VALIDATION_RULES.TAG.NAME_MIN_LENGTH,
            VALIDATION_RULES.TAG.NAME_MAX_LENGTH
        );
    },

    /**
     * 验证标签slug
     */
    slug(value: string): string | null {
        if (!value) return baseValidators.required(value);
        return baseValidators.pattern(
            value,
            REGEX_PATTERNS.SLUG,
            "URL别名只能包含小写字母、数字和连字符"
        );
    },
};

// ===== 复合验证函数 =====
export const validators = {
    /**
     * 验证登录表单
     */
    validateLogin(data: LoginCredentials): FormErrors {
        const errors: FormErrors = {};

        const usernameError = userValidators.username(data.username);
        if (usernameError) errors.username = usernameError;

        const passwordError = baseValidators.required(data.password);
        if (passwordError) errors.password = passwordError;

        return errors;
    },

    /**
     * 验证注册表单
     */
    validateRegister(data: RegisterData): FormErrors {
        const errors: FormErrors = {};

        const usernameError = userValidators.username(data.username);
        if (usernameError) errors.username = usernameError;

        const emailError = userValidators.email(data.email);
        if (emailError) errors.email = emailError;

        const passwordError = userValidators.password(data.password);
        if (passwordError) errors.password = passwordError;

        return errors;
    },

    /**
     * 验证用户创建表单
     */
    validateUserCreate(data: UserCreateData): FormErrors {
        const errors: FormErrors = {};

        const usernameError = userValidators.username(data.username);
        if (usernameError) errors.username = usernameError;

        const emailError = userValidators.email(data.email);
        if (emailError) errors.email = emailError;

        const passwordError = userValidators.password(data.password);
        if (passwordError) errors.password = passwordError;

        return errors;
    },

    /**
     * 验证文章创建表单
     */
    validatePostCreate(data: CreatePostData): FormErrors {
        const errors: FormErrors = {};

        const titleError = postValidators.title(data.title);
        if (titleError) errors.title = titleError;

        const contentError = postValidators.content(data.content);
        if (contentError) errors.content = contentError;

        if (data.excerpt) {
            const excerptError = postValidators.excerpt(data.excerpt);
            if (excerptError) errors.excerpt = excerptError;
        }

        if (data.thumbnail) {
            const thumbnailError = postValidators.thumbnail(data.thumbnail);
            if (thumbnailError) errors.thumbnail = thumbnailError;
        }

        return errors;
    },

    /**
     * 验证分类创建表单
     */
    validateCategoryCreate(data: CreateCategoryData): FormErrors {
        const errors: FormErrors = {};

        const nameError = categoryValidators.name(data.name);
        if (nameError) errors.name = nameError;

        if (data.description) {
            const descriptionError = categoryValidators.description(
                data.description
            );
            if (descriptionError) errors.description = descriptionError;
        }

        if (data.slug) {
            const slugError = categoryValidators.slug(data.slug);
            if (slugError) errors.slug = slugError;
        }

        return errors;
    },

    /**
     * 验证标签创建表单
     */
    validateTagCreate(data: CreateTagData): FormErrors {
        const errors: FormErrors = {};

        const nameError = tagValidators.name(data.name);
        if (nameError) errors.name = nameError;

        if (data.slug) {
            const slugError = tagValidators.slug(data.slug);
            if (slugError) errors.slug = slugError;
        }

        return errors;
    },

    /**
     * 验证修改密码表单
     */
    validateChangePassword(data: {
        current_password: string;
        new_password: string;
        confirm_password: string;
    }): FormErrors {
        const errors: FormErrors = {};

        const currentPasswordError = baseValidators.required(
            data.current_password
        );
        if (currentPasswordError)
            errors.current_password = currentPasswordError;

        const newPasswordError = userValidators.password(data.new_password);
        if (newPasswordError) errors.new_password = newPasswordError;

        const confirmPasswordError = userValidators.confirmPassword(
            data.confirm_password,
            data.new_password
        );
        if (confirmPasswordError)
            errors.confirm_password = confirmPasswordError;

        return errors;
    },

    /**
     * 检查表单是否有错误
     */
    hasErrors(errors: FormErrors): boolean {
        return Object.keys(errors).length > 0;
    },

    /**
     * 获取第一个错误信息
     */
    getFirstError(errors: FormErrors): string | null {
        const keys = Object.keys(errors);
        if (keys.length === 0) return null;
        const firstKey = keys[0];
        const error = errors[firstKey];
        return Array.isArray(error) ? error[0] : error;
    },
};

// ===== 异步验证函数 =====
export const asyncValidators = {
    /**
     * 验证用户名是否可用（需要API调用）
     */
    async validateUsernameAvailable(
        username: string,
        checkFunction: (username: string) => Promise<boolean>
    ): Promise<string | null> {
        const basicError = userValidators.username(username);
        if (basicError) return basicError;

        try {
            const isAvailable = await checkFunction(username);
            return isAvailable ? null : "用户名已被使用";
        } catch (error) {
            return "验证用户名时发生错误";
        }
    },

    /**
     * 验证邮箱是否可用（需要API调用）
     */
    async validateEmailAvailable(
        email: string,
        checkFunction: (email: string) => Promise<boolean>
    ): Promise<string | null> {
        const basicError = userValidators.email(email);
        if (basicError) return basicError;

        try {
            const isAvailable = await checkFunction(email);
            return isAvailable ? null : "邮箱已被使用";
        } catch (error) {
            return "验证邮箱时发生错误";
        }
    },

    /**
     * 验证slug是否可用（需要API调用）
     */
    async validateSlugAvailable(
        slug: string,
        checkFunction: (slug: string) => Promise<boolean>,
        type: "post" | "category" | "tag" = "post"
    ): Promise<string | null> {
        const basicError = baseValidators.pattern(
            slug,
            REGEX_PATTERNS.SLUG,
            "URL别名只能包含小写字母、数字和连字符"
        );
        if (basicError) return basicError;

        try {
            const isAvailable = await checkFunction(slug);
            return isAvailable ? null : `${type}的URL别名已被使用`;
        } catch (error) {
            return "验证URL别名时发生错误";
        }
    },
};

// ===== 导出所有验证器 =====
export {
    baseValidators,
    userValidators,
    postValidators,
    categoryValidators,
    tagValidators,
    validators,
    asyncValidators,
};
