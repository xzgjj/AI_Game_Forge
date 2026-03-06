<script lang="ts">
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import { authStore } from './stores/auth.store';

  // 导入页面组件
  import LoginPage from './components/auth/LoginPage.svelte';
  import Dashboard from './components/dashboard/Dashboard.svelte';
  import ConfigWizard from './components/wizard/ConfigWizard.svelte';
  import AICanvas from './components/canvas/AICanvas.svelte';

  // 当前页面状态
  let currentPage: 'login' | 'dashboard' | 'wizard' | 'canvas' = 'login';
  let isLoading = true;

  onMount(async () => {
    // 检查用户是否已登录
    await checkAuthStatus();
    isLoading = false;
  });

  async function checkAuthStatus() {
    // TODO: 调用IPC检查会话状态
    const isAuthenticated = false; // 临时值

    if (isAuthenticated) {
      currentPage = 'dashboard';
    } else {
      currentPage = 'login';
    }
  }

  function handleLoginSuccess() {
    currentPage = 'dashboard';
  }

  function handleLogout() {
    currentPage = 'login';
  }

  function navigateTo(page: typeof currentPage) {
    currentPage = page;
  }
</script>

<svelte:head>
  <title>GameCraft AI Studio</title>
</svelte:head>

<div class="app-container">
  {#if isLoading}
    <div class="loading-container">
      <div class="loading-spinner"></div>
      <div class="loading-text">正在初始化应用...</div>
    </div>
  {:else}
    {#if currentPage === 'login'}
      <LoginPage on:loginSuccess={handleLoginSuccess} />
    {:else if currentPage === 'dashboard'}
      <Dashboard on:logout={handleLogout} on:navigate={navigateTo} />
    {:else if currentPage === 'wizard'}
      <ConfigWizard on:complete={() => navigateTo('canvas')} on:back={() => navigateTo('dashboard')} />
    {:else if currentPage === 'canvas'}
      <AICanvas on:back={() => navigateTo('dashboard')} />
    {/if}
  {/if}
</div>

<style>
  .app-container {
    width: 100vw;
    height: 100vh;
    background: linear-gradient(135deg, #0f172a 0%, #1e293b 100%);
    overflow: hidden;
  }

  .loading-container {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    height: 100%;
    gap: 20px;
  }

  .loading-spinner {
    width: 50px;
    height: 50px;
    border: 3px solid rgba(109, 40, 217, 0.3);
    border-top-color: #6d28d9;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  .loading-text {
    color: #cbd5e1;
    font-size: 16px;
    font-weight: 500;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }
</style>
