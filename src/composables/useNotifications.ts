import { ref } from 'vue';

export interface Notification {
  id: number;
  message: string;
  type: 'success' | 'error' | 'info';
  duration?: number;
}

const notifications = ref<Notification[]>([]);
let nextId = 1;

export function useNotifications() {
  function add(message: string, type: 'success' | 'error' | 'info' = 'info', duration = 3000) {
    const id = nextId++;
    const notification: Notification = { id, message, type, duration };

    notifications.value.push(notification);

    if (duration > 0) {
      setTimeout(() => {
        remove(id);
      }, duration);
    }

    return id;
  }

  function remove(id: number) {
    const index = notifications.value.findIndex(n => n.id === id);
    if (index !== -1) {
      notifications.value.splice(index, 1);
    }
  }

  function success(message: string, duration?: number) {
    return add(message, 'success', duration);
  }

  function error(message: string, duration?: number) {
    return add(message, 'error', duration);
  }

  function info(message: string, duration?: number) {
    return add(message, 'info', duration);
  }

  return {
    notifications,
    add,
    remove,
    success,
    error,
    info,
  };
}
