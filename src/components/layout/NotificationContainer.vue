<script setup lang="ts">
import { useNotifications } from '@/composables/useNotifications';
import { CheckCircle2, XCircle, Info, X } from 'lucide-vue-next';

const { notifications, remove } = useNotifications();

const getIcon = (type: string) => {
  switch (type) {
    case 'success': return CheckCircle2;
    case 'error': return XCircle;
    case 'info': return Info;
    default: return Info;
  }
};
</script>

<template>
  <div class="notification-container">
    <TransitionGroup name="notification">
      <div
        v-for="notification in notifications"
        :key="notification.id"
        :class="['notification', `notification-${notification.type}`]"
      >
        <div class="notification-icon">
          <component :is="getIcon(notification.type)" :size="20" />
        </div>
        <div class="notification-message">{{ notification.message }}</div>
        <button class="notification-close" @click="remove(notification.id)">
          <X :size="16" />
        </button>
      </div>
    </TransitionGroup>
  </div>
</template>

<style scoped lang="scss">
.notification {
  &-container {
    position: fixed;
    bottom: 1.5rem;
    right: 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    z-index: 1000;
    pointer-events: none;
    max-width: 420px;
    width: calc(100vw - 3rem);
  }

  display: flex;
  align-items: center;
  gap: 0.875rem;
  padding: 1rem 1.25rem;
  background: linear-gradient(135deg,
    color-mix(in srgb, var(--bg-elevated) 96%, var(--accent-primary) 4%) 0%,
    var(--bg-elevated) 100%
  );
  border: 1px solid var(--border-default);
  border-radius: 12px;
  box-shadow:
    0 12px 32px -8px var(--scrim-medium),
    0 4px 12px -2px var(--scrim-light),
    0 0 0 1px color-mix(in srgb, var(--border-default) 70%, var(--notification-color) 30%);
  backdrop-filter: blur(12px) saturate(150%);
  pointer-events: auto;
  position: relative;
  overflow: hidden;

  &::before {
    content: '';
    position: absolute;
    left: 0;
    top: 0;
    bottom: 0;
    width: 3px;
    background: var(--notification-color);
    border-radius: 12px 0 0 12px;
    pointer-events: none;
  }

  &::after {
    content: '';
    position: absolute;
    inset: 0;
    background: linear-gradient(135deg,
      color-mix(in srgb, var(--notification-color) 8%, transparent 92%) 0%,
      transparent 100%
    );
    pointer-events: none;
    border-radius: 12px;
  }

  &-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    border-radius: 8px;
    background: linear-gradient(135deg,
      color-mix(in srgb, var(--notification-color) 15%, transparent 85%) 0%,
      color-mix(in srgb, var(--notification-color) 8%, transparent 92%) 100%
    );
    color: var(--notification-color);
    flex-shrink: 0;
    position: relative;
    z-index: 1;
  }

  &-message {
    flex: 1;
    color: var(--text-primary);
    font-size: 0.875rem;
    line-height: 1.5;
    font-weight: 500;
    position: relative;
    z-index: 1;
    word-break: break-word;
  }

  &-close {
    background: none;
    border: none;
    padding: 0.25rem;
    cursor: pointer;
    color: var(--text-tertiary);
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    transition: all 0.2s ease;
    position: relative;
    z-index: 1;

    &:hover {
      background: var(--bg-hover);
      color: var(--text-primary);
    }

    &:active {
      transform: scale(0.9);
    }
  }

  /* Variants */
  &-success {
    --notification-color: var(--feedback-success);
  }

  &-error {
    --notification-color: var(--feedback-error);
  }

  &-info {
    --notification-color: var(--feedback-info);
  }

  /* Transitions */
  &-enter-active {
    animation: notificationSlideIn 0.4s cubic-bezier(0.16, 1, 0.3, 1);
  }

  &-leave-active {
    animation: notificationSlideOut 0.3s cubic-bezier(0.4, 0, 1, 1);
  }

  &-move {
    transition: transform 0.4s cubic-bezier(0.16, 1, 0.3, 1);
  }
}

@keyframes notificationSlideIn {
  from {
    opacity: 0;
    transform: translateX(calc(100% + 3rem)) scale(0.9);
  }
  to {
    opacity: 1;
    transform: translateX(0) scale(1);
  }
}

@keyframes notificationSlideOut {
  from {
    opacity: 1;
    transform: translateX(0) scale(1);
  }
  to {
    opacity: 0;
    transform: translateX(calc(100% + 3rem)) scale(0.9);
  }
}
</style>
