<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import LoginForm from './LoginForm.svelte';
  import WechatQR from './WechatQR.svelte';
  import PhoneVerify from './PhoneVerify.svelte';
  import EmailRegister from './EmailRegister.svelte';

  export let onLoginSuccess: () => void;

  // 当前显示的登录方式
  let currentMethod: 'form' | 'wechat' | 'phone' | 'email' = 'form';
  let isLoading = false;
  let errorMessage = '';

  function switchMethod(method: typeof currentMethod) {
    currentMethod = method;
    errorMessage = '';
  }

  async function handleEmailLogin(email: string, password: string) {
    isLoading = true;
    errorMessage = '';

    try {
      // TODO: 调用IPC登录接口
      // const result = await invoke('login', { email, password });
      await new Promise(resolve => setTimeout(resolve, 1000)); // 模拟API调用

      // 登录成功
      onLoginSuccess();
    } catch (error: any) {
      errorMessage = error.message || '登录失败，请检查邮箱和密码';
    } finally {
      isLoading = false;
    }
  }

  async function handleWechatLogin() {
    // TODO: 微信扫码登录
  }

  async function handlePhoneLogin(phone: string, code: string) {
    // TODO: 手机验证码登录
  }

  async function handleEmailRegister(email: string, password: string, code: string) {
    // TODO: 邮箱注册
  }
</script>

<div class="login-page">
  <div class="login-container">
    <div class="login-header">
      <div class="logo">
        <i class="fas fa-gamepad"></i>
        <h1>GameCraft AI Studio</h1>
      </div>
      <p class="subtitle">AI协作式游戏生成工具</p>
    </div>

    <div class="login-content">
      {#if currentMethod === 'form'}
        <LoginForm
          on:submit={handleEmailLogin}
          on:switchToWechat={() => switchMethod('wechat')}
          on:switchToPhone={() => switchMethod('phone')}
          on:switchToRegister={() => switchMethod('email')}
          {isLoading}
          {errorMessage}
        />
      {:else if currentMethod === 'wechat'}
        <WechatQR
          on:back={() => switchMethod('form')}
          on:success={onLoginSuccess}
        />
      {:else if currentMethod === 'phone'}
        <PhoneVerify
          on:back={() => switchMethod('form')}
          on:submit={handlePhoneLogin}
        />
      {:else if currentMethod === 'email'}
        <EmailRegister
          on:back={() => switchMethod('form')}
          on:submit={handleEmailRegister}
        />
      {/if}

      <div class="login-footer">
        <p class="version">v0.1.0 Beta</p>
        <p class="copyright">© 2026 GameCraft AI Studio. 保留所有权利。</p>
      </div>
    </div>
  </div>
</div>

<style>
  .login-page {
    width: 100vw;
    height: 100vh;
    display: flex;
    justify-content: center;
    align-items: center;
    background: linear-gradient(135deg, #0f172a 0%, #1e293b 100%);
    padding: 20px;
  }

  .login-container {
    background: rgba(30, 41, 59, 0.8);
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 16px;
    padding: 40px;
    width: 100%;
    max-width: 400px;
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  }

  .login-header {
    text-align: center;
    margin-bottom: 30px;
  }

  .logo {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 12px;
    margin-bottom: 10px;
  }

  .logo i {
    font-size: 32px;
    color: #6d28d9;
  }

  .logo h1 {
    font-size: 24px;
    font-weight: 700;
    background: linear-gradient(135deg, #6d28d9 0%, #a855f7 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .subtitle {
    color: #cbd5e1;
    font-size: 14px;
    opacity: 0.8;
  }

  .login-content {
    margin-bottom: 20px;
  }

  .login-footer {
    text-align: center;
    margin-top: 30px;
    padding-top: 20px;
    border-top: 1px solid rgba(255, 255, 255, 0.1);
  }

  .version {
    color: #94a3b8;
    font-size: 12px;
    margin-bottom: 5px;
  }

  .copyright {
    color: #64748b;
    font-size: 11px;
  }
</style>
