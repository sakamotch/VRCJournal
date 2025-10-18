<script setup lang="ts">
import { BarChart3 } from 'lucide-vue-next';
import { useI18n } from 'vue-i18n';

import BaseEmptyState from '@/components/base/BaseEmptyState.vue';

const { t } = useI18n();
</script>

<template>
  <BaseEmptyState
    :title="t('stats.title')"
    :description="t('stats.description')"
    :icon="BarChart3"
  />
</template>
