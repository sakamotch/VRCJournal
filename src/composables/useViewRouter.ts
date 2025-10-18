import { type Component, computed, type Ref } from 'vue';

import type { NavigationView } from '@/components/layout/Navigation.vue';
import InstancesView from '@/components/views/InstancesView.vue';
import PlayersView from '@/components/views/PlayersView.vue';
import PhotosView from '@/components/views/PhotosView.vue';
import StatsView from '@/components/views/StatsView.vue';
import WorldsView from '@/components/views/WorldsView.vue';

const VIEW_COMPONENTS: Record<NavigationView, Component> = {
  timeline: InstancesView,
  worlds: WorldsView,
  players: PlayersView,
  photos: PhotosView,
  stats: StatsView,
};

export function useViewRouter(currentView: Ref<NavigationView>) {
  const currentComponent = computed(() => VIEW_COMPONENTS[currentView.value]);

  return {
    currentComponent,
  };
}
