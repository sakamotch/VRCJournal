<script setup lang="ts">
import type { Screenshot } from "@/types";
import { convertFileSrc } from '@tauri-apps/api/core';
import Button from "./common/Button.vue";
import { Folder, Trash2 } from 'lucide-vue-next';
import dayjs from "dayjs";
import "dayjs/locale/ja";

dayjs.locale("ja");

interface Props {
  screenshots: Screenshot[];
}

interface Emits {
  (e: "viewScreenshot", filePath: string): void;
  (e: "openDirectory", filePath: string): void;
}

defineProps<Props>();
const emit = defineEmits<Emits>();

function formatScreenshotTime(dateStr: string): string {
  return dayjs(dateStr).format("HH:mm:ss");
}
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
          <Trash2 :size="32" class="deleted-icon" />
          <div class="deleted-text">削除済み</div>
        </div>
        <div class="screenshot-time">
          {{ formatScreenshotTime(screenshot.takenAt) }}
        </div>
      </div>
    </div>
    <div v-else class="no-screenshots">
      このセッションではスクリーンショットが撮影されていません
    </div>
    <Button
      v-if="screenshots.length > 0"
      @click="emit('openDirectory', screenshots[0].filePath)"
    >
      <Folder :size="16" />
      <span>フォルダを開く</span>
    </Button>
  </div>
</template>

<style scoped>
.screenshot-list {
  margin-top: 1rem;
  padding: 1rem;
  background: linear-gradient(135deg,
    var(--bg-sunken) 0%,
    color-mix(in srgb, var(--bg-sunken) 98%, var(--color-indigo-500) 2%) 100%
  );
  border-radius: 8px;
  border: 1px solid color-mix(in srgb, var(--border-subtle) 95%, var(--color-indigo-400) 5%);
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
  border-radius: 8px;
  overflow: hidden;
  background: linear-gradient(135deg,
    var(--bg-surface) 0%,
    color-mix(in srgb, var(--bg-surface) 97%, var(--color-indigo-500) 3%) 100%
  );
  border: 2px solid var(--screenshot-border);
  transition: all 0.3s ease;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
}

.screenshot-item:hover {
  transform: scale(1.05) translateY(-2px);
  border-color: color-mix(in srgb, var(--screenshot-hover-border) 60%, var(--color-indigo-400) 40%);
  box-shadow: 0 8px 20px rgba(99, 102, 241, 0.2);
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
  margin-bottom: 0.5rem;
  color: var(--text-tertiary);
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

</style>
