<script setup lang="ts">
import { convertFileSrc } from '@tauri-apps/api/core';

interface Props {
  filePath: string | null;
}

interface Emits {
  (e: "close"): void;
}

defineProps<Props>();
const emit = defineEmits<Emits>();
</script>

<template>
  <div v-if="filePath" class="modal-overlay" @click="emit('close')">
    <div class="modal-content" @click.stop>
      <button class="modal-close" @click="emit('close')">×</button>
      <img
        :src="convertFileSrc(filePath)"
        alt="Screenshot"
        class="modal-image"
      />
    </div>
  </div>
</template>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background-color: var(--bg-overlay);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  position: relative;
  max-width: 90vw;
  max-height: 90vh;
  background-color: var(--bg-elevated);
  border-radius: 8px;
  overflow: hidden;
  box-shadow: var(--shadow-xl);
}

.modal-close {
  position: absolute;
  top: -40px;
  right: 0;
  background: none;
  border: none;
  color: var(--text-on-color);
  font-size: 2rem;
  cursor: pointer;
  padding: 0.5rem;
  line-height: 1;
  transition: opacity 0.2s;
}

.modal-close:hover {
  opacity: 0.7;
}

.modal-image {
  max-width: 90vw;
  max-height: 90vh;
  object-fit: contain;
  display: block;
}
</style>
