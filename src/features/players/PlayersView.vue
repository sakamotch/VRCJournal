<script setup lang="ts">
import { Users } from 'lucide-vue-next';
import { useI18n } from 'vue-i18n';

import BaseEmptyState from '@/components/base/BaseEmptyState.vue';

const { t } = useI18n();
</script>

<template>
  <BaseEmptyState
    :title="t('player.title')"
    :description="t('player.description')"
    :icon="Users"
  />
</template>
