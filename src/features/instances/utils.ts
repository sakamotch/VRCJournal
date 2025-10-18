import dayjs from "dayjs";

import type { Instance, Player } from "./types";

/**
 * フォーマットされたプレイヤー名を返す
 *
 * Join時の名前と現在の名前が異なる場合は、両方を表示する。
 * 例: "旧名 (現在の名前)"
 *
 * @param player - プレイヤー情報
 * @returns フォーマットされたプレイヤー名
 *
 * @example
 * ```ts
 * // 名前が変わっていない場合
 * formatPlayerName({ displayName: 'Alice', displayNameAtJoin: 'Alice', ... })
 * // => 'Alice'
 *
 * // 名前が変わっている場合
 * formatPlayerName({ displayName: 'Bob', displayNameAtJoin: 'Alice', ... })
 * // => 'Alice (Bob)'
 * ```
 */
export function formatPlayerName(player: Player): string {
  if (player.displayNameAtJoin !== player.displayName) {
    return `${player.displayNameAtJoin} (${player.displayName})`;
  }
  return player.displayName;
}

/**
 * プレイヤーがインスタンスの最後まで在席していたかを判定
 *
 * 以下の条件でtrueを返す:
 * - インスタンスが進行中で、プレイヤーが退出していない
 * - プレイヤーの退出時刻がnull（記録されていない）
 * - プレイヤーの退出時刻がインスタンス終了時刻と1秒以内の誤差
 *
 * @param player - プレイヤー情報
 * @param instance - インスタンス情報
 * @returns 最後まで在席していた場合true、それ以外false
 */
export function isPlayerStayedUntilEnd(player: Player, instance: Instance): boolean {
  if (!instance.endedAt) {
    return player.leftAt === null;
  }

  if (!player.leftAt) {
    return true;
  }

  try {
    const instanceEndTime = dayjs(instance.endedAt);
    const playerLeftTime = dayjs(player.leftAt);
    return Math.abs(instanceEndTime.diff(playerLeftTime)) <= 1000;
  } catch {
    return false;
  }
}
