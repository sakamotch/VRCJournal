import { ref } from 'vue';
import type { NavigationView } from '@/components/layout/Navigation.vue';

export function useAppState() {
  const showSettings = ref(false);
  const currentView = ref<NavigationView>('timeline');

  function navigateToView(view: NavigationView) {
    currentView.value = view;
  }

  return {
    showSettings,
    currentView,
    navigateToView,
  };
}
