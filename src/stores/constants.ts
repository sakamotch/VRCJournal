/**
 * localStorage キーの一元管理
 *
 * 全てのキーは "VRCJournal-" プレフィックスで統一
 */

export const STORAGE_KEYS = {
  /** テーマ設定 */
  THEME: 'VRCJournal-theme',

  /** 言語設定 */
  LOCALE: 'VRCJournal-locale',
} as const;
