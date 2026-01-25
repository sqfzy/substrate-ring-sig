import { writable } from 'svelte/store';

// ================= 日志系统 =================
export const statusLogs = writable<string[]>([]);

export function addLog(action: string, data: any = '') {
  const timestamp = new Date().toLocaleTimeString();
  const strData = typeof data === 'object' ? JSON.stringify(data, (k, v) => {
      if (v instanceof Uint8Array || (v && v.type === 'Buffer')) {
        return `[Bytes len=${v.length || Object.keys(v).length}]`;
      }
      return v;
  }) : data;
  const message = `[${timestamp}] [${action}] ${strData}`;
  console.log(`[${action}]`, data);
  statusLogs.update(logs => [message, ...logs.slice(0, 99)]);
}

// ================= Toast 通知系统 =================
export type ToastType = 'success' | 'error' | 'info';

export interface Toast {
  id: number;
  message: string;
  type: ToastType;
}

export const toasts = writable<Toast[]>([]);

let toastIdCounter = 0;

export function addToast(message: string, type: ToastType = 'info') {
  const id = toastIdCounter++;
  toasts.update(all => [...all, { id, message, type }]);
  
  // 3秒后自动消失
  setTimeout(() => {
    toasts.update(all => all.filter(t => t.id !== id));
  }, 3000);
}
