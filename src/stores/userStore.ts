import { ref } from 'vue';
import { STORAGE_KEYS } from './constants';
import { ALL_USERS } from '@/types';
import type { UserFilterId } from '@/types';

function getSavedUserId(): UserFilterId {
  const saved = localStorage.getItem(STORAGE_KEYS.SELECTED_USER);
  if (saved === null) return ALL_USERS;
  const parsed = parseInt(saved, 10);
  return isNaN(parsed) ? ALL_USERS : parsed;
}

const selectedUserId = ref<UserFilterId>(ALL_USERS);

export function useUserSelection() {
  return {
    selectedUserId,
    selectUser: (userId: UserFilterId) => {
      selectedUserId.value = userId;
      localStorage.setItem(STORAGE_KEYS.SELECTED_USER, userId.toString());
    },
    initSelectedUser: () => {
      selectedUserId.value = getSavedUserId();
    },
  };
}
