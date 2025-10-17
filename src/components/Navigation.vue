<script setup lang="ts">
import { Calendar, Globe, Users, Camera, BarChart3 } from 'lucide-vue-next';

export type NavigationView = 'timeline' | 'worlds' | 'people' | 'photos' | 'stats';

interface Props {
  currentView: NavigationView;
}

interface Emits {
  (e: 'navigate', view: NavigationView): void;
}

defineProps<Props>();
const emit = defineEmits<Emits>();

const navItems: { id: NavigationView; label: string; icon: any }[] = [
  { id: 'timeline', label: 'タイムライン', icon: Calendar },
  { id: 'worlds', label: 'ワールド', icon: Globe },
  { id: 'people', label: '人物', icon: Users },
  { id: 'photos', label: 'フォト', icon: Camera },
  { id: 'stats', label: '統計', icon: BarChart3 },
];
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

<style scoped>
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
}

.nav-item::before {
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

.nav-item:hover {
  color: var(--text-primary);
}

.nav-item:hover::before {
  opacity: 1;
}

.nav-item.active {
  color: var(--interactive-default);
  font-weight: 600;
  border-left-color: var(--interactive-default);
}

.nav-item.active::before {
  background: linear-gradient(135deg,
    color-mix(in srgb, var(--bg-elevated) 85%, var(--accent-primary-light) 15%) 0%,
    color-mix(in srgb, var(--bg-elevated) 90%, var(--accent-secondary-light) 10%) 100%
  );
  opacity: 1;
}

.nav-icon {
  flex-shrink: 0;
}

.nav-label {
  display: none;
}
</style>
