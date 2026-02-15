<script lang="ts">
  import { onMount } from 'svelte';
  import { votingApi, currentBlock } from '$lib/stores';
  import { formatBlockTime, downloadAsFile } from '$lib/utils';
  import { VoteChoice, type UserKeyPair } from '$lib/voting-api';

  // 使用 $state 定义响应式状态
  let activePolls = $state<any[]>([]);
  let votedPolls = $state<any[]>([]);
  let completedPolls = $state<any[]>([]);
  let loading = $state(false);
  let message = $state('');

  // 用户密钥管理
  let userKeyPair = $state<UserKeyPair | null>(null);
  let privateKeyInput = $state('');
  let showKeyImport = $state(false);

  // 投票操作
  let selectedPoll = $state<any>(null);
  let voteChoice = $state<VoteChoice>(VoteChoice.Yes);
  let ringPublicKeys = $state<string[]>([]);
  let secretIndex = $state(0);

  // 导入私钥
  function importPrivateKey() {
    if (!privateKeyInput.trim()) {
      message = '请输入私钥';
      return;
    }

    try {
      const publicKey = $votingApi!.derivePublicKey(privateKeyInput.trim());
      userKeyPair = {
        privateKey: privateKeyInput.trim(),
        publicKey,
      };
      message = '密钥导入成功';
      showKeyImport = false;
      privateKeyInput = '';
      loadPolls();
    } catch (err) {
      message = `导入失败: ${err}`;
    }
  }

  // 生成新密钥
  function generateNewKey() {
    userKeyPair = $votingApi!.generateKeyPair();
    message = '新密钥已生成,请务必保存!';
    // 下载密钥
    downloadAsFile(
      `私钥: ${userKeyPair.privateKey}\n公钥: ${userKeyPair.publicKey}`,
      'my-key.txt'
    );
  }

  // 加载所有投票
  async function loadPolls() {
    try {
      if (!$votingApi) return;
      const count = await $votingApi.getPollCount();
      
      // 使用临时数组收集数据
      const _active: any[] = [];
      const _voted: any[] = [];
      const _completed: any[] = [];

      for (let i = 0; i < count; i++) {
        const poll = await $votingApi.getPoll(i);
        if (!poll) continue;

        // 检查是否已投票
        const hasVoted = userKeyPair
          ? await $votingApi.hasUserVoted(i, userKeyPair.privateKey)
          : false;
        
        const pollWithVoteStatus = { ...poll, hasVoted };

        if (poll.status === 'Active') {
          _active.push(pollWithVoteStatus);
        } else if (poll.status === 'Completed') {
          _completed.push(pollWithVoteStatus);
        }

        if (hasVoted) {
          _voted.push(pollWithVoteStatus);
        }
      }

      // 更新状态
      activePolls = _active;
      votedPolls = _voted;
      completedPolls = _completed;
    } catch (err) {
      console.error('加载投票列表失败:', err);
      message = `加载失败: ${err}`;
    }
  }

  // 准备投票
  async function prepareTovote(poll: any) {
    if (!userKeyPair) {
      message = '请先导入或生成密钥';
      showKeyImport = true;
      return;
    }

    selectedPoll = poll;
    // 获取环签名组
    try {
      // FIX: ring_id -> ringId
      const ring = await $votingApi!.getRing(poll.ringId);
      if (!ring) {
        message = '无法获取环签名组';
        return;
      }

      // 确保所有公钥都转换为纯 Hex 字符串 (处理 Uint8Array 情况)
      ringPublicKeys = ring.map((k) => {
        if (typeof k === 'string') {
          return k.replace(/^0x/, '');
        }
        // Uint8Array 转 Hex String
        return Array.from(k)
          .map((b) => b.toString(16).padStart(2, '0'))
          .join('');
      });

      // 查找用户公钥在环中的位置
      const userPubKey = userKeyPair.publicKey.replace(/^0x/, '');
      secretIndex = ringPublicKeys.findIndex((k) => k === userPubKey);

      if (secretIndex === -1) {
        message = '您不在此投票的环签名组中,无法投票';
        selectedPoll = null;
        return;
      }

      message = `准备就绪,您是环中第 ${secretIndex + 1} 个成员`;
    } catch (err) {
      message = `准备失败: ${err}`;
      selectedPoll = null;
    }
  }

  // 提交投票
  async function submitVote() {
    if (!userKeyPair || !selectedPoll) return;
    loading = true;
    try {
      await $votingApi!.vote({
        pollId: selectedPoll.pollId, // poll_id -> pollId
        choice: voteChoice,
        userPrivateKey: userKeyPair.privateKey,
        ringPublicKeys,
        secretIndex,
      });
      message = '投票成功!您的选票已加密并提交到区块链';
      selectedPoll = null;
      await loadPolls();
    } catch (err) {
      message = `投票失败: ${err}`;
    } finally {
      loading = false;
    }
  }

  // 取消投票
  function cancelVote() {
    selectedPoll = null;
    message = '';
  }

  onMount(() => {
    if ($votingApi) {
      loadPolls();
    }
  });
</script>

<div class="student-page">
  <header>
    <h1>🎓 学生投票界面</h1>
    <div class="stats">
      <span>当前区块: {$currentBlock}</span>
      <span>进行中: {activePolls.length}</span>
      <span>已投票: {votedPolls.length}</span>
    </div>
  </header>

  {#if message}
    <div class="message">{message}</div>
  {/if}

  <!-- 密钥管理 -->
  <section class="card key-management">
    {#if !userKeyPair}
      <h2>🔑 密钥管理</h2>
      <p class="hint">请先导入现有密钥或生成新密钥来参与投票</p>
      
      {#if !showKeyImport}
        <div class="key-actions">
          <button onclick={() => showKeyImport = true}>导入密钥</button>
          <button onclick={generateNewKey} class="primary">生成新密钥</button>
        </div>
      {/if}

      {#if showKeyImport}
        <div class="import-form">
          <input
            type="password"
            bind:value={privateKeyInput}
            placeholder="输入您的私钥 (hex)"
          />
          <div class="form-actions">
            <button onclick={importPrivateKey}>导入</button>
            <button onclick={() => showKeyImport = false}>取消</button>
          </div>
        </div>
      {/if}
    {:else}
      <div class="key-info">
        <h2>🔑 我的密钥</h2>
        <div class="key-display">
          <div class="key-row">
            <span class="label">公钥:</span>
            <code onclick={() => navigator.clipboard.writeText(userKeyPair!.publicKey)}>
              {userKeyPair.publicKey.slice(0, 20)}...
            </code>
            <button onclick={() => navigator.clipboard.writeText(userKeyPair!.publicKey)}>
              复制
            </button>
          </div>
          <div class="key-row">
            <span class="label">私钥:</span>
            <code class="private">••••••••••••••••</code>
            <button onclick={() => navigator.clipboard.writeText(userKeyPair!.privateKey)}>
              复制
            </button>
          </div>
        </div>
        <button onclick={() => { userKeyPair = null; loadPolls(); }} class="danger">
          清除密钥
        </button>
      </div>
    {/if}
  </section>

  <!-- 进行中的投票 -->
  <section class="card">
    <div class="section-header">
      <h2>📊 进行中的投票</h2>
      <button onclick={loadPolls}>刷新</button>
    </div>

    {#if activePolls.length > 0}
      <div class="polls-list">
        {#each activePolls as poll}
          <div class="poll-item" class:voted={poll.hasVoted}>
            <div class="poll-content">
              <div class="poll-main">
                <h3>{poll.description}</h3>
                <div class="poll-meta">
                  <!-- poll_id -> pollId -->
                  <span>ID: {poll.pollId}</span>
                  <span>截止: {formatBlockTime(poll.deadline, $currentBlock)}</span>
                  {#if poll.hasVoted}
                    <span class="voted-badge">✓ 已投票</span>
                  {/if}
                </div>
              </div>

              <div class="poll-action">
                {#if poll.hasVoted}
                  <button disabled>已投票</button>
                {:else if userKeyPair}
                  <button onclick={() => prepareTovote(poll)} class="primary">
                    投票
                  </button>
                {:else}
                  <button disabled>需要密钥</button>
                {/if}
              </div>
            </div>
          </div>
        {/each}
      </div>
    {:else}
      <p class="hint">暂无进行中的投票</p>
    {/if}
  </section>

  <!-- 我的投票记录 -->
  {#if votedPolls.length > 0}
    <section class="card">
      <h2>📝 我的投票记录</h2>
      <div class="voted-list">
        {#each votedPolls as poll}
          <div class="voted-item">
            <div class="voted-info">
              <strong>{poll.description}</strong>
              <span class="status status-{poll.status.toLowerCase()}">
                {poll.status}
              </span>
            </div>
            <div class="voted-meta">
              <span>ID: {poll.pollId}</span>
              <span>已投票 ✓</span>
            </div>
          </div>
        {/each}
      </div>
    </section>
  {/if}

  <!-- 已完成的投票 -->
  {#if completedPolls.length > 0}
    <section class="card">
      <h2>✅ 已完成的投票</h2>
      <div class="completed-list">
        {#each completedPolls as poll}
          <div class="completed-item">
            <div class="completed-header">
              <h3>{poll.description}</h3>
              {#if poll.hasVoted}
                <span class="participated">我参与了此投票</span>
              {/if}
            </div>

            {#if poll.tally}
              <div class="results">
                <div class="result-bar">
                  <div class="result-segment yes" 
                       style="width: {poll.tally.yes / (poll.tally.yes + poll.tally.no + poll.tally.abstain) * 100}%">
                    <span>赞成 {poll.tally.yes}</span>
                  </div>
                  <div class="result-segment no" 
                       style="width: {poll.tally.no / (poll.tally.yes + poll.tally.no + poll.tally.abstain) * 100}%">
                    <span>反对 {poll.tally.no}</span>
                  </div>
                  <div class="result-segment abstain" 
                       style="width: {poll.tally.abstain / (poll.tally.yes + poll.tally.no + poll.tally.abstain) * 100}%">
                    <span>弃权 {poll.tally.abstain}</span>
                  </div>
                </div>

                <div class="result-stats">
                  <div class="stat">
                    <span class="number yes">{poll.tally.yes}</span>
                    <span class="label">赞成</span>
                  </div>
                  <div class="stat">
                    <span class="number no">{poll.tally.no}</span>
                    <span class="label">反对</span>
                  </div>
                  <div class="stat">
                    <span class="number abstain">{poll.tally.abstain}</span>
                    <span class="label">弃权</span>
                  </div>
                </div>
              </div>
            {:else}
              <p class="hint">等待公布结果...</p>
            {/if}
          </div>
        {/each}
      </div>
    </section>
  {/if}
</div>

<!-- 投票模态框 -->
{#if selectedPoll}
  <div class="modal" onclick={cancelVote}>
    <div class="modal-content vote-modal" onclick={(e) => e.stopPropagation()}>
      <button class="close" onclick={cancelVote}>×</button>
      
      <h2>投票: {selectedPoll.description}</h2>

      <div class="vote-info">
        <p><strong>投票 ID:</strong> {selectedPoll.pollId}</p>
        <p><strong>截止时间:</strong> {formatBlockTime(selectedPoll.deadline, $currentBlock)}</p>
        <p><strong>您在环中的位置:</strong> 第 {secretIndex + 1} 个成员</p>
      </div>

      <div class="vote-choice-section">
        <h3>请选择您的立场:</h3>
        
        <div class="choice-buttons">
          <button
            class="choice-btn"
            class:selected={voteChoice === VoteChoice.Yes}
            onclick={() => voteChoice = VoteChoice.Yes}
          >
            <span class="icon">✅</span>
            <span class="text">赞成</span>
          </button>

          <button
            class="choice-btn"
            class:selected={voteChoice === VoteChoice.No}
            onclick={() => voteChoice = VoteChoice.No}
          >
            <span class="icon">❌</span>
            <span class="text">反对</span>
          </button>

          <button
            class="choice-btn"
            class:selected={voteChoice === VoteChoice.Abstain}
            onclick={() => voteChoice = VoteChoice.Abstain}
          >
            <span class="icon">⚪</span>
            <span class="text">弃权</span>
          </button>
        </div>
      </div>

      <div class="privacy-note">
        <p>🔒 <strong>隐私保护说明:</strong></p>
        <ul>
          <li>您的投票将通过环签名技术加密</li>
          <li>系统只能验证您在环中,但无法识别具体身份</li>
          <li>投票内容通过同态加密保护,只有管理员持有解密私钥</li>
          <li>密钥镜像确保一人一票,防止重复投票</li>
        </ul>
      </div>

      <div class="modal-actions">
        <button onclick={cancelVote}>取消</button>
        <button onclick={submitVote} disabled={loading} class="primary">
          {loading ? '提交中...' : '确认投票'}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  /* 样式保持不变 */
  .student-page { max-width: 1200px; margin: 0 auto; padding: 2rem; }
  header { margin-bottom: 2rem; }
  h1 { margin: 0 0 1rem 0; }
  .stats { display: flex; gap: 2rem; color: #666; }
  .card { background: white; border-radius: 8px; padding: 1.5rem; margin-bottom: 1.5rem; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }
  h2 { margin: 0 0 1rem 0; font-size: 1.25rem; }
  .message { padding: 1rem; background: #d1ecf1; border: 1px solid #bee5eb; border-radius: 4px; margin-bottom: 1rem; }
  .hint { color: #666; font-style: italic; }
  /* 密钥管理 */
  .key-management { background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; }
  .key-management h2 { color: white; }
  .key-actions { display: flex; gap: 1rem; margin-top: 1rem; }
  .import-form { margin-top: 1rem; }
  .import-form input { width: 100%; padding: 0.5rem; border: 1px solid #ddd; border-radius: 4px; margin-bottom: 0.5rem; }
  .form-actions { display: flex; gap: 0.5rem; }
  .key-info { color: white; }
  .key-display { background: rgba(255,255,255,0.1); padding: 1rem; border-radius: 4px; margin: 1rem 0; }
  .key-row { display: flex; align-items: center; gap: 1rem; margin: 0.5rem 0; }
  .key-row .label { width: 60px; font-weight: bold; }
  .key-row code { flex: 1; background: rgba(0,0,0,0.2); padding: 0.5rem; border-radius: 4px; cursor: pointer; font-family: monospace; }
  .key-row code.private { letter-spacing: 2px; }
  button { padding: 0.5rem 1rem; background: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer; }
  button:disabled { opacity: 0.5; cursor: not-allowed; }
  button.primary { background: #28a745; }
  button.danger { background: #dc3545; }
  /* 投票列表 */
  .section-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 1rem; }
  .polls-list { display: flex; flex-direction: column; gap: 1rem; }
  .poll-item { border: 2px solid #e0e0e0; border-radius: 8px; padding: 1rem; transition: all 0.2s; }
  .poll-item:hover { border-color: #007bff; box-shadow: 0 4px 8px rgba(0,0,0,0.1); }
  .poll-item.voted { background: #f0f8ff; border-color: #28a745; }
  .poll-content { display: flex; justify-content: space-between; align-items: center; }
  .poll-main { flex: 1; }
  .poll-main h3 { margin: 0 0 0.5rem 0; font-size: 1.1rem; }
  .poll-meta { display: flex; gap: 1rem; color: #666; font-size: 0.875rem; }
  .voted-badge { color: #28a745; font-weight: bold; }
  .poll-action { margin-left: 1rem; }
  /* 已投票列表 */
  .voted-list { display: flex; flex-direction: column; gap: 0.5rem; }
  .voted-item { padding: 1rem; background: #f9f9f9; border-left: 4px solid #28a745; border-radius: 4px; }
  .voted-info { display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.5rem; }
  .voted-meta { display: flex; gap: 1rem; color: #666; font-size: 0.875rem; }
  .status { padding: 0.25rem 0.5rem; border-radius: 4px; font-size: 0.875rem; }
  .status-active { background: #28a745; color: white; }
  .status-tallying { background: #ffc107; color: black; }
  .status-completed { background: #6c757d; color: white; }
  /* 已完成列表 */
  .completed-list { display: flex; flex-direction: column; gap: 1.5rem; }
  .completed-item { border: 1px solid #e0e0e0; border-radius: 8px; padding: 1.5rem; }
  .completed-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 1rem; }
  .completed-header h3 { margin: 0; }
  .participated { padding: 0.25rem 0.5rem; background: #28a745; color: white; border-radius: 4px; font-size: 0.875rem; }
  .results { margin-top: 1rem; }
  .result-bar { display: flex; height: 40px; border-radius: 4px; overflow: hidden; margin-bottom: 1rem; }
  .result-segment { display: flex; align-items: center; justify-content: center; color: white; font-weight: bold; transition: all 0.3s; }
  .result-segment span { font-size: 0.875rem; }
  .result-segment.yes { background: #28a745; }
  .result-segment.no { background: #dc3545; }
  .result-segment.abstain { background: #6c757d; }
  .result-stats { display: flex; justify-content: space-around; margin-top: 1rem; }
  .stat { text-align: center; }
  .stat .number { display: block; font-size: 2rem; font-weight: bold; }
  .stat .number.yes { color: #28a745; }
  .stat .number.no { color: #dc3545; }
  .stat .number.abstain { color: #6c757d; }
  .stat .label { display: block; color: #666; font-size: 0.875rem; margin-top: 0.25rem; }
  /* 模态框 */
  .modal { position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.5); display: flex; align-items: center; justify-content: center; z-index: 1000; }
  .modal-content { background: white; border-radius: 8px; padding: 2rem; max-width: 600px; width: 90%; max-height: 90vh; overflow-y: auto; position: relative; }
  .close { position: absolute; top: 1rem; right: 1rem; background: transparent; color: #666; font-size: 2rem; border: none; cursor: pointer; }
  .vote-info { background: #f9f9f9; padding: 1rem; border-radius: 4px; margin: 1rem 0; }
  .vote-choice-section { margin: 2rem 0; }
  .choice-buttons { display: grid; grid-template-columns: repeat(3, 1fr); gap: 1rem; margin-top: 1rem; }
  .choice-btn { display: flex; flex-direction: column; align-items: center; gap: 0.5rem; padding: 1.5rem 1rem; background: white; border: 2px solid #e0e0e0; border-radius: 8px; cursor: pointer; transition: all 0.2s; }
  .choice-btn:hover { border-color: #007bff; transform: translateY(-2px); box-shadow: 0 4px 8px rgba(0,0,0,0.1); }
  .choice-btn.selected { border-color: #007bff; background: #e7f3ff; }
  .choice-btn .icon { font-size: 2rem; }
  .choice-btn .text { font-weight: bold; }
  .privacy-note { background: #fff3cd; border: 1px solid #ffc107; border-radius: 4px; padding: 1rem; margin: 1rem 0; }
  .privacy-note ul { margin: 0.5rem 0 0 1.5rem; padding: 0; }
  .privacy-note li { margin: 0.25rem 0; font-size: 0.875rem; }
  .modal-actions { display: flex; justify-content: flex-end; gap: 0.5rem; margin-top: 2rem; }
  @media (max-width: 768px) {
    .student-page { padding: 1rem; }
    .choice-buttons { grid-template-columns: 1fr; }
    .stats { flex-direction: column; gap: 0.5rem; }
    .poll-content { flex-direction: column; align-items: flex-start; }
    .poll-action { margin-left: 0; margin-top: 1rem; }
  }
</style>
