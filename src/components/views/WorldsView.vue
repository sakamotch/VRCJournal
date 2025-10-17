<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import EmptyState from '../common/EmptyState.vue';
import { Globe } from 'lucide-vue-next';

const { t } = useI18n();
</script>

<template>
  <EmptyState
    :title="t('world.title')"
    :description="t('world.description')"
    :icon="Globe"
  />
</template>

<style scoped>
</style>
