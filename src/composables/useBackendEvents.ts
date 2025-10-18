import { ref, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';

export interface BackendEventHandlers {
  onBackendReady?: () => void | Promise<void>;
  onLocalPlayerUpdated?: () => void;
  onInstanceCreated?: () => void;
  onInstanceEnded?: (instanceId: number, endedAt: string) => void;
  onPlayerJoined?: () => void;
  onPlayerLeft?: () => void;
}

export function useBackendEvents(handlers: BackendEventHandlers) {
  const isBackendReady = ref(false);
  let unlistenFn: UnlistenFn | null = null;
  let unlistenReadyFn: UnlistenFn | null = null;

  async function checkInitialReadyState() {
    try {
      const ready = await invoke<boolean>('is_backend_ready');
      if (ready) {
        isBackendReady.value = true;
        await handlers.onBackendReady?.();
      }
    } catch (err) {
      console.error('Failed to check backend ready status:', err);
    }
  }

  async function setupListeners() {
    unlistenReadyFn = await listen('backend-ready', async () => {
      if (!isBackendReady.value) {
        isBackendReady.value = true;
        await handlers.onBackendReady?.();
      }
    });

    unlistenFn = await listen<any>('log-event', (event) => {
      const processedEvent = event.payload;

      switch (processedEvent.type) {
        case 'LocalPlayerUpdated':
          handlers.onLocalPlayerUpdated?.();
          break;

        case 'InstanceCreated':
          handlers.onInstanceCreated?.();
          break;

        case 'InstanceEnded':
          handlers.onInstanceEnded?.(processedEvent.instance_id, processedEvent.ended_at);
          break;

        case 'PlayerJoined':
        case 'PlayerLeft':
          handlers.onPlayerJoined?.();
          break;
      }
    });
  }

  function cleanup() {
    if (unlistenFn) {
      unlistenFn();
    }
    if (unlistenReadyFn) {
      unlistenReadyFn();
    }
  }

  onMounted(async () => {
    await setupListeners();
    await checkInitialReadyState();
  });

  onUnmounted(() => {
    cleanup();
  });

  return {
    isBackendReady,
  };
}
