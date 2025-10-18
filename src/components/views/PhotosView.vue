<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import EmptyState from '../common/EmptyState.vue';
import { Camera } from 'lucide-vue-next';

const { t } = useI18n();
</script>

<template>
  <EmptyState
    :title="t('screenshot.title')"
    :description="t('screenshot.description')"
    :icon="Camera"
  />
</template>

<style scoped lang="scss">
</style>
