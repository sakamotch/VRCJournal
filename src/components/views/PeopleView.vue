<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import EmptyState from '../common/EmptyState.vue';
import { Users } from 'lucide-vue-next';

const { t } = useI18n();
</script>

<template>
  <EmptyState
    :title="t('player.title')"
    :description="t('player.description')"
    :icon="Users"
  />
</template>

<style scoped>
</style>
