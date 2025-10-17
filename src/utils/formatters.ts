import type { Session, Player } from "@/types";
import dayjs from "dayjs";

export function formatPlayerName(player: Player): string {
  // 名前が変更されている場合は「旧名前(新しい名前)」の形式で表示
  if (player.displayNameAtJoin !== player.displayName) {
    return `${player.displayNameAtJoin} (${player.displayName})`;
  }
  return player.displayName;
}

export function isPlayerStayedUntilEnd(player: Player, session: Session): boolean {
  // セッションが進行中の場合はleftAtで判定
  if (!session.endedAt) {
    return player.leftAt === null;
  }

  // プレイヤーのleftAtがnullなら最後まで在席
  if (!player.leftAt) {
    return true;
  }

  // leftAtがセッション終了時刻と1秒以内なら最後まで在席
  try {
    const sessionEndTime = dayjs(session.endedAt);
    const playerLeftTime = dayjs(player.leftAt);
    return Math.abs(sessionEndTime.diff(playerLeftTime)) <= 1000;
  } catch {
    return false;
  }
}
