<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { userRole } from '$lib/stores';

  onMount(() => {
    // 根据用户角色自动跳转
    const unsubscribe = userRole.subscribe((role) => {
      if (role === 'admin') {
        goto('/admin');
      } else if (role === 'teacher') {
        goto('/teacher');
      } else if (role === 'student') {
        goto('/student');
      }
    });

    return unsubscribe;
  });
</script>

<div class="home">
  <h2>欢迎使用环签名投票系统</h2>
  <p>请先登录以访问您的面板</p>
</div>

<style>
  .home {
    text-align: center;
    padding: 4rem 2rem;
  }

  h2 {
    font-size: 2rem;
    margin: 0 0 1rem 0;
  }

  p {
    color: #666;
  }
</style>
