import { type Component, computed, type Ref } from 'vue';

import type { NavigationView } from '@/components/app/AppNavigation.vue';
import InstancesView from '@/features/instances/InstancesView.vue';
import PhotosView from '@/features/photos/PhotosView.vue';
import PlayersView from '@/features/players/PlayersView.vue';
import StatsView from '@/features/stats/StatsView.vue';
import WorldsView from '@/features/worlds/WorldsView.vue';

const VIEW_COMPONENTS: Record<NavigationView, Component> = {
  timeline: InstancesView,
  worlds: WorldsView,
  players: PlayersView,
  photos: PhotosView,
  stats: StatsView,
};

export function useRouter(currentView: Ref<NavigationView>) {
  const currentComponent = computed(() => VIEW_COMPONENTS[currentView.value]);

  return {
    currentComponent,
  };
}
