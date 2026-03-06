import type { AuthSession, LoginResponse, UserInfo } from '$lib/types/auth.types';
import { safeInvoke } from '$lib/services/tauri.service';
import { isValidEmail, isValidPhone, isStrongPassword, sanitizeText } from '$lib/services/validation.service';

interface TauriLoginRequest {
  method: 'WechatQr' | 'PhoneCode' | 'EmailPassword' | { OAuth: string };
  credentials: Record<string, string>;
}

function nowIso(): string {
  return new Date().toISOString();
}

function mockUser(partial?: Partial<UserInfo>): UserInfo {
  return {
    id: 'mock-user-id',
    email: partial?.email,
    phone: partial?.phone,
    username: partial?.username ?? 'creator',
    display_name: 'Game Creator',
    avatar_url: undefined,
    role: 'user',
    preferences: {},
    created_at: nowIso(),
  };
}

function asLoginResponse(user: UserInfo, token = 'mock-token'): LoginResponse {
  return {
    success: true,
    user,
    token,
  };
}

async function login(request: TauriLoginRequest, fallbackUser: UserInfo): Promise<LoginResponse> {
  return safeInvoke<LoginResponse>(
    'login',
    { request },
    async () => asLoginResponse(fallbackUser),
  );
}

export async function loginWithEmail(email: string, password: string): Promise<LoginResponse> {
  const normalizedEmail = sanitizeText(email);

  if (!isValidEmail(normalizedEmail) || !isStrongPassword(password)) {
    return {
      success: false,
      error: '请输入有效邮箱和至少8位密码',
    };
  }

  return login(
    {
      method: 'EmailPassword',
      credentials: {
        email: normalizedEmail,
        password,
      },
    },
    mockUser({ email: normalizedEmail, username: normalizedEmail.split('@')[0] }),
  );
}

export async function loginWithWechat(authCode: string): Promise<LoginResponse> {
  const code = sanitizeText(authCode);

  if (!code) {
    return {
      success: false,
      error: '微信授权码不能为空',
    };
  }

  return login(
    {
      method: 'WechatQr',
      credentials: {
        auth_code: code,
      },
    },
    mockUser({ username: 'wechat_user' }),
  );
}

export async function loginWithPhone(phone: string, code: string): Promise<LoginResponse> {
  const normalizedPhone = sanitizeText(phone);
  const normalizedCode = sanitizeText(code);

  if (!isValidPhone(normalizedPhone) || normalizedCode.length < 4) {
    return {
      success: false,
      error: '手机号或验证码格式不正确',
    };
  }

  return login(
    {
      method: 'PhoneCode',
      credentials: {
        phone: normalizedPhone,
        code: normalizedCode,
      },
    },
    mockUser({ phone: normalizedPhone, username: 'phone_user' }),
  );
}

export async function loginWithOAuth(provider: string, code: string): Promise<LoginResponse> {
  const normalizedProvider = sanitizeText(provider);
  const normalizedCode = sanitizeText(code);

  if (!normalizedProvider || !normalizedCode) {
    return {
      success: false,
      error: 'OAuth 参数缺失',
    };
  }

  return login(
    {
      method: { OAuth: normalizedProvider },
      credentials: {
        provider: normalizedProvider,
        code: normalizedCode,
      },
    },
    mockUser({ username: `${normalizedProvider}_user` }),
  );
}

export async function registerByEmail(email: string, password: string, verificationCode: string): Promise<LoginResponse> {
  const normalizedEmail = sanitizeText(email);
  const normalizedCode = sanitizeText(verificationCode);

  if (!isValidEmail(normalizedEmail) || !isStrongPassword(password) || normalizedCode.length < 4) {
    return {
      success: false,
      error: '注册信息格式不正确',
    };
  }

  return safeInvoke<LoginResponse>(
    'register_email',
    {
      email: normalizedEmail,
      password,
      verificationCode: normalizedCode,
    },
    async () => asLoginResponse(mockUser({ email: normalizedEmail, username: normalizedEmail.split('@')[0] })),
  );
}

export async function validateSession(token: string): Promise<UserInfo | null> {
  const normalizedToken = sanitizeText(token);

  if (!normalizedToken) {
    return null;
  }

  return safeInvoke<UserInfo | null>(
    'validate_session',
    { token: normalizedToken },
    async () => mockUser({ email: 'user@example.com', username: 'user' }),
  );
}

export async function logoutSession(sessionId: string): Promise<boolean> {
  const normalizedSessionId = sanitizeText(sessionId);

  if (!normalizedSessionId) {
    return true;
  }

  return safeInvoke<boolean>('logout', { sessionId: normalizedSessionId }, async () => true);
}

export function buildSession(token: string, userId: string): AuthSession {
  const now = nowIso();

  return {
    id: `${userId}-session`,
    user_id: userId,
    device_id: 'desktop-device',
    device_type: 'desktop',
    token,
    refresh_token: undefined,
    expires_at: new Date(Date.now() + 24 * 60 * 60 * 1000).toISOString(),
    last_accessed: now,
  };
}
