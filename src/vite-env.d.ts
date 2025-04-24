/// <reference types="vite/client" />

declare module "*.vue" {
  import type { DefineComponent } from "vue";
  const component: DefineComponent<{}, {}, any>;
  export default component;
}

declare module '@tauri-apps/api/tauri' {
  export function invoke<T>(command: string, args?: Record<string, unknown>): Promise<T>;
}

declare module '@tauri-apps/api/dialog' {
  export function open(options: {
    multiple?: boolean;
    filters?: Array<{
      name: string;
      extensions: string[];
    }>;
  }): Promise<string | string[] | null>;
}
