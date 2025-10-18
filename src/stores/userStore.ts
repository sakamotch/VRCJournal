import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { LocalUser, UserFilterId } from '@/types';
import { ALL_USERS } from '@/types';
import { STORAGE_KEYS } from './constants';

function getSavedUserId(): UserFilterId {
  const saved = localStorage.getItem(STORAGE_KEYS.SELECTED_USER);
  if (saved === null) return ALL_USERS;
  const parsed = parseInt(saved, 10);
  return isNaN(parsed) ? ALL_USERS : parsed;
}

export const useUserStore = defineStore('user', () => {
  // State
  const localUsers = ref<LocalUser[]>([]);
  const selectedUserId = ref<UserFilterId>(ALL_USERS);

  // Getters
  const selectedUser = computed(() => {
    if (selectedUserId.value === ALL_USERS) return null;
    return localUsers.value.find(u => u.id === selectedUserId.value);
  });

  const userCount = computed(() => localUsers.value.length);

  // Actions
  async function loadUsers() {
    try {
      const users = await invoke<LocalUser[]>('get_local_users');
      localUsers.value = users;

      // 選択中のユーザーが存在しない場合、ALL_USERSにリセット
      if (selectedUserId.value !== ALL_USERS) {
        const userExists = users.some(u => u.id === selectedUserId.value);
        if (!userExists) {
          selectedUserId.value = ALL_USERS;
          localStorage.setItem(STORAGE_KEYS.SELECTED_USER, ALL_USERS.toString());
        }
      }
    } catch (error) {
      console.error('Failed to load users:', error);
    }
  }

  function selectUser(userId: UserFilterId) {
    selectedUserId.value = userId;
    localStorage.setItem(STORAGE_KEYS.SELECTED_USER, userId.toString());
  }

  function initSelectedUser() {
    selectedUserId.value = getSavedUserId();
  }

  return {
    // State
    localUsers,
    selectedUserId,
    // Getters
    selectedUser,
    userCount,
    // Actions
    loadUsers,
    selectUser,
    initSelectedUser,
  };
});
