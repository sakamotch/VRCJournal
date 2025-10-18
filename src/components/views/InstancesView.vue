<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import InstanceCard from "@/components/features/instance/InstanceCard.vue";
import ScreenshotModal from "@/components/features/screenshot/ScreenshotModal.vue";
import { useInstances } from "@/composables/useInstances";
import { useScreenshot } from "@/composables/useScreenshot";
import { useBackendEvents } from "@/composables/useBackendEvents";
import { useUserStore } from "@/stores/userStore";
import { storeToRefs } from "pinia";
import dayjs from "dayjs";

const { t, locale } = useI18n();

const userStore = useUserStore();
const { selectedUserId } = storeToRefs(userStore);
const {
  instances,
  isLoading,
  loadInstances,
  openInviteUrl,
  openUserPage,
  updateInstanceEnd,
} = useInstances();
const {
  selectedScreenshot,
  viewScreenshot,
  closeScreenshotModal,
  openScreenshotDirectory,
} = useScreenshot();

useBackendEvents({
  onInstanceCreated: () => loadInstances(selectedUserId.value),
  onInstanceEnded: updateInstanceEnd,
  onPlayerJoined: () => loadInstances(selectedUserId.value),
});

const instancesByDate = computed(() => {
  // locale.valueを依存関係に追加してロケール変更時に再計算
  locale.value;

  const groups: { date: string; displayDate: string; instances: any[] }[] = [];

  instances.value.forEach(instance => {
    const date = dayjs(instance.startedAt);
    const dateKey = date.format("YYYY/MM/DD");

    const displayDate = formatDateHeader(date);

    let group = groups.find(g => g.date === dateKey);
    if (!group) {
      group = { date: dateKey, displayDate, instances: [] };
      groups.push(group);
    }
    group.instances.push(instance);
  });

  return groups;
});

function formatDateHeader(date: dayjs.Dayjs): string {
  const today = dayjs();
  const yesterday = dayjs().subtract(1, "day");

  if (date.isSame(today, "day")) {
    return `${t('common.today')} - ${date.format("LL")}`;
  } else if (date.isSame(yesterday, "day")) {
    return `${t('common.yesterday')} - ${date.format("LL")}`;
  } else {
    return date.format("LL (ddd)");
  }
}
</script>

<template>
  <div class="instance-list-container">
    <h2>{{ t('instance.title') }}</h2>

    <div v-if="isLoading" class="loading">{{ t('common.loading') }}</div>

    <div v-else-if="instances.length === 0" class="empty">
      {{ t('instance.noInstances') }}
    </div>

    <div v-else class="instance-list">
      <div v-for="group in instancesByDate" :key="group.date" class="date-group">
        <div class="date-header">
          {{ group.displayDate }}
        </div>
        <div class="date-instances">
          <InstanceCard
            v-for="instance in group.instances"
            :key="instance.id"
            :instance="instance"
            @open-invite="openInviteUrl"
            @open-user-page="openUserPage"
            @view-screenshot="viewScreenshot"
            @open-directory="openScreenshotDirectory"
          />
        </div>
      </div>
    </div>

    <ScreenshotModal
      :file-path="selectedScreenshot"
      @close="closeScreenshotModal"
    />
  </div>
</template>

<style scoped lang="scss">
.instance-list {
  &-container {
    flex: 1;
    padding: 1.5rem;
    overflow-y: auto;

    h2 {
      margin: 0 0 1rem 0;
      font-size: 1.3rem;
      color: var(--text-primary);
    }
  }

  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.loading,
.empty {
  text-align: center;
  padding: 2rem;
  color: var(--text-tertiary);
}

.date {
  &-group {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  &-header {
    font-size: 0.875rem;
    font-weight: 600;
    background: linear-gradient(90deg,
      transparent 0%,
      color-mix(in srgb, var(--text-secondary) 100%, transparent 0%) 50%,
      transparent 100%
    );
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 0.75rem 0;
    position: sticky;
    top: 0;
    z-index: 1;
    backdrop-filter: blur(8px);
    background-color: color-mix(in srgb, var(--bg-base) 90%, transparent 10%);

    &::before,
    &::after {
      content: '';
      flex: 1;
      height: 2px;
      background: linear-gradient(90deg,
        transparent 0%,
        color-mix(in srgb, var(--border-default) 70%, var(--accent-primary-light) 30%) 30%,
        color-mix(in srgb, var(--border-default) 50%, var(--accent-secondary-light) 50%) 50%,
        color-mix(in srgb, var(--border-default) 70%, var(--accent-primary-light) 30%) 70%,
        transparent 100%
      );
      border-radius: 1px;
    }
  }

  &-instances {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
}
</style>
