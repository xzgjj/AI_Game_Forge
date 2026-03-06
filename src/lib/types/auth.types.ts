export interface LoginRequest {
  method: 'email' | 'wechat' | 'phone' | 'oauth';
  credentials: Record<string, string>;
}

export interface LoginResponse {
  success: boolean;
  user?: UserInfo;
  token?: string;
  error?: string;
}

export interface UserInfo {
  id: string;
  email?: string;
  phone?: string;
  username: string;
  display_name?: string;
  avatar_url?: string;
  role: UserRole;
  preferences: Record<string, any>;
  created_at: string;
}

export type UserRole = 'guest' | 'user' | 'pro' | 'admin';

export interface AuthSession {
  id: string;
  user_id: string;
  device_id: string;
  device_type: DeviceType;
  token: string;
  refresh_token?: string;
  expires_at: string;
  last_accessed: string;
}

export type DeviceType = 'desktop' | 'mobile' | 'tablet' | 'unknown';

export interface RegisterRequest {
  email: string;
  password: string;
  verification_code: string;
  username?: string;
}

export interface VerificationRequest {
  email?: string;
  phone?: string;
  type: 'register' | 'reset_password' | 'change_email';
}

export interface WechatAuthParams {
  app_id: string;
  redirect_uri: string;
  state?: string;
}

export interface PhoneAuthParams {
  phone: string;
  code: string;
}

export interface OAuthProvider {
  id: string;
  name: string;
  icon: string;
  auth_url: string;
  scopes: string[];
}

export const OAUTH_PROVIDERS: OAuthProvider[] = [
  {
    id: 'github',
    name: 'GitHub',
    icon: 'github',
    auth_url: 'https://github.com/login/oauth/authorize',
    scopes: ['user:email'],
  },
  {
    id: 'google',
    name: 'Google',
    icon: 'google',
    auth_url: 'https://accounts.google.com/o/oauth2/auth',
    scopes: ['profile', 'email'],
  },
];

export interface AuthError {
  code: string;
  message: string;
  details?: Record<string, any>;
}

export const AUTH_ERRORS: Record<string, AuthError> = {
  INVALID_CREDENTIALS: {
    code: 'INVALID_CREDENTIALS',
    message: '邮箱或密码错误',
  },
  USER_NOT_FOUND: {
    code: 'USER_NOT_FOUND',
    message: '用户不存在',
  },
  USER_DISABLED: {
    code: 'USER_DISABLED',
    message: '账户已禁用',
  },
  EMAIL_NOT_VERIFIED: {
    code: 'EMAIL_NOT_VERIFIED',
    message: '邮箱未验证',
  },
  PHONE_NOT_VERIFIED: {
    code: 'PHONE_NOT_VERIFIED',
    message: '手机号未验证',
  },
  INVALID_VERIFICATION_CODE: {
    code: 'INVALID_VERIFICATION_CODE',
    message: '验证码错误或已过期',
  },
  TOO_MANY_ATTEMPTS: {
    code: 'TOO_MANY_ATTEMPTS',
    message: '尝试次数过多，请稍后再试',
  },
  WECHAT_AUTH_FAILED: {
    code: 'WECHAT_AUTH_FAILED',
    message: '微信授权失败',
  },
  OAUTH_FAILED: {
    code: 'OAUTH_FAILED',
    message: '第三方登录失败',
  },
  NETWORK_ERROR: {
    code: 'NETWORK_ERROR',
    message: '网络连接失败',
  },
  SERVER_ERROR: {
    code: 'SERVER_ERROR',
    message: '服务器错误',
  },
};

export interface AuthConfig {
  session_expiry_hours: number;
  max_login_attempts: number;
  lockout_minutes: number;
  require_email_verification: boolean;
  require_phone_verification: boolean;
  enabled_providers: string[];
}

export interface DeviceInfo {
  device_id: string;
  device_type: DeviceType;
  user_agent: string;
  os: string;
  browser: string;
  screen_resolution?: string;
  language: string;
  timezone: string;
}
