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
  <Transition name="modal">
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
  </Transition>
</template>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: radial-gradient(circle at center,
    color-mix(in srgb, var(--bg-overlay) 100%, transparent 0%) 0%,
    color-mix(in srgb, var(--bg-overlay) 92%, var(--interactive-subtle) 8%) 100%
  );
  backdrop-filter: blur(8px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  position: relative;
  max-width: 90vw;
  max-height: 90vh;
  background: linear-gradient(135deg,
    var(--bg-elevated) 0%,
    color-mix(in srgb, var(--bg-elevated) 98%, var(--accent-primary) 2%) 100%
  );
  border-radius: 16px;
  overflow: hidden;
  box-shadow: 0 20px 80px var(--scrim-heavy),
              0 0 0 1px color-mix(in srgb, var(--border-default) 70%, var(--accent-primary-light) 30%);
}

/* Transition classes */
.modal-enter-active {
  animation: modalFadeIn 0.2s ease;
}

.modal-enter-active .modal-content {
  animation: modalZoomIn 0.3s ease;
}

.modal-leave-active {
  animation: modalFadeOut 0.2s ease;
}

.modal-leave-active .modal-content {
  animation: modalZoomOut 0.2s ease;
}

@keyframes modalFadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

@keyframes modalFadeOut {
  from {
    opacity: 1;
  }
  to {
    opacity: 0;
  }
}

@keyframes modalZoomIn {
  from {
    transform: scale(0.95);
    opacity: 0;
  }
  to {
    transform: scale(1);
    opacity: 1;
  }
}

@keyframes modalZoomOut {
  from {
    transform: scale(1);
    opacity: 1;
  }
  to {
    transform: scale(0.95);
    opacity: 0;
  }
}

.modal-close {
  position: absolute;
  top: -50px;
  right: 0;
  background: linear-gradient(135deg,
    color-mix(in srgb, var(--scrim-strong) 90%, var(--feedback-error-light) 10%) 0%,
    color-mix(in srgb, var(--scrim-strong) 95%, var(--feedback-error-dark) 5%) 100%
  );
  backdrop-filter: blur(8px);
  border: 1px solid var(--overlay-medium);
  border-radius: 50%;
  color: var(--text-on-color);
  font-size: 2rem;
  cursor: pointer;
  padding: 0.5rem;
  line-height: 1;
  width: 3rem;
  height: 3rem;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.3s ease;
}

.modal-close:hover {
  background: linear-gradient(135deg,
    color-mix(in srgb, var(--scrim-heavy) 80%, var(--feedback-error-light) 20%) 0%,
    color-mix(in srgb, var(--scrim-heavy) 90%, var(--feedback-error-dark) 10%) 100%
  );
  transform: rotate(90deg);
  box-shadow: 0 4px 12px color-mix(in srgb, var(--feedback-error-dark) 30%, transparent);
}

.modal-image {
  max-width: 90vw;
  max-height: 90vh;
  object-fit: contain;
  display: block;
}
</style>
