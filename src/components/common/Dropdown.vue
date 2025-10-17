<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from 'vue';

interface Props {
  modelValue: boolean;
  align?: 'left' | 'right';
}

interface Emits {
  (e: 'update:modelValue', value: boolean): void;
}

const props = withDefaults(defineProps<Props>(), {
  align: 'right',
});

const emit = defineEmits<Emits>();

const dropdownRef = ref<HTMLElement | null>(null);

function handleClickOutside(event: MouseEvent) {
  const target = event.target as HTMLElement;
  if (dropdownRef.value && !dropdownRef.value.contains(target)) {
    emit('update:modelValue', false);
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside);
});

onBeforeUnmount(() => {
  document.removeEventListener('click', handleClickOutside);
});
</script>

<template>
  <div class="dropdown" ref="dropdownRef">
    <div class="dropdown-trigger" @click="emit('update:modelValue', !modelValue)">
      <slot name="trigger" />
    </div>
    <Transition name="dropdown">
      <div v-if="modelValue" :class="['dropdown-menu', `align-${align}`]">
        <slot />
      </div>
    </Transition>
  </div>
</template>

<style scoped>
.dropdown {
  position: relative;
  display: inline-block;
}

.dropdown-trigger {
  cursor: pointer;
}

.dropdown-menu {
  position: absolute;
  top: calc(100% + 0.5rem);
  min-width: 200px;
  background: linear-gradient(135deg,
    var(--bg-surface) 0%,
    color-mix(in srgb, var(--bg-surface) 98%, var(--accent-primary) 2%) 100%
  );
  border: 1px solid color-mix(in srgb, var(--border-default) 80%, var(--accent-primary-light) 20%);
  border-radius: 10px;
  box-shadow: var(--shadow-xl),
              0 0 0 1px color-mix(in srgb, var(--border-default) 70%, var(--accent-primary-light) 30%);
  padding: 0.5rem;
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  max-height: 300px;
  overflow-y: auto;
  backdrop-filter: blur(8px);
  z-index: 100;
}

.dropdown-menu.align-left {
  left: 0;
}

.dropdown-menu.align-right {
  right: 0;
}

/* Transition animations */
.dropdown-enter-active,
.dropdown-leave-active {
  transition: all 0.2s ease;
}

.dropdown-enter-from,
.dropdown-leave-to {
  opacity: 0;
  transform: translateY(-8px);
}

.dropdown-enter-to,
.dropdown-leave-from {
  opacity: 1;
  transform: translateY(0);
}
</style>
