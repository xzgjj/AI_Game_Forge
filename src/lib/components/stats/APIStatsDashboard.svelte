<script lang="ts">
  import { onMount } from 'svelte';
  import { safeInvoke } from '$lib/services/tauri.service';

  interface ProviderStat {
    name: string;
    requests: number;
    tokens: number;
    cost: number;
  }

  interface UsageStats {
    total_requests: number;
    total_tokens: number;
    total_cost: number;
    by_provider: ProviderStat[];
  }

  let loading = true;
  let stats: UsageStats = {
    total_requests: 0,
    total_tokens: 0,
    total_cost: 0,
    by_provider: [],
  };

  onMount(async () => {
    const result = await safeInvoke<UsageStats>(
      'get_usage_stats',
      { period: 'Week' },
      async () => ({
        total_requests: 42,
        total_tokens: 128000,
        total_cost: 19.76,
        by_provider: [
          { name: 'openai', requests: 20, tokens: 68000, cost: 11.2 },
          { name: 'claude', requests: 14, tokens: 45000, cost: 6.4 },
          { name: 'zhipu', requests: 8, tokens: 15000, cost: 2.16 },
        ],
      }),
    );

    stats = result;
    loading = false;
  });
</script>

<section class="panel">
  <h3>API 统计面板</h3>

  {#if loading}
    <p>正在加载统计数据...</p>
  {:else}
    <div class="kpis">
      <article>
        <small>请求数</small>
        <strong>{stats.total_requests}</strong>
      </article>
      <article>
        <small>Token</small>
        <strong>{stats.total_tokens}</strong>
      </article>
      <article>
        <small>成本</small>
        <strong>${stats.total_cost.toFixed(2)}</strong>
      </article>
    </div>

    <table>
      <thead>
        <tr>
          <th>提供商</th>
          <th>请求数</th>
          <th>Token</th>
          <th>成本</th>
        </tr>
      </thead>
      <tbody>
        {#each stats.by_provider as row (row.name)}
          <tr>
            <td>{row.name}</td>
            <td>{row.requests}</td>
            <td>{row.tokens}</td>
            <td>${row.cost.toFixed(2)}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
</section>

<style>
  .panel {
    display: flex;
    flex-direction: column;
    gap: 12px;
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 12px;
    padding: 16px;
    background: rgba(15, 23, 42, 0.5);
  }

  h3 {
    font-size: 16px;
    color: #f8fafc;
  }

  p {
    color: #94a3b8;
  }

  .kpis {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 10px;
  }

  article {
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 10px;
    padding: 10px;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  small {
    color: #94a3b8;
  }

  strong {
    color: #f8fafc;
  }

  table {
    width: 100%;
    border-collapse: collapse;
  }

  th,
  td {
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
    padding: 8px;
    text-align: left;
    color: #cbd5e1;
    font-size: 13px;
  }
</style>
