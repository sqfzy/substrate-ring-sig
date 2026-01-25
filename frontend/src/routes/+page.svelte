<script lang="ts">
  import { onMount } from 'svelte';
  import { ApiPromise, WsProvider } from '@polkadot/api';
  import { statusLogs, addLog, toasts, addToast } from '$lib/store';
  import { ensureStorageExists, sleep, PALLET_NAME, hexToU8a } from '$lib/utils';
  import { u8aToHex } from '@polkadot/util';
  import { fade, fly } from 'svelte/transition';
  
  // 只导入身份恢复功能，去掉生成功能
  import { myPrivateKey, myPublicKey, restoreIdentity } from '$lib/identity';
  
  import init, {
    encrypt_vote,
    sign_blsag,
  } from '../lib/wasm'; 

  // ================= 状态变量 =================
  let api: ApiPromise | null = null;
  let isWasmLoaded = false;
  let isConnected = false;
  let blockNumber = 0;
  let polls: any[] = [];
  let rings: any[] = [];
  let formVote = { pollId: 0, option: 'A' };
  let isVotingMap: Record<number, boolean> = {}; 

  // ================= 初始化 =================
  onMount(async () => {
    try {
      addLog('INIT', 'Initializing WASM...');
      await init();
      isWasmLoaded = true;

      addLog('CONN', 'Connecting to Substrate...');
      const wsProvider = new WsProvider('ws://127.0.0.1:9944');
      api = await ApiPromise.create({ provider: wsProvider });
      isConnected = true;
      addToast('已连接到 Substrate 节点', 'success');

      api.rpc.chain.subscribeNewHeads((header) => {
        blockNumber = header.number.toNumber();
      });

      if (api) {
        await fetchPolls();
        await fetchRings();
      }
    } catch (err) { addLog('ERROR', err); addToast('连接失败', 'error'); }
  });

  // ================= 数据拉取 =================
  async function fetchPolls() {
    if (!api || !ensureStorageExists(api, PALLET_NAME, 'polls')) return;
    try {
      const entries = await api.query[PALLET_NAME].polls.entries();
      
      // 并行获取每个 Poll 的详细信息和真实票数
      const pollsData = await Promise.all(entries.map(async ([key, codec]) => {
        const pollId = key.args[0].toNumber();
        const basicData = codec.toJSON() as any;

        // 【修复】手动查询 EncryptedVotes 的长度作为票数
        // 这样即使后端 Poll 结构体没更新计数，前端也能显示正确票数
        let realVoteCount = 0;
        try {
            const votesStorage = await api!.query[PALLET_NAME].encryptedVotes(pollId);
            const votesVec = votesStorage.toJSON() as any[];
            if (votesVec) {
                realVoteCount = votesVec.length;
            }
        } catch(e) {
            console.warn(`Failed to count votes for poll #${pollId}`, e);
        }

        return { 
            id: pollId, 
            ...basicData,
            voteCount: realVoteCount // 覆盖或新增该字段
        };
      }));

      polls = pollsData.sort((a, b) => b.id - a.id);
      addLog('FETCH', `Polls updated: ${polls.length}`);
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

  // ================= 投票逻辑 =================
  async function castVote(pollId: number, optionText: string) {
    if (!api || !$myPrivateKey) { addToast("请先导入您的身份私钥", "error"); return; }

    try {
      isVotingMap[pollId] = true; 
      const targetPoll = polls.find(p => p.id === pollId);
      const targetRing = rings.find(r => r.id === targetPoll.ringId);
      
      if (!targetRing) { throw new Error(`Ring #${targetPoll.ringId} 不存在`); }

      const ringKeysHex: string[] = targetRing.publicKeys.map((k: string) => k.replace(/^0x/, ''));
      const myPubNoPrefix = $myPublicKey.replace(/^0x/, '');
      const secretIndex = ringKeysHex.indexOf(myPubNoPrefix);
      
      if (secretIndex === -1) {
        addToast(`未授权：您的公钥不在 Ring #${targetPoll.ringId} 中`, 'error');
        isVotingMap[pollId] = false;
        return;
      }

      // === 核心修复: 构造混淆环 (Decoys) ===
      const decoys = ringKeysHex.filter((_, idx) => idx !== secretIndex);

      addToast("正在生成零知识证明...", "info");
      await sleep(50); 

      const plaintext = new TextEncoder().encode(optionText);
      let pollPubKeyRaw = targetPoll.pollPublicKey;
      if (typeof pollPubKeyRaw === 'object' && !Array.isArray(pollPubKeyRaw)) pollPubKeyRaw = Object.values(pollPubKeyRaw);
      let pollPubKeyHex = Array.isArray(pollPubKeyRaw) ? Buffer.from(pollPubKeyRaw).toString('hex') : pollPubKeyRaw.toString().replace(/^0x/, '');

      const [ephemeralPubHex, ciphertextHex] = encrypt_vote(pollPubKeyHex, plaintext, new Uint8Array([]));
      
      // Construct Message: Ephemeral Pub + Ciphertext
      const msgBytes = new Uint8Array([...hexToU8a(ephemeralPubHex), ...hexToU8a(ciphertextHex)]);
      
      addToast("正在计算环签名...", "info");
      
      const resultVec = sign_blsag(decoys, msgBytes, $myPrivateKey, secretIndex);
      
      const challengeHex = resultVec[0];
      const keyImageHex = resultVec[1];

      const resultLen = resultVec.length;
      const actualRingSize = (resultLen - 2) / 2;
      
      if (resultLen < 2 || (resultLen - 2) % 2 !== 0) {
          throw new Error(`WASM 返回数据长度异常: ${resultLen}`);
      }

      const responsesHex = resultVec.slice(2 + actualRingSize);

      addToast("签名完成，正在提交...", "info");

      await api.tx[PALLET_NAME].vote(
          pollId,
          `0x${ephemeralPubHex}`, 
          `0x${ciphertextHex}`, 
          `0x${challengeHex}`,
          responsesHex.map(r => `0x${r}`), 
          `0x${keyImageHex}`
        )
        .send(({ status }) => {
           if (status.isInBlock) {
             addToast(`投票成功！`, 'success');
             fetchPolls(); // 现在 fetchPolls 会从链上拉取最新的真实票数
             isVotingMap[pollId] = false;
           } else if (status.isFinalized) {
             isVotingMap[pollId] = false;
           }
        });
    } catch (e) { 
      addLog('ERROR', e); 
      addToast("投票失败，请查看日志", 'error'); 
      isVotingMap[pollId] = false;
    }
  }
</script>

<div class="app-root">
  <!-- Toast Container -->
  <div class="toast-container">
    {#each $toasts as toast (toast.id)}
      <div class="toast {toast.type}" in:fly={{ y: 20, duration: 300 }} out:fade>
        {toast.message}
      </div>
    {/each}
  </div>

  <header>
    <div class="logo">
      <h1>Nazgul Voting</h1>
      <span class="version">学生端</span>
    </div>
    <div class="status-bar">
      <div class="status-item">
        <span class="dot {isWasmLoaded ? 'ok' : ''}"></span> WASM
      </div>
      <div class="status-item">
        <span class="dot {isConnected ? 'ok' : ''}"></span> NET
      </div>
      <div class="status-item block-info">
        📦 {blockNumber}
      </div>
    </div>
  </header>

  <div class="grid-layout">
    <!-- LEFT SIDEBAR -->
    <aside>
      <div class="card identity-card">
        <div class="card-header"><h3>🔐 身份导入 (Import Identity)</h3></div>
        <div class="card-body">
          <p class="hint">请输入学校分发的私钥以激活投票权限。</p>
          <input type="password" value={$myPrivateKey} on:input={(e) => restoreIdentity(e.target.value)} placeholder="粘贴私钥..." class="private-key-input" />
          
          {#if $myPublicKey}
             <div class="key-box">
               <span class="key-label">IDENTIFIED AS</span>
               <code>{$myPublicKey.slice(0, 16)}...</code>
               <div class="badge-active">已激活</div>
             </div>
          {:else}
             <div class="status-offline">未连接身份</div>
          {/if}
        </div>
      </div>
      
      <!-- Console 日志现在只显示在侧边栏下方 -->
      <div class="console">
        <div class="console-header">System Logs</div>
        <div class="console-body">
          {#each $statusLogs as log}
            <div class="log-line">{log}</div>
          {/each}
        </div>
      </div>
    </aside>

    <!-- RIGHT CONTENT -->
    <section>
      <div class="section-header">
        <h2>🗳️ 投票大厅 (Voting Hall)</h2>
        <button class="icon-btn" on:click={fetchPolls} title="Refresh">🔄</button>
      </div>

      <div class="poll-list">
        {#each polls as poll}
          <div class="poll-card">
            <div class="poll-top">
              <span class="poll-id">#{poll.id}</span>
              <span class="status-badge {poll.status?.toLowerCase()}">{poll.status}</span>
            </div>
            
            <h3 class="poll-title">{poll.description ? (new TextDecoder().decode(hexToU8a(poll.description))) : '无描述'}</h3>
            
            <div class="poll-info-grid">
              <div class="info-item"><span>Ring Group</span> <strong>ID: {poll.ringId}</strong></div>
              <div class="info-item"><span>Deadline</span> <strong>Block {poll.deadline}</strong></div>
              <!-- 这里直接使用我们在 fetchPolls 中手动计算的 voteCount -->
              <div class="info-item"><span>Votes</span> <strong>{poll.voteCount || 0}</strong></div>
            </div>

            <div class="poll-actions">
              {#if poll.status === 'Active'}
                 <input type="text" bind:value={formVote.option} placeholder="Option (A/B)" class="vote-input"/>
                 <button 
                    class="vote-btn" 
                    on:click={() => castVote(poll.id, formVote.option)}
                    disabled={isVotingMap[poll.id] || !$myPublicKey}
                 >
                   {isVotingMap[poll.id] ? '正在加密提交...' : '匿名投票'}
                 </button>
              {:else}
                 <div class="ended-msg">🔒 投票已结束</div>
              {/if}
            </div>
          </div>
        {:else}
          <div class="empty-state">暂无活动投票</div>
        {/each}
      </div>
    </section>
  </div>
</div>

<style>
  .app-root { max-width: 1200px; margin: 0 auto; padding: 20px; }
  
  /* 复用之前的 CSS 变量 */
  header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 30px; }
  .logo h1 { margin: 0; color: var(--text); }
  .version { background: var(--surface); padding: 2px 8px; border-radius: 4px; font-size: 0.7em; color: var(--success); font-weight: bold; margin-left: 10px; }
  
  .status-bar { display: flex; gap: 20px; font-size: 0.9em; }
  .status-item { display: flex; align-items: center; gap: 8px; }
  .dot { width: 8px; height: 8px; border-radius: 50%; background: var(--error); }
  .dot.ok { background: var(--success); box-shadow: 0 0 8px var(--success); }
  .block-info { background: var(--surface); padding: 4px 12px; border-radius: 20px; font-family: monospace; }

  .grid-layout { display: grid; grid-template-columns: 320px 1fr; gap: 30px; }

  .card { background: var(--surface); border-radius: 12px; border: 1px solid var(--border); overflow: hidden; margin-bottom: 24px; }
  .card-header { padding: 16px; background: rgba(0,0,0,0.2); border-bottom: 1px solid var(--border); }
  .card-header h3 { margin: 0; font-size: 1rem; color: var(--primary); }
  .card-body { padding: 20px; }
  
  .hint { font-size: 0.85em; color: var(--muted); margin-bottom: 15px; }
  .private-key-input { width: 100%; background: var(--bg-dark); border: 1px solid var(--border); color: var(--accent); padding: 12px; border-radius: 8px; box-sizing: border-box; font-family: monospace; }
  
  .key-box { margin-top: 15px; background: var(--bg-dark); padding: 10px; border-radius: 6px; border: 1px solid var(--success); display: flex; flex-direction: column; position: relative; }
  .key-label { font-size: 0.6em; color: var(--muted); font-weight: bold; margin-bottom: 4px; }
  .badge-active { position: absolute; top: 10px; right: 10px; font-size: 0.7em; background: var(--success); color: #000; padding: 2px 6px; border-radius: 4px; font-weight: bold; }
  .status-offline { margin-top: 10px; color: var(--muted); font-style: italic; text-align: center; }

  .section-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px; }
  .icon-btn { background: var(--surface); border: 1px solid var(--border); color: var(--text); width: 36px; height: 36px; border-radius: 8px; cursor: pointer; }

  .poll-card { background: var(--surface); border-radius: 12px; padding: 24px; margin-bottom: 20px; border: 1px solid var(--border); transition: transform 0.2s; }
  .poll-card:hover { border-color: var(--primary); transform: translateY(-2px); }
  
  .poll-top { display: flex; justify-content: space-between; margin-bottom: 12px; }
  .poll-id { font-family: monospace; color: var(--muted); }
  .status-badge { padding: 4px 10px; border-radius: 20px; font-size: 0.75rem; font-weight: bold; text-transform: uppercase; }
  .status-badge.active { background: rgba(158, 206, 106, 0.2); color: var(--success); }

  .poll-title { margin: 0 0 20px 0; font-size: 1.2rem; font-weight: 500; }

  .poll-info-grid { display: grid; grid-template-columns: repeat(3, 1fr); gap: 10px; background: var(--bg-dark); padding: 12px; border-radius: 8px; margin-bottom: 20px; }
  .info-item { display: flex; flex-direction: column; font-size: 0.8rem; }
  .info-item span { color: var(--muted); margin-bottom: 4px; }
  .info-item strong { color: var(--primary); font-family: monospace; font-size: 1rem; }

  .poll-actions { display: flex; gap: 10px; }
  .vote-input { width: 80px; background: var(--bg); border: 1px solid var(--border); color: white; padding: 10px; border-radius: 6px; text-align: center; font-weight: bold; }
  .vote-btn { flex: 1; background: var(--primary); color: #1a1b26; border: none; border-radius: 6px; font-weight: bold; cursor: pointer; }
  .vote-btn:disabled { opacity: 0.5; cursor: not-allowed; background: var(--border); }
  
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
