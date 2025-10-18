import { createI18n } from 'vue-i18n';
import ja from './locales/ja';
import en from './locales/en';
import type { Locale } from '@/types';

export const i18n = createI18n({
  legacy: false,
  locale: 'ja',
  fallbackLocale: 'en',
  messages: {
    ja,
    en,
  },
});

export function setI18nLocale(locale: Locale) {
  i18n.global.locale.value = locale;
}
