<script>
  import { page } from '$app/stores';
</script>

<div class="layout-container">
  <nav class="main-nav">
    <div class="nav-content">
      <div class="brand">Nazgul Voting</div>
      <div class="links">
        <a href="/" class:active={$page.url.pathname === '/'}>🗳️ 投票大厅</a>
        <a href="/admin" class:active={$page.url.pathname === '/admin'}>🔧 管理员后台</a>
      </div>
    </div>
  </nav>

  <slot />
</div>

<style>
  :global(:root) {
    --bg: #1a1b26;
    --bg-dark: #16161e;
    --surface: #24283b;
    --border: #414868;
    --primary: #7aa2f7;
    --secondary: #bb9af7;
    --accent: #e0af68;
    --success: #9ece6a;
    --error: #f7768e;
    --text: #c0caf5;
    --muted: #565f89;
  }

  :global(body) {
    background-color: var(--bg);
    color: var(--text);
    font-family: 'Inter', 'JetBrains Mono', sans-serif;
    margin: 0;
  }

  .main-nav {
    background: var(--bg-dark);
    border-bottom: 1px solid var(--border);
    padding: 0 20px;
  }

  .nav-content {
    max-width: 1200px;
    margin: 0 auto;
    display: flex;
    justify-content: space-between;
    align-items: center;
    height: 60px;
  }

  .brand {
    font-weight: 800;
    color: var(--primary);
    font-size: 1.2rem;
  }

  .links a {
    color: var(--muted);
    text-decoration: none;
    margin-left: 20px;
    padding: 8px 12px;
    border-radius: 6px;
    transition: all 0.2s;
    font-weight: 500;
  }

  .links a:hover {
    color: var(--text);
    background: rgba(255,255,255,0.05);
  }

  .links a.active {
    color: var(--bg);
    background: var(--primary);
  }
</style>
