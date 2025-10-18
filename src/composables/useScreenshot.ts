import { invoke } from '@tauri-apps/api/core';
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';

import { useNotifications } from './useNotifications';

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
      await invoke('open_screenshot_directory', { filePath });
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
