import { ref, watch } from 'vue';

export type Theme = 'light' | 'dark' | 'system';

const THEME_STORAGE_KEY = 'vrcjournal-theme';

const theme = ref<Theme>((localStorage.getItem(THEME_STORAGE_KEY) as Theme) || 'system');

function applyTheme(newTheme: Theme) {
  const root = document.documentElement;

  if (newTheme === 'system') {
    root.removeAttribute('data-theme');
  } else {
    root.setAttribute('data-theme', newTheme);
  }
}

watch(theme, (newTheme) => {
  localStorage.setItem(THEME_STORAGE_KEY, newTheme);
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
