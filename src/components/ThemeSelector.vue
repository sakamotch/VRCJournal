<script setup lang="ts">
import { useTheme } from '@/stores/themeStore';
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
  background: linear-gradient(135deg,
    var(--bg-sunken) 0%,
    color-mix(in srgb, var(--bg-sunken) 97%, var(--color-indigo-500) 3%) 100%
  );
  padding: 0.25rem;
  border-radius: 8px;
  border: 1px solid color-mix(in srgb, var(--border-subtle) 90%, var(--color-indigo-400) 10%);
}

.theme-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 0.75rem;
  background-color: transparent;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  color: var(--text-secondary);
  font-size: 0.875rem;
  transition: all 0.3s ease;
  position: relative;
  overflow: hidden;
}

.theme-button::before {
  content: '';
  position: absolute;
  inset: 0;
  background: linear-gradient(135deg,
    color-mix(in srgb, var(--bg-hover) 93%, var(--color-indigo-400) 7%) 0%,
    color-mix(in srgb, var(--bg-hover) 96%, var(--color-violet-400) 4%) 100%
  );
  opacity: 0;
  transition: opacity 0.3s ease;
  border-radius: 6px;
}

.theme-button:hover {
  color: var(--interactive-default);
}

.theme-button:hover::before {
  opacity: 1;
}

.theme-button.active {
  background: linear-gradient(135deg,
    color-mix(in srgb, var(--bg-surface) 88%, var(--color-indigo-400) 12%) 0%,
    color-mix(in srgb, var(--bg-surface) 92%, var(--color-violet-400) 8%) 100%
  );
  color: var(--interactive-default);
  font-weight: 600;
  box-shadow: 0 2px 8px color-mix(in srgb, var(--color-indigo-500) 15%, transparent),
              0 0 0 1px color-mix(in srgb, var(--border-default) 70%, var(--color-indigo-400) 30%);
}

.theme-button.active::before {
  opacity: 0;
}

.theme-button > * {
  position: relative;
  z-index: 1;
}

.theme-button .icon {
  flex-shrink: 0;
}

.theme-button .label {
  white-space: nowrap;
}
</style>
