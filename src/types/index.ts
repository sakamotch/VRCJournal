export interface LocalUser {
  id: number;
  displayName: string;
  userId: string;
  firstAuthenticatedAt: string;
  lastAuthenticatedAt: string;
}

export type Locale = 'ja' | 'en';

export type Theme = 'light' | 'dark' | 'cyberpunk' | 'pastel' | 'aurora' | 'system';

export const ALL_USERS = 0 as const;
export type UserFilterId = number;
