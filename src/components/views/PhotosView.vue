<script setup lang="ts">
import EmptyState from '../common/EmptyState.vue';
import { Camera } from 'lucide-vue-next';
</script>

<template>
  <EmptyState
    title="フォト"
    description="写真ギャラリー・アルバム機能を実装予定"
    :icon="Camera"
  />
</template>

<style scoped>
</style>
