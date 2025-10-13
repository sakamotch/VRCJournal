<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from '@tauri-apps/plugin-opener';

const logFilePath = ref("");
const result = ref("");
const isLoading = ref(false);
const isWatching = ref(false);
const autoLogPath = ref("");

async function selectAndParseLogFile() {
  isLoading.value = true;
  result.value = "Processing...";

  try {
    const filePath = logFilePath.value.trim();
    if (!filePath) {
      result.value = "Error: Please enter a file path";
      return;
    }

    const response = await invoke<string>("test_parse_log_file", {
      filePath: filePath
    });
    result.value = response;
  } catch (error) {
    result.value = `Error: ${error}`;
  } finally {
    isLoading.value = false;
  }
}

async function startAutoWatch() {
  isLoading.value = true;
  result.value = "Starting automatic log watching...";

  try {
    const response = await invoke<string>("start_log_watching");
    result.value = `✓ ${response}\n\nログファイルを自動検知して監視を開始しました。\nVRChatでワールドに入ったり、プレイヤーと会ったりすると、\n自動的にデータベースに記録されます。`;
    isWatching.value = true;
  } catch (error) {
    result.value = `Error: ${error}`;
    isWatching.value = false;
  } finally {
    isLoading.value = false;
  }
}

async function getStats() {
  isLoading.value = true;
  try {
    const stats = await invoke<string>("get_database_stats");
    result.value = stats;
  } catch (error) {
    result.value = `Error: ${error}`;
  } finally {
    isLoading.value = false;
  }
}

async function getLogPath() {
  try {
    const path = await invoke<string>("get_log_path");
    autoLogPath.value = path;
  } catch (error) {
    autoLogPath.value = `Error: ${error}`;
  }
}

// 起動時にログパスを取得
getLogPath();
</script>

<template>
  <main class="container">
    <h1>VRCJournal - Log Parser Test</h1>

    <!-- 自動監視セクション -->
    <div class="test-section highlight">
      <h2>🔍 自動ログ監視（推奨）</h2>
      <p>VRChatのログファイルを自動検知して、リアルタイムで監視します</p>
      <p v-if="autoLogPath" class="hint">検出されたログディレクトリ: {{ autoLogPath }}</p>

      <div class="input-group">
        <button
          @click="startAutoWatch"
          :disabled="isLoading || isWatching"
          class="primary"
        >
          {{ isWatching ? '✓ 監視中' : '自動監視を開始' }}
        </button>
        <button
          @click="getStats"
          :disabled="isLoading"
          class="secondary"
        >
          DB統計を表示
        </button>
      </div>
    </div>

    <!-- 手動テストセクション -->
    <div class="test-section">
      <h2>📁 手動ログファイルテスト</h2>
      <p>特定のログファイルを指定してテストできます</p>
      <p class="hint">例: C:\Users\YourName\AppData\LocalLow\VRChat\VRChat\output_log_2025-01-15_12-34-56.txt</p>

      <div class="input-group">
        <input
          v-model="logFilePath"
          placeholder="Enter log file path..."
          class="file-input"
        />
        <button
          @click="selectAndParseLogFile"
          :disabled="isLoading"
        >
          Parse & Save to DB
        </button>
      </div>
    </div>

    <!-- 結果表示 -->
    <div v-if="result" class="result">
      <h3>Result:</h3>
      <pre>{{ result }}</pre>
    </div>
  </main>
</template>

<style scoped>
.test-section {
  margin-top: 2rem;
  text-align: left;
  max-width: 800px;
  margin-left: auto;
  margin-right: auto;
  padding: 1.5rem;
  border: 1px solid #dee2e6;
  border-radius: 8px;
  background-color: #fff;
}

.test-section.highlight {
  border-color: #396cd8;
  border-width: 2px;
  background-color: #f0f7ff;
}

.test-section h2 {
  margin-top: 0;
  margin-bottom: 1rem;
}

.test-section p {
  margin-bottom: 0.5rem;
}

.hint {
  font-size: 0.9em;
  color: #666;
  font-style: italic;
}

.input-group {
  display: flex;
  gap: 0.5rem;
  margin-bottom: 0;
  flex-wrap: wrap;
}

.file-input {
  flex: 1;
  min-width: 300px;
}

button.primary {
  background-color: #396cd8;
  color: white;
}

button.primary:hover:not(:disabled) {
  background-color: #2851a3;
}

button.secondary {
  background-color: #6c757d;
  color: white;
}

button.secondary:hover:not(:disabled) {
  background-color: #5a6268;
}

button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.result {
  margin-top: 1.5rem;
  padding: 1.5rem;
  background-color: #f8f9fa;
  border-radius: 8px;
  border: 1px solid #dee2e6;
  max-width: 800px;
  margin-left: auto;
  margin-right: auto;
}

.result h3 {
  margin-top: 0;
  margin-bottom: 0.5rem;
}

.result pre {
  white-space: pre-wrap;
  word-wrap: break-word;
  font-family: monospace;
  font-size: 0.9em;
  line-height: 1.5;
  margin: 0;
}

@media (prefers-color-scheme: dark) {
  .test-section {
    background-color: #1a1a1a;
    border-color: #444;
  }

  .test-section.highlight {
    background-color: #1a2332;
    border-color: #396cd8;
  }

  .hint {
    color: #aaa;
  }

  .result {
    background-color: #0f0f0f;
    border-color: #444;
  }

  button.primary {
    background-color: #4178db;
  }

  button.primary:hover:not(:disabled) {
    background-color: #5a8de6;
  }

  button.secondary {
    background-color: #495057;
  }
}
</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>