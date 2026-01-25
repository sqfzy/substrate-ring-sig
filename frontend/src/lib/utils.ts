import { ApiPromise } from '@polkadot/api';
import { addLog } from './store';

// 硬编码 Pallet 名字，统一管理
export const PALLET_NAME = 'ringSigVoting';

/**
 * 核心防御性检查函数
 * 确保指定的 Module 和 Storage Item 在链上元数据中存在
 */
export function ensureStorageExists(api: ApiPromise, pallet: string, storage: string): boolean {
  // 1. 检查模块是否存在
  if (!api.query[pallet]) {
    addLog('FATAL', `模块 '${pallet}' 未找到！请检查链 Runtime 配置或 Pallet 名称拼写。`);
    return false;
  }

  // 2. 检查存储项是否存在
  if (!api.query[pallet][storage]) {
    const availableKeys = Object.keys(api.query[pallet]);
    addLog('ERROR', `存储项 '${storage}' 未找到！模块 '${pallet}' 下可用的存储项有: [${availableKeys.join(', ')}]`);
    return false;
  }

  return true;
}

/**
 * Hex 字符串转 Uint8Array (兼容 0x 前缀)
 */
export const hexToU8a = (hex: string): Uint8Array => {
  if (!hex) return new Uint8Array([]);
  if (hex.startsWith('0x')) hex = hex.slice(2);
  // 处理奇数长度
  if (hex.length % 2 !== 0) hex = '0' + hex;
  
  const match = hex.match(/.{1,2}/g);
  if (!match) return new Uint8Array([]);
  
  return new Uint8Array(match.map(byte => parseInt(byte, 16)));
};

/**
 * 字符串转 Hex (用于 description/metadata)
 */
export const stringToHex = (str: string): string => {
  return '0x' + Array.from(new TextEncoder().encode(str))
    .map(b => b.toString(16).padStart(2, '0'))
    .join('');
};

/**
 * 简单的延迟函数，给 UI 渲染让出时间
 */
export const sleep = (ms: number) => new Promise(r => setTimeout(r, ms));
