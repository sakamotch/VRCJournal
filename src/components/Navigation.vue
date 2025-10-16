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
  width: 200px;
  background-color: var(--bg-surface);
  border-right: 1px solid var(--border-default);
  display: flex;
  flex-direction: column;
  padding: 1rem 0;
  gap: 0.25rem;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.75rem 1rem;
  background: none;
  border: none;
  cursor: pointer;
  color: var(--text-secondary);
  font-size: 0.875rem;
  font-weight: 500;
  transition: all 0.2s;
  border-left: 3px solid transparent;
}

.nav-item:hover {
  background-color: var(--bg-hover);
  color: var(--text-primary);
}

.nav-item.active {
  background-color: var(--bg-elevated);
  color: var(--text-primary);
  border-left-color: var(--interactive-default);
}

.nav-icon {
  flex-shrink: 0;
}

.nav-label {
  white-space: nowrap;
}
</style>
