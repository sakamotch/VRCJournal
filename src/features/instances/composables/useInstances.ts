import { storeToRefs } from 'pinia';
import { ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';

import { useNotifications } from '@/composables/useNotifications';
import { useUserStore } from '@/stores/userStore';

import * as api from '../api';
import type { Instance } from '../types';

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
      const result = await api.getInstances(localUserId);
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
      const url = await api.openInviteUrl(instance.worldId, instance.instanceId);
      success(`${t('notification.inviteOpened')}: ${url}`);
    } catch (err) {
      showError(`${t('common.error')}: ${err}`);
    }
  }

  async function openUserPage(userId: string) {
    try {
      const url = await api.openUserPage(userId);
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
