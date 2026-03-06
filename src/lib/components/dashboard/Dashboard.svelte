<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import ProjectManager from '$lib/components/project/ProjectManager.svelte';
  import UserSettings from '$lib/components/settings/UserSettings.svelte';
  import APIStatsDashboard from '$lib/components/stats/APIStatsDashboard.svelte';

  type DashboardTab = 'overview' | 'projects' | 'settings' | 'stats';

  const dispatch = createEventDispatcher<{
    logout: undefined;
    navigate: 'wizard' | 'canvas';
  }>();

  let activeTab: DashboardTab = 'overview';

  function goTo(route: 'wizard' | 'canvas'): void {
    dispatch('navigate', route);
  }
</script>

<section class="dashboard">
  <header class="topbar">
    <h2>GameCraft 控制台</h2>
    <div class="actions">
      <button type="button" on:click={() => goTo('wizard')}>配置向导</button>
      <button type="button" on:click={() => goTo('canvas')}>AI 画布</button>
      <button type="button" class="danger" on:click={() => dispatch('logout')}>退出登录</button>
    </div>
  </header>

  <nav class="tabs">
    <button type="button" class:active={activeTab === 'overview'} on:click={() => (activeTab = 'overview')}>总览</button>
    <button type="button" class:active={activeTab === 'projects'} on:click={() => (activeTab = 'projects')}>项目管理</button>
    <button type="button" class:active={activeTab === 'settings'} on:click={() => (activeTab = 'settings')}>用户设置</button>
    <button type="button" class:active={activeTab === 'stats'} on:click={() => (activeTab = 'stats')}>API统计</button>
  </nav>

  <main>
    {#if activeTab === 'overview'}
      <section class="overview">
        <article>
          <h3>创作主路径</h3>
          <p>登录 -> 配置向导 -> AI 生成 -> 人工改稿 -> 版本归档</p>
        </article>
        <article>
          <h3>本周目标</h3>
          <p>完成表示层联调与最小测试，打通认证与统计入口。</p>
        </article>
      </section>
    {:else if activeTab === 'projects'}
      <ProjectManager />
    {:else if activeTab === 'settings'}
      <UserSettings />
    {:else if activeTab === 'stats'}
      <APIStatsDashboard />
    {/if}
  </main>
</section>

<style>
  .dashboard {
    display: grid;
    grid-template-rows: auto auto 1fr;
    height: 100%;
    color: #f8fafc;
  }

  .topbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 20px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
    background: rgba(15, 23, 42, 0.8);
  }

  h2 {
    font-size: 20px;
  }

  .actions {
    display: flex;
    gap: 10px;
  }

  button {
    padding: 8px 12px;
    border-radius: 8px;
    border: 1px solid rgba(255, 255, 255, 0.15);
    color: #cbd5e1;
    background: rgba(15, 23, 42, 0.7);
  }

  button.danger {
    border-color: rgba(239, 68, 68, 0.5);
    color: #fca5a5;
  }

  .tabs {
    display: flex;
    gap: 8px;
    padding: 12px 20px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
    background: rgba(15, 23, 42, 0.5);
  }

  .tabs button.active {
    background: #6d28d9;
    color: #fff;
    border-color: transparent;
  }

  main {
    padding: 16px 20px;
    overflow: auto;
  }

  .overview {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 12px;
  }

  article {
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 12px;
    padding: 14px;
    background: rgba(15, 23, 42, 0.5);
  }

  article h3 {
    margin-bottom: 8px;
    font-size: 16px;
  }

  article p {
    color: #94a3b8;
    font-size: 14px;
  }
</style>
