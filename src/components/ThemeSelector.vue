<script setup lang="ts">
import { useTheme, type Theme } from '@/stores/themeStore';
import { Sun, Moon, Monitor } from 'lucide-vue-next';

const { theme, setTheme } = useTheme();
</script>

<template>
  <div class="theme-selector">
    <button
      :class="['theme-button', { active: theme === 'light' }]"
      @click="setTheme('light')"
      title="ライト"
    >
      <Sun :size="18" class="icon" />
      <span class="label">ライト</span>
    </button>
    <button
      :class="['theme-button', { active: theme === 'dark' }]"
      @click="setTheme('dark')"
      title="ダーク"
    >
      <Moon :size="18" class="icon" />
      <span class="label">ダーク</span>
    </button>
    <button
      :class="['theme-button', { active: theme === 'system' }]"
      @click="setTheme('system')"
      title="システム設定"
    >
      <Monitor :size="18" class="icon" />
      <span class="label">システム設定</span>
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
  flex-shrink: 0;
}

.theme-button .label {
  white-space: nowrap;
}
</style>
