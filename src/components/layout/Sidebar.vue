<script setup lang="ts">
import type { LocalUser } from "@/types";

interface Props {
  users: LocalUser[];
  selectedUserId: number | null;
}

interface Emits {
  (e: "selectUser", userId: number | null): void;
}

defineProps<Props>();
const emit = defineEmits<Emits>();
</script>

<template>
  <aside class="sidebar">
    <h2>アカウント</h2>
    <ul class="user-list">
      <li
        :class="{ active: selectedUserId === null }"
        @click="emit('selectUser', null)"
      >
        全て表示
      </li>
      <li
        v-for="user in users"
        :key="user.id"
        :class="{ active: selectedUserId === user.id }"
        @click="emit('selectUser', user.id)"
      >
        {{ user.displayName }}
      </li>
    </ul>
  </aside>
</template>

<style scoped lang="scss">
.sidebar {
  width: 250px;
  background-color: var(--sidebar-bg);
  border-right: 1px solid var(--border-default);
  padding: 1rem;
  overflow-y: auto;

  h2 {
    margin-top: 0;
    font-size: 1.1rem;
    color: var(--sidebar-text-secondary);
  }
}

.user-list {
  list-style: none;
  padding: 0;
  margin: 0;

  li {
    padding: 0.75rem;
    margin-bottom: 0.5rem;
    background-color: var(--bg-elevated);
    border-radius: 4px;
    cursor: pointer;
    transition: background-color 0.2s;
    color: var(--sidebar-text);

    &:hover {
      background-color: var(--sidebar-item-hover);
    }

    &.active {
      background-color: var(--sidebar-item-active);
      color: var(--interactive-default);
    }
  }
}
</style>
