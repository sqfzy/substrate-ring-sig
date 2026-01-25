<script lang="ts">
  import { onMount } from 'svelte';
  import { ApiPromise, WsProvider } from '@polkadot/api';
  import { statusLogs, addLog, toasts, addToast } from '$lib/store';
  import { ensureStorageExists, PALLET_NAME, hexToU8a } from '$lib/utils';
  import { fade, fly } from 'svelte/transition';
  
  import RingManager from '../../components/RingManager.svelte';
  import PollAdmin from '../../components/PollAdmin.svelte';
  
  import init from '../../lib/wasm'; 

  let api: ApiPromise | null = null;
  let isWasmLoaded = false;
  let isConnected = false;
  let blockNumber = 0;
  let polls: any[] = [];
  let rings: any[] = [];

  onMount(async () => {
    try {
      addLog('INIT', 'Initializing WASM (Admin)...');
      await init();
      isWasmLoaded = true;

      const wsProvider = new WsProvider('ws://127.0.0.1:9944');
      api = await ApiPromise.create({ provider: wsProvider });
      isConnected = true;
      addToast('管理员后台已连接', 'success');

      api.rpc.chain.subscribeNewHeads((header) => {
        blockNumber = header.number.toNumber();
      });

      if (api) {
        await fetchPolls();
        await fetchRings();
      }
    } catch (err) { addLog('ERROR', err); }
  });

  async function fetchPolls() {
    if (!api || !ensureStorageExists(api, PALLET_NAME, 'polls')) return;
    try {
      const entries = await api.query[PALLET_NAME].polls.entries();
      polls = entries.map(([key, codec]) => {
        const pollId = key.args[0].toNumber();
        return { id: pollId, ...codec.toJSON() as any };
      }).sort((a, b) => b.id - a.id);
    } catch (e) { console.error(e); }
  }

  async function fetchRings() {
    if (!api || !ensureStorageExists(api, PALLET_NAME, 'rings')) return;
    try {
      const entries = await api.query[PALLET_NAME].rings.entries();
      rings = entries.map(([key, codec]) => ({
        id: key.args[0].toNumber(),
        publicKeys: codec.toJSON()
      }));
    } catch (e) { console.error(e); }
  }
</script>

<div class="admin-root">
  <!-- Toast Container -->
  <div class="toast-container">
    {#each $toasts as toast (toast.id)}
      <div class="toast {toast.type}" in:fly={{ y: 20, duration: 300 }} out:fade>
        {toast.message}
      </div>
    {/each}
  </div>

  <header>
    <div class="header-left">
      <h1>管理员控制台</h1>
      <span class="role-badge">ADMIN</span>
    </div>
    <div class="status-indicators">
      <span>当前区块: {blockNumber}</span>
      <span class="dot {isConnected ? 'ok' : ''}"></span>
    </div>
  </header>

  <div class="admin-grid">
    <div class="column">
      <RingManager {api} {rings} {fetchRings} />
      
      <!-- Console 日志 -->
      <div class="console">
        <div class="console-header">System Logs</div>
        <div class="console-body">
          {#each $statusLogs as log}
            <div class="log-line">{log}</div>
          {/each}
        </div>
      </div>
    </div>

    <div class="column">
      <PollAdmin {api} {blockNumber} {fetchPolls} />
      
      <!-- 简单展示当前投票列表，方便管理员查看ID -->
      <div class="simple-list">
        <h3>当前投票列表预览</h3>
        {#each polls as poll}
          <div class="mini-item">
            <span class="id">#{poll.id}</span>
            <span class="desc">{poll.description ? (new TextDecoder().decode(hexToU8a(poll.description))) : '无描述'}</span>
            <span class="status {poll.status?.toLowerCase()}">{poll.status}</span>
          </div>
        {/each}
      </div>
    </div>
  </div>
</div>

<style>
  .admin-root { max-width: 1200px; margin: 0 auto; padding: 20px; }

  header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 30px; border-bottom: 1px solid var(--border); padding-bottom: 20px; }
  h1 { margin: 0; color: var(--text); }
  .role-badge { background: var(--error); color: #1a1b26; padding: 4px 8px; border-radius: 4px; font-weight: bold; margin-left: 10px; font-size: 0.8em; }
  
  .status-indicators { display: flex; align-items: center; gap: 10px; font-family: monospace; color: var(--muted); }
  .dot { width: 8px; height: 8px; border-radius: 50%; background: var(--error); }
  .dot.ok { background: var(--success); box-shadow: 0 0 8px var(--success); }

  .admin-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 30px; }

  .simple-list { background: var(--bg-dark); padding: 15px; border-radius: 8px; border: 1px solid var(--border); margin-top: 20px; }
  .simple-list h3 { margin: 0 0 10px 0; font-size: 0.9em; color: var(--muted); }
  .mini-item { display: flex; justify-content: space-between; padding: 8px 0; border-bottom: 1px dashed var(--border); font-size: 0.85em; }
  .mini-item .id { font-family: monospace; color: var(--accent); }
  .mini-item .status.active { color: var(--success); }
  
  .console { margin-top: 20px; background: #0f0f14; border-radius: 8px; border: 1px solid var(--border); height: 200px; display: flex; flex-direction: column; }
  .console-header { background: #1a1b26; padding: 8px 16px; font-size: 0.8em; color: var(--muted); border-bottom: 1px solid var(--border); }
  .console-body { flex: 1; overflow-y: auto; padding: 12px; font-family: 'JetBrains Mono', monospace; font-size: 0.8em; }
  .log-line { border-bottom: 1px solid #1a1b26; padding: 4px 0; color: var(--text); opacity: 0.8; }

  .toast-container { position: fixed; top: 80px; right: 20px; z-index: 1000; display: flex; flex-direction: column; gap: 10px; }
  .toast { padding: 12px 20px; border-radius: 8px; color: white; font-size: 0.9em; box-shadow: 0 4px 12px rgba(0,0,0,0.3); }
  .toast.success { background: var(--success); color: #1a1b26; }
  .toast.error { background: var(--error); color: #1a1b26; }
  .toast.info { background: var(--secondary); color: #1a1b26; }
</style>
