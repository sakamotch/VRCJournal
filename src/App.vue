<script setup lang="ts">
import { useI18n } from "vue-i18n";
import Navigation from "@/components/layout/Navigation.vue";
import Settings from "@/components/settings/Settings.vue";
import NotificationContainer from "@/components/layout/NotificationContainer.vue";
import UserSelector from "@/components/layout/UserSelector.vue";
import IconButton from "@/components/common/IconButton.vue";
import { Settings as SettingsIcon } from "lucide-vue-next";
import { useUsers } from "@/composables/useUsers";
import { useUserSelection } from "@/stores/userStore";
import { useBackendEvents } from "@/composables/useBackendEvents";
import { useAppState } from "@/composables/useAppState";
import { useViewRouter } from "@/composables/useViewRouter";

const { t } = useI18n();

const { localUsers, loadUsers } = useUsers();
const { selectedUserId, selectUser } = useUserSelection();
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
          <UserSelector
            :users="localUsers"
            :selected-user-id="selectedUserId"
            @select="selectUser"
          />
          <IconButton @click="showSettings = true" :title="t('settings.title')">
            <SettingsIcon :size="20" class="settings-icon" />
          </IconButton>
        </div>
      </div>
    </header>

    <div class="content">
      <Navigation
        :current-view="currentView"
        @navigate="navigateToView"
      />

      <main class="main-content">
        <component :is="currentComponent" />
      </main>
    </div>

    <Settings
      :show="showSettings"
      @close="showSettings = false"
    />

    <NotificationContainer />
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
