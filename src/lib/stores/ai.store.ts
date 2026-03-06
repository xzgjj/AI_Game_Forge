import { writable } from 'svelte/store';
import type { GenerationResult, ProviderUsage } from '$lib/types/ai.types';

interface AIState {
  generating: boolean;
  latestResult: GenerationResult | null;
  usage: ProviderUsage[];
}

const initialState: AIState = {
  generating: false,
  latestResult: null,
  usage: [],
};

function createAIStore() {
  const { subscribe, update, set } = writable<AIState>(initialState);

  return {
    subscribe,

    setGenerating(generating: boolean): void {
      update((state) => ({ ...state, generating }));
    },

    setLatestResult(result: GenerationResult): void {
      update((state) => ({ ...state, latestResult: result }));
    },

    setUsage(usage: ProviderUsage[]): void {
      update((state) => ({ ...state, usage }));
    },

    reset(): void {
      set(initialState);
    },
  };
}

export const aiStore = createAIStore();
