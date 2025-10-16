import type { Session, Player } from "@/types";

export function formatDateTime(dateStr: string): string {
  try {
    const date = new Date(dateStr);
    return date.toLocaleString("ja-JP");
  } catch {
    return dateStr;
  }
}

export function formatDate(dateStr: string): string {
  try {
    const date = new Date(dateStr);
    return date.toLocaleDateString("ja-JP", { year: "numeric", month: "2-digit", day: "2-digit" });
  } catch {
    return dateStr;
  }
}

export function formatTime(dateStr: string): string {
  try {
    const date = new Date(dateStr);
    return date.toLocaleTimeString("ja-JP", { hour: "2-digit", minute: "2-digit" });
  } catch {
    return dateStr;
  }
}

export function formatDuration(session: Session): string {
  // 異常終了（interrupted）の場合
  if (session.status === 'interrupted') {
    return "不明";
  }

  // 進行中の場合
  if (!session.endedAt) {
    return "進行中";
  }

  // 通常の終了時間計算
  try {
    const start = new Date(session.startedAt);
    const end = new Date(session.endedAt);
    const diff = end.getTime() - start.getTime();
    const minutes = Math.floor(diff / 60000);
    const hours = Math.floor(minutes / 60);
    const mins = minutes % 60;

    if (hours > 0) {
      return `${hours}時間${mins}分`;
    } else {
      return `${mins}分`;
    }
  } catch {
    return "-";
  }
}

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
    const sessionEndTime = new Date(session.endedAt).getTime();
    const playerLeftTime = new Date(player.leftAt).getTime();
    return Math.abs(sessionEndTime - playerLeftTime) <= 1000;
  } catch {
    return false;
  }
}
