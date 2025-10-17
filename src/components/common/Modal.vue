<script setup lang="ts">
interface Props {
  show: boolean;
  title?: string;
  maxWidth?: string;
}

interface Emits {
  (e: 'close'): void;
}

withDefaults(defineProps<Props>(), {
  maxWidth: '600px',
});

const emit = defineEmits<Emits>();
</script>

<template>
  <Teleport to="body">
    <Transition name="modal">
      <div v-if="show" class="modal-overlay" @click="emit('close')">
        <div class="modal-panel" :style="{ maxWidth }" @click.stop>
          <div v-if="title" class="modal-header">
            <h2>{{ title }}</h2>
            <button class="close-button" @click="emit('close')">×</button>
          </div>
          <div class="modal-content">
            <slot />
          </div>
          <div v-if="$slots.footer" class="modal-footer">
            <slot name="footer" />
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
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
    color-mix(in srgb, var(--bg-overlay) 95%, var(--color-indigo-900) 5%) 100%
  );
  backdrop-filter: blur(4px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-panel {
  width: 90%;
  max-height: 80vh;
  background: linear-gradient(135deg,
    var(--bg-surface) 0%,
    color-mix(in srgb, var(--bg-surface) 98%, var(--color-indigo-500) 2%) 100%
  );
  border-radius: 16px;
  box-shadow: 0 20px 60px var(--alpha-black-32),
              0 0 0 1px color-mix(in srgb, var(--border-default) 80%, var(--color-indigo-400) 20%);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.5rem;
  border-bottom: 2px solid transparent;
  border-image: linear-gradient(90deg,
    transparent 0%,
    color-mix(in srgb, var(--border-default) 70%, var(--color-indigo-400) 30%) 30%,
    color-mix(in srgb, var(--border-default) 50%, var(--color-violet-400) 50%) 50%,
    color-mix(in srgb, var(--border-default) 70%, var(--color-indigo-400) 30%) 70%,
    transparent 100%
  ) 1;
}

.modal-header h2 {
  margin: 0;
  font-size: 1.25rem;
  color: var(--text-primary);
}

.close-button {
  background: none;
  border: none;
  font-size: 2rem;
  color: var(--text-secondary);
  cursor: pointer;
  padding: 0;
  width: 2rem;
  height: 2rem;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 8px;
  transition: all 0.3s ease;
  position: relative;
  overflow: hidden;
}

.close-button::before {
  content: '';
  position: absolute;
  inset: 0;
  background: linear-gradient(135deg,
    color-mix(in srgb, var(--bg-hover) 90%, var(--color-red-400) 10%) 0%,
    color-mix(in srgb, var(--bg-hover) 95%, var(--color-red-500) 5%) 100%
  );
  opacity: 0;
  transition: opacity 0.3s ease;
  border-radius: 8px;
}

.close-button:hover {
  color: var(--feedback-error);
}

.close-button:hover::before {
  opacity: 1;
}

.modal-content {
  padding: 1.5rem;
  overflow-y: auto;
  flex: 1;
}

.modal-footer {
  padding: 1rem 1.5rem;
  border-top: 1px solid var(--border-subtle);
  display: flex;
  justify-content: flex-end;
  gap: 0.75rem;
}

/* Transition animations */
.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.2s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

.modal-enter-active .modal-panel {
  animation: slideUp 0.3s ease;
}

.modal-leave-active .modal-panel {
  animation: slideDown 0.2s ease;
}

@keyframes slideUp {
  from {
    transform: translateY(20px);
    opacity: 0;
  }
  to {
    transform: translateY(0);
    opacity: 1;
  }
}

@keyframes slideDown {
  from {
    transform: translateY(0);
    opacity: 1;
  }
  to {
    transform: translateY(20px);
    opacity: 0;
  }
}
</style>
