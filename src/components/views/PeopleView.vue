<script setup lang="ts">
import EmptyState from '../common/EmptyState.vue';
import { Users } from 'lucide-vue-next';
</script>

<template>
  <EmptyState
    title="人物"
    description="よく会う人・人物メモ・統計機能を実装予定"
    :icon="Users"
  />
</template>

<style scoped>
</style>
