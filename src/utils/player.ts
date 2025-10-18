import dayjs from "dayjs";

import type { Instance, Player } from "@/types";

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
