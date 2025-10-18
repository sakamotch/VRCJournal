<script setup lang="ts">
import { Heart, Monitor, Moon, Sparkles,Sun, Zap } from 'lucide-vue-next';
import { storeToRefs } from 'pinia';
import { useI18n } from 'vue-i18n';

import { useThemeStore } from '@/stores/themeStore';

const { t } = useI18n();
const themeStore = useThemeStore();
const { theme } = storeToRefs(themeStore);
const { setTheme } = themeStore;
</script>

<template>
  <div class="theme-selector">
    <button
      :class="['theme-button', { active: theme === 'light' }]"
      :title="t('settings.themes.light')"
      @click="setTheme('light')"
    >
      <Sun
        :size="18"
        class="icon"
      />
      <span class="label">{{ t('settings.themes.light') }}</span>
    </button>
    <button
      :class="['theme-button', { active: theme === 'dark' }]"
      :title="t('settings.themes.dark')"
      @click="setTheme('dark')"
    >
      <Moon
        :size="18"
        class="icon"
      />
      <span class="label">{{ t('settings.themes.dark') }}</span>
    </button>
    <button
      :class="['theme-button', { active: theme === 'cyberpunk' }]"
      :title="t('settings.themes.cyberpunk')"
      @click="setTheme('cyberpunk')"
    >
      <Zap
        :size="18"
        class="icon"
      />
      <span class="label">{{ t('settings.themes.cyberpunk') }}</span>
    </button>
    <button
      :class="['theme-button', { active: theme === 'pastel' }]"
      :title="t('settings.themes.pastel')"
      @click="setTheme('pastel')"
    >
      <Heart
        :size="18"
        class="icon"
      />
      <span class="label">{{ t('settings.themes.pastel') }}</span>
    </button>
    <button
      :class="['theme-button', { active: theme === 'aurora' }]"
      :title="t('settings.themes.aurora')"
      @click="setTheme('aurora')"
    >
      <Sparkles
        :size="18"
        class="icon"
      />
      <span class="label">{{ t('settings.themes.aurora') }}</span>
    </button>
    <button
      :class="['theme-button', { active: theme === 'system' }]"
      :title="t('settings.themes.system')"
      @click="setTheme('system')"
    >
      <Monitor
        :size="18"
        class="icon"
      />
      <span class="label">{{ t('settings.themes.system') }}</span>
    </button>
  </div>
</template>

<style scoped lang="scss">
.theme {
  &-selector {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 0.5rem;
    background: linear-gradient(135deg,
      var(--bg-sunken) 0%,
      color-mix(in srgb, var(--bg-sunken) 97%, var(--accent-primary) 3%) 100%
    );
    padding: 0.5rem;
    border-radius: 8px;
    border: 1px solid color-mix(in srgb, var(--border-subtle) 90%, var(--accent-primary-light) 10%);
  }

  &-button {
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

    &::before {
      content: '';
      position: absolute;
      inset: 0;
      background: linear-gradient(135deg,
        color-mix(in srgb, var(--bg-hover) 93%, var(--accent-primary-light) 7%) 0%,
        color-mix(in srgb, var(--bg-hover) 96%, var(--accent-secondary-light) 4%) 100%
      );
      opacity: 0;
      transition: opacity 0.3s ease;
      border-radius: 6px;
    }

    &:hover {
      color: var(--interactive-default);

      &::before {
        opacity: 1;
      }
    }

    &.active {
      background: linear-gradient(135deg,
        color-mix(in srgb, var(--bg-surface) 88%, var(--accent-primary-light) 12%) 0%,
        color-mix(in srgb, var(--bg-surface) 92%, var(--accent-secondary-light) 8%) 100%
      );
      color: var(--interactive-default);
      font-weight: 600;
      box-shadow: 0 2px 8px color-mix(in srgb, var(--accent-primary) 15%, transparent),
                  0 0 0 1px color-mix(in srgb, var(--border-default) 70%, var(--accent-primary-light) 30%);

      &::before {
        opacity: 0;
      }
    }

    & > * {
      position: relative;
      z-index: 1;
    }

    .icon {
      flex-shrink: 0;
    }

    .label {
      white-space: nowrap;
    }
  }
}
</style>
