<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  export let isLoading = false;
  export let errorMessage = '';

  const dispatch = createEventDispatcher();

  let email = '';
  let password = '';
  let rememberMe = false;

  function handleSubmit(e: Event) {
    e.preventDefault();
    if (!email || !password) return;

    dispatch('submit', { email, password });
  }

  function handleWechatLogin() {
    dispatch('switchToWechat');
  }

  function handlePhoneLogin() {
    dispatch('switchToPhone');
  }

  function handleRegister() {
    dispatch('switchToRegister');
  }
</script>

<form on:submit={handleSubmit} class="login-form">
  <div class="form-group">
    <label for="email">邮箱地址</label>
    <input
      id="email"
      type="email"
      bind:value={email}
      placeholder="your@email.com"
      required
      disabled={isLoading}
    />
  </div>

  <div class="form-group">
    <label for="password">密码</label>
    <input
      id="password"
      type="password"
      bind:value={password}
      placeholder="••••••••"
      required
      disabled={isLoading}
    />
  </div>

  {#if errorMessage}
    <div class="error-message">
      <i class="fas fa-exclamation-circle"></i>
      <span>{errorMessage}</span>
    </div>
  {/if}

  <div class="form-options">
    <label class="checkbox">
      <input type="checkbox" bind:checked={rememberMe} disabled={isLoading} />
      <span>记住我</span>
    </label>
    <a href="#" class="forgot-password">忘记密码？</a>
  </div>

  <button type="submit" class="login-button" disabled={isLoading}>
    {#if isLoading}
      <i class="fas fa-spinner fa-spin"></i>
      登录中...
    {:else}
      <i class="fas fa-sign-in-alt"></i>
      登录
    {/if}
  </button>

  <div class="divider">
    <span>或</span>
  </div>

  <div class="social-login">
    <button type="button" class="social-button wechat" on:click={handleWechatLogin} disabled={isLoading}>
      <i class="fab fa-weixin"></i>
      微信扫码登录
    </button>

    <button type="button" class="social-button phone" on:click={handlePhoneLogin} disabled={isLoading}>
      <i class="fas fa-mobile-alt"></i>
      手机验证登录
    </button>
  </div>

  <div class="register-link">
    还没有账号？
    <a href="#" on:click|preventDefault={handleRegister}>立即注册</a>
  </div>

  <div class="third-party-login">
    <p>使用第三方账号登录：</p>
    <div class="third-party-buttons">
      <button type="button" class="third-party-button github" disabled={isLoading}>
        <i class="fab fa-github"></i>
        GitHub
      </button>
      <button type="button" class="third-party-button google" disabled={isLoading}>
        <i class="fab fa-google"></i>
        Google
      </button>
    </div>
  </div>
</form>

<style>
  .login-form {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  label {
    font-size: 14px;
    font-weight: 500;
    color: #cbd5e1;
  }

  input {
    padding: 12px 16px;
    background: rgba(15, 23, 42, 0.6);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 8px;
    color: #f8fafc;
    font-size: 14px;
    transition: border-color 0.2s ease;
  }

  input:focus {
    outline: none;
    border-color: #6d28d9;
  }

  input::placeholder {
    color: #64748b;
  }

  input:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .error-message {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 12px;
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.3);
    border-radius: 8px;
    color: #fca5a5;
    font-size: 14px;
  }

  .error-message i {
    font-size: 16px;
  }

  .form-options {
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 14px;
  }

  .checkbox {
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
    color: #cbd5e1;
  }

  .checkbox input {
    margin: 0;
  }

  .forgot-password {
    color: #6d28d9;
    text-decoration: none;
    transition: color 0.2s ease;
  }

  .forgot-password:hover {
    color: #a855f7;
  }

  .login-button {
    padding: 14px;
    background: linear-gradient(135deg, #6d28d9 0%, #a855f7 100%);
    border: none;
    border-radius: 8px;
    color: white;
    font-size: 16px;
    font-weight: 600;
    cursor: pointer;
    transition: opacity 0.2s ease;
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 10px;
  }

  .login-button:hover:not(:disabled) {
    opacity: 0.9;
  }

  .login-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .divider {
    display: flex;
    align-items: center;
    text-align: center;
    margin: 10px 0;
    color: #64748b;
    font-size: 14px;
  }

  .divider::before,
  .divider::after {
    content: '';
    flex: 1;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  }

  .divider span {
    padding: 0 15px;
  }

  .social-login {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .social-button {
    padding: 12px;
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 8px;
    background: rgba(15, 23, 42, 0.6);
    color: #cbd5e1;
    font-size: 14px;
    cursor: pointer;
    transition: all 0.2s ease;
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 10px;
  }

  .social-button:hover:not(:disabled) {
    background: rgba(255, 255, 255, 0.05);
    border-color: rgba(255, 255, 255, 0.2);
  }

  .social-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .social-button.wechat {
    color: #07c160;
  }

  .social-button.phone {
    color: #3b82f6;
  }

  .register-link {
    text-align: center;
    font-size: 14px;
    color: #94a3b8;
    margin-top: 10px;
  }

  .register-link a {
    color: #6d28d9;
    text-decoration: none;
    font-weight: 600;
    margin-left: 5px;
    transition: color 0.2s ease;
  }

  .register-link a:hover {
    color: #a855f7;
  }

  .third-party-login {
    margin-top: 20px;
    padding-top: 20px;
    border-top: 1px solid rgba(255, 255, 255, 0.1);
  }

  .third-party-login p {
    font-size: 14px;
    color: #94a3b8;
    margin-bottom: 12px;
    text-align: center;
  }

  .third-party-buttons {
    display: flex;
    gap: 12px;
  }

  .third-party-button {
    flex: 1;
    padding: 10px;
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 8px;
    background: rgba(15, 23, 42, 0.6);
    color: #cbd5e1;
    font-size: 14px;
    cursor: pointer;
    transition: all 0.2s ease;
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 8px;
  }

  .third-party-button:hover:not(:disabled) {
    background: rgba(255, 255, 255, 0.05);
  }

  .third-party-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .third-party-button.github {
    color: #f8fafc;
    background: rgba(36, 41, 47, 0.8);
  }

  .third-party-button.google {
    color: #f8fafc;
    background: rgba(219, 68, 55, 0.8);
  }
</style>
