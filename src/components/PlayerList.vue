<script setup lang="ts">
import type { Player, Session } from "@/types";
import { formatPlayerName, formatTime, isPlayerStayedUntilEnd } from "@/utils/formatters";

interface Props {
  players: Player[];
  session: Session;
}

interface Emits {
  (e: "openUserPage", userId: string): void;
}

defineProps<Props>();
const emit = defineEmits<Emits>();
</script>

<template>
  <div class="player-list">
    <h4>Join履歴</h4>
    <div class="player-items">
      <div
        v-for="player in players"
        :key="`${player.id}-${player.joinedAt}`"
        class="player-item"
        :class="{ 'player-stayed': isPlayerStayedUntilEnd(player, session) }"
      >
        <div class="player-info" @click="emit('openUserPage', player.userId)">
          <span class="player-name">{{ formatPlayerName(player) }}</span>
          <span class="player-icon">🔗</span>
        </div>
        <div class="player-times">
          <span class="player-time">入室: {{ formatTime(player.joinedAt) }}</span>
          <span class="player-time" v-if="player.leftAt">退室: {{ formatTime(player.leftAt) }}</span>
          <span class="player-time player-time-active" v-else>在室中</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.player-list {
  margin-top: 1rem;
  padding: 1rem;
  background-color: var(--player-bg);
  border-radius: 4px;
}

.player-list h4 {
  margin: 0 0 0.75rem 0;
  font-size: 0.95rem;
  color: var(--text-secondary);
}

.player-items {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.player-item {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  padding: 0.5rem 0.75rem;
  background-color: var(--player-item-bg);
  border-radius: 4px;
  transition: all 0.2s;
}

.player-item:hover {
  background-color: var(--player-hover);
}

.player-info {
  display: flex;
  justify-content: space-between;
  align-items: center;
  cursor: pointer;
}

.player-info:hover {
  opacity: 0.8;
}

.player-times {
  display: flex;
  gap: 1rem;
  font-size: 0.75rem;
  color: var(--text-tertiary);
}

.player-time {
  white-space: nowrap;
}

.player-time-active {
  color: var(--status-active);
  font-weight: 500;
}

.player-name {
  font-weight: 500;
  color: var(--text-primary);
}

.player-icon {
  opacity: 0.5;
  transition: opacity 0.2s;
}

.player-info:hover .player-icon {
  opacity: 1;
}

.player-item.player-stayed {
  border-left: 3px solid var(--player-stayed-border);
  background-color: var(--player-stayed-bg);
}

.player-item.player-stayed:hover {
  background-color: var(--player-stayed-hover);
}
</style>
