<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher<{
    back: undefined;
    submit: { phone: string; code: string };
  }>();

  let phone = '';
  let code = '';

  function handleSubmit(event: Event): void {
    event.preventDefault();
    dispatch('submit', { phone, code });
  }
</script>

<form class="panel" on:submit={handleSubmit}>
  <h2>手机号验证登录</h2>
  <p>输入手机号和验证码完成登录。</p>

  <input type="tel" bind:value={phone} placeholder="+8613800000000" required />
  <input type="text" bind:value={code} placeholder="短信验证码" required />

  <div class="actions">
    <button type="submit" class="primary">登录</button>
    <button type="button" on:click={() => dispatch('back')}>返回</button>
  </div>
</form>

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
    background: #3b82f6;
    border-color: transparent;
    color: white;
    font-weight: 700;
  }
</style>
