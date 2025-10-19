<script setup lang="ts">
import dayjs from "dayjs";
import { ChevronDown, ChevronRight, ExternalLink, Shirt } from "lucide-vue-next";
import { ref, watchEffect } from "vue";
import { useI18n } from "vue-i18n";

import * as api from "../api";
import type { AvatarChange, Instance, Player } from "../types";
import { formatPlayerName, isPlayerStayedUntilEnd } from "../utils";

const { t, locale } = useI18n();

interface Props {
  players: Player[];
  instance: Instance;
}

interface Emits {
  (e: "openUserPage", userId: string): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

const expandedAvatars = ref<Set<number>>(new Set());
const avatarHistories = ref<Map<number, AvatarChange[]>>(new Map());
const loadingHistories = ref(false);

// プレイヤーリスト読み込み時にアバター履歴を一括取得
watchEffect(async () => {
  if (props.players.length > 0 && avatarHistories.value.size === 0) {
    loadingHistories.value = true;
    try {
      const histories = await api.getInstanceAvatarHistories(props.instance.id);
      avatarHistories.value = new Map(
        Object.entries(histories).map(([k, v]) => [Number(k), v as AvatarChange[]])
      );
    } catch (error) {
      console.error('Failed to load avatar histories:', error);
    } finally {
      loadingHistories.value = false;
    }
  }
});

function formatPlayerTime(dateStr: string): string {
  // locale.valueを依存関係に追加
  locale.value;
  return dayjs(dateStr).format('LT');
}

function formatAvatarChangeTime(dateStr: string, joinedAt: string): string {
  // locale.valueを依存関係に追加
  locale.value;
  const changeTime = dayjs(dateStr);
  const joinTime = dayjs(joinedAt);

  // 日付が異なる場合は日付も表示
  if (!changeTime.isSame(joinTime, 'day')) {
    return changeTime.format('M/D LT');
  }
  return changeTime.format('LT');
}

function toggleAvatarHistory(instancePlayerId: number, event: Event) {
  event.stopPropagation(); // プレイヤークリックイベントを防ぐ

  if (expandedAvatars.value.has(instancePlayerId)) {
    expandedAvatars.value.delete(instancePlayerId);
  } else {
    expandedAvatars.value.add(instancePlayerId);
  }
}

function getAvatarHistory(instancePlayerId: number): AvatarChange[] {
  return avatarHistories.value.get(instancePlayerId) || [];
}

function isAvatarExpanded(instancePlayerId: number): boolean {
  return expandedAvatars.value.has(instancePlayerId);
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
      >
        <div
          class="player-item-content"
          @click="emit('openUserPage', player.userId)"
        >
          <div class="player-info">
            <div class="player-name-row">
              <span class="player-name">{{ formatPlayerName(player) }}</span>
              <span
                v-if="player.lastAvatarName"
                class="player-avatar"
              >
                <Shirt :size="12" />
                <span class="player-avatar-name">{{ player.lastAvatarName }}</span>
              </span>
            </div>
            <ExternalLink
              :size="14"
              class="player-icon"
            />
          </div>
          <div class="player-times">
            <span class="player-time">{{ t('instance.joined') }}: {{ formatPlayerTime(player.joinedAt) }}</span>
            <span
              v-if="player.leftAt"
              class="player-time"
            >{{ t('instance.left') }}: {{ formatPlayerTime(player.leftAt) }}</span>
            <span
              v-else
              class="player-time player-time-active"
            >{{ t('instance.inInstance') }}</span>
            <button
              v-if="player.avatarChangeCount > 0"
              class="player-time player-time-button"
              @click="toggleAvatarHistory(player.instancePlayerId, $event)"
            >
              <Shirt :size="12" />
              <span>{{ t('instance.avatarCount', { count: player.avatarChangeCount }) }}</span>
              <component
                :is="isAvatarExpanded(player.instancePlayerId) ? ChevronDown : ChevronRight"
                :size="12"
              />
            </button>
          </div>

          <!-- アバター使用履歴 -->
          <div
            v-if="isAvatarExpanded(player.instancePlayerId) && player.avatarChangeCount > 0"
            class="avatar-history"
          >
            <div
              v-if="loadingHistories"
              class="avatar-loading"
            >
              {{ t('common.loading') }}
            </div>
            <div
              v-else
              class="avatar-history-list"
            >
              <div
                v-for="(change, index) in getAvatarHistory(player.instancePlayerId)"
                :key="`${player.instancePlayerId}-${change.changedAt}-${index}`"
                class="avatar-history-item"
              >
                <div class="avatar-history-item-content">
                  <div class="avatar-history-info">
                    <Shirt
                      :size="12"
                      class="avatar-history-icon"
                    />
                    <span class="avatar-history-name">{{ change.avatarName }}</span>
                  </div>
                  <span class="avatar-history-time">{{ formatAvatarChangeTime(change.changedAt, player.joinedAt) }}</span>
                </div>
              </div>
            </div>
          </div>
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
      margin: 0 0 0.75rem;
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
    border-radius: 6px;
    cursor: pointer;

    &.player-stayed {
      padding: 1px 1px 1px 3px;
      background: linear-gradient(180deg, var(--interactive-default) 0%, var(--accent-secondary) 100%);
    }

    &-content {
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
      position: relative;
      overflow: hidden;
      border: 1px solid transparent;
      z-index: 2;

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

      & > * {
        position: relative;
        z-index: 1;
      }
    }

    &.player-stayed &-content {
      padding-left: calc(0.75rem - 3px);
      border: 0;
      background: linear-gradient(135deg,
        color-mix(in srgb, var(--player-item-bg) 95%, var(--interactive-default) 5%) 0%,
        color-mix(in srgb, var(--player-item-bg) 97%, var(--accent-secondary) 3%) 100%
      );
    }

    &:hover &-content {
      border-color: color-mix(in srgb, var(--border-default) 80%, var(--accent-primary-light) 20%);

      &::before {
        opacity: 1;
      }
    }

    // アバター履歴エリアにカーソルがある時はhover効果を無効化
    &:has(.avatar-history:hover) &-content {
      border-color: transparent;

      &::before {
        opacity: 0;
      }
    }

    .player-name {
      font-weight: 500;
      color: var(--text-primary);
    }

    .player-icon {
      opacity: 0.5;
      transition: opacity 0.2s;
    }

    &:hover .player-name {
      color: var(--interactive-default);
    }

    &:hover .player-icon {
      opacity: 1;
    }

    // アバター履歴エリアにカーソルがある時はhover効果を無効化
    &:has(.avatar-history:hover) .player-name {
      color: var(--text-primary);
    }

    &:has(.avatar-history:hover) .player-icon {
      opacity: 0.5;
    }
  }

  &-info {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  &-name-row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    flex-wrap: wrap;
  }

  &-avatar {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
    padding: 0.125rem 0.375rem;
    background: color-mix(in srgb, var(--bg-base) 95%, var(--accent-primary) 5%);
    border-radius: 3px;
    font-size: 0.7rem;
    color: var(--text-secondary);
    opacity: 0.8;

    &-name {
      max-width: 200px;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }
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

    &-button {
      display: inline-flex;
      align-items: center;
      gap: 0.25rem;
      padding: 0;
      background: none;
      border: none;
      color: var(--text-tertiary);
      cursor: pointer;
      transition: all 0.2s ease;
      font-size: 0.75rem;

      &:hover {
        color: var(--interactive-default);
      }
    }
  }
}

.avatar {
  &-history {
    margin-top: 0.75rem;
  }

  &-loading {
    padding: 0.5rem;
    text-align: center;
    color: var(--text-tertiary);
    font-size: 0.75rem;
  }

  &-history-list {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
  }

  &-history-item {
    &-content {
      display: flex;
      justify-content: space-between;
      align-items: center;
      gap: 0.5rem;
      padding: 0.25rem 0.5rem;
      background: color-mix(in srgb, var(--bg-base) 95%, var(--accent-primary) 5%);
      border-radius: 4px;
      font-size: 0.75rem;
    }
  }

  &-history-info {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    overflow: hidden;
  }

  &-history-icon {
    opacity: 0.5;
    flex-shrink: 0;
  }

  &-history-name {
    color: var(--text-primary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  &-history-time {
    color: var(--text-tertiary);
    white-space: nowrap;
    flex-shrink: 0;
    font-size: 0.7rem;
  }
}
</style>
