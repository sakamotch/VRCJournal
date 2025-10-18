import { invoke } from '@tauri-apps/api/core';
import { storeToRefs } from 'pinia';
import { ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';

import { useUserStore } from '@/stores/userStore';
import type { Instance } from '@/types';

import { useNotifications } from './useNotifications';

export function useInstances() {
  const { t } = useI18n();
  const { success, error: showError } = useNotifications();
  const userStore = useUserStore();
  const { selectedUserId } = storeToRefs(userStore);
  const instances = ref<Instance[]>([]);
  const isLoading = ref(false);

  async function loadInstances(localUserId: number) {
    isLoading.value = true;
    try {
      const result = await invoke<Instance[]>('get_instances', {
        localUserId,
        limit: 100,
      });
      instances.value = result;
    } catch (err) {
      console.error('Failed to load instances:', err);
      showError(`${t('error.instanceLoad')}: ${err}`);
    } finally {
      isLoading.value = false;
    }
  }

  async function openInviteUrl(instance: Instance) {
    try {
      const url = await invoke<string>('open_invite_url', {
        worldId: instance.worldId,
        instanceId: instance.instanceId,
      });

      success(`${t('notification.inviteOpened')}: ${url}`);
    } catch (err) {
      showError(`${t('common.error')}: ${err}`);
    }
  }

  async function openUserPage(userId: string) {
    try {
      const url = await invoke<string>('open_user_page', {
        userId: userId,
      });

      success(`${t('notification.userPageOpened')}: ${url}`);
    } catch (err) {
      showError(`${t('common.error')}: ${err}`);
    }
  }

  function updateInstanceEnd(instanceId: number, endedAt: string) {
    const instance = instances.value.find(s => s.id === instanceId);
    if (instance) {
      instance.endedAt = endedAt;
      instance.status = 'completed';
    }
  }

  watch(selectedUserId, () => {
    loadInstances(selectedUserId.value);
  }, { immediate: true });

  return {
    instances,
    isLoading,
    loadInstances,
    openInviteUrl,
    openUserPage,
    updateInstanceEnd,
  };
}
