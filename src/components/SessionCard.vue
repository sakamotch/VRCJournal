<script setup lang="ts">
import { ref } from "vue";
import type { Session, Player, Screenshot } from "@/types";
import { formatDateTime, formatDuration } from "@/utils/formatters";
import PlayerList from "./PlayerList.vue";
import ScreenshotList from "./ScreenshotList.vue";
import { invoke } from "@tauri-apps/api/core";

interface Props {
  session: Session;
}

interface Emits {
  (e: "openInvite", session: Session): void;
  (e: "openUserPage", userId: string): void;
  (e: "viewScreenshot", filePath: string): void;
  (e: "openDirectory", filePath: string): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

const playersExpanded = ref(false);
const screenshotsExpanded = ref(false);
const players = ref<Player[] | null>(null);
const screenshots = ref<Screenshot[] | null>(null);

async function togglePlayers() {
  playersExpanded.value = !playersExpanded.value;

  if (playersExpanded.value && players.value === null) {
    try {
      const result = await invoke<Player[]>("get_session_players", {
        sessionId: props.session.id,
      });
      players.value = result;
    } catch (error) {
      console.error("Failed to load players:", error);
    }
  }
}

async function toggleScreenshots() {
  screenshotsExpanded.value = !screenshotsExpanded.value;

  if (screenshotsExpanded.value && screenshots.value === null) {
    try {
      const result = await invoke<Screenshot[]>("get_session_screenshots", {
        sessionId: props.session.id,
      });
      screenshots.value = result;
    } catch (error) {
      console.error("Failed to load screenshots:", error);
    }
  }
}
</script>

<template>
  <div class="session-card">
    <div class="session-main">
      <h3 class="world-name">
        {{ session.worldName || session.worldId }}
      </h3>
      <div class="session-info">
        <span class="user-name">{{ session.userName }}</span>
        <span class="time">{{ formatDateTime(session.startedAt) }}</span>
        <span
          :class="['duration', { 'duration-interrupted': session.status === 'interrupted' }]"
          :title="session.status === 'interrupted' ? 'VRChatが予期せず終了した可能性があります' : ''"
        >
          {{ formatDuration(session) }}
        </span>
        <span
          class="player-count clickable"
          @click="togglePlayers"
          :title="playersExpanded ? 'プレイヤーを非表示' : 'プレイヤーを表示'"
        >
          👥 {{ session.playerCount }}人
          {{ playersExpanded ? '▼' : '▶' }}
        </span>
        <span
          v-if="session.screenshotCount > 0"
          class="screenshot-count clickable"
          @click="toggleScreenshots"
          :title="screenshotsExpanded ? '写真を非表示' : '写真を表示'"
        >
          📷 {{ session.screenshotCount }}枚
          {{ screenshotsExpanded ? '▼' : '▶' }}
        </span>
      </div>
    </div>

    <!-- プレイヤーリスト -->
    <PlayerList
      v-if="playersExpanded && players"
      :players="players"
      :session="session"
      @open-user-page="(userId) => emit('openUserPage', userId)"
    />
    <div v-else-if="playersExpanded" class="loading">
      読み込み中...
    </div>

    <!-- スクリーンショットリスト -->
    <ScreenshotList
      v-if="screenshotsExpanded && screenshots"
      :screenshots="screenshots"
      @view-screenshot="(filePath) => emit('viewScreenshot', filePath)"
      @open-directory="(filePath) => emit('openDirectory', filePath)"
    />
    <div v-else-if="screenshotsExpanded" class="loading">
      読み込み中...
    </div>

    <div class="session-details">
      <div class="detail-item">
        <span class="label">Instance:</span>
        <span class="value">{{ session.instanceId }}</span>
      </div>
      <button @click="emit('openInvite', session)" class="open-url-button">
        🚀 ワールドを開く
      </button>
    </div>
  </div>
</template>

<style scoped>
.session-card {
  background-color: var(--session-card-bg);
  border: 1px solid var(--session-card-border);
  border-radius: 8px;
  padding: 1rem;
  transition: box-shadow 0.2s;
}

.session-card:hover {
  box-shadow: var(--session-card-hover-shadow);
}

.session-main {
  margin-bottom: 0.5rem;
}

.world-name {
  margin: 0 0 0.5rem 0;
  font-size: 1.1rem;
  color: var(--text-primary);
}

.session-info {
  display: flex;
  gap: 1rem;
  flex-wrap: wrap;
  font-size: 0.9rem;
  color: var(--text-tertiary);
}

.user-name {
  font-weight: 600;
  color: var(--interactive-default);
}

.duration {
  color: var(--feedback-success);
}

.duration-interrupted {
  color: var(--feedback-warning);
}

.player-count {
  color: var(--feedback-error);
}

.screenshot-count {
  color: var(--color-purple-600);
}

.clickable {
  cursor: pointer;
  user-select: none;
  transition: color 0.2s;
}

.clickable:hover {
  color: var(--text-secondary);
}

.loading {
  text-align: center;
  padding: 1rem;
  color: var(--text-tertiary);
  font-size: 0.9rem;
}

.session-details {
  margin-top: 0.5rem;
  padding-top: 0.5rem;
  border-top: 1px solid var(--border-subtle);
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 1rem;
}

.detail-item {
  font-size: 0.85rem;
  color: var(--text-tertiary);
  flex: 1;
}

.open-url-button {
  padding: 0.4rem 0.8rem;
  background-color: var(--interactive-default);
  color: var(--text-on-color);
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.85rem;
  white-space: nowrap;
  transition: background-color 0.2s;
}

.open-url-button:hover {
  background-color: var(--interactive-hover);
}

.label {
  font-weight: 600;
  margin-right: 0.5rem;
}

.value {
  font-family: monospace;
  font-size: 0.8rem;
}
</style>
