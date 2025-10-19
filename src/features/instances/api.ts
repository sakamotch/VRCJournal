import { invoke } from '@tauri-apps/api/core';

import type { AvatarChange, Instance, Player, Screenshot } from './types';

/**
 * Instances API - Tauri backend呼び出しを集約
 */

export async function getInstances(localUserId: number, limit: number = 100): Promise<Instance[]> {
  return await invoke<Instance[]>('get_instances', {
    localUserId,
    limit,
  });
}

export async function getInstancePlayers(instanceId: number): Promise<Player[]> {
  return await invoke<Player[]>('get_instance_players', {
    instanceId,
  });
}

export async function getPlayerAvatarHistory(instancePlayerId: number): Promise<AvatarChange[]> {
  return await invoke<AvatarChange[]>('get_player_avatar_history', {
    instancePlayerId,
  });
}

export async function getInstanceAvatarHistories(
  instanceId: number,
): Promise<Record<number, AvatarChange[]>> {
  return await invoke<Record<number, AvatarChange[]>>('get_instance_avatar_histories', {
    instanceId,
  });
}

export async function getInstanceScreenshots(instanceId: number): Promise<Screenshot[]> {
  return await invoke<Screenshot[]>('get_instance_screenshots', {
    instanceId,
  });
}

export async function openInviteUrl(worldId: string, instanceId: string): Promise<string> {
  return await invoke<string>('open_invite_url', {
    worldId,
    instanceId,
  });
}

export async function openUserPage(userId: string): Promise<string> {
  return await invoke<string>('open_user_page', {
    userId,
  });
}

export async function openScreenshotDirectory(filePath: string): Promise<void> {
  await invoke('open_screenshot_directory', { filePath });
}
