<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { LocalUser, Session } from "@/types";
import Sidebar from "@/components/Sidebar.vue";
import SessionList from "@/components/SessionList.vue";
import ScreenshotModal from "@/components/ScreenshotModal.vue";
import Settings from "@/components/Settings.vue";
import { Settings as SettingsIcon } from "lucide-vue-next";

const isLoading = ref(false);
const message = ref("");
const localUsers = ref<LocalUser[]>([]);
const sessions = ref<Session[]>([]);
const selectedUserId = ref<number | null>(null);
const selectedScreenshot = ref<string | null>(null);
const showSettings = ref(false);

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
  } catch (error) {
    console.error("Failed to load sessions:", error);
    message.value = `セッション読み込みエラー: ${error}`;
  } finally {
    isLoading.value = false;
  }
}

function selectUser(userId: number | null) {
  selectedUserId.value = userId;
  loadSessions();
}

async function openInviteUrl(session: Session) {
  try {
    const url = await invoke<string>("open_invite_url", {
      worldId: session.worldId,
      instanceId: session.instanceId,
    });

    message.value = `招待URLを開きました: ${url}`;

    setTimeout(() => {
      if (message.value.startsWith("招待URLを開きました")) {
        message.value = "";
      }
    }, 3000);
  } catch (error) {
    message.value = `エラー: ${error}`;
  }
}

async function openScreenshotDirectory(filePath: string) {
  try {
    await invoke("open_screenshot_directory", { filePath });
  } catch (error) {
    console.error("Failed to open directory:", error);
    message.value = `ディレクトリを開けませんでした: ${error}`;
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

    message.value = `ユーザーページを開きました: ${url}`;

    setTimeout(() => {
      if (message.value.startsWith("ユーザーページを開きました")) {
        message.value = "";
      }
    }, 3000);
  } catch (error) {
    message.value = `エラー: ${error}`;
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
        <button class="settings-button" @click="showSettings = true" title="設定">
          <SettingsIcon :size="20" />
        </button>
      </div>
    </header>

    <div v-if="message" class="message">{{ message }}</div>

    <div class="content">
      <Sidebar
        :users="localUsers"
        :selected-user-id="selectedUserId"
        @select-user="selectUser"
      />

      <SessionList
        :sessions="sessions"
        :is-loading="isLoading"
        @open-invite="openInviteUrl"
        @open-user-page="openUserPage"
        @view-screenshot="viewScreenshot"
        @open-directory="openScreenshotDirectory"
      />
    </div>

    <ScreenshotModal
      :file-path="selectedScreenshot"
      @close="closeScreenshotModal"
    />

    <Settings
      v-if="showSettings"
      @close="showSettings = false"
    />
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
  background-color: var(--header-bg);
  color: var(--header-text);
  box-shadow: var(--shadow-sm);
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

.settings-button {
  background: none;
  border: none;
  cursor: pointer;
  padding: 0.5rem;
  transition: transform 0.2s;
  color: var(--header-text);
  display: flex;
  align-items: center;
  justify-content: center;
}

.settings-button:hover {
  transform: rotate(90deg);
}

.message {
  padding: 1rem 1.5rem;
  background-color: var(--message-bg);
  border-left: 4px solid var(--message-border);
  color: var(--message-text);
  margin: 0;
  white-space: pre-wrap;
}

.content {
  display: flex;
  flex: 1;
  overflow: hidden;
}
</style>
