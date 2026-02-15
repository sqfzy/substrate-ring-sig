<script lang="ts">
  import { onMount } from 'svelte';
  import { votingApi, currentUser, currentBlock } from '$lib/stores';
  import { formatAddress, copyToClipboard, downloadAsFile } from '$lib/utils';
  import type { UserKeyPair } from '$lib/voting-api';
  
  let users = $state<UserKeyPair[]>([]);
  let ringId = $state<number | null>(null);
  let teachers = $state<string[]>([]);
  let polls = $state<any[]>([]);
  let loading = $state(false);
  let message = $state('');

  // 生成用户
  let userCount = $state(3);
  function generateUsers() {
    users = [];
    if (!$votingApi) return;
    const _users = [];
    for (let i = 0; i < userCount; i++) {
      _users.push($votingApi.generateKeyPair());
    }
    users = _users;
    message = `生成了 ${userCount} 个用户密钥对`;
  }

  // 导出用户密钥
  function exportUsers() {
    const content = users.map((u, i) => 
      `用户 ${i + 1}:\n私钥: ${u.privateKey}\n公钥: ${u.publicKey}\n`
    ).join('\n');
    downloadAsFile(content, 'users.txt');
    message = '用户密钥已导出';
  }

  // 注册环签名组
  async function registerRing() {
    if (users.length === 0) {
      message = '请先生成用户';
      return;
    }

    loading = true;
    try {
      const publicKeys = users.map(u => u.publicKey);
      ringId = await $votingApi!.registerRing($currentUser!, publicKeys);
      message = `环签名组已注册 (ID: ${ringId})`;
      await loadPolls();
    } catch (err) {
      message = `注册失败: ${err}`;
    } finally {
      loading = false;
    }
  }

  // 授权教师
  let teacherAddress = $state('');
  async function authorizeTeacher() {
    if (!teacherAddress.trim()) {
      message = '请输入教师地址';
      return;
    }

    loading = true;
    try {
      await $votingApi!.authorizeTeacher($currentUser!, teacherAddress.trim());
      // Reactively update array
      teachers = [...teachers, teacherAddress.trim()];
      message = `已授权教师: ${formatAddress(teacherAddress)}`;
      teacherAddress = '';
    } catch (err) {
      message = `授权失败: ${err}`;
    } finally {
      loading = false;
    }
  }

  // 撤销教师
  async function revokeTeacher(address: string) {
    loading = true;
    try {
      await $votingApi!.revokeTeacher($currentUser!, address);
      teachers = teachers.filter(t => t !== address);
      message = `已撤销教师: ${formatAddress(address)}`;
    } catch (err) {
      message = `撤销失败: ${err}`;
    } finally {
      loading = false;
    }
  }

  // 触发计票
  async function startTallying(pollId: number) {
    loading = true;
    try {
      await $votingApi!.startTallying($currentUser!, pollId);
      message = `投票 ${pollId} 已进入计票状态`;
      await loadPolls();
    } catch (err) {
      message = `触发计票失败: ${err}`;
    } finally {
      loading = false;
    }
  }

  // 强制更改所有权
  let transferPollId = $state(0);
  let newOwner = $state('');
  async function changeOwner() {
    if (!newOwner.trim()) {
      message = '请输入新所有者地址';
      return;
    }

    loading = true;
    try {
      const tx = $votingApi!.baseApi.changeOwner(transferPollId, newOwner.trim());
      await $votingApi!.baseApi.submitAndWait(tx, $currentUser!);
      message = `投票 ${transferPollId} 所有权已转移`;
      newOwner = '';
      await loadPolls();
    } catch (err) {
      message = `转移失败: ${err}`;
    } finally {
      loading = false;
    }
  }

  // 加载投票列表
  async function loadPolls() {
    try {
      if (!$votingApi) return;
      const count = await $votingApi.getPollCount();
      const _polls = [];
      for (let i = 0; i < count; i++) {
        const poll = await $votingApi.getPoll(i);
        if (poll) _polls.push(poll);
      }
      polls = _polls;
    } catch (err) {
      console.error('加载投票列表失败:', err);
    }
  }

  onMount(() => {
    if ($votingApi) {
      loadPolls();
    }
  });
</script>

<div class="admin-page">
  <!-- 内容与样式保持不变，省略以节省空间 -->
  <header>
    <h1>🔐 管理员控制面板</h1>
    <div class="stats">
      <span>当前区块: {$currentBlock}</span>
      <span>环签名组: {ringId ?? '未注册'}</span>
      <span>投票数: {polls.length}</span>
    </div>
  </header>

  {#if message}
    <div class="message">{message}</div>
  {/if}

  <!-- 1. 用户管理 -->
  <section class="card">
    <h2>👥 用户管理</h2>
    <div class="form-row">
      <input type="number" bind:value={userCount} min="1" max="100" />
      <button onclick={generateUsers}>生成用户</button>
      {#if users.length > 0}
        <button onclick={exportUsers}>导出密钥</button>
      {/if}
    </div>

    {#if users.length > 0}
      <div class="users-list">
        <p>已生成 {users.length} 个用户:</p>
        {#each users as user, i}
          <div class="user-item">
            <span class="user-label">用户 {i + 1}</span>
            <code onclick={() => copyToClipboard(user.publicKey)}>
              {user.publicKey.slice(0, 16)}...
            </code>
          </div>
        {/each}
      </div>
    {/if}
  </section>

  <!-- 2. 环签名组注册 -->
  <section class="card">
    <h2>🔗 环签名组</h2>
    {#if ringId !== null}
      <p class="success">环签名组 ID: {ringId} (包含 {users.length} 个用户)</p>
    {:else if users.length > 0}
      <button onclick={registerRing} disabled={loading}>
        {loading ? '注册中...' : '注册环签名组'}
      </button>
    {:else}
      <p class="hint">请先生成用户</p>
    {/if}
  </section>

  <!-- 3. 教师管理 -->
  <section class="card">
    <h2>👨‍🏫 教师管理</h2>
    <div class="form-row">
      <input
        type="text"
        bind:value={teacherAddress}
        placeholder="教师地址"
      />
      <button onclick={authorizeTeacher} disabled={loading}>
        授权教师
      </button>
    </div>

    {#if teachers.length > 0}
      <div class="teachers-list">
        <p>已授权教师:</p>
        {#each teachers as teacher}
          <div class="teacher-item">
            <code>{formatAddress(teacher)}</code>
            <button onclick={() => revokeTeacher(teacher)}>撤销</button>
          </div>
        {/each}
      </div>
    {/if}
  </section>

  <!-- 4. 投票监控 -->
  <section class="card">
    <h2>📊 投票监控</h2>
    <button onclick={loadPolls}>刷新</button>

    {#if polls.length > 0}
      <table>
        <thead>
          <tr>
            <th>ID</th>
            <th>描述</th>
            <th>状态</th>
            <th>截止区块</th>
            <th>所有者</th>
            <th>操作</th>
          </tr>
        </thead>
        <tbody>
          {#each polls as poll}
            <tr>
              <td>{poll.pollId}</td> <!-- poll_id -> pollId -->
              <td>{poll.description}</td>
              <td>
                <span class="status status-{poll.status.toLowerCase()}">
                  {poll.status}
                </span>
              </td>
              <td>{poll.deadline}</td>
              <td>{formatAddress(poll.owner)}</td>
              <td>
                {#if poll.status === 'Active'}
                  <button onclick={() => startTallying(poll.pollId)}>
                    触发计票
                  </button>
                {/if}
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    {:else}
      <p class="hint">暂无投票</p>
    {/if}
  </section>

  <!-- 5. 紧急操作 -->
  <section class="card danger">
    <h2>⚠️ 紧急操作</h2>
    <div class="form-row">
      <input type="number" bind:value={transferPollId} placeholder="投票 ID" />
      <input type="text" bind:value={newOwner} placeholder="新所有者地址" />
      <button onclick={changeOwner} disabled={loading} class="danger-btn">
        转移所有权
      </button>
    </div>
  </section>
</div>

<style>
  /* 样式保持不变 */
  .admin-page { max-width: 1200px; margin: 0 auto; padding: 2rem; }
  header { margin-bottom: 2rem; }
  h1 { margin: 0 0 1rem 0; }
  .stats { display: flex; gap: 2rem; color: #666; }
  .card { background: white; border-radius: 8px; padding: 1.5rem; margin-bottom: 1.5rem; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }
  .card.danger { border: 2px solid #dc3545; }
  h2 { margin: 0 0 1rem 0; font-size: 1.25rem; }
  .form-row { display: flex; gap: 0.5rem; margin-bottom: 1rem; }
  input { flex: 1; padding: 0.5rem; border: 1px solid #ddd; border-radius: 4px; }
  button { padding: 0.5rem 1rem; background: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer; }
  button:disabled { opacity: 0.5; cursor: not-allowed; }
  button.danger-btn { background: #dc3545; }
  .message { padding: 1rem; background: #d1ecf1; border: 1px solid #bee5eb; border-radius: 4px; margin-bottom: 1rem; }
  .success { color: #28a745; font-weight: bold; }
  .hint { color: #666; font-style: italic; }
  .users-list, .teachers-list { margin-top: 1rem; }
  .user-item, .teacher-item { display: flex; align-items: center; gap: 1rem; padding: 0.5rem; background: #f9f9f9; margin: 0.5rem 0; border-radius: 4px; }
  code { font-family: monospace; cursor: pointer; }
  code:hover { background: #e0e0e0; }
  table { width: 100%; border-collapse: collapse; margin-top: 1rem; }
  th, td { padding: 0.75rem; text-align: left; border-bottom: 1px solid #ddd; }
  th { background: #f9f9f9; font-weight: bold; }
  .status { padding: 0.25rem 0.5rem; border-radius: 4px; font-size: 0.875rem; }
  .status-active { background: #28a745; color: white; }
  .status-tallying { background: #ffc107; color: black; }
  .status-completed { background: #6c757d; color: white; }
</style>
