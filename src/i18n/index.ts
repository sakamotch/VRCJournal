import { createI18n } from 'vue-i18n';
import ja from './locales/ja';
import en from './locales/en';

export type { Locale } from '@/stores/localeStore';

export const i18n = createI18n({
  legacy: false,
  locale: 'ja',
  fallbackLocale: 'en',
  messages: {
    ja,
    en,
  },
});
