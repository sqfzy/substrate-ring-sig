<script lang="ts">
  // import '../app.css';
  import { onMount } from 'svelte';
  import { initWasm } from '$lib/voting-api';
  import ConnectWallet from '$lib/components/ConnectWallet.svelte';
  import { page } from '$app/stores';

  let initialized = $state(false);

  onMount(async () => {
    try {
      await initWasm();
      initialized = true;
    } catch (e) {
      console.error("Failed to initialize WASM:", e);
    }
  });
</script>

<div class="app-container">
  <nav>
    <div class="logo">🗳️ Ring Voting</div>
    <div class="nav-links">
      <a href="/" class:active={$page.url.pathname === '/'}>首页</a>
      <a href="/student" class:active={$page.url.pathname === '/student'}>学生</a>
      <a href="/teacher" class:active={$page.url.pathname === '/teacher'}>教师</a>
      <a href="/admin" class:active={$page.url.pathname === '/admin'}>管理</a>
    </div>
  </nav>

  <!-- 全局钱包连接组件 -->
  <ConnectWallet />

  <main>
    {#if initialized}
      <slot />
    {:else}
      <div class="loading">Loading System (WASM)...</div>
    {/if}
  </main>

  <footer>
    <p>Ring Signature Voting System - Based on Polkadot & Rust</p>
  </footer>
</div>

<style>
  :global(body) {
    margin: 0;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
    background: #f0f2f5;
  }

  .app-container {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
  }

  nav {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1rem 2rem;
    background: #ffffff;
    box-shadow: 0 2px 4px rgba(0,0,0,0.05);
  }

  .logo {
    font-size: 1.5rem;
    font-weight: bold;
    color: #333;
  }

  .nav-links {
    display: flex;
    gap: 1.5rem;
  }

  .nav-links a {
    text-decoration: none;
    color: #666;
    font-weight: 500;
    padding: 0.5rem 0;
    border-bottom: 2px solid transparent;
    transition: all 0.2s;
  }

  .nav-links a:hover {
    color: #007bff;
  }

  .nav-links a.active {
    color: #007bff;
    border-bottom-color: #007bff;
  }

  main {
    flex: 1;
    padding: 0;
  }

  .loading {
    text-align: center;
    margin-top: 2rem;
    font-size: 1.2rem;
    color: #666;
  }

  footer {
    text-align: center;
    padding: 1.5rem;
    color: #888;
    background: #f9f9f9;
    border-top: 1px solid #e0e0e0;
  }
</style>
