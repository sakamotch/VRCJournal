export interface LocalUser {
  id: number;
  displayName: string;
  userId: string;
  firstAuthenticatedAt: string;
  lastAuthenticatedAt: string;
}

export interface Session {
  id: number;
  localUserId: number;
  userName: string;
  startedAt: string;
  endedAt: string | null;
  worldId: string;
  worldName: string | null;
  instanceId: string;
  status: string;
  playerCount: number;
  screenshotCount: number;
}

export interface Player {
  id: number;
  displayName: string;
  displayNameAtJoin: string;
  userId: string;
  firstSeenAt: string;
  lastSeenAt: string;
  joinedAt: string;
  leftAt: string | null;
}

export interface Screenshot {
  id: number;
  filePath: string;
  takenAt: string;
  exists: boolean;
}
