<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher<{
    back: undefined;
    submit: { authCode: string };
  }>();

  let authCode = 'wechat-mock-auth-code';

  function handleSubmit(): void {
    dispatch('submit', { authCode });
  }
</script>

<div class="panel">
  <h2>微信扫码登录</h2>
  <p>请使用微信扫描二维码，或使用测试授权码继续。</p>

  <div class="qr-placeholder">
    <span>QR</span>
  </div>

  <input type="text" bind:value={authCode} placeholder="输入授权码（测试）" />

  <div class="actions">
    <button type="button" class="primary" on:click={handleSubmit}>完成登录</button>
    <button type="button" on:click={() => dispatch('back')}>返回</button>
  </div>
</div>

<style>
  .panel {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  h2 {
    font-size: 20px;
    color: #f8fafc;
  }

  p {
    font-size: 14px;
    color: #94a3b8;
  }

  .qr-placeholder {
    width: 180px;
    height: 180px;
    margin: 0 auto;
    border: 1px dashed rgba(255, 255, 255, 0.3);
    border-radius: 12px;
    display: grid;
    place-items: center;
    color: #cbd5e1;
  }

  input {
    padding: 10px;
    border-radius: 8px;
    border: 1px solid rgba(255, 255, 255, 0.15);
    background: rgba(15, 23, 42, 0.7);
    color: #f8fafc;
  }

  .actions {
    display: flex;
    gap: 10px;
  }

  button {
    flex: 1;
    padding: 10px;
    border-radius: 8px;
    border: 1px solid rgba(255, 255, 255, 0.2);
    background: rgba(15, 23, 42, 0.7);
    color: #e2e8f0;
  }

  button.primary {
    background: #07c160;
    border-color: transparent;
    color: #0f172a;
    font-weight: 700;
  }
</style>
