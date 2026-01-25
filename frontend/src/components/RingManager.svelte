<script lang="ts">
  import { ApiPromise } from '@polkadot/api';
  import { Keyring } from '@polkadot/api';
  import { addLog, addToast } from '$lib/store';
  import { PALLET_NAME } from '$lib/utils'; 
  import { myPublicKey } from '$lib/identity';
  import { onDestroy } from 'svelte';

  export let api: ApiPromise | null;
  export let rings: any[] = [];
  export let fetchRings: () => Promise<void>;

  let newRingKeys: string = '';
  let isSubmitting = false; 
  let timeoutId: any = null;
  let fileInput: HTMLInputElement;

  function addMyKey() {
    if ($myPublicKey) {
      const cleanKey = $myPublicKey.replace(/^0x/, '');
      if (newRingKeys.includes(cleanKey)) {
        addToast("你的公钥已经在列表中了", 'info');
        return;
      }
      // 如果框里有内容，换行追加
      newRingKeys = newRingKeys.trim() ? `${newRingKeys.trim()}\n${cleanKey}` : cleanKey;
    } else {
      addToast("请先在身份管理处导入私钥", 'error');
    }
  }

  // 处理文件上传
  async function handleFileUpload(event: Event) {
    const target = event.target as HTMLInputElement;
    if (target.files && target.files.length > 0) {
      const file = target.files[0];
      try {
        const text = await file.text();
        // 清洗数据：按行分割，去空格，过滤空行
        const keys = text.split('\n')
            .map(k => k.trim())
            .filter(k => k.length > 0);
        
        if (keys.length === 0) {
            addToast("文件内容为空或格式不正确", "error");
            return;
        }

        newRingKeys = keys.join('\n');
        addToast(`成功读取 ${keys.length} 个公钥`, 'success');
      } catch (e) {
        addToast("文件读取失败", 'error');
        console.error(e);
      }
    }
  }

  function resetState() {
    isSubmitting = false;
    if (timeoutId) clearTimeout(timeoutId);
    if (fileInput) fileInput.value = ''; // 重置文件输入框
  }

  async function registerRing() {
    if (!api) return;
    
    const keys = newRingKeys.split('\n').map(k => k.trim()).filter(k => k.length > 0);

    if (keys.length < 2) {
      addToast("为了匿名性，一个环至少需要包含 2 个公钥", 'error');
      return;
    }

    try {
      isSubmitting = true;
      addLog('ADMIN', `正在注册包含 ${keys.length} 个成员的新环...`);
      
      const keyring = new Keyring({ type: 'sr25519' });
      const alice = keyring.addFromUri('//Alice');
      const ringParam = keys.map(k => `0x${k.replace(/^0x/, '')}`);

      timeoutId = setTimeout(() => {
        if (isSubmitting) {
          addLog('WARN', '交易响应超时，重置按钮状态');
          addToast('交易响应超时，请检查日志', 'error');
          resetState();
        }
      }, 30000);

      await api.tx[PALLET_NAME].registerRing(ringParam)
        .signAndSend(alice, async ({ status, dispatchError }) => {
          try {
            if (dispatchError) {
              if (dispatchError.isModule) {
                const decoded = api!.registry.findMetaError(dispatchError.asModule);
                addLog('ERROR', `注册失败: ${decoded.section}.${decoded.name}`);
                addToast(`注册失败: ${decoded.name}`, 'error');
              } else {
                addLog('ERROR', `注册失败: ${dispatchError.toString()}`);
                addToast('注册失败', 'error');
              }
              resetState();
              return;
            }

            if (status.isInBlock) {
              addLog('TX', `交易已打包: ${status.asInBlock}`);
              addToast('环注册成功！正在刷新...', 'success');
              
              setTimeout(async () => {
                  await fetchRings();
                  resetState();
                  newRingKeys = ''; 
              }, 1000);
            } else if (status.isFinalized) {
               addLog('TX', `交易已确认 (Finalized): ${status.asFinalized}`);
               if (isSubmitting) {
                  addToast('环注册已确认！', 'success');
                  await fetchRings();
                  resetState();
                  newRingKeys = ''; 
               }
            }
          } catch (err) {
            console.error(err);
            addLog('ERROR', `Callback error: ${err}`);
          }
        });
        
    } catch (e) {
      addLog('ERROR', `注册环异常: ${e}`);
      addToast(`注册异常: ${e}`, 'error');
      resetState();
    }
  }

  onDestroy(() => { if (timeoutId) clearTimeout(timeoutId); });
</script>

<div class="card ring-manager">
  <div class="card-header">
    <h3>📡 公钥环注册 (Ring Registration)</h3>
  </div>
  
  <div class="card-body">
    <p class="hint">请上传包含多个学生公钥的文件（每行一个 Hex 字符串），或手动粘贴。</p>

    <!-- 文件上传区域 -->
    <div class="upload-area">
      <input 
        type="file" 
        accept=".txt,.csv" 
        on:change={handleFileUpload} 
        bind:this={fileInput}
        disabled={isSubmitting}
        class="file-input"
      />
      <div class="helper-text">支持 .txt 文件，每行一个 64 字符的 Hex 公钥</div>
    </div>

    <div class="input-group">
      <div class="textarea-wrapper">
        <textarea 
            rows="5" 
            bind:value={newRingKeys} 
            placeholder="公钥预览区域 (可手动编辑)..." 
            disabled={isSubmitting}
        ></textarea>
      </div>
      <div class="button-row">
        <button class="text-btn" on:click={addMyKey} disabled={isSubmitting}>+ 将我的公钥加入列表</button>
        <span class="count-badge">当前数量: {newRingKeys.split('\n').filter(k=>k.trim()).length}</span>
      </div>
    </div>
    
    <button 
      class="primary-btn full-width" 
      on:click={registerRing} 
      disabled={!api || isSubmitting}
    >
      {#if isSubmitting}
        <span class="loading-dots">注册上链中...</span>
      {:else}
        提交注册 (Register Ring)
      {/if}
    </button>

    {#if rings.length > 0}
      <div class="ring-list-mini">
        <label>链上已存在的 Ring ID:</label>
        <div class="tags">
          {#each rings as r}
            <div class="ring-tag" title={`包含 ${r.publicKeys.length} 个成员`}>
              <span class="ring-id">ID: {r.id}</span>
              <span class="member-count">👥 {r.publicKeys.length}</span>
            </div>
          {/each}
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  .card {
    background: var(--surface);
    border-radius: 12px;
    border: 1px solid var(--border);
    overflow: hidden;
    margin-bottom: 24px;
    transition: transform 0.2s;
  }
  .card-header {
    background: rgba(0,0,0,0.2);
    padding: 16px 20px;
    border-bottom: 1px solid var(--border);
  }
  .card-body { padding: 20px; }
  
  h3 { margin: 0; color: var(--primary); font-size: 1.1em; font-weight: 600; }
  .hint { font-size: 0.85em; color: var(--muted); margin-bottom: 16px; }

  /* 文件上传样式 */
  .upload-area {
    background: var(--bg-dark);
    border: 1px dashed var(--border);
    padding: 15px;
    border-radius: 8px;
    margin-bottom: 15px;
    text-align: center;
  }
  .file-input { color: var(--text); font-size: 0.9em; }
  .helper-text { font-size: 0.75em; color: var(--muted); margin-top: 5px; }

  textarea {
    width: 100%;
    background: var(--bg-dark);
    border: 1px solid var(--border);
    color: var(--accent); /* 使用强调色显示公钥 */
    padding: 12px;
    border-radius: 8px;
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.8em;
    resize: vertical;
    transition: border-color 0.2s;
  }
  textarea:focus { outline: none; border-color: var(--primary); }

  .button-row { display: flex; justify-content: space-between; align-items: center; margin-top: 8px; margin-bottom: 16px; }
  .text-btn {
    background: transparent;
    color: var(--secondary);
    border: 1px dashed var(--border);
    padding: 4px 8px;
    font-size: 0.8em;
    cursor: pointer;
    border-radius: 4px;
  }
  .text-btn:hover { background: rgba(187, 154, 247, 0.1); border-color: var(--secondary); }
  .count-badge { font-size: 0.8em; color: var(--muted); }

  .primary-btn {
    background: var(--primary);
    color: #1a1b26;
    border: none;
    padding: 12px;
    border-radius: 8px;
    font-weight: 700;
    cursor: pointer;
    transition: filter 0.2s;
  }
  .primary-btn:hover:not(:disabled) { filter: brightness(1.1); }
  .primary-btn:disabled { background: var(--border); color: var(--muted); cursor: not-allowed; }
  .full-width { width: 100%; }

  .ring-list-mini { margin-top: 20px; padding-top: 16px; border-top: 1px dashed var(--border); }
  .tags { display: flex; flex-wrap: wrap; gap: 8px; margin-top: 8px; }
  .ring-tag {
    background: var(--bg-dark);
    border: 1px solid var(--border);
    padding: 4px 10px;
    border-radius: 20px;
    font-size: 0.8em;
    display: flex;
    gap: 6px;
    align-items: center;
  }
  .ring-id { color: var(--success); font-weight: bold; }
</style>
