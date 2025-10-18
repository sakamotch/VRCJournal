<script setup lang="ts">
import { Settings as SettingsIcon } from "lucide-vue-next";
import { storeToRefs } from "pinia";
import { useI18n } from "vue-i18n";

import BaseIconButton from "@/components/common/BaseIconButton.vue";
import AppNavigation from "@/components/layout/AppNavigation.vue";
import TheNotificationContainer from "@/components/layout/TheNotificationContainer.vue";
import TheUserSelector from "@/components/layout/TheUserSelector.vue";
import SettingsModal from "@/components/settings/SettingsModal.vue";
import { useAppState } from "@/composables/useAppState";
import { useBackendEvents } from "@/composables/useBackendEvents";
import { useViewRouter } from "@/composables/useViewRouter";
import { useUserStore } from "@/stores/userStore";

const { t } = useI18n();

const userStore = useUserStore();
const { localUsers, selectedUserId } = storeToRefs(userStore);
const { loadUsers, selectUser } = userStore;

const { showSettings, currentView, navigateToView } = useAppState();
const { currentComponent } = useViewRouter(currentView);

useBackendEvents({
  onBackendReady: loadUsers,
  onLocalPlayerUpdated: loadUsers,
});
</script>

<template>
  <div class="app">
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
            @click="showSettings = true"
          >
            <SettingsIcon
              :size="20"
              class="settings-icon"
            />
          </BaseIconButton>
        </div>
      </div>
    </header>

    <div class="content">
      <AppNavigation
        :current-view="currentView"
        @navigate="navigateToView"
      />

      <main class="main-content">
        <component :is="currentComponent" />
      </main>
    </div>

    <SettingsModal
      :show="showSettings"
      @close="showSettings = false"
    />

    <TheNotificationContainer />
  </div>
</template>

<style scoped lang="scss">
.app {
  display: flex;
  flex-direction: column;
  height: 100vh;
}

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

  :deep(.icon-button:hover) .settings-icon {
    transform: rotate(90deg);
  }
}

.settings-icon {
  transition: transform 0.2s;
}

.content {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.main-content {
  flex: 1;
  overflow-y: auto;
  background-color: var(--bg-base);
}
</style>
