<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { Calendar, Globe, Users, Camera, BarChart3 } from 'lucide-vue-next';

const { t } = useI18n();

export type NavigationView = 'timeline' | 'worlds' | 'people' | 'photos' | 'stats';

interface Props {
  currentView: NavigationView;
}

interface Emits {
  (e: 'navigate', view: NavigationView): void;
}

defineProps<Props>();
const emit = defineEmits<Emits>();

const navItems = computed(() => [
  { id: 'timeline' as NavigationView, label: t('navigation.instances'), icon: Calendar },
  { id: 'worlds' as NavigationView, label: t('navigation.worlds'), icon: Globe },
  { id: 'people' as NavigationView, label: t('navigation.people'), icon: Users },
  { id: 'photos' as NavigationView, label: t('navigation.photos'), icon: Camera },
  { id: 'stats' as NavigationView, label: t('navigation.stats'), icon: BarChart3 },
]);
</script>

<template>
  <nav class="navigation">
    <button
      v-for="item in navItems"
      :key="item.id"
      :class="['nav-item', { active: currentView === item.id }]"
      @click="emit('navigate', item.id)"
      :title="item.label"
    >
      <component :is="item.icon" :size="24" class="nav-icon" />
      <span class="nav-label">{{ item.label }}</span>
    </button>
  </nav>
</template>

<style scoped lang="scss">
.navigation {
  width: 72px;
  background: linear-gradient(180deg,
    var(--bg-surface) 0%,
    color-mix(in srgb, var(--bg-surface) 97%, var(--accent-primary) 3%) 100%
  );
  border-right: 1px solid var(--border-default);
  display: flex;
  flex-direction: column;
  padding: 1rem 0;
  gap: 0.5rem;
}

.nav-item {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0.75rem;
  background: none;
  border: none;
  cursor: pointer;
  color: var(--text-secondary);
  transition: all 0.3s ease;
  border-left: 3px solid transparent;
  position: relative;

  &::before {
    content: '';
    position: absolute;
    inset: 4px;
    background: linear-gradient(135deg,
      transparent 0%,
      color-mix(in srgb, transparent 90%, var(--accent-primary-light) 10%) 100%
    );
    border-radius: 8px;
    opacity: 0;
    transition: opacity 0.3s ease;
    z-index: -1;
  }

  &:hover {
    color: var(--text-primary);

    &::before {
      opacity: 1;
    }
  }

  &.active {
    color: var(--interactive-default);
    font-weight: 600;
    border-left-color: var(--interactive-default);

    &::before {
      background: linear-gradient(135deg,
        color-mix(in srgb, var(--bg-elevated) 85%, var(--accent-primary-light) 15%) 0%,
        color-mix(in srgb, var(--bg-elevated) 90%, var(--accent-secondary-light) 10%) 100%
      );
      opacity: 1;
    }
  }
}

.nav-icon {
  flex-shrink: 0;
}

.nav-label {
  display: none;
}
</style>
