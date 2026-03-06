import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface User {
  id: string;
  email: string;
  username: string;
  avatar_url?: string;
  role: 'guest' | 'user' | 'pro' | 'admin';
  preferences: Record<string, any>;
}

export interface AuthState {
  isAuthenticated: boolean;
  user: User | null;
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

function createAuthStore() {
  const { subscribe, set, update } = writable<AuthState>(initialState);

  return {
    subscribe,

    // 登录
    async login(email: string, password: string) {
      update(state => ({ ...state, loading: true, error: null }));

      try {
        // TODO: 调用IPC登录接口
        // const result = await invoke('login', { email, password });

        // 模拟成功登录
        const mockUser: User = {
          id: 'user-123',
          email,
          username: email.split('@')[0],
          role: 'user',
          preferences: {},
        };

        set({
          isAuthenticated: true,
          user: mockUser,
          token: 'mock-jwt-token',
          loading: false,
          error: null,
        });

        // 保存到本地存储
        localStorage.setItem('auth_token', 'mock-jwt-token');
      } catch (error: any) {
        update(state => ({
          ...state,
          loading: false,
          error: error.message || '登录失败',
        }));
      }
    },

    // 微信登录
    async wechatLogin() {
      // TODO: 实现微信登录
    },

    // 手机登录
    async phoneLogin(phone: string, code: string) {
      // TODO: 实现手机登录
    },

    // 邮箱注册
    async emailRegister(email: string, password: string, code: string) {
      // TODO: 实现邮箱注册
    },

    // 登出
    async logout() {
      try {
        // TODO: 调用IPC登出接口
        // await invoke('logout');
      } catch (error) {
        console.error('Logout error:', error);
      } finally {
        // 清除本地状态
        localStorage.removeItem('auth_token');
        set(initialState);
      }
    },

    // 检查会话状态
    async checkSession() {
      const token = localStorage.getItem('auth_token');
      if (!token) {
        set(initialState);
        return;
      }

      update(state => ({ ...state, loading: true }));

      try {
        // TODO: 调用IPC验证会话接口
        // const user = await invoke('validate_session', { token });

        // 模拟验证成功
        const mockUser: User = {
          id: 'user-123',
          email: 'user@example.com',
          username: 'user',
          role: 'user',
          preferences: {},
        };

        set({
          isAuthenticated: true,
          user: mockUser,
          token,
          loading: false,
          error: null,
        });
      } catch (error) {
        // 会话无效，清除本地存储
        localStorage.removeItem('auth_token');
        set(initialState);
      }
    },

    // 更新用户信息
    async updateUser(userData: Partial<User>) {
      update(state => {
        if (!state.user) return state;

        return {
          ...state,
          user: { ...state.user, ...userData },
        };
      });

      try {
        // TODO: 调用IPC更新用户信息接口
        // await invoke('update_user', userData);
      } catch (error) {
        console.error('Update user error:', error);
      }
    },

    // 更新用户偏好
    async updatePreferences(preferences: Record<string, any>) {
      update(state => {
        if (!state.user) return state;

        return {
          ...state,
          user: {
            ...state.user,
            preferences: { ...state.user.preferences, ...preferences },
          },
        };
      });

      try {
        // TODO: 调用IPC更新偏好接口
        // await invoke('update_preferences', { preferences });
      } catch (error) {
        console.error('Update preferences error:', error);
      }
    },

    // 清除错误
    clearError() {
      update(state => ({ ...state, error: null }));
    },
  };
}

export const authStore = createAuthStore();

// 应用启动时检查会话
if (typeof window !== 'undefined') {
  authStore.checkSession();
}
