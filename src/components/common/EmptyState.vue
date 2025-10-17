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
      <component v-if="icon" :is="icon" :size="48" class="empty-icon" />
      <h2>{{ title }}</h2>
      <p v-if="description">{{ description }}</p>
      <slot />
    </div>
  </div>
</template>

<style scoped>
.empty-state {
  padding: 2rem;
  min-height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: radial-gradient(circle at center,
    color-mix(in srgb, var(--bg-base) 100%, transparent 0%) 0%,
    color-mix(in srgb, var(--bg-base) 98%, var(--color-indigo-500) 2%) 100%
  );
}

.empty-state-card {
  text-align: center;
  padding: 3rem;
  background: linear-gradient(135deg,
    color-mix(in srgb, var(--bg-surface) 95%, var(--color-indigo-400) 5%) 0%,
    color-mix(in srgb, var(--bg-surface) 98%, var(--color-violet-400) 2%) 100%
  );
  border-radius: 16px;
  border: 1px solid color-mix(in srgb, var(--border-default) 85%, var(--color-indigo-400) 15%);
  box-shadow: 0 8px 32px color-mix(in srgb, var(--color-indigo-500) 10%, transparent);
  max-width: 500px;
  animation: fadeInScale 0.5s ease;
}

@keyframes fadeInScale {
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

.empty-state-card h2 {
  margin: 0 0 1rem 0;
  background: linear-gradient(135deg,
    var(--text-primary) 0%,
    color-mix(in srgb, var(--text-primary) 80%, var(--color-indigo-400) 20%) 100%
  );
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  font-size: 2rem;
}

.empty-state-card p {
  margin: 0 0 1.5rem 0;
  color: var(--text-secondary);
  line-height: 1.6;
}
</style>
