import { ref } from 'vue';
import { STORAGE_KEYS } from './constants';
import type { Theme } from '@/types';

function getSavedTheme(): Theme {
  const saved = localStorage.getItem(STORAGE_KEYS.THEME);
  return (
    saved === 'light' ||
    saved === 'dark' ||
    saved === 'cyberpunk' ||
    saved === 'pastel' ||
    saved === 'aurora' ||
    saved === 'system'
  ) ? saved : 'system';
}

function applyTheme(newTheme: Theme) {
  const root = document.documentElement;

  if (newTheme === 'system') {
    root.removeAttribute('data-theme');
  } else {
    root.setAttribute('data-theme', newTheme);
  }
}

const theme = ref<Theme>('system');

export function useTheme() {
  return {
    theme,
    setTheme: (newTheme: Theme) => {
      theme.value = newTheme;
      localStorage.setItem(STORAGE_KEYS.THEME, newTheme);
      applyTheme(newTheme);
    },
    initTheme: () => {
      const savedTheme = getSavedTheme();
      theme.value = savedTheme;
      applyTheme(savedTheme);
    }
  };
}
