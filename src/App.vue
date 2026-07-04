<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { useFavoritesStore } from './stores/favorites'
import { useLibraryStore } from './stores/library'
import Sidebar from './components/Sidebar.vue'
import Topbar from './components/Topbar.vue'
import RightPanel from './components/RightPanel.vue'
import PlayerBar from './components/PlayerBar.vue'
import MiniPlayer from './components/MiniPlayer.vue'
import LibraryView from './views/LibraryView.vue'
import HomeView from './views/HomeView.vue'
import LikedSongsView from './views/LikedSongsView.vue'
import NowPlayingView from './views/NowPlayingView.vue'
import QueueView from './views/QueueView.vue'
import SearchView from './views/SearchView.vue'
import SettingsView from './views/SettingsView.vue'
import SetupView from './views/SetupView.vue'
import { usePlayerStore } from './stores/player'
import type { ViewName } from './types'

const isMini = ref(window.location.search.includes('mini=1'))

const player = usePlayerStore()
const favorites = useFavoritesStore()
const library = useLibraryStore()
const currentView = ref<ViewName>('library')
const showSetup = ref(false)

// --- Keyboard Shortcuts ---
function onKeyDown(e: KeyboardEvent) {
  if (isMini.value) return
  const target = e.target as HTMLElement
  if (target.tagName === 'INPUT' || target.tagName === 'TEXTAREA') return

  switch (e.code) {
    case 'Space':
      e.preventDefault()
      player.isPlaying ? player.pause() : player.resume()
      break
    case 'ArrowLeft':
      if (e.ctrlKey) { e.preventDefault(); player.prev() }
      break
    case 'ArrowRight':
      if (e.ctrlKey) { e.preventDefault(); player.next() }
      break
    case 'ArrowUp':
      e.preventDefault()
      player.setVolume(Math.min(1, player.volume + 0.05))
      break
    case 'ArrowDown':
      e.preventDefault()
      player.setVolume(Math.max(0, player.volume - 0.05))
      break
    case 'KeyS':
      if (e.ctrlKey) { e.preventDefault(); player.toggleShuffle() }
      break
    case 'KeyR':
      if (e.ctrlKey) { e.preventDefault(); player.cycleRepeat() }
      break
    case 'Escape':
      if (currentView.value !== 'library') currentView.value = 'library'
      break
    case 'KeyM':
      if (e.ctrlKey) { e.preventDefault(); currentView.value = 'now-playing' }
      break
    case 'KeyQ':
      if (e.ctrlKey) { e.preventDefault(); currentView.value = 'queue' }
      break
  }
}

// --- Drag & Drop folder/files ---
const dragging = ref(false)
let dragCleanup: (() => void) | null = null

async function setupDragDrop() {
  try {
    const win = getCurrentWindow()
    dragCleanup = await win.onDragDropEvent((event) => {
      dragging.value = event.payload.type === 'over' || event.payload.type === 'enter'
      if (event.payload.type === 'drop') {
        dragging.value = false
        handleDroppedPaths(event.payload.paths)
      }
      if (event.payload.type === 'leave') dragging.value = false
    })
  } catch {
    // Not in Tauri context
  }
}

async function handleDroppedPaths(paths: string[]) {
  for (const p of paths) {
    // Check if it's a folder or audio file
    const ext = p.split('.').pop()?.toLowerCase()
    const audioExts = ['flac', 'wav', 'aiff', 'aif', 'dsd', 'mp3', 'aac', 'ogg', 'opus', 'm4a', 'ape', 'wv', 'tak']
    if (ext && audioExts.includes(ext)) {
      // Single file — add to queue and play
      try {
        const meta: any = await invoke('read_track_metadata', { path: p })
        if (meta) player.playAll([meta])
      } catch {}
    } else {
      // Folder — scan it
      library.scanFolder(p)
      await invoke('set_setting', { key: 'music_folder', value: p })
      break
    }
  }
}

onMounted(async () => {
  if (isMini.value) return

  document.addEventListener('keydown', onKeyDown)

  // Check first-run setup
  try {
    const done = await invoke('get_setting', { key: 'setup_done' })
    if (done !== 'true') showSetup.value = true
  } catch { showSetup.value = true }

  await player.init()
  await favorites.load()
  await setupDragDrop()
  await library.loadRecentlyPlayed()
  await library.loadMostPlayed()
})

onUnmounted(() => {
  document.removeEventListener('keydown', onKeyDown)
  dragCleanup?.()
})
</script>

<template>
  <SetupView v-if="showSetup" @done="showSetup = false" />
  <div v-if="isMini" class="mini-shell">
    <MiniPlayer />
  </div>
  <div v-else class="shell">
    <!-- Drag overlay -->
    <div v-if="dragging" class="drag-overlay">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4M7 10l5 5 5-5M12 15V3"/></svg>
      <span>drop to scan</span>
    </div>

    <div class="shell-content">
      <div class="shell-topbar">
        <Topbar />
      </div>

      <div class="shell-sidebar">
        <Sidebar v-model:view="currentView" />
      </div>

      <main class="shell-main">
        <Transition name="fade" mode="out-in">
          <HomeView v-if="currentView === 'library'" key="home" />
          <LibraryView v-else-if="currentView === 'your-library'" key="your-library" />
          <LikedSongsView v-else-if="currentView === 'liked-songs'" key="liked-songs" />
          <NowPlayingView v-else-if="currentView === 'now-playing'" key="now-playing" />
          <QueueView v-else-if="currentView === 'queue'" key="queue" />
          <SearchView v-else-if="currentView === 'search'" key="search" />
          <SettingsView v-else-if="currentView === 'settings'" key="settings" />
        </Transition>
      </main>

      <div class="shell-right">
        <RightPanel />
      </div>
    </div>

    <div class="shell-player">
      <PlayerBar />
    </div>
  </div>
</template>

<style>
@import './styles/global.css';
</style>

<style scoped>
.shell {
  height: 100vh;
  display: flex;
  flex-direction: column;
  background: var(--background);
  overflow: hidden;
}

.shell-content {
  flex: 1;
  display: grid;
  grid-template-columns: var(--sidebar-width) 1fr var(--right-panel-width);
  grid-template-rows: var(--topbar-height) 1fr;
  grid-template-areas:
    "topbar  topbar topbar"
    "sidebar main   right";
  gap: 8px;
  padding: 8px 8px 0 8px;
  min-height: 0;
}

.shell-topbar { grid-area: topbar; }
.shell-sidebar { grid-area: sidebar; background: var(--panel-bg); border-radius: var(--radius-base); overflow: hidden; }
.shell-main { grid-area: main; background: var(--panel-bg); border-radius: var(--radius-base); overflow: hidden; }
.shell-right { grid-area: right; background: var(--panel-bg); border-radius: var(--radius-base); overflow: hidden; }
.shell-player { height: var(--player-height); flex-shrink: 0; }

.mini-shell {
  height: 100vh;
  background: var(--panel-bg);
  overflow: hidden;
}

.fade-enter-active { transition: opacity 0.15s ease; }
.fade-leave-active { transition: opacity 0.1s ease; }
.fade-enter-from, .fade-leave-to { opacity: 0; }

/* Drag overlay */
.drag-overlay {
  position: fixed;
  inset: 0;
  z-index: 9999;
  background: var(--overlay);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 16px;
  font-size: 18px;
  font-weight: 700;
  color: var(--main);
  backdrop-filter: blur(4px);
}
.drag-overlay svg { width: 48px; height: 48px; }
</style>
