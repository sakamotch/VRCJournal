<script setup lang="ts">
import EmptyState from '../common/EmptyState.vue';
import { Globe } from 'lucide-vue-next';
</script>

<template>
  <EmptyState
    title="ワールド"
    description="ワールド一覧・お気に入り・統計機能を実装予定"
    :icon="Globe"
  />
</template>

<style scoped>
</style>
