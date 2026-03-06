export type GameGenre = 'RPG' | 'Adventure' | 'Puzzle' | 'Strategy' | 'Other';
export type ArtStyle = 'Pixel' | 'HandDrawn' | 'Cartoon3D' | 'Realistic' | 'Other';
export type NarrativeTone = 'Light' | 'Epic' | 'Mystery' | 'SciFi' | 'Other';

export interface GameSpec {
  id: string;
  name: string;
  genre: GameGenre;
  artStyle: ArtStyle;
  narrativeTone: NarrativeTone;
  targetPlatforms: string[];
  advancedNotes: string;
  updatedAt: string;
}

export interface ProjectSummary {
  id: string;
  name: string;
  updatedAt: string;
  version: string;
}
