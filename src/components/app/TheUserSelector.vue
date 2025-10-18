<script setup lang="ts">
import { ChevronDown } from 'lucide-vue-next';
import { computed, onBeforeUnmount,onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';

import type { LocalUser, UserFilterId } from '@/types';
import { ALL_USERS } from '@/types';

const props = defineProps<{
  users: LocalUser[];
  selectedUserId: UserFilterId;
}>();

const emit = defineEmits<{
  select: [userId: UserFilterId];
}>();

const { t } = useI18n();
const showDropdown = ref(false);

const selectedUserName = computed(() => {
  if (props.selectedUserId === ALL_USERS) {
    return t('user.allAccounts');
  }
  const user = props.users.find(u => u.id === props.selectedUserId);
  return user?.displayName || t('user.allAccounts');
});

function selectUser(userId: UserFilterId) {
  emit('select', userId);
  showDropdown.value = false;
}

function handleClickOutside(event: MouseEvent) {
  const target = event.target as HTMLElement;
  if (!target.closest('.user-selector')) {
    showDropdown.value = false;
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside);
});

onBeforeUnmount(() => {
  document.removeEventListener('click', handleClickOutside);
});
</script>

<template>
  <div class="user-selector">
    <button
      class="user-selector-button"
      :title="selectedUserName"
      @click="showDropdown = !showDropdown"
    >
      <span class="user-name">{{ selectedUserName }}</span>
      <ChevronDown :size="16" />
    </button>
    <div
      v-if="showDropdown"
      class="user-dropdown"
    >
      <button
        :class="['user-option', { active: selectedUserId === ALL_USERS }]"
        @click="selectUser(ALL_USERS)"
      >
        <span>{{ t('user.allAccounts') }}</span>
      </button>
      <button
        v-for="user in users"
        :key="user.id"
        :class="['user-option', { active: selectedUserId === user.id }]"
        @click="selectUser(user.id)"
      >
        <span>{{ user.displayName }}</span>
      </button>
    </div>
  </div>
</template>

<style scoped lang="scss">
.user-selector {
  position: relative;

  &-button {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.375rem 0.625rem;
    background: linear-gradient(135deg,
      var(--overlay-light) 0%,
      var(--overlay-light-medium) 100%
    );
    color: var(--header-text);
    border: 1px solid var(--overlay-medium-strong);
    border-radius: 8px;
    cursor: pointer;
    font-size: 0.875rem;
    transition: all 0.3s ease;
    position: relative;
    overflow: hidden;
    backdrop-filter: blur(8px);

    &::before {
      content: '';
      position: absolute;
      inset: 0;
      background: linear-gradient(135deg,
        var(--overlay-medium-strong) 0%,
        var(--overlay-medium) 100%
      );
      opacity: 0;
      transition: opacity 0.3s ease;
    }

    &:hover {
      border-color: var(--overlay-strong);

      &::before {
        opacity: 1;
      }
    }

    & > * {
      position: relative;
      z-index: 1;
    }
  }
}

.user-name {
  max-width: 150px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.user-dropdown {
  position: absolute;
  top: calc(100% + 0.5rem);
  right: 0;
  min-width: 200px;
  background: linear-gradient(135deg,
    var(--bg-surface) 0%,
    color-mix(in srgb, var(--bg-surface) 98%, var(--accent-primary) 2%) 100%
  );
  border: 1px solid color-mix(in srgb, var(--border-default) 80%, var(--accent-primary-light) 20%);
  border-radius: 10px;
  box-shadow: var(--shadow-xl),
              0 0 0 1px color-mix(in srgb, var(--border-default) 70%, var(--accent-primary-light) 30%);
  padding: 0.5rem;
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  max-height: 300px;
  overflow-y: auto;
  backdrop-filter: blur(8px);
  animation: dropdown-slide 0.2s ease;
}

@keyframes dropdown-slide {
  from {
    opacity: 0;
    transform: translateY(-8px);
  }

  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.user-option {
  padding: 0.5rem 0.75rem;
  background: none;
  border: none;
  text-align: left;
  cursor: pointer;
  color: var(--text-primary);
  font-size: 0.875rem;
  border-radius: 6px;
  transition: all 0.3s ease;
  position: relative;
  overflow: hidden;

  &::before {
    content: '';
    position: absolute;
    inset: 0;
    background: linear-gradient(135deg,
      color-mix(in srgb, var(--bg-hover) 92%, var(--accent-primary-light) 8%) 0%,
      color-mix(in srgb, var(--bg-hover) 96%, var(--accent-secondary-light) 4%) 100%
    );
    opacity: 0;
    transition: opacity 0.3s ease;
    border-radius: 6px;
  }

  &:hover {
    color: var(--interactive-default);

    &::before {
      opacity: 1;
    }
  }

  &.active {
    background: linear-gradient(135deg,
      color-mix(in srgb, var(--bg-elevated) 88%, var(--accent-primary-light) 12%) 0%,
      color-mix(in srgb, var(--bg-elevated) 92%, var(--accent-secondary-light) 8%) 100%
    );
    color: var(--interactive-default);
    font-weight: 600;
    box-shadow: 0 2px 6px color-mix(in srgb, var(--accent-primary) 10%, transparent);

    &::before {
      opacity: 0;
    }
  }

  & > * {
    position: relative;
    z-index: 1;
  }
}
</style>
