/**
 * localStorage キーの一元管理
 *
 * 全てのキーは "VRCJournal-*" で統一
 */

export const STORAGE_KEYS = {
  THEME: 'VRCJournal-theme',
  LOCALE: 'VRCJournal-locale',
} as const;
