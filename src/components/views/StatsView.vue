<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import EmptyState from '../common/EmptyState.vue';
import { BarChart3 } from 'lucide-vue-next';

const { t } = useI18n();
</script>

<template>
  <EmptyState
    :title="t('stats.title')"
    :description="t('stats.description')"
    :icon="BarChart3"
  />
</template>

<style scoped>
</style>
