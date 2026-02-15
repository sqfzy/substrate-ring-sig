/**
 * 工具函数
 */

/**
 * 格式化区块号为相对时间
 */
export function formatBlockTime(blockNumber: number, currentBlock: number): string {
  const diff = blockNumber - currentBlock;
  if (diff <= 0) return '已过期';
  
  const minutes = Math.floor(diff / 10); // 假设 6s 一个块
  if (minutes < 60) return `${minutes} 分钟`;
  
  const hours = Math.floor(minutes / 60);
  if (hours < 24) return `${hours} 小时`;
  
  const days = Math.floor(hours / 24);
  return `${days} 天`;
}

/**
 * 格式化地址
 */
export function formatAddress(address: string): string {
  return `${address.slice(0, 6)}...${address.slice(-4)}`;
}

/**
 * 复制到剪贴板
 */
export async function copyToClipboard(text: string): Promise<void> {
  await navigator.clipboard.writeText(text);
}

/**
 * 下载为文件
 */
export function downloadAsFile(content: string, filename: string): void {
  const blob = new Blob([content], { type: 'text/plain' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = filename;
  a.click();
  URL.revokeObjectURL(url);
}
