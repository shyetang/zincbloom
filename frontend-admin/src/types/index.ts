// 这个类型与后端 UserPublic DTO 的结构保持一致
export interface User {
    id: string; // UUID 在前端通常作为字符串处理
    username: string;
    email: string;
    created_at: string; // TIMESTAMPTZ 转换为字符串
    roles: string[];
}