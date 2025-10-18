<script setup lang="ts">
import { Settings as SettingsIcon } from "lucide-vue-next";
import { storeToRefs } from "pinia";
import { useI18n } from "vue-i18n";

import TheUserSelector from "@/components/app/TheUserSelector.vue";
import BaseIconButton from "@/components/base/BaseIconButton.vue";
import { useUserStore } from "@/stores/userStore";

const { t } = useI18n();

const userStore = useUserStore();
const { localUsers, selectedUserId } = storeToRefs(userStore);
const { selectUser } = userStore;

interface Emits {
  (e: "openSettings"): void;
}

const emit = defineEmits<Emits>();
</script>

<template>
  <header class="header">
    <div class="header-content">
      <h1>VRCJournal</h1>
      <div class="header-actions">
        <TheUserSelector
          :users="localUsers"
          :selected-user-id="selectedUserId"
          @select="selectUser"
        />
        <BaseIconButton
          :title="t('settings.title')"
          @click="emit('openSettings')"
        >
          <SettingsIcon
            :size="20"
            class="settings-icon"
          />
        </BaseIconButton>
      </div>
    </div>
  </header>
</template>

<style scoped lang="scss">
.header {
  padding: 1rem 1.5rem;
  background: var(--header-bg);
  color: var(--header-text);
  box-shadow: var(--shadow-md);
  position: relative;
  z-index: 10;

  h1 {
    margin: 0;
    font-size: 1.5rem;
  }
}

.header-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 1rem;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.settings-icon {
  transition: transform 0.2s;

  .header-actions :deep(.icon-button:hover) & {
    transform: rotate(90deg);
  }
}
</style>
