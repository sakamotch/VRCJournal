import { defineStore } from 'pinia';
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

export const useThemeStore = defineStore('theme', () => {
  // State
  const theme = ref<Theme>('system');

  // Actions
  function setTheme(newTheme: Theme) {
    theme.value = newTheme;
    localStorage.setItem(STORAGE_KEYS.THEME, newTheme);
    applyTheme(newTheme);
  }

  function initTheme() {
    const savedTheme = getSavedTheme();
    theme.value = savedTheme;
    applyTheme(savedTheme);
  }

  return {
    // State
    theme,
    // Actions
    setTheme,
    initTheme,
  };
});
