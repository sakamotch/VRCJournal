import { defineStore } from 'pinia';
import { ref } from 'vue';
import { locale as getSystemLocale } from '@tauri-apps/plugin-os';
import { STORAGE_KEYS } from './constants';
import type { Locale } from '@/types';

async function detectSystemLocale(): Promise<Locale> {
  try {
    const systemLocale = await getSystemLocale();
    return systemLocale?.toLowerCase().startsWith('ja') ? 'ja' : 'en';
  } catch (error) {
    console.warn('Failed to detect system locale, defaulting to ja:', error);
    return 'ja';
  }
}

function getSavedLocale(): Locale | null {
  const saved = localStorage.getItem(STORAGE_KEYS.LOCALE);
  return (saved === 'ja' || saved === 'en') ? saved : null;
}

async function getInitialLocale(): Promise<Locale> {
  return getSavedLocale() ?? await detectSystemLocale();
}

export const useLocaleStore = defineStore('locale', () => {
  // State
  const locale = ref<Locale>('ja');

  // Actions
  function setLocale(newLocale: Locale) {
    locale.value = newLocale;
    localStorage.setItem(STORAGE_KEYS.LOCALE, newLocale);
  }

  async function initLocale() {
    const initialLocale = await getInitialLocale();
    locale.value = initialLocale;
    localStorage.setItem(STORAGE_KEYS.LOCALE, initialLocale);
  }

  return {
    // State
    locale,
    // Actions
    setLocale,
    initLocale,
  };
});
