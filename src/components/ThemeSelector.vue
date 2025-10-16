<script setup lang="ts">
import { useTheme, type Theme } from '@/stores/themeStore';

const { theme, setTheme } = useTheme();

const themeOptions: { value: Theme; label: string; icon: string }[] = [
  { value: 'light', label: 'ライト', icon: '☀️' },
  { value: 'dark', label: 'ダーク', icon: '🌙' },
  { value: 'system', label: 'システム設定', icon: '💻' },
];
</script>

<template>
  <div class="theme-selector">
    <button
      v-for="option in themeOptions"
      :key="option.value"
      :class="['theme-button', { active: theme === option.value }]"
      @click="setTheme(option.value)"
      :title="option.label"
    >
      <span class="icon">{{ option.icon }}</span>
      <span class="label">{{ option.label }}</span>
    </button>
  </div>
</template>

<style scoped>
.theme-selector {
  display: flex;
  gap: 0.25rem;
  background-color: var(--bg-sunken);
  padding: 0.25rem;
  border-radius: 6px;
}

.theme-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 0.75rem;
  background-color: transparent;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  color: var(--text-secondary);
  font-size: 0.875rem;
  transition: all 0.2s;
}

.theme-button:hover {
  background-color: var(--bg-hover);
  color: var(--text-primary);
}

.theme-button.active {
  background-color: var(--bg-surface);
  color: var(--text-primary);
  box-shadow: var(--shadow-xs);
}

.theme-button .icon {
  font-size: 1rem;
}

.theme-button .label {
  white-space: nowrap;
}
</style>
