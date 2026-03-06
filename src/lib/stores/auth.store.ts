import { writable } from 'svelte/store';
import type { UserInfo } from '$lib/types/auth.types';
import {
  buildSession,
  loginWithEmail,
  loginWithOAuth,
  loginWithPhone,
  loginWithWechat,
  logoutSession,
  registerByEmail,
  validateSession,
} from '$lib/services/auth.service';

export interface AuthState {
  isAuthenticated: boolean;
  user: UserInfo | null;
  token: string | null;
  loading: boolean;
  error: string | null;
}

const initialState: AuthState = {
  isAuthenticated: false,
  user: null,
  token: null,
  loading: false,
  error: null,
};

const TOKEN_KEY = 'auth_token';
const SESSION_KEY = 'auth_session';

function getStorage(): Storage | null {
  if (typeof window === 'undefined') {
    return null;
  }

  return window.localStorage;
}

function persistToken(token: string, userId: string): void {
  const storage = getStorage();

  if (!storage) {
    return;
  }

  storage.setItem(TOKEN_KEY, token);
  storage.setItem(SESSION_KEY, JSON.stringify(buildSession(token, userId)));
}

function clearPersistedToken(): void {
  const storage = getStorage();

  if (!storage) {
    return;
  }

  storage.removeItem(TOKEN_KEY);
  storage.removeItem(SESSION_KEY);
}

function getPersistedToken(): string | null {
  const storage = getStorage();
  return storage ? storage.getItem(TOKEN_KEY) : null;
}

function createAuthStore() {
  const { subscribe, set, update } = writable<AuthState>(initialState);

  async function applyAuthResult(result: { success: boolean; token?: string; user?: UserInfo; error?: string }): Promise<boolean> {
    if (!result.success || !result.token || !result.user) {
      update((state) => ({
        ...state,
        loading: false,
        error: result.error ?? '登录失败，请重试',
      }));

      return false;
    }

    persistToken(result.token, result.user.id);

    set({
      isAuthenticated: true,
      user: result.user,
      token: result.token,
      loading: false,
      error: null,
    });

    return true;
  }

  return {
    subscribe,

    async login(email: string, password: string): Promise<boolean> {
      update((state) => ({ ...state, loading: true, error: null }));
      const result = await loginWithEmail(email, password);
      return applyAuthResult(result);
    },

    async wechatLogin(authCode: string): Promise<boolean> {
      update((state) => ({ ...state, loading: true, error: null }));
      const result = await loginWithWechat(authCode);
      return applyAuthResult(result);
    },

    async phoneLogin(phone: string, code: string): Promise<boolean> {
      update((state) => ({ ...state, loading: true, error: null }));
      const result = await loginWithPhone(phone, code);
      return applyAuthResult(result);
    },

    async emailRegister(email: string, password: string, code: string): Promise<boolean> {
      update((state) => ({ ...state, loading: true, error: null }));
      const result = await registerByEmail(email, password, code);
      return applyAuthResult(result);
    },

    async oauthLogin(provider: string, code: string): Promise<boolean> {
      update((state) => ({ ...state, loading: true, error: null }));
      const result = await loginWithOAuth(provider, code);
      return applyAuthResult(result);
    },

    async logout(): Promise<void> {
      const token = getPersistedToken();
      if (token) {
        await logoutSession(token);
      }

      clearPersistedToken();
      set(initialState);
    },

    async checkSession(): Promise<boolean> {
      const token = getPersistedToken();
      if (!token) {
        set(initialState);
        return false;
      }

      update((state) => ({ ...state, loading: true, error: null }));
      const user = await validateSession(token);

      if (!user) {
        clearPersistedToken();
        set(initialState);
        return false;
      }

      set({
        isAuthenticated: true,
        user,
        token,
        loading: false,
        error: null,
      });

      return true;
    },

    updateUser(userData: Partial<UserInfo>): void {
      update((state) => {
        if (!state.user) {
          return state;
        }

        return {
          ...state,
          user: { ...state.user, ...userData },
        };
      });
    },

    clearError(): void {
      update((state) => ({ ...state, error: null }));
    },
  };
}

export const authStore = createAuthStore();

if (typeof window !== 'undefined') {
  void authStore.checkSession();
}
