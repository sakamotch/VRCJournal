<script setup lang="ts">
import dayjs from "dayjs";
import { Calendar, Camera, ChevronDown, ChevronRight, Clock, ExternalLink,Users } from "lucide-vue-next";
import { computed,ref } from "vue";
import { useI18n } from "vue-i18n";

import BaseButton from "@/components/base/BaseButton.vue";
import BaseCard from "@/components/base/BaseCard.vue";

import * as api from "../api";
import type { Instance, Player, Screenshot } from "../types";
import PlayerList from "./PlayerList.vue";
import ScreenshotList from "./ScreenshotList.vue";

const { t, locale } = useI18n();

interface Props {
  instance: Instance;
}

interface Emits {
  (e: "openInvite", instance: Instance): void;
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

const instanceDate = computed(() => {
  // locale.valueを依存関係に追加
  locale.value;
  return dayjs(props.instance.startedAt).format('L');
});

const instanceStartTime = computed(() => {
  locale.value;
  return dayjs(props.instance.startedAt).format('LT');
});

const instanceEndTime = computed(() => {
  locale.value;
  return props.instance.endedAt ? dayjs(props.instance.endedAt).format('LT') : '';
});

const instanceDuration = computed(() => {
  if (props.instance.status === 'interrupted') {
    return t('instance.unknown');
  }
  if (!props.instance.endedAt) {
    return t('instance.ongoing');
  }

  const start = dayjs(props.instance.startedAt);
  const end = dayjs(props.instance.endedAt);
  const diff = end.diff(start);
  const dur = dayjs.duration(diff);

  const hours = Math.floor(dur.asHours());
  const minutes = dur.minutes();

  if (hours > 0) {
    return t('instance.durationHours', { hours, minutes });
  } else {
    return t('instance.durationMinutes', { minutes });
  }
});

async function togglePlayers() {
  playersExpanded.value = !playersExpanded.value;

  if (playersExpanded.value && players.value === null) {
    try {
      const result = await api.getInstancePlayers(props.instance.id);
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
      const result = await api.getInstanceScreenshots(props.instance.id);
      screenshots.value = result;
    } catch (error) {
      console.error("Failed to load screenshots:", error);
    }
  }
}
</script>

<template>
  <BaseCard
    class="instance-card"
    :class="{ 'event-sync-failed': instance.status === 'event_sync_failed' }"
    :title="instance.status === 'event_sync_failed' ? t('instance.eventSyncFailedWarning') : ''"
    :hoverable="false"
  >
    <div class="instance-header">
      <h3 class="world-name">
        {{ instance.worldName || instance.worldId }}
        <span
          v-if="instance.status === 'event_sync_failed'"
          class="error-badge"
        >
          {{ t('instance.connectionError') }}
        </span>
      </h3>
      <span class="user-name">{{ instance.userName }}</span>
    </div>
    <div class="instance-info">
      <span class="info-item date">
        <Calendar :size="16" />
        {{ instanceDate }}
      </span>
      <span
        class="info-item time"
        :title="instance.status === 'interrupted' ? t('instance.interruptedWarning') : ''"
      >
        <Clock :size="16" />
        {{ instanceStartTime }}
        <template v-if="instance.endedAt && instance.status !== 'event_sync_failed'">
          〜 {{ instanceEndTime }} ({{ instanceDuration }})
        </template>
        <template v-else-if="instance.status !== 'event_sync_failed'">
          〜 {{ instanceDuration }}
        </template>
      </span>
      <span
        v-if="instance.status !== 'event_sync_failed'"
        class="info-item player-count clickable"
        :title="playersExpanded ? t('instance.hidePlayers') : t('instance.showPlayers')"
        @click="togglePlayers"
      >
        <Users :size="16" />
        {{ t('instance.playerCount', { count: instance.playerCount }) }}
        <ChevronDown
          v-if="playersExpanded"
          :size="14"
        />
        <ChevronRight
          v-else
          :size="14"
        />
      </span>
      <span
        v-if="instance.screenshotCount > 0 && instance.status !== 'event_sync_failed'"
        class="info-item screenshot-count clickable"
        :title="screenshotsExpanded ? t('instance.hidePhotos') : t('instance.showPhotos')"
        @click="toggleScreenshots"
      >
        <Camera :size="16" />
        {{ t('instance.photoCount', { count: instance.screenshotCount }) }}
        <ChevronDown
          v-if="screenshotsExpanded"
          :size="14"
        />
        <ChevronRight
          v-else
          :size="14"
        />
      </span>
    </div>

    <template v-if="instance.status !== 'event_sync_failed'">
      <!-- プレイヤーリスト -->
      <PlayerList
        v-if="playersExpanded && players"
        :players="players"
        :instance="instance"
        @open-user-page="(userId) => emit('openUserPage', userId)"
      />
      <div
        v-else-if="playersExpanded"
        class="loading"
      >
        {{ t('instance.loading') }}
      </div>

      <!-- スクリーンショットリスト -->
      <ScreenshotList
        v-if="screenshotsExpanded && screenshots"
        :screenshots="screenshots"
        @view-screenshot="(filePath) => emit('viewScreenshot', filePath)"
        @open-directory="(filePath) => emit('openDirectory', filePath)"
      />
      <div
        v-else-if="screenshotsExpanded"
        class="loading"
      >
        {{ t('instance.loading') }}
      </div>

      <div class="instance-details">
        <div class="detail-item">
          <span class="label">Instance:</span>
          <span class="value">{{ instance.instanceId }}</span>
        </div>
        <BaseButton @click="emit('openInvite', instance)">
          <ExternalLink :size="16" />
          <span>{{ t('instance.openWorld') }}</span>
        </BaseButton>
      </div>
    </template>
  </BaseCard>
</template>

<style scoped lang="scss">
.instance {
  &-header {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    margin-bottom: 0.5rem;
  }

  &-info {
    display: flex;
    gap: 1rem;
    flex-wrap: wrap;
    font-size: 0.9rem;
    color: var(--text-tertiary);
  }

  &-details {
    margin-top: 0.5rem;
    padding-top: 0.5rem;
    border-top: 1px solid var(--border-subtle);
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
  }
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

  &:hover {
    color: var(--text-primary);
  }
}

.loading {
  text-align: center;
  padding: 1rem;
  color: var(--text-tertiary);
  font-size: 0.9rem;
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

.instance-card.event-sync-failed {
  opacity: 0.7;
}

.error-badge {
  display: inline-block;
  margin-left: 0.5rem;
  padding: 0.125rem 0.5rem;
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--text-secondary);
  background: var(--bg-sunken);
  border: 1px solid var(--border-subtle);
  border-radius: 4px;
}
</style>
