import type { Session, Player } from "@/types";
import dayjs from "dayjs";
import "dayjs/locale/ja";
import relativeTime from "dayjs/plugin/relativeTime";
import duration from "dayjs/plugin/duration";

// プラグイン設定
dayjs.extend(relativeTime);
dayjs.extend(duration);
dayjs.locale("ja");

export function formatDateTime(dateStr: string): string {
  try {
    return dayjs(dateStr).format("YYYY/MM/DD HH:mm:ss");
  } catch {
    return dateStr;
  }
}

export function formatDate(dateStr: string): string {
  try {
    return dayjs(dateStr).format("YYYY/MM/DD");
  } catch {
    return dateStr;
  }
}

export function formatTime(dateStr: string): string {
  try {
    return dayjs(dateStr).format("HH:mm");
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
    const start = dayjs(session.startedAt);
    const end = dayjs(session.endedAt);
    const diff = end.diff(start);
    const dur = dayjs.duration(diff);

    const hours = Math.floor(dur.asHours());
    const minutes = dur.minutes();

    if (hours > 0) {
      return `${hours}時間${minutes}分`;
    } else {
      return `${minutes}分`;
    }
  } catch {
    return "-";
  }
}

// 相対時刻表示（例: "2時間前"）
export function formatRelativeTime(dateStr: string): string {
  try {
    return dayjs(dateStr).fromNow();
  } catch {
    return dateStr;
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
    const sessionEndTime = dayjs(session.endedAt);
    const playerLeftTime = dayjs(player.leftAt);
    return Math.abs(sessionEndTime.diff(playerLeftTime)) <= 1000;
  } catch {
    return false;
  }
}
