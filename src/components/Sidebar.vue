<script setup lang="ts">
import { computed } from 'vue'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import { usePlayerStore } from '../stores/player'
import type { ViewName } from '../types'

const props = defineProps<{ view: ViewName }>()
const emit = defineEmits<{ 'update:view': [view: ViewName] }>()

const player = usePlayerStore()
const isActive = computed(() => (v: ViewName) => props.view === v)

async function openMiniPlayer() {
  try {
    const existing = WebviewWindow.getByLabel('mini-player')
    if (existing) {
      await existing.show()
      await existing.setFocus()
      return
    }
    const mini = new WebviewWindow('mini-player', {
      url: '/?mini=1',
      title: 'Rifly Mini',
      width: 320,
      height: 480,
      minWidth: 280,
      minHeight: 400,
      decorations: false,
      alwaysOnTop: true,
      resizable: false,
    })
    mini.once('tauri://created', () => {})
    mini.once('tauri://error', () => {})
  } catch {}
}
</script>

<template>
  <aside class="sidebar">
    <div class="brand">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="12" cy="12" r="3"/>
        <circle cx="12" cy="12" r="8" stroke-dasharray="4 2"/>
        <path d="M12 2v4M12 18v4M2 12h4M18 12h4"/>
      </svg>
      <span>rifly</span>
    </div>

    <div class="nav-section">
      <button
        :class="['nav-item', { active: isActive('library') }]"
        @click="emit('update:view', 'library')"
        title="Home"
      >
        <svg viewBox="0 0 24 24" fill="currentColor"><path d="M12 3l9 8h-3v8h-4v-6h-4v6H6v-8H3l9-8z"/></svg>
        <span>Home</span>
      </button>
    </div>

    <div class="nav-divider" />

    <div class="nav-section">
      <div class="nav-label">playback</div>

      <button
        :class="['nav-item', { active: isActive('now-playing') }]"
        @click="emit('update:view', 'now-playing')"
        title="Now Playing"
      >
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="3" y="3" width="18" height="18" rx="2"/>
          <circle cx="12" cy="12" r="3"/>
          <circle cx="12" cy="12" r="6"/>
        </svg>
        <span>Now Playing</span>
        <span v-if="player.isPlaying" class="nav-indicator" />
      </button>

      <button
        :class="['nav-item', { active: isActive('queue') }]"
        @click="emit('update:view', 'queue')"
        title="Queue"
      >
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <line x1="8" y1="6" x2="21" y2="6"/>
          <line x1="8" y1="12" x2="21" y2="12"/>
          <line x1="8" y1="18" x2="21" y2="18"/>
          <line x1="3" y1="6" x2="3.01" y2="6"/>
          <line x1="3" y1="12" x2="3.01" y2="12"/>
          <line x1="3" y1="18" x2="3.01" y2="18"/>
        </svg>
        <span>Queue</span>
        <span v-if="player.queue.length > 0" class="nav-badge">{{ player.queue.length }}</span>
      </button>

      <button class="nav-item" @click="openMiniPlayer" title="Mini Player">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="2" y="3" width="20" height="14" rx="2"/><path d="M12 17v4M8 21h8"/><rect x="12" y="11" width="6" height="4" rx="1"/></svg>
        <span>Mini Player</span>
      </button>
    </div>

    <div class="nav-divider" />

    <div class="nav-section">
      <button
        :class="['nav-item', { active: isActive('settings') }]"
        @click="emit('update:view', 'settings')"
        title="Settings"
      >
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="3"/>
          <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/>
        </svg>
        <span>Settings</span>
      </button>
    </div>
  </aside>
</template>

<style scoped>
.sidebar {
  display: flex;
  flex-direction: column;
  height: 100%;
  gap: 4px;
  padding: 16px 12px;
  overflow-y: auto;
}

.brand {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 0 16px 16px;
  font-size: 18px;
  font-weight: 900;
  color: var(--foreground);
  letter-spacing: -0.5px;
}
.brand svg { width: 22px; height: 22px; color: var(--main); }

.nav-section { display: flex; flex-direction: column; gap: 2px; }
.nav-label { font-size: 10px; font-weight: 800; text-transform: uppercase; letter-spacing: 1.2px; color: var(--text-muted); padding: 8px 16px 4px; }
.nav-divider { height: 1px; background: var(--border); margin: 6px 8px; }

.nav-item {
  display: flex;
  align-items: center;
  gap: 12px;
  background: transparent;
  border: none;
  color: var(--text-muted);
  padding: 10px 16px;
  border-radius: 6px;
  cursor: pointer;
  font-family: var(--font);
  font-size: 14px;
  font-weight: 700;
  transition: all 0.15s;
  width: 100%;
  text-align: left;
  position: relative;
}
.nav-item:hover { color: var(--foreground); background: color-mix(in srgb, var(--foreground) 6%, transparent); }
.nav-item.active { color: var(--foreground); background: color-mix(in srgb, var(--foreground) 8%, transparent); }
.nav-item svg { width: 20px; height: 20px; flex-shrink: 0; }

.nav-indicator {
  width: 8px; height: 8px;
  border-radius: 50%;
  background: var(--main);
  margin-left: auto;
  animation: pulse 1.5s ease infinite;
}
.nav-badge {
  margin-left: auto;
  font-size: 11px;
  font-weight: 800;
  color: var(--text-muted);
  background: var(--secondary-background);
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 0 8px;
  line-height: 18px;
  min-width: 20px;
  text-align: center;
}
@keyframes pulse { 0%, 100% { opacity: 1; } 50% { opacity: 0.4; } }
</style>
