<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { get } from 'svelte/store';
  import LoginForm from './LoginForm.svelte';
  import WechatQR from './WechatQR.svelte';
  import PhoneVerify from './PhoneVerify.svelte';
  import EmailRegister from './EmailRegister.svelte';
  import { authStore } from '$lib/stores/auth.store';

  const dispatch = createEventDispatcher<{ loginSuccess: undefined }>();

  let currentMethod: 'form' | 'wechat' | 'phone' | 'email' = 'form';
  let isLoading = false;
  let errorMessage = '';

  function switchMethod(method: typeof currentMethod): void {
    currentMethod = method;
    errorMessage = '';
  }

  async function runAuth(action: () => Promise<boolean>, fallbackError: string): Promise<void> {
    isLoading = true;
    errorMessage = '';

    const success = await action();
    const state = get(authStore);

    if (success && state.isAuthenticated) {
      dispatch('loginSuccess');
    } else {
      errorMessage = state.error ?? fallbackError;
    }

    isLoading = false;
  }

  async function handleEmailLogin(event: CustomEvent<{ email: string; password: string }>): Promise<void> {
    const { email, password } = event.detail;
    await runAuth(() => authStore.login(email, password), '登录失败，请检查邮箱和密码');
  }

  async function handleWechatLogin(event: CustomEvent<{ authCode: string }>): Promise<void> {
    const { authCode } = event.detail;
    await runAuth(() => authStore.wechatLogin(authCode), '微信登录失败');
  }

  async function handlePhoneLogin(event: CustomEvent<{ phone: string; code: string }>): Promise<void> {
    const { phone, code } = event.detail;
    await runAuth(() => authStore.phoneLogin(phone, code), '手机验证码登录失败');
  }

  async function handleEmailRegister(event: CustomEvent<{ email: string; password: string; code: string }>): Promise<void> {
    const { email, password, code } = event.detail;
    await runAuth(() => authStore.emailRegister(email, password, code), '邮箱注册失败');
  }

  async function handleOAuthLogin(event: CustomEvent<{ provider: string }>): Promise<void> {
    const { provider } = event.detail;
    const mockOAuthCode = `${provider}-mock-code`;
    await runAuth(() => authStore.oauthLogin(provider, mockOAuthCode), `${provider} 登录失败`);
  }
</script>

<div class="login-page">
  <div class="login-container">
    <div class="login-header">
      <div class="logo">
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
          on:oauth={handleOAuthLogin}
          {isLoading}
          {errorMessage}
        />
      {:else if currentMethod === 'wechat'}
        <WechatQR
          on:back={() => switchMethod('form')}
          on:submit={handleWechatLogin}
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
    background: rgba(30, 41, 59, 0.85);
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 16px;
    padding: 40px;
    width: 100%;
    max-width: 460px;
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  }

  .login-header {
    text-align: center;
    margin-bottom: 30px;
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
    margin-top: 6px;
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
