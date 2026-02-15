/**
 * 全局状态管理
 */

import { writable } from 'svelte/store';
import type { ApiPromise } from '@polkadot/api';
import type { KeyringPair } from '@polkadot/keyring/types';
import type { VotingAPI } from './voting-api.ts';

// API 实例
export const api = writable<ApiPromise | null>(null);
export const votingApi = writable<VotingAPI | null>(null);

// 当前用户
export const currentUser = writable<KeyringPair | null>(null);
export const userRole = writable<'admin' | 'teacher' | 'student' | null>(null);

// 连接状态
export const isConnected = writable(false);
export const currentBlock = writable(0);

// 加载状态
export const isLoading = writable(false);

// 错误信息
export const error = writable<string | null>(null);
