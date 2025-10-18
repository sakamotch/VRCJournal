import { ref } from 'vue';
import { useI18n } from 'vue-i18n';

import { useNotifications } from '@/composables/useNotifications';

import * as api from '../api';

export function useScreenshot() {
  const { t } = useI18n();
  const { error: showError } = useNotifications();
  const selectedScreenshot = ref<string | null>(null);

  function viewScreenshot(filePath: string) {
    selectedScreenshot.value = filePath;
  }

  function closeScreenshotModal() {
    selectedScreenshot.value = null;
  }

  async function openScreenshotDirectory(filePath: string) {
    try {
      await api.openScreenshotDirectory(filePath);
    } catch (err) {
      console.error('Failed to open directory:', err);
      showError(`${t('error.openDirectory')}: ${err}`);
    }
  }

  return {
    selectedScreenshot,
    viewScreenshot,
    closeScreenshotModal,
    openScreenshotDirectory,
  };
}
