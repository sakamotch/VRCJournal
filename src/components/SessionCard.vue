<script setup lang="ts">
import { ref } from "vue";
import type { Session, Player, Screenshot } from "@/types";
import { formatDuration, formatTime, formatDate } from "@/utils/formatters";
import PlayerList from "./PlayerList.vue";
import ScreenshotList from "./ScreenshotList.vue";
import { invoke } from "@tauri-apps/api/core";
import { Calendar, Clock, Users, Camera, ChevronDown, ChevronRight, ExternalLink } from "lucide-vue-next";

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
    <div class="session-header">
      <h3 class="world-name">
        {{ session.worldName || session.worldId }}
      </h3>
      <span class="user-name">{{ session.userName }}</span>
    </div>
    <div class="session-info">
      <span class="info-item date">
        <Calendar :size="16" />
        {{ formatDate(session.startedAt) }}
      </span>
      <span
        class="info-item time"
        :title="session.status === 'interrupted' ? 'VRChatが予期せず終了した可能性があります' : ''"
      >
        <Clock :size="16" />
        {{ formatTime(session.startedAt) }}
        <template v-if="session.endedAt">
          〜 {{ formatTime(session.endedAt) }} ({{ formatDuration(session) }})
        </template>
        <template v-else-if="session.status === 'interrupted'">
          〜 不明
        </template>
        <template v-else>
          〜 進行中
        </template>
      </span>
      <span
        class="info-item player-count clickable"
        @click="togglePlayers"
        :title="playersExpanded ? 'プレイヤーを非表示' : 'プレイヤーを表示'"
      >
        <Users :size="16" />
        {{ session.playerCount }}人
        <ChevronDown v-if="playersExpanded" :size="14" />
        <ChevronRight v-else :size="14" />
      </span>
      <span
        v-if="session.screenshotCount > 0"
        class="info-item screenshot-count clickable"
        @click="toggleScreenshots"
        :title="screenshotsExpanded ? '写真を非表示' : '写真を表示'"
      >
        <Camera :size="16" />
        {{ session.screenshotCount }}枚
        <ChevronDown v-if="screenshotsExpanded" :size="14" />
        <ChevronRight v-else :size="14" />
      </span>
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
        <ExternalLink :size="16" />
        ワールドを開く
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

.session-header {
  display: flex;
  justify-content: space-between;
  align-items: baseline;
  margin-bottom: 0.5rem;
}

.world-name {
  margin: 0;
  font-size: 1.1rem;
  color: var(--text-primary);
  flex: 1;
}

.user-name {
  font-size: 0.8rem;
  color: var(--text-tertiary);
  white-space: nowrap;
  margin-left: 1rem;
}

.session-info {
  display: flex;
  gap: 1rem;
  flex-wrap: wrap;
  font-size: 0.9rem;
  color: var(--text-tertiary);
}

.info-item {
  display: flex;
  align-items: center;
  gap: 0.375rem;
}

.date {
  color: var(--text-secondary);
  font-weight: 500;
}

.time {
  color: var(--text-tertiary);
}

.player-count {
  color: var(--text-tertiary);
}

.screenshot-count {
  color: var(--text-tertiary);
}

.clickable {
  cursor: pointer;
  user-select: none;
  transition: color 0.2s;
}

.clickable:hover {
  color: var(--text-primary);
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
  padding: 0.5rem 0.875rem;
  background-color: var(--bg-elevated);
  color: var(--text-primary);
  border: 1px solid var(--border-default);
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.875rem;
  font-weight: 500;
  white-space: nowrap;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.open-url-button:hover {
  background-color: var(--bg-hover);
  border-color: var(--border-hover);
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
