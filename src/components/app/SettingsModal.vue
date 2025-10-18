<script setup lang="ts">
import { useI18n } from 'vue-i18n';

import BaseModal from '@/components/base/BaseModal.vue';

import SettingsLanguageSelector from './SettingsLanguageSelector.vue';
import SettingsThemeSelector from './SettingsThemeSelector.vue';

const { t } = useI18n();

interface Props {
  show: boolean;
}

interface Emits {
  (e: 'close'): void;
}

defineProps<Props>();
const emit = defineEmits<Emits>();
</script>

<template>
  <BaseModal
    :show="show"
    :title="t('settings.title')"
    @close="emit('close')"
  >
    <div class="setting-section">
      <h3>{{ t('settings.theme') }}</h3>
      <div class="setting-item">
        <SettingsThemeSelector />
      </div>
    </div>
    <div class="setting-section">
      <h3>{{ t('settings.language') }}</h3>
      <div class="setting-item">
        <SettingsLanguageSelector />
      </div>
    </div>
  </BaseModal>
</template>

<style scoped lang="scss">
.setting {
  &-section {
    margin-bottom: 2rem;

    &:last-child {
      margin-bottom: 0;
    }

    h3 {
      margin: 0 0 1rem;
      font-size: 1rem;
      color: var(--text-secondary);
      font-weight: 600;
    }
  }

  &-item {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;

    label {
      font-size: 0.875rem;
      color: var(--text-primary);
      font-weight: 500;
    }
  }
}
</style>
