<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { useI18n } from 'vue-i18n';

import { useLocaleStore } from '@/stores/localeStore';
import type { Locale } from '@/types';

const { t } = useI18n();
const localeStore = useLocaleStore();
const { locale } = storeToRefs(localeStore);
const { setLocale } = localeStore;

function changeLocale(newLocale: Locale) {
  setLocale(newLocale);
}
</script>

<template>
  <div class="language-selector">
    <button
      :class="['language-button', { active: locale === 'ja' }]"
      :title="t('settings.languages.ja')"
      @click="changeLocale('ja')"
    >
      <span class="label">{{ t('settings.languages.ja') }}</span>
    </button>
    <button
      :class="['language-button', { active: locale === 'en' }]"
      :title="t('settings.languages.en')"
      @click="changeLocale('en')"
    >
      <span class="label">{{ t('settings.languages.en') }}</span>
    </button>
  </div>
</template>

<style scoped lang="scss">
.language {
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
    justify-content: center;
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

    .label {
      white-space: nowrap;
    }
  }
}
</style>
