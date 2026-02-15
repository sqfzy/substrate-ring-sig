<script lang="ts">
  import { onMount } from 'svelte';
  import { votingApi, currentUser, currentBlock } from '$lib/stores';
  import { formatBlockTime, formatAddress } from '$lib/utils';
  import { VoteChoice } from '$lib/voting-api';
  
  // 状态变量升级为 Runes
  let myPolls = $state<any[]>([]);
  let loading = $state(false);
  let message = $state('');

  // 创建投票表单
  let createForm = $state({
    ringId: 0,
    description: '',
    metadata: '',
    durationBlocks: 1000,
  });
  let pollPrivateKey = $state('');

  // 查看投票详情
  let selectedPoll = $state<any>(null);
  let decryptedVotes = $state<any[]>([]);
  let tallyResult = $state<any>(null);
  let pollPrivKeyInput = $state('');

  // 创建投票
  async function createPoll() {
    if (!createForm.description.trim()) {
      message = '请输入投票描述';
      return;
    }

    loading = true;
    try {
      const deadline = $currentBlock + createForm.durationBlocks;
      const { pollId, pollPrivateKey: privKey } = await $votingApi!.createPoll(
        $currentUser!,
        {
          ringId: createForm.ringId,
          description: createForm.description,
          metadata: createForm.metadata || '无额外信息',
          deadline,
        }
      );
      pollPrivateKey = privKey;
      message = `投票已创建 (ID: ${pollId})\n请保存私钥: ${privKey}`;
      
      // 重置表单
      createForm.description = '';
      createForm.metadata = '';
      
      await loadMyPolls();
    } catch (err) {
      message = `创建失败: ${err}`;
    } finally {
      loading = false;
    }
  }

  // 加载我的投票
  async function loadMyPolls() {
    try {
      if (!$votingApi || !$currentUser) return;
      const count = await $votingApi.getPollCount();
      const _myPolls = [];
      
      for (let i = 0; i < count; i++) {
        const poll = await $votingApi.getPoll(i);
        if (poll && poll.owner === $currentUser.address) {
          // 获取投票数
          const votes = await $votingApi.baseApi.getEncryptedVotes(i);
          _myPolls.push({ ...poll, voteCount: votes.length });
        }
      }
      myPolls = _myPolls;
    } catch (err) {
      console.error('加载投票列表失败:', err);
    }
  }

  // 暂停投票
  async function pausePoll(pollId: number) {
    loading = true;
    try {
      await $votingApi!.pausePoll($currentUser!, pollId, '教师暂停');
      message = `投票 ${pollId} 已暂停`;
      await loadMyPolls();
    } catch (err) {
      message = `暂停失败: ${err}`;
    } finally {
      loading = false;
    }
  }

  // 恢复投票
  async function resumePoll(pollId: number) {
    loading = true;
    try {
      await $votingApi!.activePoll($currentUser!, pollId);
      message = `投票 ${pollId} 已恢复`;
      await loadMyPolls();
    } catch (err) {
      message = `恢复失败: ${err}`;
    } finally {
      loading = false;
    }
  }

  // 取消投票
  async function cancelPoll(pollId: number) {
    if (!confirm('确定要取消此投票吗?')) return;
    loading = true;
    try {
      await $votingApi!.cancelPoll($currentUser!, pollId, '教师取消');
      message = `投票 ${pollId} 已取消`;
      await loadMyPolls();
    } catch (err) {
      message = `取消失败: ${err}`;
    } finally {
      loading = false;
    }
  }

  // 查看投票详情
  async function viewPollDetails(poll: any) {
    selectedPoll = poll;
    decryptedVotes = [];
    tallyResult = null;
  }

  // 解密并计票
  async function decryptAndTally() {
    if (!pollPrivKeyInput.trim()) {
      message = '请输入投票私钥';
      return;
    }

    loading = true;
    try {
      const result = await $votingApi!.decryptAndTallyVotes(
        selectedPoll.pollId, // poll_id -> pollId
        pollPrivKeyInput
      );
      decryptedVotes = result.decryptedVotes;
      tallyResult = result.tally;
      message = `成功解密 ${decryptedVotes.length} 张选票`;
    } catch (err) {
      message = `解密失败: ${err}`;
    } finally {
      loading = false;
    }
  }

  // 发布计票结果
  async function publishTally() {
    if (!tallyResult) {
      message = '请先解密并计票';
      return;
    }

    loading = true;
    try {
      await $votingApi!.publishTallyResult(
        $currentUser!,
        selectedPoll.pollId, // poll_id -> pollId
        tallyResult,
        pollPrivKeyInput
      );
      message = '计票结果已发布到链上';
      await loadMyPolls();
      selectedPoll = null;
    } catch (err) {
      message = `发布失败: ${err}`;
    } finally {
      loading = false;
    }
  }

  function getVoteChoiceName(choice: VoteChoice): string {
    switch (choice) {
      case VoteChoice.Yes: return '赞成';
      case VoteChoice.No: return '反对';
      case VoteChoice.Abstain: return '弃权';
      default: return '未知';
    }
  }

  onMount(() => {
    if ($votingApi) {
      loadMyPolls();
    }
  });
</script>

<div class="teacher-page">
  <header>
    <h1>👨‍🏫 教师控制面板</h1>
    <div class="stats">
      <span>当前区块: {$currentBlock}</span>
      <span>我的投票: {myPolls.length}</span>
    </div>
  </header>

  {#if message}
    <div class="message">{message}</div>
  {/if}

  <!-- 创建投票 -->
  <section class="card">
    <h2>📝 创建新投票</h2>
    <div class="create-form">
      <div class="form-group">
        <label>环签名组 ID:</label>
        <input type="number" bind:value={createForm.ringId} />
      </div>

      <div class="form-group">
        <label>投票描述:</label>
        <input
          type="text"
          bind:value={createForm.description}
          placeholder="例如: 是否同意提案 X?"
        />
      </div>

      <div class="form-group">
        <label>详细信息 (可选):</label>
        <textarea
          bind:value={createForm.metadata}
          placeholder="投票的详细说明..."
          rows="3"
        ></textarea>
      </div>

      <div class="form-group">
        <label>持续时间 (区块数):</label>
        <input type="number" bind:value={createForm.durationBlocks} />
        <span class="hint">
          约 {formatBlockTime($currentBlock + createForm.durationBlocks, $currentBlock)}
        </span>
      </div>

      <button onclick={createPoll} disabled={loading}>
        {loading ? '创建中...' : '创建投票'}
      </button>

      {#if pollPrivateKey}
        <div class="private-key-box">
          <p><strong>⚠️ 请保存投票私钥 (仅显示一次):</strong></p>
          <code>{pollPrivateKey}</code>
          <button onclick={() => navigator.clipboard.writeText(pollPrivateKey)}>
            复制
          </button>
        </div>
      {/if}
    </div>
  </section>

  <!-- 我的投票列表 -->
  <section class="card">
    <h2>📊 我的投票</h2>
    <button onclick={loadMyPolls}>刷新</button>

    {#if myPolls.length > 0}
      <div class="polls-grid">
        {#each myPolls as poll}
          <div class="poll-card">
            <div class="poll-header">
              <h3>{poll.description}</h3>
              <span class="status status-{poll.status.toLowerCase()}">
                {poll.status}
              </span>
            </div>

            <div class="poll-info">
              <div class="info-row">
                <span>ID:</span>
                <strong>{poll.pollId}</strong> <!-- poll_id -> pollId -->
              </div>
              <div class="info-row">
                <span>投票数:</span>
                <strong>{poll.voteCount}</strong>
              </div>
              <div class="info-row">
                <span>截止:</span>
                <strong>
                  {formatBlockTime(poll.deadline, $currentBlock)}
                  ({poll.deadline})
                </strong>
              </div>
            </div>

            <div class="poll-actions">
              {#if poll.status === 'Active'}
                <button onclick={() => pausePoll(poll.pollId)}>暂停</button>
                <button onclick={() => cancelPoll(poll.pollId)} class="danger">
                  取消
                </button>
              {:else if poll.status === 'Paused'}
                <button onclick={() => resumePoll(poll.pollId)}>恢复</button>
                <button onclick={() => cancelPoll(poll.pollId)} class="danger">
                  取消
                </button>
              {:else if poll.status === 'Tallying'}
                <button onclick={() => viewPollDetails(poll)}>查看详情</button>
              {:else if poll.status === 'Completed'}
                <button onclick={() => viewPollDetails(poll)}>查看结果</button>
              {/if}
            </div>

            {#if poll.tally}
              <div class="poll-results">
                <p><strong>计票结果:</strong></p>
                <div class="tally">
                  <span>✅ 赞成: {poll.tally.yes}</span>
                  <span>❌ 反对: {poll.tally.no}</span>
                  <span>⚪ 弃权: {poll.tally.abstain}</span>
                </div>
              </div>
            {/if}
          </div>
        {/each}
      </div>
    {:else}
      <p class="hint">暂无投票,点击上方创建新投票</p>
    {/if}
  </section>
</div>

<!-- 投票详情模态框 -->
{#if selectedPoll}
  <div class="modal" onclick={() => selectedPoll = null}>
    <div class="modal-content" onclick={(e) => e.stopPropagation()}>
      <button class="close" onclick={() => selectedPoll = null}>×</button>
      
      <h2>投票详情 - {selectedPoll.description}</h2>

      {#if selectedPoll.status === 'Tallying' && !tallyResult}
        <div class="decrypt-section">
          <h3>解密选票</h3>
          <input
            type="password"
            bind:value={pollPrivKeyInput}
            placeholder="输入投票私钥"
          />
          <button onclick={decryptAndTally} disabled={loading}>
            {loading ? '解密中...' : '解密并计票'}
          </button>
        </div>
      {/if}

      {#if tallyResult}
        <div class="tally-results">
          <h3>计票结果</h3>
          <div class="tally-chart">
            <div class="tally-item">
              <span class="label">✅ 赞成</span>
              <div class="bar">
                <div
                  class="fill yes"
                  style="width: {(tallyResult.yes / decryptedVotes.length * 100)}%"
                ></div>
              </div>
              <span class="count">{tallyResult.yes}</span>
            </div>

            <div class="tally-item">
              <span class="label">❌ 反对</span>
              <div class="bar">
                <div
                  class="fill no"
                  style="width: {(tallyResult.no / decryptedVotes.length * 100)}%"
                ></div>
              </div>
              <span class="count">{tallyResult.no}</span>
            </div>

            <div class="tally-item">
              <span class="label">⚪ 弃权</span>
              <div class="bar">
                <div
                  class="fill abstain"
                  style="width: {(tallyResult.abstain / decryptedVotes.length * 100)}%"
                ></div>
              </div>
              <span class="count">{tallyResult.abstain}</span>
            </div>
          </div>

          <div class="decrypted-votes">
            <h4>已解密选票 ({decryptedVotes.length}):</h4>
            {#each decryptedVotes as vote, i}
              <div class="vote-item">
                <span>#{i + 1}</span>
                <span class="vote-choice">{getVoteChoiceName(vote.choice)}</span>
                <code>{vote.keyImage.slice(0, 16)}...</code> <!-- key_image -> keyImage -->
              </div>
            {/each}
          </div>

          {#if selectedPoll.status === 'Tallying'}
            <button onclick={publishTally} class="primary" disabled={loading}>
              {loading ? '发布中...' : '发布计票结果到链上'}
            </button>
          {/if}
        </div>
      {/if}

      {#if selectedPoll.status === 'Completed' && selectedPoll.tally}
        <div class="completed-results">
          <h3>最终结果</h3>
          <div class="final-tally">
            <div class="tally-stat">
              <span class="number">{selectedPoll.tally.yes}</span>
              <span class="label">赞成</span>
            </div>
            <div class="tally-stat">
              <span class="number">{selectedPoll.tally.no}</span>
              <span class="label">反对</span>
            </div>
            <div class="tally-stat">
              <span class="number">{selectedPoll.tally.abstain}</span>
              <span class="label">弃权</span>
            </div>
          </div>
        </div>
      {/if}
    </div>
  </div>
{/if}

<style>
  /* 样式保持不变 */
  .teacher-page { max-width: 1200px; margin: 0 auto; padding: 2rem; }
  header { margin-bottom: 2rem; }
  h1 { margin: 0 0 1rem 0; }
  .stats { display: flex; gap: 2rem; color: #666; }
  .card { background: white; border-radius: 8px; padding: 1.5rem; margin-bottom: 1.5rem; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }
  h2 { margin: 0 0 1rem 0; font-size: 1.25rem; }
  .message { padding: 1rem; background: #d1ecf1; border: 1px solid #bee5eb; border-radius: 4px; margin-bottom: 1rem; white-space: pre-line; }
  .create-form { max-width: 600px; }
  .form-group { margin-bottom: 1rem; }
  label { display: block; margin-bottom: 0.5rem; font-weight: bold; }
  input, textarea { width: 100%; padding: 0.5rem; border: 1px solid #ddd; border-radius: 4px; }
  button { padding: 0.5rem 1rem; background: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer; margin-right: 0.5rem; }
  button:disabled { opacity: 0.5; cursor: not-allowed; }
  button.danger { background: #dc3545; }
  button.primary { background: #28a745; }
  .hint { color: #666; font-size: 0.875rem; margin-left: 0.5rem; }
  .private-key-box { margin-top: 1rem; padding: 1rem; background: #fff3cd; border: 1px solid #ffc107; border-radius: 4px; }
  .private-key-box code { display: block; padding: 0.5rem; background: white; margin: 0.5rem 0; word-break: break-all; }
  .polls-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(350px, 1fr)); gap: 1rem; margin-top: 1rem; }
  .poll-card { border: 1px solid #ddd; border-radius: 8px; padding: 1rem; }
  .poll-header { display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 1rem; }
  .poll-header h3 { margin: 0; font-size: 1.1rem; flex: 1; }
  .status { padding: 0.25rem 0.5rem; border-radius: 4px; font-size: 0.875rem; }
  .status-active { background: #28a745; color: white; }
  .status-tallying { background: #ffc107; color: black; }
  .status-completed { background: #6c757d; color: white; }
  .status-paused { background: #fd7e14; color: white; }
  .poll-info { margin-bottom: 1rem; }
  .info-row { display: flex; justify-content: space-between; padding: 0.25rem 0; }
  .poll-actions { display: flex; gap: 0.5rem; margin-top: 1rem; }
  .poll-results { margin-top: 1rem; padding-top: 1rem; border-top: 1px solid #ddd; }
  .tally { display: flex; gap: 1rem; margin-top: 0.5rem; }
  .modal { position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.5); display: flex; align-items: center; justify-content: center; z-index: 1000; }
  .modal-content { background: white; border-radius: 8px; padding: 2rem; max-width: 800px; max-height: 90vh; overflow-y: auto; position: relative; }
  .close { position: absolute; top: 1rem; right: 1rem; background: transparent; color: #666; font-size: 2rem; border: none; cursor: pointer; }
  .decrypt-section { margin: 1rem 0; }
  .tally-chart { margin: 1rem 0; }
  .tally-item { display: flex; align-items: center; gap: 1rem; margin: 0.5rem 0; }
  .tally-item .label { width: 80px; }
  .bar { flex: 1; height: 30px; background: #e0e0e0; border-radius: 4px; overflow: hidden; }
  .fill { height: 100%; transition: width 0.3s; }
  .fill.yes { background: #28a745; }
  .fill.no { background: #dc3545; }
  .fill.abstain { background: #6c757d; }
  .count { width: 50px; text-align: right; font-weight: bold; }
  .decrypted-votes { margin-top: 1.5rem; }
  .vote-item { display: flex; gap: 1rem; padding: 0.5rem; background: #f9f9f9; margin: 0.25rem 0; border-radius: 4px; }
  .vote-choice { font-weight: bold; color: #007bff; }
  .completed-results { margin-top: 1rem; }
  .final-tally { display: flex; gap: 2rem; justify-content: center; margin: 2rem 0; }
  .tally-stat { text-align: center; }
  .tally-stat .number { display: block; font-size: 3rem; font-weight: bold; color: #007bff; }
  .tally-stat .label { display: block; color: #666; margin-top: 0.5rem; }
</style>
