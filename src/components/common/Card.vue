<script setup lang="ts">
interface Props {
  hoverable?: boolean;
  clickable?: boolean;
}

withDefaults(defineProps<Props>(), {
  hoverable: true,
  clickable: false,
});
</script>

<template>
  <div :class="['card', { hoverable, clickable }]">
    <slot />
  </div>
</template>

<style scoped lang="scss">
.card {
  background: linear-gradient(135deg,
    var(--bg-surface) 0%,
    color-mix(in srgb, var(--bg-surface) 95%, var(--accent-primary) 5%) 100%
  );
  border: 1px solid var(--border-subtle);
  border-radius: 12px;
  padding: 1rem;
  transition: all 0.3s ease;
  position: relative;
  overflow: hidden;

  &::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 2px;
    background: linear-gradient(90deg,
      transparent 0%,
      var(--accent-primary-light) 50%,
      transparent 100%
    );
    opacity: 0;
    transition: opacity 0.3s ease;
  }

  &.hoverable:hover {
    box-shadow: 0 8px 24px color-mix(in srgb, var(--accent-primary) 15%, transparent);
    transform: translateY(-2px);
    border-color: color-mix(in srgb, var(--border-default) 70%, var(--accent-primary-light) 30%);

    &::before {
      opacity: 0.6;
    }
  }

  &.clickable {
    cursor: pointer;
  }
}
</style>
