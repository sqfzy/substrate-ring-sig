<script lang="ts">
  import { ApiPromise, WsProvider } from '@polkadot/api';
  import { Keyring } from '@polkadot/keyring';
  import { api, votingApi, currentUser, isConnected, currentBlock, userRole } from '$lib/stores';
  import { createVotingAPI } from '$lib/voting-api';
  
  let wsUrl = $state('ws://localhost:9944');
  let mnemonic = $state('');
  let connecting = $state(false);
  let errorMsg = $state('');

  async function connect() {
    connecting = true;
    errorMsg = '';
    try {
      // 连接到节点
      const provider = new WsProvider(wsUrl);
      const apiInstance = await ApiPromise.create({ provider });
      
      api.set(apiInstance);
      votingApi.set(createVotingAPI(apiInstance));
      isConnected.set(true);
      // 订阅区块
      await apiInstance.rpc.chain.subscribeNewHeads((header) => {
        currentBlock.set(header.number.toNumber());
      });
      console.log('Connected to', wsUrl);
    } catch (err) {
      errorMsg = `连接失败: ${err}`;
      console.error(err);
    } finally {
      connecting = false;
    }
  }

  async function login() {
    if (!mnemonic.trim()) {
      errorMsg = '请输入助记词或私钥';
      return;
    }

    try {
      const keyring = new Keyring({ type: 'sr25519' });
      const pair = keyring.addFromUri(mnemonic.trim());
      
      currentUser.set(pair);

      // 检测用户角色
      const votingApiInstance = $votingApi;
      if (votingApiInstance) {
        // 假设 Alice 是管理员
        if (mnemonic.includes('//Alice')) {
          userRole.set('admin');
        } else if (await votingApiInstance.isTeacher(pair.address)) {
          userRole.set('teacher');
        } else {
          userRole.set('student');
        }
      }

      mnemonic = '';
      // 清空输入
    } catch (err) {
      errorMsg = `登录失败: ${err}`;
      console.error(err);
    }
  }

  function disconnect() {
    $api?.disconnect();
    api.set(null);
    votingApi.set(null);
    currentUser.set(null);
    userRole.set(null);
    isConnected.set(false);
  }
</script>

<div class="connect-wallet">
  {#if !$isConnected}
    <div class="connect-form">
      <h3>连接到区块链节点</h3>
      <input
        type="text"
        bind:value={wsUrl}
        placeholder="ws://localhost:9944"
        disabled={connecting}
      />
      <button onclick={connect} disabled={connecting}>
        {connecting ? '连接中...' : '连接'}
      </button>
    </div>
  {:else if !$currentUser}
    <div class="login-form">
      <h3>登录账户</h3>
      <input
        type="password"
        bind:value={mnemonic}
        placeholder="输入助记词或 //Alice"
      />
      <button onclick={login}>登录</button>
      <p class="hint">测试账户: //Alice (管理员), //Bob (教师), //Charlie (学生)</p>
    </div>
  {:else}
    <div class="user-info">
      <span>👤 {$currentUser.address.slice(0, 8)}...</span>
      <span class="role">{$userRole}</span>
      <button onclick={disconnect}>断开</button>
    </div>
  {/if}

  {#if errorMsg}
    <p class="error">{errorMsg}</p>
  {/if}
</div>

<style>
  /* 样式保持不变 */
  .connect-wallet { padding: 1rem; border-bottom: 1px solid #e0e0e0; background: #f9f9f9; }
  .connect-form, .login-form { max-width: 500px; margin: 0 auto; }
  h3 { margin: 0 0 1rem 0; font-size: 1.2rem; }
  input { width: 100%; padding: 0.5rem; margin-bottom: 0.5rem; border: 1px solid #ddd; border-radius: 4px; }
  button { padding: 0.5rem 1rem; background: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer; }
  button:disabled { opacity: 0.5; cursor: not-allowed; }
  button:hover:not(:disabled) { background: #0056b3; }
  .user-info { display: flex; gap: 1rem; align-items: center; justify-content: center; }
  .role { padding: 0.25rem 0.5rem; background: #28a745; color: white; border-radius: 4px; font-size: 0.875rem; }
  .hint { font-size: 0.875rem; color: #666; margin-top: 0.5rem; }
  .error { color: #dc3545; margin-top: 0.5rem; text-align: center; }
</style>
