<script setup lang="ts">
import { ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { setLocale, type Locale } from '@/i18n';
import Dropdown from './common/Dropdown.vue';

const { t, locale } = useI18n();

const languages = ref<Array<{ value: Locale; label: string }>>([
  { value: 'ja', label: t('settings.languages.ja') },
  { value: 'en', label: t('settings.languages.en') },
]);

const currentLocale = ref<Locale>(locale.value as Locale);

watch(currentLocale, (newLocale) => {
  setLocale(newLocale);
});

watch(locale, () => {
  languages.value = [
    { value: 'ja', label: t('settings.languages.ja') },
    { value: 'en', label: t('settings.languages.en') },
  ];
});
</script>

<template>
  <Dropdown
    v-model="currentLocale"
    :options="languages"
  />
</template>
