<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, onBeforeUnmount } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { LocalUser, Session } from "@/types";
import Navigation, { type NavigationView } from "@/components/Navigation.vue";
import SessionList from "@/components/SessionList.vue";
import WorldsView from "@/components/views/WorldsView.vue";
import PeopleView from "@/components/views/PeopleView.vue";
import PhotosView from "@/components/views/PhotosView.vue";
import StatsView from "@/components/views/StatsView.vue";
import ScreenshotModal from "@/components/ScreenshotModal.vue";
import Settings from "@/components/Settings.vue";
import NotificationContainer from "@/components/NotificationContainer.vue";
import { Settings as SettingsIcon, ChevronDown } from "lucide-vue-next";
import { useNotifications } from "@/composables/useNotifications";

const { success, error: showError, info } = useNotifications();
const isLoading = ref(false);
const localUsers = ref<LocalUser[]>([]);
const sessions = ref<Session[]>([]);
const selectedUserId = ref<number | null>(null);
const selectedScreenshot = ref<string | null>(null);
const showSettings = ref(false);
const currentView = ref<NavigationView>('timeline');
const showUserDropdown = ref(false);

const selectedUser = computed(() => {
  if (selectedUserId.value === null) return null;
  return localUsers.value.find(u => u.id === selectedUserId.value);
});

const selectedUserName = computed(() => {
  return selectedUser.value?.displayName || "全アカウント";
});

async function loadUsers() {
  try {
    const users = await invoke<LocalUser[]>("get_local_users");
    localUsers.value = users;
  } catch (error) {
    console.error("Failed to load users:", error);
  }
}

async function loadSessions() {
  isLoading.value = true;
  try {
    const result = await invoke<Session[]>("get_sessions", {
      localUserId: selectedUserId.value,
      limit: 100,
    });
    sessions.value = result;
  } catch (err) {
    console.error("Failed to load sessions:", err);
    showError(`セッション読み込みエラー: ${err}`);
  } finally {
    isLoading.value = false;
  }
}

function selectUser(userId: number | null) {
  selectedUserId.value = userId;
  showUserDropdown.value = false;
  loadSessions();
}

function navigateToView(view: NavigationView) {
  currentView.value = view;
}

function handleClickOutside(event: MouseEvent) {
  const target = event.target as HTMLElement;
  if (!target.closest('.user-selector')) {
    showUserDropdown.value = false;
  }
}

async function openInviteUrl(session: Session) {
  try {
    const url = await invoke<string>("open_invite_url", {
      worldId: session.worldId,
      instanceId: session.instanceId,
    });

    success(`招待URLを開きました: ${url}`);
  } catch (err) {
    showError(`エラー: ${err}`);
  }
}

async function openScreenshotDirectory(filePath: string) {
  try {
    await invoke("open_screenshot_directory", { filePath });
  } catch (err) {
    console.error("Failed to open directory:", err);
    showError(`ディレクトリを開けませんでした: ${err}`);
  }
}

function viewScreenshot(filePath: string) {
  selectedScreenshot.value = filePath;
}

function closeScreenshotModal() {
  selectedScreenshot.value = null;
}

async function openUserPage(userId: string) {
  try {
    const url = await invoke<string>("open_user_page", {
      userId: userId,
    });

    success(`ユーザーページを開きました: ${url}`);
  } catch (err) {
    showError(`エラー: ${err}`);
  }
}

let unlistenFn: UnlistenFn | null = null;

onMounted(async () => {
  loadUsers();
  loadSessions();

  unlistenFn = await listen<any>("log-event", (event) => {
    const processedEvent = event.payload;

    switch (processedEvent.type) {
      case "LocalPlayerUpdated":
        loadUsers();
        break;

      case "SessionCreated":
        loadSessions();
        break;

      case "SessionEnded":
        const endedSession = sessions.value.find(s => s.id === processedEvent.session_id);
        if (endedSession) {
          endedSession.endedAt = processedEvent.ended_at;
          endedSession.status = 'completed';
        }
        break;

      case "PlayerJoined":
      case "PlayerLeft":
        // セッションをリロードしてプレイヤー数を更新
        loadSessions();
        break;
    }
  });

  // ドロップダウンの外クリック検知
  document.addEventListener('click', handleClickOutside);
});

onBeforeUnmount(() => {
  document.removeEventListener('click', handleClickOutside);
});

onUnmounted(() => {
  if (unlistenFn) {
    unlistenFn();
  }
});
</script>

<template>
  <div class="app">
    <header class="header">
      <div class="header-content">
        <h1>VRCJournal</h1>
        <div class="header-actions">
          <div class="user-selector">
            <button
              class="user-selector-button"
              @click="showUserDropdown = !showUserDropdown"
              :title="selectedUserName"
            >
              <span class="user-name">{{ selectedUserName }}</span>
              <ChevronDown :size="16" />
            </button>
            <div v-if="showUserDropdown" class="user-dropdown">
              <button
                :class="['user-option', { active: selectedUserId === null }]"
                @click="selectUser(null)"
              >
                <span>全アカウント</span>
              </button>
              <button
                v-for="user in localUsers"
                :key="user.id"
                :class="['user-option', { active: selectedUserId === user.id }]"
                @click="selectUser(user.id)"
              >
                <span>{{ user.displayName }}</span>
              </button>
            </div>
          </div>
          <button class="settings-button" @click="showSettings = true" title="設定">
            <SettingsIcon :size="20" class="settings-icon" />
          </button>
        </div>
      </div>
    </header>

    <div class="content">
      <Navigation
        :current-view="currentView"
        @navigate="navigateToView"
      />

      <main class="main-content">
        <SessionList
          v-if="currentView === 'timeline'"
          :sessions="sessions"
          :is-loading="isLoading"
          @open-invite="openInviteUrl"
          @open-user-page="openUserPage"
          @view-screenshot="viewScreenshot"
          @open-directory="openScreenshotDirectory"
        />
        <WorldsView v-else-if="currentView === 'worlds'" />
        <PeopleView v-else-if="currentView === 'people'" />
        <PhotosView v-else-if="currentView === 'photos'" />
        <StatsView v-else-if="currentView === 'stats'" />
      </main>
    </div>

    <ScreenshotModal
      :file-path="selectedScreenshot"
      @close="closeScreenshotModal"
    />

    <Settings
      :show="showSettings"
      @close="showSettings = false"
    />

    <NotificationContainer />
  </div>
</template>

<style scoped>
.app {
  display: flex;
  flex-direction: column;
  height: 100vh;
}

.header {
  padding: 1rem 1.5rem;
  background: var(--header-bg);
  color: var(--header-text);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  position: relative;
  z-index: 10;
}

.header-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 1rem;
}

.header h1 {
  margin: 0;
  font-size: 1.5rem;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.user-selector {
  position: relative;
}

.user-selector-button {
  display: flex;
  align-items: center;
  gap: 0.375rem;
  padding: 0.375rem 0.625rem;
  background: linear-gradient(135deg,
    rgba(255, 255, 255, 0.05) 0%,
    rgba(255, 255, 255, 0.08) 100%
  );
  color: var(--header-text);
  border: 1px solid rgba(255, 255, 255, 0.15);
  border-radius: 8px;
  cursor: pointer;
  font-size: 0.875rem;
  transition: all 0.3s ease;
  position: relative;
  overflow: hidden;
  backdrop-filter: blur(8px);
}

.user-selector-button::before {
  content: '';
  position: absolute;
  inset: 0;
  background: linear-gradient(135deg,
    rgba(255, 255, 255, 0.15) 0%,
    rgba(255, 255, 255, 0.1) 100%
  );
  opacity: 0;
  transition: opacity 0.3s ease;
}

.user-selector-button:hover {
  border-color: rgba(255, 255, 255, 0.3);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  transform: translateY(-1px);
}

.user-selector-button:hover::before {
  opacity: 1;
}

.user-selector-button > * {
  position: relative;
  z-index: 1;
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
    color-mix(in srgb, var(--bg-surface) 98%, var(--color-indigo-500) 2%) 100%
  );
  border: 1px solid color-mix(in srgb, var(--border-default) 80%, var(--color-indigo-400) 20%);
  border-radius: 10px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2),
              0 0 0 1px color-mix(in srgb, var(--border-default) 70%, var(--color-indigo-400) 30%);
  padding: 0.5rem;
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  max-height: 300px;
  overflow-y: auto;
  backdrop-filter: blur(8px);
  animation: dropdownSlide 0.2s ease;
}

@keyframes dropdownSlide {
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
}

.user-option::before {
  content: '';
  position: absolute;
  inset: 0;
  background: linear-gradient(135deg,
    color-mix(in srgb, var(--bg-hover) 92%, var(--color-indigo-400) 8%) 0%,
    color-mix(in srgb, var(--bg-hover) 96%, var(--color-violet-400) 4%) 100%
  );
  opacity: 0;
  transition: opacity 0.3s ease;
  border-radius: 6px;
}

.user-option:hover {
  color: var(--interactive-default);
}

.user-option:hover::before {
  opacity: 1;
}

.user-option.active {
  background: linear-gradient(135deg,
    color-mix(in srgb, var(--bg-elevated) 88%, var(--color-indigo-400) 12%) 0%,
    color-mix(in srgb, var(--bg-elevated) 92%, var(--color-violet-400) 8%) 100%
  );
  color: var(--interactive-default);
  font-weight: 600;
  box-shadow: 0 2px 6px rgba(99, 102, 241, 0.1);
}

.user-option.active::before {
  opacity: 0;
}

.user-option > * {
  position: relative;
  z-index: 1;
}

.settings-button {
  background: linear-gradient(135deg,
    rgba(255, 255, 255, 0.05) 0%,
    rgba(255, 255, 255, 0.08) 100%
  );
  border: 1px solid rgba(255, 255, 255, 0.15);
  border-radius: 8px;
  cursor: pointer;
  padding: 0.375rem;
  transition: all 0.3s ease;
  color: var(--header-text);
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
  overflow: hidden;
  backdrop-filter: blur(8px);
}

.settings-button::before {
  content: '';
  position: absolute;
  inset: 0;
  background: linear-gradient(135deg,
    rgba(255, 255, 255, 0.15) 0%,
    rgba(255, 255, 255, 0.1) 100%
  );
  opacity: 0;
  transition: opacity 0.3s ease;
}

.settings-button:hover {
  border-color: rgba(255, 255, 255, 0.3);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  transform: translateY(-1px);
}

.settings-button:hover::before {
  opacity: 1;
}

.settings-button > * {
  position: relative;
  z-index: 1;
}

.settings-icon {
  transition: transform 0.2s;
}

.settings-button:hover .settings-icon {
  transform: rotate(90deg);
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
