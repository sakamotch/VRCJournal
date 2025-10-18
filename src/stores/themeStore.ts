import { ref, watch } from 'vue';
import { STORAGE_KEYS } from './constants';

export type Theme = 'light' | 'dark' | 'cyberpunk' | 'pastel' | 'aurora' | 'system';

const theme = ref<Theme>((localStorage.getItem(STORAGE_KEYS.THEME) as Theme) || 'system');

function applyTheme(newTheme: Theme) {
  const root = document.documentElement;

  if (newTheme === 'system') {
    root.removeAttribute('data-theme');
  } else {
    root.setAttribute('data-theme', newTheme);
  }
}

watch(theme, (newTheme) => {
  localStorage.setItem(STORAGE_KEYS.THEME, newTheme);
  applyTheme(newTheme);
}, { immediate: true });

export function useTheme() {
  return {
    theme,
    setTheme: (newTheme: Theme) => {
      theme.value = newTheme;
    }
  };
}
