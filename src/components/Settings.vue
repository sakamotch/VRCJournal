<script setup lang="ts">
import ThemeSelector from './ThemeSelector.vue';

interface Emits {
  (e: 'close'): void;
}

const emit = defineEmits<Emits>();
</script>

<template>
  <div class="settings-overlay" @click="emit('close')">
    <div class="settings-panel" @click.stop>
      <div class="settings-header">
        <h2>設定</h2>
        <button class="close-button" @click="emit('close')">×</button>
      </div>

      <div class="settings-content">
        <div class="setting-section">
          <h3>外観</h3>
          <div class="setting-item">
            <label>テーマ</label>
            <ThemeSelector />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.settings-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background-color: var(--bg-overlay);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.settings-panel {
  width: 90%;
  max-width: 600px;
  max-height: 80vh;
  background-color: var(--bg-surface);
  border-radius: 12px;
  box-shadow: var(--shadow-xl);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.settings-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.5rem;
  border-bottom: 1px solid var(--border-default);
}

.settings-header h2 {
  margin: 0;
  font-size: 1.25rem;
  color: var(--text-primary);
}

.close-button {
  background: none;
  border: none;
  font-size: 2rem;
  color: var(--text-secondary);
  cursor: pointer;
  padding: 0;
  width: 2rem;
  height: 2rem;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  transition: all 0.2s;
}

.close-button:hover {
  background-color: var(--bg-hover);
  color: var(--text-primary);
}

.settings-content {
  padding: 1.5rem;
  overflow-y: auto;
}

.setting-section {
  margin-bottom: 2rem;
}

.setting-section:last-child {
  margin-bottom: 0;
}

.setting-section h3 {
  margin: 0 0 1rem 0;
  font-size: 1rem;
  color: var(--text-secondary);
  font-weight: 600;
}

.setting-item {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.setting-item label {
  font-size: 0.875rem;
  color: var(--text-primary);
  font-weight: 500;
}
</style>
