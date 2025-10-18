<script setup lang="ts">
import { Globe } from 'lucide-vue-next';
import { useI18n } from 'vue-i18n';

import BaseEmptyState from '@/components/base/BaseEmptyState.vue';

const { t } = useI18n();
</script>

<template>
  <BaseEmptyState
    :title="t('world.title')"
    :description="t('world.description')"
    :icon="Globe"
  />
</template>
