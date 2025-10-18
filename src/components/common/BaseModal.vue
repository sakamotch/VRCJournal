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
  title: undefined,
  maxWidth: '600px',
});

const emit = defineEmits<Emits>();
</script>

<template>
  <Teleport to="body">
    <Transition name="modal">
      <div
        v-if="show"
        class="modal-overlay"
        @click="emit('close')"
      >
        <div
          class="modal-panel"
          :style="{ maxWidth }"
          @click.stop
        >
          <div
            v-if="title"
            class="modal-header"
          >
            <h2>{{ title }}</h2>
            <button
              class="close-button"
              @click="emit('close')"
            >
              ×
            </button>
          </div>
          <div class="modal-content">
            <slot />
          </div>
          <div
            v-if="$slots.footer"
            class="modal-footer"
          >
            <slot name="footer" />
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped lang="scss">
.modal {
  &-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background: radial-gradient(circle at center,
      color-mix(in srgb, var(--bg-overlay) 100%, transparent 0%) 0%,
      color-mix(in srgb, var(--bg-overlay) 95%, var(--interactive-subtle) 5%) 100%
    );
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  &-panel {
    width: 90%;
    max-height: 80vh;
    background: linear-gradient(135deg,
      var(--bg-surface) 0%,
      color-mix(in srgb, var(--bg-surface) 98%, var(--accent-primary) 2%) 100%
    );
    border-radius: 16px;
    box-shadow: 0 20px 60px var(--scrim-strong),
                0 0 0 1px color-mix(in srgb, var(--border-default) 80%, var(--accent-primary-light) 20%);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  &-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem;
    border-bottom: 2px solid transparent;
    border-image: linear-gradient(90deg,
      transparent 0%,
      color-mix(in srgb, var(--border-default) 70%, var(--accent-primary-light) 30%) 30%,
      color-mix(in srgb, var(--border-default) 50%, var(--accent-secondary-light) 50%) 50%,
      color-mix(in srgb, var(--border-default) 70%, var(--accent-primary-light) 30%) 70%,
      transparent 100%
    ) 1;

    h2 {
      margin: 0;
      font-size: 1.25rem;
      color: var(--text-primary);
    }
  }

  &-content {
    padding: 1.5rem;
    overflow-y: auto;
    flex: 1;
  }

  &-footer {
    padding: 1rem 1.5rem;
    border-top: 1px solid var(--border-subtle);
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
  }
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

  &::before {
    content: '';
    position: absolute;
    inset: 0;
    background: linear-gradient(135deg,
      color-mix(in srgb, var(--bg-hover) 90%, var(--feedback-error-light) 10%) 0%,
      color-mix(in srgb, var(--bg-hover) 95%, var(--feedback-error-dark) 5%) 100%
    );
    opacity: 0;
    transition: opacity 0.3s ease;
    border-radius: 8px;
  }

  &:hover {
    color: var(--feedback-error);

    &::before {
      opacity: 1;
    }
  }
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
  animation: slide-up 0.3s ease;
}

.modal-leave-active .modal-panel {
  animation: slide-down 0.2s ease;
}

@keyframes slide-up {
  from {
    transform: translateY(20px);
    opacity: 0;
  }

  to {
    transform: translateY(0);
    opacity: 1;
  }
}

@keyframes slide-down {
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
