// Markdown 渲染 Composable
import MarkdownIt from "markdown-it";
import hljs from "highlight.js";
import anchor from "markdown-it-anchor";
import toc from "markdown-it-table-of-contents";

interface MarkdownOptions {
  html?: boolean;
  breaks?: boolean;
  linkify?: boolean;
  typographer?: boolean;
  highlight?: boolean;
  anchor?: boolean;
  toc?: boolean;
  insertTocMarker?: boolean; // 是否在HTML中插入TOC标记
}

interface TocItem {
  level: number;
  title: string;
  anchor: string;
  children: TocItem[];
}

interface MarkdownResult {
  html: string;
  toc: TocItem[];
}

// 创建 Markdown 实例的工厂函数
function createMarkdownInstance(options: MarkdownOptions = {}): MarkdownIt {
  const md: MarkdownIt = new MarkdownIt({
    html: options.html ?? true,
    breaks: options.breaks ?? true,
    linkify: options.linkify ?? true,
    typographer: options.typographer ?? true,
    highlight: options.highlight !== false
      ? (str: string, lang: string): string => {
          if (lang && hljs.getLanguage(lang)) {
            try {
              return `<pre class="hljs"><code class="hljs-${lang}">${hljs.highlight(str, { language: lang }).value}</code></pre>`;
            }
            catch {
              // Failed to highlight, fallback to escaped HTML
            }
          }
          return `<pre class="hljs"><code>${md.utils.escapeHtml(str)}</code></pre>`;
        }
      : undefined,
  });

  // 添加锚点插件
  if (options.anchor !== false) {
    md.use(anchor, {
      permalink: anchor.permalink.linkInsideHeader({
        symbol: "#",
        placement: "before",
      }),
      level: [1, 2, 3, 4, 5, 6],
      slugify: (s: string) => s.toLowerCase().replace(/\s+/g, "-").replace(/[^\w-]+/g, ""),
    });
  }

  // 添加目录插件 - 只用于提取目录数据，不在HTML中插入
  if (options.toc !== false) {
    md.use(toc, {
      includeLevel: [1, 2, 3, 4, 5, 6],
      containerClass: "table-of-contents",
      slugify: (s: string) => s.toLowerCase().replace(/\s+/g, "-").replace(/[^\w-]+/g, ""),
      markerPattern: /^\[\[toc\]\]/im, // 只有明确标记才插入HTML
      listType: "ul",
      format: (anchor: string, html: string) => html,
      transformContainerOpen: () => "", // 不渲染容器开始标签
      transformContainerClose: () => "", // 不渲染容器结束标签
    });
  }

  return md;
}

// 提取目录的函数
function extractToc(html: string): TocItem[] {
  const toc: TocItem[] = [];
  const stack: TocItem[] = [];

  // 使用正则表达式匹配标题
  const headingRegex = /<h([1-6])[^>]*id="([^"]*)"[^>]*>(.*?)<\/h[1-6]>/g;
  let match;

  while ((match = headingRegex.exec(html)) !== null) {
    const level = parseInt(match[1]);
    const anchor = match[2];
    const title = match[3].replace(/<[^>]*>/g, ""); // 移除 HTML 标签

    const item: TocItem = {
      level,
      title,
      anchor,
      children: [],
    };

    // 处理层级关系
    while (stack.length > 0 && stack[stack.length - 1].level >= level) {
      stack.pop();
    }

    if (stack.length === 0) {
      toc.push(item);
    }
    else {
      stack[stack.length - 1].children.push(item);
    }

    stack.push(item);
  }

  return toc;
}

// 主要的 Composable 函数
export function useMarkdown(options: MarkdownOptions = {}) {
  const md = createMarkdownInstance(options);

  // 渲染 Markdown 文本
  const render = (content: string): MarkdownResult => {
    if (!content || typeof content !== "string") {
      return { html: "", toc: [] };
    }

    try {
      const html = md.render(content);
      const toc = extractToc(html);

      return { html, toc };
    }
    catch (error) {
      console.error("Markdown 渲染错误:", error);
      return {
        html: `<div class="error">Markdown 渲染失败: ${error instanceof Error ? error.message : "未知错误"}</div>`,
        toc: [],
      };
    }
  };

  // 渲染行内 Markdown（不包含块级元素）
  const renderInline = (content: string): string => {
    if (!content || typeof content !== "string") {
      return "";
    }

    try {
      return md.renderInline(content);
    }
    catch (error) {
      console.error("Markdown 行内渲染错误:", error);
      return content;
    }
  };

  // 验证 Markdown 内容
  const validate = (content: string): boolean => {
    if (!content || typeof content !== "string") {
      return false;
    }

    try {
      md.render(content);
      return true;
    }
    catch (error) {
      console.error("Markdown 验证失败:", error);
      return false;
    }
  };

  // 提取纯文本（用于摘要）
  const extractText = (content: string): string => {
    if (!content || typeof content !== "string") {
      return "";
    }

    try {
      const html = md.render(content);
      // 移除 HTML 标签
      return html.replace(/<[^>]*>/g, "").trim();
    }
    catch (error) {
      console.error("提取纯文本错误:", error);
      return content;
    }
  };

  return {
    render,
    renderInline,
    validate,
    extractText,
  };
}

// 单独导出渲染函数（用于简单场景）
export function renderMarkdown(content: string, options: MarkdownOptions = {}): MarkdownResult {
  const { render } = useMarkdown(options);
  return render(content);
}

// 导出类型
export type { MarkdownOptions, MarkdownResult, TocItem };
