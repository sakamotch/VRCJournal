<script setup lang="ts">
import type { Screenshot } from "@/types";
import { convertFileSrc } from '@tauri-apps/api/core';

interface Props {
  screenshots: Screenshot[];
}

interface Emits {
  (e: "viewScreenshot", filePath: string): void;
  (e: "openDirectory", filePath: string): void;
}

defineProps<Props>();
const emit = defineEmits<Emits>();
</script>

<template>
  <div class="screenshot-list">
    <h4>スクリーンショット</h4>
    <div v-if="screenshots.length > 0" class="screenshot-grid">
      <div
        v-for="screenshot in screenshots"
        :key="screenshot.id"
        class="screenshot-item"
        :class="{ 'screenshot-deleted': !screenshot.exists }"
        @click="screenshot.exists && emit('viewScreenshot', screenshot.filePath)"
      >
        <img
          v-if="screenshot.exists"
          :src="convertFileSrc(screenshot.filePath)"
          :alt="`Screenshot ${screenshot.id}`"
          class="screenshot-thumbnail"
        />
        <div v-else class="screenshot-deleted-placeholder">
          <div class="deleted-icon">🗑️</div>
          <div class="deleted-text">削除済み</div>
        </div>
        <div class="screenshot-time">
          {{ new Date(screenshot.takenAt).toLocaleTimeString('ja-JP') }}
        </div>
      </div>
    </div>
    <div v-else class="no-screenshots">
      このセッションではスクリーンショットが撮影されていません
    </div>
    <button
      v-if="screenshots.length > 0"
      @click="emit('openDirectory', screenshots[0].filePath)"
      class="open-folder-button"
    >
      📁 フォルダを開く
    </button>
  </div>
</template>

<style scoped>
.screenshot-list {
  margin-top: 1rem;
  padding: 1rem;
  background-color: var(--bg-sunken);
  border-radius: 4px;
}

.screenshot-list h4 {
  margin: 0 0 0.75rem 0;
  font-size: 0.95rem;
  color: var(--text-secondary);
}

.screenshot-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
  gap: 0.75rem;
  margin-bottom: 0.75rem;
}

.screenshot-item {
  position: relative;
  cursor: pointer;
  border-radius: 4px;
  overflow: hidden;
  background-color: var(--bg-surface);
  border: 1px solid var(--screenshot-border);
  transition: transform 0.2s, border-color 0.2s;
}

.screenshot-item:hover {
  transform: scale(1.05);
  border-color: var(--screenshot-hover-border);
}

.screenshot-item.screenshot-deleted {
  cursor: not-allowed;
  opacity: 0.6;
}

.screenshot-item.screenshot-deleted:hover {
  transform: none;
  border-color: var(--screenshot-border);
}

.screenshot-thumbnail {
  width: 100%;
  height: 120px;
  object-fit: cover;
  display: block;
}

.screenshot-deleted-placeholder {
  width: 100%;
  height: 120px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  background-color: var(--bg-hover);
}

.deleted-icon {
  font-size: 2rem;
  margin-bottom: 0.25rem;
}

.deleted-text {
  font-size: 0.8rem;
  color: var(--text-tertiary);
}

.screenshot-time {
  padding: 0.25rem 0.5rem;
  background-color: var(--alpha-black-80);
  color: white;
  font-size: 0.75rem;
  text-align: center;
}

.no-screenshots {
  text-align: center;
  padding: 1rem;
  color: var(--text-tertiary);
  font-size: 0.9rem;
}

.open-folder-button {
  padding: 0.4rem 0.8rem;
  background-color: var(--color-purple-600);
  color: var(--text-on-color);
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.85rem;
  transition: background-color 0.2s;
}

.open-folder-button:hover {
  background-color: var(--color-purple-700);
}
</style>
