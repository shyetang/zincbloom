// 后端数据转换工具函数
import type {
  Post,
  PostWithContent,
  PaginatedResponse,
  FrontendPaginatedResponse,
  BackendPostsResponse,
  PostsResponse,
} from "~/types";

// 将后端的 Post 数据转换为前端期望的格式
export function transformPost(backendPost: Post): PostWithContent {
  const { content_markdown, ...restPost } = backendPost;

  return {
    ...restPost,
    content: content_markdown, // 映射 content_markdown 到 content
    // 如果没有 excerpt，从内容中提取
    excerpt: restPost.excerpt || extractExcerpt(content_markdown),
  };
}

// 将后端的分页响应转换为前端期望的格式
export function transformPaginatedResponse<T>(
  backendResponse: PaginatedResponse<T>,
): FrontendPaginatedResponse<T> {
  return {
    data: backendResponse.items,
    pagination: {
      page: backendResponse.page,
      per_page: backendResponse.page_size,
      total: backendResponse.total_items,
      total_pages: backendResponse.total_pages,
    },
  };
}

// 转换文章列表响应
export function transformPostsResponse(backendResponse: BackendPostsResponse): PostsResponse {
  const transformed = transformPaginatedResponse(backendResponse);
  return {
    data: transformed.data.map((post: Post) => transformPost(post)) as PostWithContent[],
    pagination: transformed.pagination,
  };
}

// 从 Markdown 内容中提取摘要
export function extractExcerpt(content: string, maxLength: number = 200): string {
  if (!content) return "";

  // 移除 Markdown 标记
  const plainText = content
    .replace(/#{1,6}\s+/g, "") // 移除标题标记
    .replace(/\*\*(.*?)\*\*/g, "$1") // 移除粗体标记
    .replace(/\*(.*?)\*/g, "$1") // 移除斜体标记
    .replace(/`(.*?)`/g, "$1") // 移除行内代码标记
    .replace(/\[(.*?)\]\(.*?\)/g, "$1") // 移除链接，保留文本
    .replace(/!\[.*?\]\(.*?\)/g, "") // 移除图片
    .replace(/```[\s\S]*?```/g, "") // 移除代码块
    .replace(/\n\s*\n/g, " ") // 替换多个换行为空格
    .trim();

  return plainText.length > maxLength
    ? plainText.substring(0, maxLength) + "..."
    : plainText;
}

// API 查询参数转换
export function transformQueryParams(params: Record<string, any>): Record<string, string> {
  const transformed: Record<string, string> = {};

  Object.entries(params).forEach(([key, value]) => {
    if (value !== undefined && value !== null && value !== "") {
      // 特殊处理分页参数
      if (key === "per_page") {
        transformed["page_size"] = String(value);
      }
      else {
        transformed[key] = String(value);
      }
    }
  });

  return transformed;
}

// URL 查询字符串构建
export function buildQueryString(params: Record<string, any>): string {
  const transformedParams = transformQueryParams(params);
  const searchParams = new URLSearchParams(transformedParams);
  return searchParams.toString();
}
