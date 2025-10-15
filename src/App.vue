<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { convertFileSrc } from '@tauri-apps/api/core';

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
  screenshotCount: number;
}

interface Player {
  id: number;
  displayName: string;
  displayNameAtJoin: string;
  userId: string;
  firstSeenAt: string;
  lastSeenAt: string;
}

interface Screenshot {
  id: number;
  filePath: string;
  takenAt: string;
}

const isLoading = ref(false);
const message = ref("");
const localUsers = ref<LocalUser[]>([]);
const sessions = ref<Session[]>([]);
const selectedUserId = ref<number | null>(null);
const expandedSessions = ref<Set<number>>(new Set());
const sessionPlayers = ref<Map<number, Player[]>>(new Map());
const expandedScreenshots = ref<Set<number>>(new Set());
const sessionScreenshots = ref<Map<number, Screenshot[]>>(new Map());
const selectedScreenshot = ref<string | null>(null);

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

async function toggleSessionScreenshots(sessionId: number) {
  if (expandedScreenshots.value.has(sessionId)) {
    // 折りたたむ
    expandedScreenshots.value.delete(sessionId);
  } else {
    // 展開する
    expandedScreenshots.value.add(sessionId);

    // スクリーンショットをまだ取得していない場合は取得
    if (!sessionScreenshots.value.has(sessionId)) {
      try {
        const screenshots = await invoke<Screenshot[]>("get_session_screenshots", {
          sessionId: sessionId,
        });
        sessionScreenshots.value.set(sessionId, screenshots);
      } catch (error) {
        console.error("Failed to load screenshots:", error);
        message.value = `スクリーンショット読み込みエラー: ${error}`;
      }
    }
  }
}

async function openScreenshotDirectory(filePath: string) {
  try {
    await invoke("open_screenshot_directory", { filePath });
  } catch (error) {
    console.error("Failed to open directory:", error);
    message.value = `ディレクトリを開けませんでした: ${error}`;
  }
}

function viewScreenshot(filePath: string) {
  selectedScreenshot.value = filePath;
}

function closeScreenshotModal() {
  selectedScreenshot.value = null;
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

function formatPlayerName(player: Player): string {
  // 名前が変更されている場合は「旧名前(新しい名前)」の形式で表示
  if (player.displayNameAtJoin !== player.displayName) {
    return `${player.displayNameAtJoin} (${player.displayName})`;
  }
  return player.displayName;
}

let unlistenFn: UnlistenFn | null = null;

onMounted(async () => {
  // 初期データ読み込み
  loadUsers();
  loadSessions();

  // Rustからのイベントをリッスン
  unlistenFn = await listen<any>("log-event", (event) => {
    // ProcessedEvent を処理
    const processedEvent = event.payload;

    switch (processedEvent.type) {
      case "LocalPlayerUpdated":
        // アカウントリストのみ更新
        loadUsers();
        break;

      case "SessionCreated":
        // 新しいセッションを先頭に追加
        loadSessions();
        break;

      case "SessionEnded":
        // 特定のセッションの終了時刻を更新
        const endedSession = sessions.value.find(s => s.id === processedEvent.session_id);
        if (endedSession) {
          endedSession.endedAt = processedEvent.ended_at;
        }
        break;

      case "PlayerJoined":
      case "PlayerLeft":
        // 特定のセッションのプレイヤー数を更新し、キャッシュを無効化
        const sessionId = processedEvent.session_id;
        const targetSession = sessions.value.find(s => s.id === sessionId);
        if (targetSession) {
          // プレイヤー数を再取得するためセッション全体をリロード
          invoke<any>("get_session_by_id", { sessionId })
            .then(updatedSession => {
              targetSession.playerCount = updatedSession.playerCount;
            })
            .catch(err => console.error("Failed to update session:", err));

          // プレイヤーリストのキャッシュを無効化
          if (sessionPlayers.value.has(sessionId)) {
            sessionPlayers.value.delete(sessionId);
            // 展開中の場合は再取得
            if (expandedSessions.value.has(sessionId)) {
              invoke<Player[]>("get_session_players", { sessionId })
                .then(players => {
                  sessionPlayers.value.set(sessionId, players);
                })
                .catch(err => console.error("Failed to reload players:", err));
            }
          }
        }
        break;
    }
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
        <h2>セッション履歴</h2>

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
                <span
                  v-if="session.screenshotCount > 0"
                  class="screenshot-count clickable"
                  @click="toggleSessionScreenshots(session.id)"
                  :title="expandedScreenshots.has(session.id) ? '写真を非表示' : '写真を表示'"
                >
                  📷 {{ session.screenshotCount }}枚
                  {{ expandedScreenshots.has(session.id) ? '▼' : '▶' }}
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
                  <span class="player-name">{{ formatPlayerName(player) }}</span>
                  <span class="player-icon">🔗</span>
                </div>
              </div>
              <div v-else class="loading-players">
                読み込み中...
              </div>
            </div>

            <!-- スクリーンショットリスト -->
            <div
              v-if="expandedScreenshots.has(session.id)"
              class="screenshot-list"
            >
              <h4>スクリーンショット</h4>
              <div
                v-if="sessionScreenshots.get(session.id) && sessionScreenshots.get(session.id)!.length > 0"
                class="screenshot-grid"
              >
                <div
                  v-for="screenshot in sessionScreenshots.get(session.id)"
                  :key="screenshot.id"
                  class="screenshot-item"
                  @click="viewScreenshot(screenshot.filePath)"
                >
                  <img
                    :src="convertFileSrc(screenshot.filePath)"
                    :alt="`Screenshot ${screenshot.id}`"
                    class="screenshot-thumbnail"
                  />
                  <div class="screenshot-time">
                    {{ new Date(screenshot.takenAt).toLocaleTimeString('ja-JP') }}
                  </div>
                </div>
              </div>
              <div v-else-if="sessionScreenshots.get(session.id) && sessionScreenshots.get(session.id)!.length === 0" class="no-screenshots">
                このセッションではスクリーンショットが撮影されていません
              </div>
              <div v-else class="loading-screenshots">
                読み込み中...
              </div>
              <button
                v-if="sessionScreenshots.get(session.id) && sessionScreenshots.get(session.id)!.length > 0"
                @click="openScreenshotDirectory(sessionScreenshots.get(session.id)![0].filePath)"
                class="open-folder-button"
              >
                📁 フォルダを開く
              </button>
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

    <!-- 画像モーダル -->
    <div v-if="selectedScreenshot" class="modal-overlay" @click="closeScreenshotModal">
      <div class="modal-content" @click.stop>
        <button class="modal-close" @click="closeScreenshotModal">×</button>
        <img
          :src="convertFileSrc(selectedScreenshot)"
          alt="Screenshot"
          class="modal-image"
        />
      </div>
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
  padding: 1rem 1.5rem;
  background-color: #2c3e50;
  color: white;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.header h1 {
  margin: 0;
  font-size: 1.5rem;
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

.main h2 {
  margin: 0 0 1rem 0;
  font-size: 1.3rem;
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

.screenshot-count {
  color: #9b59b6;
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

.screenshot-list {
  margin-top: 1rem;
  padding: 1rem;
  background-color: #f8f9fa;
  border-radius: 4px;
}

.screenshot-list h4 {
  margin: 0 0 0.75rem 0;
  font-size: 0.95rem;
  color: #495057;
}

.screenshot-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
  gap: 0.75rem;
  margin-bottom: 0.75rem;
}

.screenshot-item {
  position: relative;
  cursor: pointer;
  border-radius: 4px;
  overflow: hidden;
  background-color: white;
  transition: transform 0.2s, box-shadow 0.2s;
}

.screenshot-item:hover {
  transform: scale(1.05);
  box-shadow: 0 4px 8px rgba(0,0,0,0.2);
}

.screenshot-thumbnail {
  width: 100%;
  height: 120px;
  object-fit: cover;
  display: block;
}

.screenshot-time {
  padding: 0.25rem 0.5rem;
  background-color: rgba(0,0,0,0.7);
  color: white;
  font-size: 0.75rem;
  text-align: center;
}

.no-screenshots, .loading-screenshots {
  text-align: center;
  padding: 1rem;
  color: #6c757d;
  font-size: 0.9rem;
}

.open-folder-button {
  padding: 0.4rem 0.8rem;
  background-color: #9b59b6;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.85rem;
  transition: background-color 0.2s;
}

.open-folder-button:hover {
  background-color: #8e44ad;
}

.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background-color: rgba(0,0,0,0.9);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  position: relative;
  max-width: 90vw;
  max-height: 90vh;
}

.modal-close {
  position: absolute;
  top: -40px;
  right: 0;
  background: none;
  border: none;
  color: white;
  font-size: 2rem;
  cursor: pointer;
  padding: 0.5rem;
  line-height: 1;
}

.modal-close:hover {
  color: #ccc;
}

.modal-image {
  max-width: 90vw;
  max-height: 90vh;
  object-fit: contain;
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

  .screenshot-list {
    background-color: #1a1a1a;
  }

  .screenshot-list h4 {
    color: #b0b0b0;
  }

  .screenshot-item {
    background-color: #2a2a2a;
  }

  .screenshot-item:hover {
    background-color: #3a3a3a;
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
