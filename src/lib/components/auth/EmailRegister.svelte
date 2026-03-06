<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher<{
    back: undefined;
    submit: { email: string; password: string; code: string };
  }>();

  let email = '';
  let password = '';
  let code = '';

  function handleSubmit(event: Event): void {
    event.preventDefault();
    dispatch('submit', { email, password, code });
  }
</script>

<form class="panel" on:submit={handleSubmit}>
  <h2>邮箱注册</h2>
  <p>输入邮箱、密码和验证码后创建账号。</p>

  <input type="email" bind:value={email} placeholder="new@gamecraft.ai" required />
  <input type="password" bind:value={password} placeholder="至少8位密码" required />
  <input type="text" bind:value={code} placeholder="邮箱验证码" required />

  <div class="actions">
    <button type="submit" class="primary">注册并登录</button>
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
    background: #6d28d9;
    border-color: transparent;
    color: white;
    font-weight: 700;
  }
</style>
