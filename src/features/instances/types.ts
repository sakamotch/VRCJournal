export interface Instance {
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
  instancePlayerId: number;
  id: number;
  displayName: string;
  displayNameAtJoin: string;
  userId: string;
  firstSeenAt: string;
  lastSeenAt: string;
  joinedAt: string;
  leftAt: string | null;
  lastAvatarName: string | null;
  avatarChangeCount: number;
}

export interface AvatarChange {
  avatarName: string;
  changedAt: string;
}

export interface Screenshot {
  id: number;
  filePath: string;
  takenAt: string;
  exists: boolean;
}
