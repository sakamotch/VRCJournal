<script setup lang="ts">
import { Camera } from 'lucide-vue-next';
import { useI18n } from 'vue-i18n';

import BaseEmptyState from '@/components/base/BaseEmptyState.vue';

const { t } = useI18n();
</script>

<template>
  <BaseEmptyState
    :title="t('screenshot.title')"
    :description="t('screenshot.description')"
    :icon="Camera"
  />
</template>
