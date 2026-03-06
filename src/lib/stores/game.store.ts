import { writable } from 'svelte/store';
import type { GameSpec, ProjectSummary } from '$lib/types/game.types';

interface GameState {
  currentSpec: GameSpec | null;
  projects: ProjectSummary[];
}

const initialState: GameState = {
  currentSpec: null,
  projects: [],
};

function createGameStore() {
  const { subscribe, update, set } = writable<GameState>(initialState);

  return {
    subscribe,

    setCurrentSpec(spec: GameSpec): void {
      update((state) => ({ ...state, currentSpec: spec }));
    },

    setProjects(projects: ProjectSummary[]): void {
      update((state) => ({ ...state, projects }));
    },

    reset(): void {
      set(initialState);
    },
  };
}

export const gameStore = createGameStore();
