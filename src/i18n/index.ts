import { createI18n } from 'vue-i18n';
import { locale as getSystemLocale } from '@tauri-apps/plugin-os';
import { setDayjsLocale } from '@/utils/dayjs-config';
import ja from './locales/ja';
import en from './locales/en';

export type Locale = 'ja' | 'en';

const STORAGE_KEY = 'VRCJournal_locale';

function getSavedLocale(): Locale | null {
  const saved = localStorage.getItem(STORAGE_KEY);
  return saved === 'ja' || saved === 'en' ? saved : null;
}

function saveLocale(locale: Locale) {
  localStorage.setItem(STORAGE_KEY, locale);
}

async function detectSystemLocale(): Promise<Locale> {
  try {
    const systemLocale = await getSystemLocale();
    return systemLocale?.toLowerCase().startsWith('ja') ? 'ja' : 'en';
  } catch (error) {
    console.warn('Failed to detect system locale, defaulting to ja:', error);
    return 'ja';
  }
}

export async function getInitialLocale(): Promise<Locale> {
  return getSavedLocale() ?? await detectSystemLocale();
}

export const i18n = createI18n({
  legacy: false,
  locale: 'ja',
  fallbackLocale: 'en',
  messages: {
    ja,
    en,
  },
});

export function setLocale(locale: Locale) {
  i18n.global.locale.value = locale;
  saveLocale(locale);
  setDayjsLocale(locale);
  document.documentElement.lang = locale;
}
