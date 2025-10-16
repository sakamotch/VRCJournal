<script setup lang="ts">
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

defineProps<Props>();
const emit = defineEmits<Emits>();
</script>

<template>
  <div class="session-list-container">
    <h2>インスタンス履歴</h2>

    <div v-if="isLoading" class="loading">読み込み中...</div>

    <div v-else-if="sessions.length === 0" class="empty">
      インスタンスがありません
    </div>

    <div v-else class="session-list">
      <SessionCard
        v-for="session in sessions"
        :key="session.id"
        :session="session"
        @open-invite="(s) => emit('openInvite', s)"
        @open-user-page="(userId) => emit('openUserPage', userId)"
        @view-screenshot="(filePath) => emit('viewScreenshot', filePath)"
        @open-directory="(filePath) => emit('openDirectory', filePath)"
      />
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
  gap: 1rem;
}
</style>
