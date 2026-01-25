<script lang="ts">
  import { ApiPromise, Keyring } from '@polkadot/api';
  import { addLog, addToast } from '$lib/store';
  import { hexToU8a, stringToHex, PALLET_NAME } from '$lib/utils';
  import { generate_private_key, derive_public_key, decrypt_vote } from '../lib/wasm';

  export let api: ApiPromise | null;
  export let blockNumber: number;
  export let fetchPolls: () => void;

  let formCreatePoll = {
    description: '评教：Rust 程序设计',
    ringId: 0,
    deadline: 100
  };

  let formTally = {
    pollId: 0,
    pollPrivateKey: ''
  };

  let isCreating = false;

  async function createPoll() {
    if (!api) return;
    try {
      isCreating = true;
      const pollPriv = generate_private_key();
      const pollPub = derive_public_key(pollPriv);

      formTally.pollPrivateKey = pollPriv; 
      addLog('ADMIN', `已生成投票私钥: ${pollPriv}`);
      
      // 使用 prompt 强制用户注意，但也用 toast 提示
      prompt("【重要】请复制并保存此私钥，用于后续计票：", pollPriv);
      addToast("私钥已生成，请务必保存！", "info");

      const keyring = new Keyring({ type: 'sr25519' });
      const alice = keyring.addFromUri('//Alice'); 
      const metadata = stringToHex("metadata"); 
      const description = stringToHex(formCreatePoll.description);
      const pollPubKeyU8a = hexToU8a(pollPub);

      await api.tx[PALLET_NAME]
        .createPoll(
          formCreatePoll.ringId,
          description,
          metadata,
          formCreatePoll.deadline + blockNumber,
          pollPubKeyU8a
        )
        .signAndSend(alice, ({ status }) => {
          if (status.isInBlock) {
            addLog('TX', `CreatePoll 成功入块`);
            addToast("投票创建成功！", "success");
            
            // 延迟刷新，确保链上状态更新
            setTimeout(() => {
                fetchPolls();
                isCreating = false;
            }, 1000);
          }
        });
    } catch (e) { 
      addLog('ERROR', e);
      addToast(`创建失败: ${e}`, 'error');
      isCreating = false;
    }
  }

  async function performTally() {
    if (!api) return;
    const pid = formTally.pollId;
    const privKey = formTally.pollPrivateKey.trim();
    
    try {
      addToast(`正在统计 Poll #${pid}...`, 'info');
      const votesCodec = await api.query[PALLET_NAME].encryptedVotes(pid);
      const votes = votesCodec.toJSON() as any[];

      if (!votes || !votes.length) {
        addToast("该投票暂无选票", 'error');
        return;
      }

      const results: Record<string, number> = {};
      const aad = new Uint8Array([]);
      let successCount = 0;

      for (const v of votes) {
        const ephPub = v.ephemeralPublicKey || v.ephemeral_public_key;
        const cipher = v.ciphertext;
        try {
          const decryptedBytes = decrypt_vote(
            ephPub.replace(/^0x/, ''),
            cipher.replace(/^0x/, ''),
            privKey,
            aad
          );
          const option = new TextDecoder().decode(decryptedBytes);
          results[option] = (results[option] || 0) + 1;
          successCount++;
        } catch (err) { console.error(err); }
      }
      
      addLog('TALLY', '统计完成', results);
      alert(`统计结果 (成功解密 ${successCount}/${votes.length}):\n${JSON.stringify(results, null, 2)}`);
    } catch (e) { 
        addLog('ERROR', e);
        addToast("统计过程出错", 'error');
    }
  }
</script>

<div class="card admin-panel">
  <div class="card-header">
    <h3>👨‍🏫 教师操作 (Admin Zone)</h3>
  </div>
  
  <div class="card-body">
    <!-- Section 1: Create Poll -->
    <div class="sub-section">
      <h4>Create Poll</h4>
      <div class="input-group">
        <label>课程描述</label>
        <input type="text" bind:value={formCreatePoll.description} class="styled-input" />
      </div>
      <div class="row">
        <div class="input-group half">
          <label>Ring ID</label>
          <input type="number" bind:value={formCreatePoll.ringId} class="styled-input" placeholder="0" />
        </div>
        <div class="input-group half">
          <label>持续区块</label>
          <input type="number" bind:value={formCreatePoll.deadline} class="styled-input" placeholder="100" />
        </div>
      </div>
      <button class="primary-btn full-width" on:click={createPoll} disabled={!api || isCreating}>
        {isCreating ? '创建中...' : '发起投票'}
      </button>
    </div>

    <div class="divider"></div>

    <!-- Section 2: Tally -->
    <div class="sub-section">
      <h4>Results Tally</h4>
      <div class="input-group">
        <label>Poll ID</label>
        <input type="number" bind:value={formTally.pollId} class="styled-input" />
      </div>
      <div class="input-group">
        <label>Poll Private Key</label>
        <input type="password" autocomplete="off" bind:value={formTally.pollPrivateKey} class="styled-input" placeholder="粘贴私钥" />
      </div>
      <button class="secondary-btn full-width" on:click={performTally}>
        解密并统计结果
      </button>
    </div>
  </div>
</div>

<style>
  .card {
    background: var(--surface);
    border-radius: 12px;
    border: 1px solid var(--border);
    margin-bottom: 24px;
  }
  .card-header {
    background: rgba(0,0,0,0.2);
    padding: 16px 20px;
    border-bottom: 1px solid var(--border);
  }
  .card-body { padding: 20px; }
  
  h3 { margin: 0; color: var(--primary); font-size: 1.1em; }
  h4 { color: var(--accent); font-size: 0.9em; text-transform: uppercase; letter-spacing: 1px; margin-bottom: 12px; }
  
  .sub-section { margin-bottom: 0; }
  .divider { height: 1px; background: var(--border); margin: 20px 0; border-top: 1px dashed rgba(255,255,255,0.1); }

  .input-group { margin-bottom: 12px; }
  .row { display: flex; gap: 12px; }
  .half { flex: 1; }

  label { display: block; font-size: 0.8em; color: var(--muted); margin-bottom: 6px; }
  .styled-input {
    width: 100%;
    background: var(--bg-dark);
    border: 1px solid var(--border);
    color: var(--text);
    padding: 10px;
    border-radius: 6px;
    transition: border 0.2s;
  }
  .styled-input:focus { outline: none; border-color: var(--primary); }

  .primary-btn {
    background: var(--primary);
    color: #1a1b26;
    border: none; padding: 10px; border-radius: 6px; font-weight: bold; cursor: pointer;
  }
  .secondary-btn {
    background: var(--secondary);
    color: #1a1b26;
    border: none; padding: 10px; border-radius: 6px; font-weight: bold; cursor: pointer;
  }
  button:disabled { opacity: 0.6; cursor: not-allowed; }
  .full-width { width: 100%; }
</style>
