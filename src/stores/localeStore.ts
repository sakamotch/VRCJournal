import { ref, watch } from 'vue';
import { locale as getSystemLocale } from '@tauri-apps/plugin-os';
import { setDayjsLocale } from '@/utils/dayjs-config';
import { STORAGE_KEYS } from './constants';

export type Locale = 'ja' | 'en';

// システムロケール検出
async function detectSystemLocale(): Promise<Locale> {
  try {
    const systemLocale = await getSystemLocale();
    return systemLocale?.toLowerCase().startsWith('ja') ? 'ja' : 'en';
  } catch (error) {
    console.warn('Failed to detect system locale, defaulting to ja:', error);
    return 'ja';
  }
}

// 保存されたロケールを取得
function getSavedLocale(): Locale | null {
  const saved = localStorage.getItem(STORAGE_KEYS.LOCALE);
  return saved === 'ja' || saved === 'en' ? saved : null;
}

// 初期ロケールを決定
async function getInitialLocale(): Promise<Locale> {
  return getSavedLocale() ?? await detectSystemLocale();
}

// グローバルなロケール状態
const locale = ref<Locale>('ja');

// 初期化フラグ
let initialized = false;

// ロケール変更を監視してlocalStorageとDOMに反映
watch(locale, (newLocale) => {
  if (initialized) {
    localStorage.setItem(STORAGE_KEYS.LOCALE, newLocale);
    setDayjsLocale(newLocale);
    document.documentElement.lang = newLocale;
  }
});

export function useLocale() {
  return {
    locale,
    setLocale: (newLocale: Locale) => {
      locale.value = newLocale;
    },
    initLocale: async () => {
      const initialLocale = await getInitialLocale();
      locale.value = initialLocale;
      setDayjsLocale(initialLocale);
      document.documentElement.lang = initialLocale;
      initialized = true;
    },
  };
}
