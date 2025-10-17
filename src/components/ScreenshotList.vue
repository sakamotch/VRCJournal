<script setup lang="ts">
import { useI18n } from "vue-i18n";
import type { Screenshot } from "@/types";
import { convertFileSrc } from '@tauri-apps/api/core';
import Button from "./common/Button.vue";
import { Folder, Trash2 } from 'lucide-vue-next';
import dayjs from "dayjs";

const { t } = useI18n();

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
    <h4>{{ t('session.screenshots') }}</h4>
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
          <div class="deleted-text">{{ t('screenshot.deleted') }}</div>
        </div>
        <div class="screenshot-time">
          <span class="screenshot-time-text">{{ formatScreenshotTime(screenshot.takenAt) }}</span>
        </div>
      </div>
    </div>
    <div v-else class="no-screenshots">
      {{ t('screenshot.noScreenshots') }}
    </div>
    <Button
      v-if="screenshots.length > 0"
      @click="emit('openDirectory', screenshots[0].filePath)"
    >
      <Folder :size="16" />
      <span>{{ t('common.openFolder') }}</span>
    </Button>
  </div>
</template>

<style scoped>
.screenshot-list {
  margin-top: 1rem;
  padding: 1rem;
  background: linear-gradient(135deg,
    var(--bg-base) 0%,
    color-mix(in srgb, var(--bg-base) 98%, var(--accent-primary) 2%) 100%
  );
  border-radius: 8px;
  border: 1px solid color-mix(in srgb, var(--border-subtle) 95%, var(--accent-primary-light) 5%);
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
    var(--player-item-bg) 0%,
    color-mix(in srgb, var(--player-item-bg) 97%, var(--accent-primary) 3%) 100%
  );
  border: 2px solid var(--screenshot-border);
  transition: all 0.3s ease;
  box-shadow: var(--shadow-sm);
  color: var(--text-primary);
}

.screenshot-item::before {
  content: '';
  position: absolute;
  inset: 0;
  background: linear-gradient(135deg,
    color-mix(in srgb, var(--bg-hover) 92%, var(--accent-primary-light) 8%) 0%,
    color-mix(in srgb, var(--bg-hover) 96%, var(--accent-secondary-light) 4%) 100%
  );
  opacity: 0;
  transition: opacity 0.3s ease;
  z-index: 0;
}

.screenshot-item:hover {
  border-color: color-mix(in srgb, var(--border-default) 80%, var(--accent-primary-light) 20%);
  box-shadow: 0 4px 12px color-mix(in srgb, var(--accent-primary) 15%, transparent);
  color: var(--interactive-default);
}

.screenshot-item:hover::before {
  opacity: 1;
}

.screenshot-item > * {
  position: relative;
  z-index: 1;
}

.screenshot-item.screenshot-deleted {
  cursor: not-allowed;
  opacity: 0.6;
}

.screenshot-item.screenshot-deleted:hover {
  transform: none;
  border-color: var(--screenshot-border);
}

.screenshot-item.screenshot-deleted:hover::before {
  opacity: 0;
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
  background: linear-gradient(135deg,
    var(--bg-sunken) 0%,
    color-mix(in srgb, var(--bg-sunken) 95%, var(--feedback-error-light) 5%) 100%
  );
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
  position: relative;
  padding: 0.25rem 0.5rem;
  background: linear-gradient(135deg,
    var(--player-item-bg) 0%,
    color-mix(in srgb, var(--player-item-bg) 97%, var(--accent-primary) 3%) 100%
  );
  font-size: 0.75rem;
  font-weight: 500;
  text-align: center;
  overflow: hidden;
}

.screenshot-time::before {
  content: '';
  position: absolute;
  inset: 0;
  background: linear-gradient(135deg,
    color-mix(in srgb, var(--bg-hover) 92%, var(--accent-primary-light) 8%) 0%,
    color-mix(in srgb, var(--bg-hover) 96%, var(--accent-secondary-light) 4%) 100%
  );
  opacity: 0;
  transition: opacity 0.3s ease;
}

.screenshot-time-text {
  position: relative;
  color: var(--text-primary);
}

.screenshot-item:hover .screenshot-time::before {
  opacity: 1;
}

.screenshot-item:hover .screenshot-time-text {
  color: var(--interactive-default);
}

.no-screenshots {
  text-align: center;
  padding: 1rem;
  color: var(--text-tertiary);
  font-size: 0.9rem;
}

</style>
