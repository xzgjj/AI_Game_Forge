export type UserRole = 'guest' | 'user' | 'pro' | 'admin';

export interface UserPreferences {
  theme: 'system' | 'light' | 'dark';
  locale: string;
  preferredProvider: string;
  monthlyBudget: number;
}

export interface UserProfile {
  id: string;
  username: string;
  email?: string;
  phone?: string;
  role: UserRole;
  avatarUrl?: string;
  preferences: UserPreferences;
}
