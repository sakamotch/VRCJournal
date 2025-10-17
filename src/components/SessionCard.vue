<script setup lang="ts">
import { ref, computed } from "vue";
import { useI18n } from "vue-i18n";
import dayjs from "dayjs";
import type { Session, Player, Screenshot } from "@/types";
import PlayerList from "./PlayerList.vue";
import ScreenshotList from "./ScreenshotList.vue";
import Button from "./common/Button.vue";
import Card from "./common/Card.vue";
import { invoke } from "@tauri-apps/api/core";
import { Calendar, Clock, Users, Camera, ChevronDown, ChevronRight, ExternalLink } from "lucide-vue-next";

const { t, locale } = useI18n();

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

const sessionDate = computed(() => {
  // locale.valueを依存関係に追加
  locale.value;
  return dayjs(props.session.startedAt).format('L');
});

const sessionStartTime = computed(() => {
  locale.value;
  return dayjs(props.session.startedAt).format('LT');
});

const sessionEndTime = computed(() => {
  locale.value;
  return props.session.endedAt ? dayjs(props.session.endedAt).format('LT') : '';
});

const sessionDuration = computed(() => {
  if (props.session.status === 'interrupted') {
    return t('session.unknown');
  }
  if (!props.session.endedAt) {
    return t('session.ongoing');
  }

  const start = dayjs(props.session.startedAt);
  const end = dayjs(props.session.endedAt);
  const diff = end.diff(start);
  const dur = dayjs.duration(diff);

  const hours = Math.floor(dur.asHours());
  const minutes = dur.minutes();

  if (hours > 0) {
    return t('session.durationHours', { hours, minutes });
  } else {
    return t('session.durationMinutes', { minutes });
  }
});

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
  <Card class="session-card" :hoverable="false">
    <div class="session-header">
      <h3 class="world-name">
        {{ session.worldName || session.worldId }}
      </h3>
      <span class="user-name">{{ session.userName }}</span>
    </div>
    <div class="session-info">
      <span class="info-item date">
        <Calendar :size="16" />
        {{ sessionDate }}
      </span>
      <span
        class="info-item time"
        :title="session.status === 'interrupted' ? t('session.interruptedWarning') : ''"
      >
        <Clock :size="16" />
        {{ sessionStartTime }}
        <template v-if="session.endedAt">
          〜 {{ sessionEndTime }} ({{ sessionDuration }})
        </template>
        <template v-else>
          〜 {{ sessionDuration }}
        </template>
      </span>
      <span
        class="info-item player-count clickable"
        @click="togglePlayers"
        :title="playersExpanded ? t('session.hidePlayers') : t('session.showPlayers')"
      >
        <Users :size="16" />
        {{ t('session.playerCount', { count: session.playerCount }) }}
        <ChevronDown v-if="playersExpanded" :size="14" />
        <ChevronRight v-else :size="14" />
      </span>
      <span
        v-if="session.screenshotCount > 0"
        class="info-item screenshot-count clickable"
        @click="toggleScreenshots"
        :title="screenshotsExpanded ? t('session.hidePhotos') : t('session.showPhotos')"
      >
        <Camera :size="16" />
        {{ t('session.photoCount', { count: session.screenshotCount }) }}
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
      {{ t('session.loading') }}
    </div>

    <!-- スクリーンショットリスト -->
    <ScreenshotList
      v-if="screenshotsExpanded && screenshots"
      :screenshots="screenshots"
      @view-screenshot="(filePath) => emit('viewScreenshot', filePath)"
      @open-directory="(filePath) => emit('openDirectory', filePath)"
    />
    <div v-else-if="screenshotsExpanded" class="loading">
      {{ t('session.loading') }}
    </div>

    <div class="session-details">
      <div class="detail-item">
        <span class="label">Instance:</span>
        <span class="value">{{ session.instanceId }}</span>
      </div>
      <Button @click="emit('openInvite', session)">
        <ExternalLink :size="16" />
        <span>{{ t('session.openWorld') }}</span>
      </Button>
    </div>
  </Card>
</template>

<style scoped>
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
  color: var(--text-secondary);
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
  color: var(--text-secondary);
}

.player-count {
  color: var(--text-secondary);
}

.screenshot-count {
  color: var(--text-secondary);
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

.label {
  font-weight: 600;
  margin-right: 0.5rem;
}

.value {
  font-family: monospace;
  font-size: 0.8rem;
}
</style>
