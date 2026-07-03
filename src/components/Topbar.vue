<script setup lang="ts">
import { ref, watch } from 'vue'
import { useUiStore } from '../stores/ui'
import { getCurrentWindow } from '@tauri-apps/api/window'

const ui = useUiStore()
const query = ref('')

let timeout: number | undefined
watch(query, (val) => {
  clearTimeout(timeout)
  timeout = window.setTimeout(() => {
    ui.triggerSearch(val)
  }, 300)
})

function handleHome() {
  ui.triggerSearch('')
  query.value = ''
}

const appWindow = getCurrentWindow()

function minimize() {
  appWindow.minimize()
}
function toggleMaximize() {
  appWindow.toggleMaximize()
}
function closeWindow() {
  appWindow.close()
}
</script>

<template>
  <header class="topbar" data-tauri-drag-region>
    <div class="t-left" data-tauri-drag-region>
      <button class="nav-btn">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M15 18l-6-6 6-6"/></svg>
      </button>
      <button class="nav-btn">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M9 18l6-6-6-6"/></svg>
      </button>
    </div>

    <div class="t-center" data-tauri-drag-region>
      <button class="home-btn" @click="handleHome">
        <svg viewBox="0 0 24 24" fill="currentColor"><path d="M12 3l9 8h-3v8h-4v-6h-4v6H6v-8H3l9-8z"/></svg>
      </button>
      <div class="search-box">
        <svg class="search-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><circle cx="11" cy="11" r="8"/><path d="M21 21l-4.35-4.35"/></svg>
        <input 
          v-model="query" 
          type="text" 
          placeholder="What do you want to play?" 
        />
        <div class="browse-icon">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="3" width="18" height="18" rx="2" ry="2"/><line x1="9" y1="3" x2="9" y2="21"/></svg>
        </div>
      </div>
    </div>

    <div class="t-right" data-tauri-drag-region>
      <button class="nav-btn">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9"/><path d="M13.73 21a2 2 0 0 1-3.46 0"/></svg>
      </button>
      <button class="nav-btn">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/></svg>
      </button>
      <div class="avatar">A</div>

      <!-- Window Controls -->
      <div class="window-controls">
        <button class="win-btn" @click="minimize" title="Minimize">
          <svg viewBox="0 0 10 10"><rect x="1" y="4" width="8" height="1" fill="currentColor"/></svg>
        </button>
        <button class="win-btn" @click="toggleMaximize" title="Maximize">
          <svg viewBox="0 0 10 10"><rect x="1" y="1" width="8" height="8" fill="none" stroke="currentColor" stroke-width="1"/></svg>
        </button>
        <button class="win-btn close-btn" @click="closeWindow" title="Close">
          <svg viewBox="0 0 10 10"><path d="M1 1l8 8M9 1L1 9" stroke="currentColor" stroke-width="1" stroke-linecap="round"/></svg>
        </button>
      </div>
    </div>
  </header>
</template>

<style scoped>
.topbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 16px;
  height: 100%;
}

.t-left {
  display: flex;
  align-items: center;
  gap: 8px;
  flex: 1;
}

.nav-btn {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  background: rgba(0,0,0,0.5);
  color: var(--text-muted);
  border: none;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  flex-shrink: 0;
  transition: all 0.2s;
}
.nav-btn:hover { color: var(--foreground); }
.nav-btn svg { width: 16px; height: 16px; pointer-events: none; }

.t-center {
  display: flex;
  align-items: center;
  gap: 8px;
  flex: 1;
  max-width: 500px;
}

.home-btn {
  width: 48px;
  height: 48px;
  border-radius: 50%;
  background: var(--panel-bg);
  color: var(--foreground);
  border: none;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.2s;
}
.home-btn:hover { transform: scale(1.05); background: var(--panel-bg-hover); }
.home-btn svg { width: 24px; height: 24px; }

.search-box {
  flex: 1;
  height: 48px;
  background: var(--panel-bg);
  border-radius: 24px;
  display: flex;
  align-items: center;
  padding: 0 16px;
  gap: 12px;
  border: 1px solid transparent;
  transition: all 0.2s;
}
.search-box:hover {
  background: var(--panel-bg-hover);
  border-color: #333;
}
.search-box:focus-within {
  border-color: #fff;
}
.search-icon { width: 20px; height: 20px; color: var(--text-muted); }
.search-box input {
  flex: 1;
  background: transparent;
  border: none;
  color: var(--foreground);
  font-size: 14px;
  outline: none;
}
.search-box input::placeholder { color: var(--text-muted); }
.browse-icon {
  color: var(--text-muted);
  width: 20px;
  height: 20px;
  cursor: pointer;
}
.browse-icon:hover { color: var(--foreground); }

.t-right {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 12px;
  flex: 1;
}

.icon-btn {
  background: transparent;
  border: none;
  color: var(--text-muted);
  width: 32px;
  height: 32px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  flex-shrink: 0;
}
.icon-btn:hover { color: var(--foreground); }
.icon-btn svg { width: 20px; height: 20px; pointer-events: none; }

.avatar {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  background: #f573a0;
  color: #000;
  font-weight: 700;
  border: 4px solid #1f1f1f;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  flex-shrink: 0;
}

.window-controls {
  display: flex;
  margin-left: 12px;
}

.win-btn {
  width: 40px;
  height: 32px;
  background: transparent;
  border: none;
  color: var(--text-muted);
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  flex-shrink: 0;
  transition: all 0.1s;
}
.win-btn:hover {
  background: rgba(255,255,255,0.1);
  color: var(--foreground);
}
.win-btn.close-btn:hover {
  background: #e81123;
  color: #fff;
}
.win-btn svg {
  width: 10px;
  height: 10px;
  pointer-events: none;
}
</style>
