<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import type { Session } from "@/types";
import SessionCard from "./SessionCard.vue";
import dayjs from "dayjs";

const { t, locale } = useI18n();

interface Props {
  sessions: Session[];
  isLoading: boolean;
}

interface Emits {
  (e: "openInvite", session: Session): void;
  (e: "openUserPage", userId: string): void;
  (e: "viewScreenshot", filePath: string): void;
  (e: "openDirectory", filePath: string): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

const sessionsByDate = computed(() => {
  // locale.valueを依存関係に追加してロケール変更時に再計算
  locale.value;

  const groups: { date: string; displayDate: string; sessions: Session[] }[] = [];

  props.sessions.forEach(session => {
    const date = dayjs(session.startedAt);
    const dateKey = date.format("YYYY/MM/DD");

    const displayDate = formatDateHeader(date);

    let group = groups.find(g => g.date === dateKey);
    if (!group) {
      group = { date: dateKey, displayDate, sessions: [] };
      groups.push(group);
    }
    group.sessions.push(session);
  });

  return groups;
});

function formatDateHeader(date: dayjs.Dayjs): string {
  const today = dayjs();
  const yesterday = dayjs().subtract(1, "day");

  if (date.isSame(today, "day")) {
    return `${t('common.today')} - ${date.format("LL")}`;
  } else if (date.isSame(yesterday, "day")) {
    return `${t('common.yesterday')} - ${date.format("LL")}`;
  } else {
    return date.format("LL (ddd)");
  }
}
</script>

<template>
  <div class="session-list-container">
    <h2>{{ t('session.title') }}</h2>

    <div v-if="isLoading" class="loading">{{ t('common.loading') }}</div>

    <div v-else-if="sessions.length === 0" class="empty">
      {{ t('session.noSessions') }}
    </div>

    <div v-else class="session-list">
      <div v-for="group in sessionsByDate" :key="group.date" class="date-group">
        <div class="date-header">
          {{ group.displayDate }}
        </div>
        <div class="date-sessions">
          <SessionCard
            v-for="session in group.sessions"
            :key="session.id"
            :session="session"
            @open-invite="(s) => emit('openInvite', s)"
            @open-user-page="(userId) => emit('openUserPage', userId)"
            @view-screenshot="(filePath) => emit('viewScreenshot', filePath)"
            @open-directory="(filePath) => emit('openDirectory', filePath)"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.session-list-container {
  flex: 1;
  padding: 1.5rem;
  overflow-y: auto;
}

.session-list-container h2 {
  margin: 0 0 1rem 0;
  font-size: 1.3rem;
  color: var(--text-primary);
}

.loading, .empty {
  text-align: center;
  padding: 2rem;
  color: var(--text-tertiary);
}

.session-list {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.date-group {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.date-header {
  font-size: 0.875rem;
  font-weight: 600;
  background: linear-gradient(90deg,
    transparent 0%,
    color-mix(in srgb, var(--text-secondary) 100%, transparent 0%) 50%,
    transparent 100%
  );
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 0.75rem 0;
  position: sticky;
  top: 0;
  z-index: 1;
  backdrop-filter: blur(8px);
  background-color: color-mix(in srgb, var(--bg-base) 90%, transparent 10%);
}

.date-header::before,
.date-header::after {
  content: '';
  flex: 1;
  height: 2px;
  background: linear-gradient(90deg,
    transparent 0%,
    color-mix(in srgb, var(--border-default) 70%, var(--accent-primary-light) 30%) 30%,
    color-mix(in srgb, var(--border-default) 50%, var(--accent-secondary-light) 50%) 50%,
    color-mix(in srgb, var(--border-default) 70%, var(--accent-primary-light) 30%) 70%,
    transparent 100%
  );
  border-radius: 1px;
}

.date-sessions {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}
</style>
