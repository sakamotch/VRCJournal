<script setup lang="ts">
defineProps<{
  title?: string;
}>();
</script>

<template>
  <button
    class="icon-button"
    :title="title"
  >
    <slot />
  </button>
</template>

<style scoped lang="scss">
.icon-button {
  background: linear-gradient(135deg,
    var(--overlay-light) 0%,
    var(--overlay-light-medium) 100%
  );
  border: 1px solid var(--overlay-medium-strong);
  border-radius: 8px;
  cursor: pointer;
  padding: 0.375rem;
  transition: all 0.3s ease;
  color: var(--header-text);
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
  overflow: hidden;
  backdrop-filter: blur(8px);

  &::before {
    content: '';
    position: absolute;
    inset: 0;
    background: linear-gradient(135deg,
      var(--overlay-medium-strong) 0%,
      var(--overlay-medium) 100%
    );
    opacity: 0;
    transition: opacity 0.3s ease;
  }

  &:hover {
    border-color: var(--overlay-strong);

    &::before {
      opacity: 1;
    }
  }

  & > :deep(*) {
    position: relative;
    z-index: 1;
  }
}
</style>
