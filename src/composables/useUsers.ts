import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { LocalUser } from '@/types';
import { ALL_USERS } from '@/types';
import { useUserSelection } from '@/stores/userStore';

export function useUsers() {
  const { selectedUserId } = useUserSelection();
  const localUsers = ref<LocalUser[]>([]);

  const selectedUser = computed(() => {
    if (selectedUserId.value === ALL_USERS) return null;
    return localUsers.value.find(u => u.id === selectedUserId.value);
  });

  async function loadUsers() {
    try {
      const users = await invoke<LocalUser[]>('get_local_users');
      localUsers.value = users;
    } catch (error) {
      console.error('Failed to load users:', error);
    }
  }

  return {
    localUsers,
    selectedUser,
    loadUsers,
  };
}
