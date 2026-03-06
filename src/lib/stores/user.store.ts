import { writable } from 'svelte/store';
import type { UserProfile } from '$lib/types/user.types';

interface UserState {
  profile: UserProfile | null;
  loading: boolean;
}

const initialState: UserState = {
  profile: null,
  loading: false,
};

function createUserStore() {
  const { subscribe, update, set } = writable<UserState>(initialState);

  return {
    subscribe,

    setProfile(profile: UserProfile): void {
      update((state) => ({ ...state, profile }));
    },

    setLoading(loading: boolean): void {
      update((state) => ({ ...state, loading }));
    },

    reset(): void {
      set(initialState);
    },
  };
}

export const userStore = createUserStore();
