<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted } from 'vue'
import { useUiStore } from '../stores/ui'
import { usePlayerStore } from '../stores/player'
import { useLibraryStore } from '../stores/library'
import { getCurrentWindow } from '@tauri-apps/api/window'

const ui = useUiStore()
const player = usePlayerStore()
const lib = useLibraryStore()
const query = ref('')

const showNotifications = ref(false)
const showProfile = ref(false)

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

// Close dropdowns on outside click
function onDocClick(e: MouseEvent) {
  const el = e.target as HTMLElement
  if (!el.closest('.notif-wrapper')) showNotifications.value = false
  if (!el.closest('.profile-wrapper')) showProfile.value = false
}
onMounted(() => document.addEventListener('click', onDocClick))
onUnmounted(() => document.removeEventListener('click', onDocClick))

function toggleNotif(e: Event) {
  e.stopPropagation()
  showNotifications.value = !showNotifications.value
  showProfile.value = false
}

function toggleProfile(e: Event) {
  e.stopPropagation()
  showProfile.value = !showProfile.value
  showNotifications.value = false
}

let appWindow: any
try {
  appWindow = getCurrentWindow()
} catch (e) {
  // Not in Tauri
}

function minimize() {
  appWindow?.minimize()
}
function toggleMaximize() {
  appWindow?.toggleMaximize()
}
function closeWindow() {
  appWindow?.close()
}

function startDrag(e: MouseEvent) {
  const target = e.target as HTMLElement
  if (target.closest('button') || target.closest('input') || target.closest('.avatar') || target.closest('.search-box') || target.closest('.notif-wrapper') || target.closest('.profile-wrapper')) {
    return
  }
  appWindow?.startDragging()
}
</script>

<template>
  <header class="topbar" @mousedown="startDrag">
    <div class="t-left">
      <button class="nav-btn">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M15 18l-6-6 6-6"/></svg>
      </button>
      <button class="nav-btn">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M9 18l6-6-6-6"/></svg>
      </button>
    </div>

    <div class="t-center">
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

    <div class="t-right">
      <!-- Notifications -->
      <div class="notif-wrapper">
        <button class="nav-btn" @click="toggleNotif" :class="{ active: showNotifications }">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9"/><path d="M13.73 21a2 2 0 0 1-3.46 0"/></svg>
        </button>

        <Transition name="dropdown">
          <div v-if="showNotifications" class="dropdown notif-dropdown">
            <div class="dd-header">
              <span class="dd-title">Notifications</span>
            </div>
            <div class="dd-body">
              <div class="dd-empty">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9"/><path d="M13.73 21a2 2 0 0 1-3.46 0"/></svg>
                <span>No notifications</span>
              </div>
            </div>
          </div>
        </Transition>
      </div>

      <!-- Profile -->
      <div class="profile-wrapper">
        <button class="nav-btn" @click="toggleProfile" :class="{ active: showProfile }">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/></svg>
        </button>

        <Transition name="dropdown">
          <div v-if="showProfile" class="dropdown profile-dropdown">
            <div class="dd-header">
              <div class="dd-profile-info">
                <div class="dd-avatar">A</div>
                <div>
                  <div class="dd-name">Audiophile</div>
                  <div class="dd-role">Local Player</div>
                </div>
              </div>
            </div>
            <div class="dd-divider" />
            <div class="dd-body">
              <div class="dd-stat">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M9 18V5l12-2v13"/><circle cx="6" cy="18" r="3"/><circle cx="18" cy="16" r="3"/></svg>
                <span>{{ lib.tracks.length }} tracks in library</span>
              </div>
              <div class="dd-stat">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="3" width="18" height="18" rx="2" ry="2"/><circle cx="12" cy="12" r="3"/></svg>
                <span>{{ lib.albums.length }} albums</span>
              </div>
              <div class="dd-stat" v-if="player.currentTrack">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><path d="M12 6v6l4 2"/></svg>
                <span>Playing: {{ player.currentTrack.format.toUpperCase() }}</span>
              </div>
            </div>
            <div class="dd-divider" />
            <div class="dd-footer">
              <span class="dd-version">Rifly v0.1.0</span>
            </div>
          </div>
        </Transition>
      </div>

      <div class="avatar" @click="toggleProfile">A</div>

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
.nav-btn:hover { color: var(--foreground); background: rgba(255,255,255,0.1); }
.nav-btn.active { color: var(--foreground); background: rgba(255,255,255,0.12); }
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

/* --- Dropdown wrappers --- */
.notif-wrapper, .profile-wrapper {
  position: relative;
}

/* --- Dropdown panel --- */
.dropdown {
  position: absolute;
  top: calc(100% + 8px);
  right: 0;
  width: 280px;
  background: var(--panel-bg);
  border: 1px solid rgba(255,255,255,0.08);
  border-radius: 10px;
  box-shadow: 0 16px 48px rgba(0,0,0,0.6);
  z-index: 100;
  overflow: hidden;
}

.dd-header {
  padding: 14px 16px 10px;
}
.dd-title {
  font-size: 14px;
  font-weight: 800;
  color: var(--foreground);
}
.dd-divider {
  height: 1px;
  background: rgba(255,255,255,0.06);
  margin: 0 12px;
}
.dd-body {
  padding: 8px 12px;
}
.dd-footer {
  padding: 10px 16px;
}
.dd-version {
  font-size: 10px;
  color: var(--text-muted);
  font-weight: 600;
}

.dd-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 24px 0;
  color: var(--text-muted);
  font-size: 12px;
}
.dd-empty svg { width: 28px; height: 28px; opacity: 0.4; }

/* Profile dropdown */
.dd-profile-info {
  display: flex;
  align-items: center;
  gap: 12px;
}
.dd-avatar {
  width: 36px;
  height: 36px;
  border-radius: 50%;
  background: #f573a0;
  color: #000;
  font-weight: 800;
  font-size: 14px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}
.dd-name {
  font-size: 14px;
  font-weight: 700;
  color: var(--foreground);
}
.dd-role {
  font-size: 11px;
  color: var(--text-muted);
}

.dd-stat {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 8px 4px;
  font-size: 12px;
  color: color-mix(in srgb, var(--foreground) 70%, transparent);
}
.dd-stat svg {
  width: 16px;
  height: 16px;
  color: var(--main);
  flex-shrink: 0;
}

/* Dropdown animation */
.dropdown-enter-active { transition: all 0.15s ease-out; }
.dropdown-leave-active { transition: all 0.1s ease-in; }
.dropdown-enter-from { opacity: 0; transform: translateY(-8px) scale(0.96); }
.dropdown-leave-to { opacity: 0; transform: translateY(-4px) scale(0.98); }

/* --- Avatar & Window Controls --- */
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
  transition: transform 0.15s;
}
.avatar:hover { transform: scale(1.1); }

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
