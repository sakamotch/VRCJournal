<script setup lang="ts">
interface Props {
  variant?: 'primary' | 'secondary' | 'ghost';
  size?: 'sm' | 'md' | 'lg';
}

withDefaults(defineProps<Props>(), {
  variant: 'secondary',
  size: 'md',
});
</script>

<template>
  <button :class="['btn', `btn-${variant}`, `btn-${size}`]">
    <slot />
  </button>
</template>

<style scoped>
.btn {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  border-radius: 6px;
  cursor: pointer;
  font-weight: 500;
  white-space: nowrap;
  transition: all 0.3s ease;
  position: relative;
  overflow: hidden;
  border: 1px solid var(--border-default);
}

.btn::before {
  content: '';
  position: absolute;
  inset: 0;
  background: linear-gradient(135deg,
    color-mix(in srgb, var(--bg-hover) 90%, var(--accent-primary-light) 10%) 0%,
    color-mix(in srgb, var(--bg-hover) 95%, var(--accent-secondary-light) 5%) 100%
  );
  opacity: 0;
  transition: opacity 0.3s ease;
  z-index: 0;
}

.btn:hover::before {
  opacity: 1;
}

.btn > * {
  position: relative;
  z-index: 1;
}

/* Variants */
.btn-primary {
  background: var(--gradient-indigo);
  color: var(--text-on-color);
  border-color: transparent;
}

.btn-primary:hover {
  box-shadow: 0 4px 12px color-mix(in srgb, var(--accent-primary) 30%, transparent);
  transform: translateY(-1px);
}

.btn-primary::before {
  background: linear-gradient(135deg,
    color-mix(in srgb, var(--accent-primary-dark) 100%, transparent 0%) 0%,
    color-mix(in srgb, var(--accent-secondary-dark) 100%, transparent 0%) 100%
  );
}

.btn-secondary {
  background: linear-gradient(135deg,
    var(--bg-elevated) 0%,
    color-mix(in srgb, var(--bg-elevated) 97%, var(--accent-primary) 3%) 100%
  );
  color: var(--text-primary);
}

.btn-secondary:hover {
  border-color: color-mix(in srgb, var(--border-default) 70%, var(--accent-primary-light) 30%);
  transform: translateY(-1px);
  box-shadow: 0 4px 12px color-mix(in srgb, var(--accent-primary) 10%, transparent);
  color: var(--interactive-default);
}

.btn-ghost {
  background: transparent;
  border: none;
}

.btn-ghost:hover {
  color: var(--interactive-default);
}

/* Sizes */
.btn-sm {
  padding: 0.375rem 0.625rem;
  font-size: 0.8125rem;
}

.btn-md {
  padding: 0.5rem 0.875rem;
  font-size: 0.875rem;
}

.btn-lg {
  padding: 0.625rem 1rem;
  font-size: 0.9375rem;
}
</style>
