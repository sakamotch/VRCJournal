<script setup lang="ts">
import AppHeader from "@/components/app/AppHeader.vue";
import AppNavigation from "@/components/app/AppNavigation.vue";
import SettingsModal from "@/components/app/SettingsModal.vue";
import TheNotificationContainer from "@/components/app/TheNotificationContainer.vue";
import { useAppState } from "@/composables/useAppState";
import { useRouter } from "@/router";

const { showSettings, currentView, navigateToView } = useAppState();
const { currentComponent } = useRouter(currentView);
</script>

<template>
  <div class="default-layout">
    <AppHeader @open-settings="showSettings = true" />

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
.default-layout {
  display: flex;
  flex-direction: column;
  height: 100vh;
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
