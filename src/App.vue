<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

interface LocalUser {
  id: number;
  displayName: string;
  userId: string;
  firstAuthenticatedAt: string;
  lastAuthenticatedAt: string;
}

interface Session {
  id: number;
  localUserId: number;
  userName: string;
  startedAt: string;
  endedAt: string | null;
  worldId: string;
  worldName: string | null;
  instanceId: string;
  playerCount: number;
}

interface Player {
  id: number;
  displayName: string;
  userId: string;
  firstSeenAt: string;
  lastSeenAt: string;
}

const isWatching = ref(false);
const isLoading = ref(false);
const message = ref("");
const localUsers = ref<LocalUser[]>([]);
const sessions = ref<Session[]>([]);
const selectedUserId = ref<number | null>(null);
const expandedSessions = ref<Set<number>>(new Set());
const sessionPlayers = ref<Map<number, Player[]>>(new Map());

async function startWatching() {
  isLoading.value = true;
  message.value = "監視を開始しています...";

  try {
    const response = await invoke<string>("start_log_watching");
    message.value = response;
    isWatching.value = true;

    // ユーザーとセッションを読み込み
    await loadUsers();
    await loadSessions();
  } catch (error) {
    message.value = `エラー: ${error}`;
    isWatching.value = false;
  } finally {
    isLoading.value = false;
  }
}

async function loadUsers() {
  try {
    const users = await invoke<LocalUser[]>("get_local_users");
    localUsers.value = users;
  } catch (error) {
    console.error("Failed to load users:", error);
  }
}

async function loadSessions() {
  isLoading.value = true;
  try {
    const result = await invoke<Session[]>("get_sessions", {
      localUserId: selectedUserId.value,
      limit: 100,
    });
    sessions.value = result;

    // セッションを更新したらプレイヤーリストのキャッシュをクリア
    sessionPlayers.value.clear();
  } catch (error) {
    console.error("Failed to load sessions:", error);
    message.value = `セッション読み込みエラー: ${error}`;
  } finally {
    isLoading.value = false;
  }
}

function selectUser(userId: number | null) {
  selectedUserId.value = userId;
  loadSessions();
}

function formatDateTime(dateStr: string): string {
  try {
    const date = new Date(dateStr);
    return date.toLocaleString("ja-JP");
  } catch {
    return dateStr;
  }
}

function formatDuration(startStr: string, endStr: string | null): string {
  if (!endStr) return "進行中";

  try {
    const start = new Date(startStr);
    const end = new Date(endStr);
    const diff = end.getTime() - start.getTime();
    const minutes = Math.floor(diff / 60000);
    const hours = Math.floor(minutes / 60);
    const mins = minutes % 60;

    if (hours > 0) {
      return `${hours}時間${mins}分`;
    } else {
      return `${mins}分`;
    }
  } catch {
    return "-";
  }
}

async function openInviteUrl(session: Session) {
  try {
    // バックエンドでURLを生成して開く
    const url = await invoke<string>("open_invite_url", {
      worldId: session.worldId,
      instanceId: session.instanceId,
    });

    message.value = `招待URLを開きました: ${url}`;

    // 3秒後にメッセージをクリア
    setTimeout(() => {
      if (message.value.startsWith("招待URLを開きました")) {
        message.value = "";
      }
    }, 3000);
  } catch (error) {
    message.value = `エラー: ${error}`;
  }
}

async function toggleSessionPlayers(sessionId: number) {
  if (expandedSessions.value.has(sessionId)) {
    // 折りたたむ
    expandedSessions.value.delete(sessionId);
  } else {
    // 展開する
    expandedSessions.value.add(sessionId);

    // プレイヤーリストをまだ取得していない場合は取得
    if (!sessionPlayers.value.has(sessionId)) {
      try {
        const players = await invoke<Player[]>("get_session_players", {
          sessionId: sessionId,
        });
        sessionPlayers.value.set(sessionId, players);
      } catch (error) {
        console.error("Failed to load players:", error);
        message.value = `プレイヤー読み込みエラー: ${error}`;
      }
    }
  }
}

async function openUserPage(userId: string) {
  try {
    const url = await invoke<string>("open_user_page", {
      userId: userId,
    });

    message.value = `ユーザーページを開きました: ${url}`;

    // 3秒後にメッセージをクリア
    setTimeout(() => {
      if (message.value.startsWith("ユーザーページを開きました")) {
        message.value = "";
      }
    }, 3000);
  } catch (error) {
    message.value = `エラー: ${error}`;
  }
}

let unlistenFn: UnlistenFn | null = null;

onMounted(async () => {
  loadUsers();
  loadSessions();

  // Rustからのイベントをリッスン
  unlistenFn = await listen("log-event-processed", () => {
    // ログイベントが処理されたら、セッション一覧を更新
    loadSessions();
  });
});

onUnmounted(() => {
  // クリーンアップ
  if (unlistenFn) {
    unlistenFn();
  }
});
</script>

<template>
  <div class="app">
    <header class="header">
      <h1>VRCJournal</h1>
      <button
        @click="startWatching"
        :disabled="isLoading || isWatching"
        class="watch-button"
      >
        {{ isWatching ? "✓ 監視中" : "ログ監視を開始" }}
      </button>
    </header>

    <div v-if="message" class="message">{{ message }}</div>

    <div class="content">
      <!-- サイドバー: ユーザー選択 -->
      <aside class="sidebar">
        <h2>アカウント</h2>
        <ul class="user-list">
          <li
            :class="{ active: selectedUserId === null }"
            @click="selectUser(null)"
          >
            全て表示
          </li>
          <li
            v-for="user in localUsers"
            :key="user.id"
            :class="{ active: selectedUserId === user.id }"
            @click="selectUser(user.id)"
          >
            {{ user.displayName }}
          </li>
        </ul>
      </aside>

      <!-- メインエリア: セッション一覧 -->
      <main class="main">
        <div class="session-header">
          <h2>セッション履歴</h2>
          <button @click="loadSessions" :disabled="isLoading" class="refresh-button">
            更新
          </button>
        </div>

        <div v-if="isLoading" class="loading">読み込み中...</div>

        <div v-else-if="sessions.length === 0" class="empty">
          セッションがありません
        </div>

        <div v-else class="session-list">
          <div v-for="session in sessions" :key="session.id" class="session-card">
            <div class="session-main">
              <h3 class="world-name">
                {{ session.worldName || session.worldId }}
              </h3>
              <div class="session-info">
                <span class="user-name">{{ session.userName }}</span>
                <span class="time">{{ formatDateTime(session.startedAt) }}</span>
                <span class="duration">{{ formatDuration(session.startedAt, session.endedAt) }}</span>
                <span
                  class="player-count clickable"
                  @click="toggleSessionPlayers(session.id)"
                  :title="expandedSessions.has(session.id) ? 'プレイヤーを非表示' : 'プレイヤーを表示'"
                >
                  👥 {{ session.playerCount }}人
                  {{ expandedSessions.has(session.id) ? '▼' : '▶' }}
                </span>
              </div>
            </div>

            <!-- プレイヤーリスト -->
            <div
              v-if="expandedSessions.has(session.id)"
              class="player-list"
            >
              <h4>参加プレイヤー</h4>
              <div
                v-if="sessionPlayers.get(session.id)"
                class="player-items"
              >
                <div
                  v-for="player in sessionPlayers.get(session.id)"
                  :key="player.id"
                  class="player-item"
                  @click="openUserPage(player.userId)"
                >
                  <span class="player-name">{{ player.displayName }}</span>
                  <span class="player-icon">🔗</span>
                </div>
              </div>
              <div v-else class="loading-players">
                読み込み中...
              </div>
            </div>

            <div class="session-details">
              <div class="detail-item">
                <span class="label">Instance:</span>
                <span class="value">{{ session.instanceId }}</span>
              </div>
              <button @click="openInviteUrl(session)" class="open-url-button">
                🚀 ワールドを開く
              </button>
            </div>
          </div>
        </div>
      </main>
    </div>
  </div>
</template>

<style scoped>
.app {
  display: flex;
  flex-direction: column;
  height: 100vh;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 1.5rem;
  background-color: #2c3e50;
  color: white;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.header h1 {
  margin: 0;
  font-size: 1.5rem;
}

.watch-button {
  padding: 0.5rem 1rem;
  background-color: #3498db;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 1rem;
}

.watch-button:hover:not(:disabled) {
  background-color: #2980b9;
}

.watch-button:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.message {
  padding: 1rem 1.5rem;
  background-color: #e8f4f8;
  border-left: 4px solid #3498db;
  margin: 0;
  white-space: pre-wrap;
}

.content {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.sidebar {
  width: 250px;
  background-color: #f8f9fa;
  border-right: 1px solid #dee2e6;
  padding: 1rem;
  overflow-y: auto;
}

.sidebar h2 {
  margin-top: 0;
  font-size: 1.1rem;
  color: #495057;
}

.user-list {
  list-style: none;
  padding: 0;
  margin: 0;
}

.user-list li {
  padding: 0.75rem;
  margin-bottom: 0.5rem;
  background-color: white;
  border-radius: 4px;
  cursor: pointer;
  transition: background-color 0.2s;
}

.user-list li:hover {
  background-color: #e9ecef;
}

.user-list li.active {
  background-color: #3498db;
  color: white;
}

.main {
  flex: 1;
  padding: 1.5rem;
  overflow-y: auto;
}

.session-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.session-header h2 {
  margin: 0;
  font-size: 1.3rem;
}

.refresh-button {
  padding: 0.5rem 1rem;
  background-color: #6c757d;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.refresh-button:hover:not(:disabled) {
  background-color: #5a6268;
}

.loading, .empty {
  text-align: center;
  padding: 2rem;
  color: #6c757d;
}

.session-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.session-card {
  background-color: white;
  border: 1px solid #dee2e6;
  border-radius: 8px;
  padding: 1rem;
  transition: box-shadow 0.2s;
}

.session-card:hover {
  box-shadow: 0 4px 8px rgba(0,0,0,0.1);
}

.session-main {
  margin-bottom: 0.5rem;
}

.world-name {
  margin: 0 0 0.5rem 0;
  font-size: 1.1rem;
  color: #2c3e50;
}

.session-info {
  display: flex;
  gap: 1rem;
  flex-wrap: wrap;
  font-size: 0.9rem;
  color: #6c757d;
}

.user-name {
  font-weight: 600;
  color: #3498db;
}

.duration {
  color: #27ae60;
}

.player-count {
  color: #e74c3c;
}

.clickable {
  cursor: pointer;
  user-select: none;
  transition: color 0.2s;
}

.clickable:hover {
  color: #c0392b;
}

.player-list {
  margin-top: 1rem;
  padding: 1rem;
  background-color: #f8f9fa;
  border-radius: 4px;
}

.player-list h4 {
  margin: 0 0 0.75rem 0;
  font-size: 0.95rem;
  color: #495057;
}

.player-items {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.player-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.5rem 0.75rem;
  background-color: white;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s;
}

.player-item:hover {
  background-color: #e9ecef;
  transform: translateX(4px);
}

.player-name {
  font-weight: 500;
  color: #2c3e50;
}

.player-icon {
  opacity: 0.5;
  transition: opacity 0.2s;
}

.player-item:hover .player-icon {
  opacity: 1;
}

.loading-players {
  text-align: center;
  padding: 1rem;
  color: #6c757d;
  font-size: 0.9rem;
}

.session-details {
  margin-top: 0.5rem;
  padding-top: 0.5rem;
  border-top: 1px solid #e9ecef;
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 1rem;
}

.detail-item {
  font-size: 0.85rem;
  color: #6c757d;
  flex: 1;
}

.open-url-button {
  padding: 0.4rem 0.8rem;
  background-color: #3498db;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.85rem;
  white-space: nowrap;
  transition: background-color 0.2s;
}

.open-url-button:hover {
  background-color: #2980b9;
}

.label {
  font-weight: 600;
  margin-right: 0.5rem;
}

.value {
  font-family: monospace;
  font-size: 0.8rem;
}

@media (prefers-color-scheme: dark) {
  .app {
    background-color: #1a1a1a;
    color: #e0e0e0;
  }

  .header {
    background-color: #1e2837;
  }

  .message {
    background-color: #2a3f4f;
    border-left-color: #4aa3df;
    color: #e0e0e0;
  }

  .sidebar {
    background-color: #2a2a2a;
    border-right-color: #3a3a3a;
  }

  .sidebar h2 {
    color: #b0b0b0;
  }

  .user-list li {
    background-color: #1a1a1a;
  }

  .user-list li:hover {
    background-color: #3a3a3a;
  }

  .user-list li.active {
    background-color: #4aa3df;
  }

  .session-card {
    background-color: #2a2a2a;
    border-color: #3a3a3a;
  }

  .world-name {
    color: #e0e0e0;
  }

  .player-list {
    background-color: #1a1a1a;
  }

  .player-list h4 {
    color: #b0b0b0;
  }

  .player-item {
    background-color: #2a2a2a;
  }

  .player-item:hover {
    background-color: #3a3a3a;
  }

  .player-name {
    color: #e0e0e0;
  }
}
</style>

<style>
/* グローバルスタイル */
body {
  margin: 0;
  padding: 0;
  overflow: hidden;
}

#app {
  margin: 0;
  padding: 0;
}
</style>
