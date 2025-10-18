import type { Player } from "@/types";

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
