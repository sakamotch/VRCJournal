<script setup lang="ts">
import { computed } from "vue";
import type { Session } from "@/types";
import SessionCard from "./SessionCard.vue";

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

// セッションを日付ごとにグループ化
const sessionsByDate = computed(() => {
  const groups: { date: string; displayDate: string; sessions: Session[] }[] = [];

  props.sessions.forEach(session => {
    const date = new Date(session.startedAt);
    const dateKey = date.toLocaleDateString("ja-JP", {
      year: "numeric",
      month: "2-digit",
      day: "2-digit"
    });

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

function formatDateHeader(date: Date): string {
  const today = new Date();
  const yesterday = new Date(today);
  yesterday.setDate(yesterday.getDate() - 1);

  const dateStr = date.toLocaleDateString("ja-JP");
  const todayStr = today.toLocaleDateString("ja-JP");
  const yesterdayStr = yesterday.toLocaleDateString("ja-JP");

  if (dateStr === todayStr) {
    return `今日 - ${date.toLocaleDateString("ja-JP", { year: "numeric", month: "long", day: "numeric" })}`;
  } else if (dateStr === yesterdayStr) {
    return `昨日 - ${date.toLocaleDateString("ja-JP", { year: "numeric", month: "long", day: "numeric" })}`;
  } else {
    return date.toLocaleDateString("ja-JP", {
      year: "numeric",
      month: "long",
      day: "numeric",
      weekday: "short"
    });
  }
}
</script>

<template>
  <div class="session-list-container">
    <h2>インスタンス履歴</h2>

    <div v-if="isLoading" class="loading">読み込み中...</div>

    <div v-else-if="sessions.length === 0" class="empty">
      インスタンスがありません
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
  color: var(--text-secondary);
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 0.75rem 0;
  position: sticky;
  top: 0;
  z-index: 1;
  background-color: var(--bg-base);
}

.date-header::before,
.date-header::after {
  content: '';
  flex: 1;
  height: 1px;
  background-color: var(--border-default);
}

.date-sessions {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}
</style>
