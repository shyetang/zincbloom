// 临时UI类型定义，直到 @nuxt/ui 类型正确解析

export interface DropdownItem {
  label: string;
  icon?: string;
  to?: string;
  click?: () => void;
}

export interface FormSubmitEvent<T = any> {
  data: T;
}

// 导出类型供组件使用
declare global {
  interface FormSubmitEvent<T = any> {
    data: T;
  }
}
