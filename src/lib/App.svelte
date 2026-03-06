<script lang="ts">
  import { onMount } from 'svelte';
  import { authStore } from '$lib/stores/auth.store';

  import LoginPage from '$lib/components/auth/LoginPage.svelte';
  import Dashboard from '$lib/components/dashboard/Dashboard.svelte';
  import ConfigWizard from '$lib/components/wizard/ConfigWizard.svelte';
  import AICanvas from '$lib/components/canvas/AICanvas.svelte';

  type AppPage = 'login' | 'dashboard' | 'wizard' | 'canvas';

  let currentPage: AppPage = 'login';
  let isLoading = true;

  onMount(async () => {
    const isAuthenticated = await authStore.checkSession();
    currentPage = isAuthenticated ? 'dashboard' : 'login';
    isLoading = false;
  });

  function handleLoginSuccess(): void {
    currentPage = 'dashboard';
  }

  async function handleLogout(): Promise<void> {
    await authStore.logout();
    currentPage = 'login';
  }

  function navigateTo(event: CustomEvent<'wizard' | 'canvas'>): void {
    currentPage = event.detail;
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
      <ConfigWizard on:complete={() => (currentPage = 'canvas')} on:back={() => (currentPage = 'dashboard')} />
    {:else if currentPage === 'canvas'}
      <AICanvas on:back={() => (currentPage = 'dashboard')} />
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
    to {
      transform: rotate(360deg);
    }
  }
</style>
