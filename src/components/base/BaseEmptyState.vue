<script setup lang="ts">
interface Props {
  title: string;
  description?: string;
  icon?: any;
}

defineProps<Props>();
</script>

<template>
  <div class="empty-state">
    <div class="empty-state-card">
      <component
        :is="icon"
        v-if="icon"
        :size="48"
        class="empty-icon"
      />
      <h2>{{ title }}</h2>
      <p v-if="description">
        {{ description }}
      </p>
      <slot />
    </div>
  </div>
</template>

<style scoped lang="scss">
.empty-state {
  padding: 2rem;
  min-height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: radial-gradient(circle at center,
    color-mix(in srgb, var(--bg-base) 100%, transparent 0%) 0%,
    color-mix(in srgb, var(--bg-base) 98%, var(--accent-primary) 2%) 100%
  );

  &-card {
    text-align: center;
    padding: 3rem;
    background: linear-gradient(135deg,
      color-mix(in srgb, var(--bg-surface) 95%, var(--accent-primary-light) 5%) 0%,
      color-mix(in srgb, var(--bg-surface) 98%, var(--accent-secondary-light) 2%) 100%
    );
    border-radius: 16px;
    border: 1px solid color-mix(in srgb, var(--border-default) 85%, var(--accent-primary-light) 15%);
    box-shadow: 0 8px 32px color-mix(in srgb, var(--accent-primary) 10%, transparent);
    max-width: 500px;
    animation: fade-in-scale 0.5s ease;

    h2 {
      margin: 0 0 1rem;
      color: var(--text-primary);
      font-size: 2rem;
    }

    p {
      margin: 0 0 1.5rem;
      color: var(--text-secondary);
      line-height: 1.6;
    }
  }
}

@keyframes fade-in-scale {
  from {
    opacity: 0;
    transform: scale(0.95);
  }

  to {
    opacity: 1;
    transform: scale(1);
  }
}

.empty-icon {
  color: var(--text-tertiary);
  margin-bottom: 1rem;
}
</style>
