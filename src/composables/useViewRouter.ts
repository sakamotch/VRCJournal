import { computed, type Component, type Ref } from 'vue';
import type { NavigationView } from '@/components/layout/Navigation.vue';
import InstancesView from '@/components/views/InstancesView.vue';
import WorldsView from '@/components/views/WorldsView.vue';
import PeopleView from '@/components/views/PeopleView.vue';
import PhotosView from '@/components/views/PhotosView.vue';
import StatsView from '@/components/views/StatsView.vue';

const VIEW_COMPONENTS: Record<NavigationView, Component> = {
  timeline: InstancesView,
  worlds: WorldsView,
  people: PeopleView,
  photos: PhotosView,
  stats: StatsView,
};

export function useViewRouter(currentView: Ref<NavigationView>) {
  const currentComponent = computed(() => VIEW_COMPONENTS[currentView.value]);

  return {
    currentComponent,
  };
}
