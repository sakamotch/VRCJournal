import { createI18n } from 'vue-i18n';

import type { Locale } from '@/types';

import en from './locales/en';
import ja from './locales/ja';

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
