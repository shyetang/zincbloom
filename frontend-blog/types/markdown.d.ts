// 声明第三方模块的类型
declare module "markdown-it-table-of-contents" {
  import type MarkdownIt from "markdown-it";

  interface TocOptions {
    includeLevel?: number[];
    containerClass?: string;
    slugify?: (s: string) => string;
    markerPattern?: RegExp;
    listType?: "ul" | "ol";
    format?: (anchor: string, htmlAnchor: string) => string;
    anchor?: {
      permalink?: boolean;
      permalinkClass?: string;
      permalinkSymbol?: string;
      permalinkBefore?: boolean;
    };
    transformLink?: (href: string) => string;
  }

  function tocPlugin(md: MarkdownIt, options?: TocOptions): void;
  export = tocPlugin;
}

declare module "markdown-it-anchor" {
  import type MarkdownIt from "markdown-it";

  interface PermalinkOptions {
    symbol?: string;
    placement?: "before" | "after";
    space?: boolean;
    class?: string;
  }

  interface AnchorOptions {
    level?: number[];
    slugify?: (s: string) => string;
    permalink?: boolean | ((options: PermalinkOptions) => (slug: string, opts: AnchorOptions, state: unknown, idx: number) => void);
    permalinkClass?: string;
    permalinkSymbol?: string;
    permalinkBefore?: boolean;
    permalinkHref?: (slug: string) => string;
    permalinkAttrs?: (slug: string) => Record<string, string>;
    permalinkSpace?: boolean;
    renderPermalink?: (slug: string, opts: AnchorOptions, state: unknown, idx: number) => void;
    tabIndex?: boolean;
  }

  interface AnchorStatic {
    (md: MarkdownIt, options?: AnchorOptions): void;
    permalink: {
      linkInsideHeader: (options: PermalinkOptions) => (slug: string, opts: AnchorOptions, state: unknown, idx: number) => void;
      ariaHidden: (options: PermalinkOptions) => (slug: string, opts: AnchorOptions, state: unknown, idx: number) => void;
      headerLink: (options: PermalinkOptions) => (slug: string, opts: AnchorOptions, state: unknown, idx: number) => void;
    };
  }

  const anchor: AnchorStatic;
  export = anchor;
}
