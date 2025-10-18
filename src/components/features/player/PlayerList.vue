<script setup lang="ts">
import { useI18n } from "vue-i18n";
import dayjs from "dayjs";
import type { Player, Instance } from "@/types";
import { formatPlayerName } from "@/utils/formatters";
import { isPlayerStayedUntilEnd } from "@/utils/player";
import { ExternalLink } from "lucide-vue-next";

const { t, locale } = useI18n();

interface Props {
  players: Player[];
  instance: Instance;
}

interface Emits {
  (e: "openUserPage", userId: string): void;
}

defineProps<Props>();
const emit = defineEmits<Emits>();

function formatPlayerTime(dateStr: string): string {
  // locale.valueを依存関係に追加
  locale.value;
  return dayjs(dateStr).format('LT');
}
</script>

<template>
  <div class="player-list">
    <h4>{{ t('instance.joinHistory') }}</h4>
    <div class="player-items">
      <div
        v-for="player in players"
        :key="`${player.id}-${player.joinedAt}`"
        class="player-item"
        :class="{ 'player-stayed': isPlayerStayedUntilEnd(player, instance) }"
        @click="emit('openUserPage', player.userId)"
      >
        <div class="player-info">
          <span class="player-name">{{ formatPlayerName(player) }}</span>
          <ExternalLink :size="14" class="player-icon" />
        </div>
        <div class="player-times">
          <span class="player-time">{{ t('instance.joined') }}: {{ formatPlayerTime(player.joinedAt) }}</span>
          <span class="player-time" v-if="player.leftAt">{{ t('instance.left') }}: {{ formatPlayerTime(player.leftAt) }}</span>
          <span class="player-time player-time-active" v-else>{{ t('instance.inInstance') }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.player {
  &-list {
    margin-top: 1rem;
    padding: 1rem;
    background: linear-gradient(135deg,
      var(--bg-base) 0%,
      color-mix(in srgb, var(--bg-base) 98%, var(--accent-primary) 2%) 100%
    );
    border-radius: 8px;
    border: 1px solid color-mix(in srgb, var(--border-subtle) 95%, var(--accent-primary-light) 5%);

    h4 {
      margin: 0 0 0.75rem 0;
      font-size: 0.95rem;
      color: var(--text-secondary);
    }
  }

  &-items {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  &-item {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    padding: 0.5rem 0.75rem;
    background: linear-gradient(135deg,
      var(--player-item-bg) 0%,
      color-mix(in srgb, var(--player-item-bg) 97%, var(--accent-primary) 3%) 100%
    );
    border-radius: 6px;
    transition: all 0.3s ease;
    cursor: pointer;
    position: relative;
    overflow: hidden;
    border: 1px solid transparent;

    &::before {
      content: '';
      position: absolute;
      inset: 0;
      background: linear-gradient(135deg,
        color-mix(in srgb, var(--bg-hover) 92%, var(--accent-primary-light) 8%) 0%,
        color-mix(in srgb, var(--bg-hover) 96%, var(--accent-secondary-light) 4%) 100%
      );
      opacity: 0;
      transition: opacity 0.3s ease;
      z-index: 0;
    }

    &:hover {
      border-color: color-mix(in srgb, var(--border-default) 80%, var(--accent-primary-light) 20%);

      .player-name {
        color: var(--interactive-default);
      }

      &::before {
        opacity: 1;
      }

      .player-icon {
        opacity: 1;
      }
    }

    & > * {
      position: relative;
      z-index: 1;
    }

    &.player-stayed {
      border-left: 3px solid transparent;
      border-image: linear-gradient(180deg, var(--interactive-default) 0%, var(--accent-secondary) 100%) 1;
      padding-left: calc(0.75rem - 2px); /* ボーダー分を調整 */
      background: linear-gradient(135deg,
        color-mix(in srgb, var(--player-item-bg) 95%, var(--interactive-default) 5%) 0%,
        color-mix(in srgb, var(--player-item-bg) 97%, var(--accent-secondary) 3%) 100%
      );
    }
  }

  &-info {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  &-times {
    display: flex;
    gap: 1rem;
    font-size: 0.75rem;
    color: var(--text-tertiary);
  }

  &-time {
    white-space: nowrap;

    &-active {
      color: var(--feedback-success);
      font-weight: 500;
    }
  }

  &-name {
    font-weight: 500;
    color: var(--text-primary);
  }

  &-icon {
    opacity: 0.5;
    transition: opacity 0.2s;
  }
}
</style>
