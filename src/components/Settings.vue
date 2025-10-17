<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import ThemeSelector from './ThemeSelector.vue';
import LanguageSelector from './LanguageSelector.vue';
import Modal from './common/Modal.vue';

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
  <Modal :show="show" :title="t('settings.title')" @close="emit('close')">
    <div class="setting-section">
      <h3>{{ t('settings.theme') }}</h3>
      <div class="setting-item">
        <ThemeSelector />
      </div>
    </div>
    <div class="setting-section">
      <h3>{{ t('settings.language') }}</h3>
      <div class="setting-item">
        <LanguageSelector />
      </div>
    </div>
  </Modal>
</template>

<style scoped>
.setting-section {
  margin-bottom: 2rem;
}

.setting-section:last-child {
  margin-bottom: 0;
}

.setting-section h3 {
  margin: 0 0 1rem 0;
  font-size: 1rem;
  color: var(--text-secondary);
  font-weight: 600;
}

.setting-item {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.setting-item label {
  font-size: 0.875rem;
  color: var(--text-primary);
  font-weight: 500;
}
</style>
