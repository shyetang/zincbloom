// API响应类型定义

import type { Category, Post, Tag } from ".";

export interface PaginatedResponse<T> {
  data: T[];
  pagination: {
    page: number;
    per_page: number;
    total: number;
    total_pages: number;
  };
}

// 直接使用类型别名，避免空接口声明
export type PostsResponse = PaginatedResponse<Post>;
export type CategoriesResponse = PaginatedResponse<Category>;
export type TagsResponse = PaginatedResponse<Tag>;

// API数据获取的返回类型
export interface ApiDataResult<T> {
  data: Ref<T>;
  pending: Ref<boolean>;
  refresh: () => Promise<void>;
}
