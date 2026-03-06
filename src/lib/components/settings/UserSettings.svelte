<script lang="ts">
  interface ProviderOption {
    id: string;
    name: string;
  }

  const providers: ProviderOption[] = [
    { id: 'openai', name: 'OpenAI' },
    { id: 'claude', name: 'Claude' },
    { id: 'zhipu', name: '智谱AI' },
  ];

  let selectedProvider = providers[0].id;
  let monthlyBudget = 300;
  let darkMode = true;
</script>

<section class="panel">
  <h3>用户设置面板</h3>

  <label>
    默认 AI 提供商
    <select bind:value={selectedProvider}>
      {#each providers as provider}
        <option value={provider.id}>{provider.name}</option>
      {/each}
    </select>
  </label>

  <label>
    月度预算上限（USD）
    <input type="number" min="0" bind:value={monthlyBudget} />
  </label>

  <label class="switch">
    <input type="checkbox" bind:checked={darkMode} />
    启用深色外观
  </label>

  <button type="button">保存设置</button>
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

  label {
    display: flex;
    flex-direction: column;
    gap: 6px;
    color: #cbd5e1;
    font-size: 14px;
  }

  select,
  input {
    padding: 8px;
    border-radius: 8px;
    border: 1px solid rgba(255, 255, 255, 0.15);
    background: rgba(30, 41, 59, 0.8);
    color: #f8fafc;
  }

  .switch {
    flex-direction: row;
    align-items: center;
  }

  button {
    align-self: flex-start;
    padding: 8px 14px;
    border-radius: 8px;
    border: 1px solid rgba(255, 255, 255, 0.15);
    background: #6d28d9;
    color: white;
  }
</style>
