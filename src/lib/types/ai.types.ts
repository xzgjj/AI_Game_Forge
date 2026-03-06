export type ContentType = 'character' | 'scene' | 'dialogue' | 'effect';

export interface GenerationRequest {
  projectId: string;
  contentType: ContentType;
  prompt: string;
  provider?: string;
}

export interface GenerationResult {
  id: string;
  contentType: ContentType;
  content: string;
  provider: string;
  tokens: number;
  createdAt: string;
}

export interface ProviderUsage {
  provider: string;
  requests: number;
  tokens: number;
  cost: number;
}
