<script setup lang="ts">
import { useBackendEvents } from "@/composables/useBackendEvents";
import DefaultLayout from "@/layouts/DefaultLayout.vue";
import { useUserStore } from "@/stores/userStore";

const userStore = useUserStore();
const { loadUsers } = userStore;

useBackendEvents({
  onBackendReady: loadUsers,
  onLocalPlayerUpdated: loadUsers,
});
</script>

<template>
  <DefaultLayout />
</template>
