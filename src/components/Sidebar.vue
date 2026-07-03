<script setup lang="ts">
import type { ViewName } from '../types'
import { usePlayerStore } from '../stores/player'
import EqBars from './EqBars.vue'

defineProps<{ view: ViewName }>()
const emit = defineEmits<{ 'update:view': [view: ViewName] }>()
const player = usePlayerStore()

const navItems: { id: ViewName; label: string; icon: string }[] = [
  {
    id: 'library',
    label: 'Home',
    icon: 'M3 22V8l9-6 9 6v14H13v-6h-2v6H3z',
  },
  {
    id: 'now-playing',
    label: 'Now Playing',
    icon: 'M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 14.5v-9l6 4.5-6 4.5z',
  },
  {
    id: 'queue',
    label: 'Queue',
    icon: 'M2 6h20v2H2zm0 5h20v2H2zm0 5h12v2H2zm18-5v6l5-3z',
  },
]
</script>

<template>
  <aside class="sidebar">
    <!-- Logo -->
    <div class="sidebar-logo" @click="emit('update:view', 'library')">
      <div class="logo-icon">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="7" cy="17" r="3" /><circle cx="17" cy="15" r="3" /><polyline points="10 17 10 5 20 3 20 15" />
        </svg>
      </div>
      <span class="logo-text">Rifly</span>
    </div>

    <!-- Navigation -->
    <nav class="sidebar-nav">
      <button
        v-for="item in navItems"
        :key="item.id"
        :class="['nav-item', { active: view === item.id }]"
        @click="emit('update:view', item.id)"
      >
        <svg viewBox="0 0 24 24" fill="currentColor" class="nav-icon">
          <path :d="item.icon" />
        </svg>
        <span class="nav-label">{{ item.label }}</span>
        <span v-if="item.id === 'queue' && player.queue.length > 0" class="nav-badge">
          {{ player.queue.length }}
        </span>
      </button>
    </nav>

    <!-- Divider -->
    <div class="sidebar-divider" />

    <!-- Your Library section -->
    <div class="sidebar-section">
      <div class="section-header">
        <svg viewBox="0 0 24 24" fill="currentColor" class="section-icon">
          <path d="M4 20V4h4v16H4zm6-2V6h4v12h-4zm6-4V8h4v8h-4z"/>
        </svg>
        <span>Your Library</span>
      </div>
    </div>

    <div class="sidebar-spacer" />

    <!-- Now playing mini card -->
    <div v-if="player.currentTrack" class="now-playing-card" @click="emit('update:view', 'now-playing')">
      <div class="np-art">
        <div v-if="player.coverArt" class="np-art-img" :style="{ backgroundImage: `url(${player.coverArt})` }" />
        <div v-else class="np-art-placeholder">
          <svg viewBox="0 0 24 24" fill="currentColor"><circle cx="7" cy="17" r="3"/><circle cx="17" cy="15" r="3"/><polyline points="10 17 10 5 20 3 20 15"/></svg>
        </div>
        <div v-if="player.isPlaying" class="np-eq">
          <EqBars :playing="player.isPlaying" size="sm" />
        </div>
      </div>
      <div class="np-info">
        <div class="np-title">{{ player.currentTrack.title }}</div>
        <div class="np-artist">{{ player.currentTrack.artist }}</div>
      </div>
    </div>

    <!-- Source mode indicator -->
    <div class="source-indicator">
      <div class="source-dot" />
      <span>Local Files</span>
    </div>
  </aside>
</template>

<style scoped>
.sidebar {
  width: var(--sidebar-width);
  background: var(--color-bg-secondary);
  display: flex;
  flex-direction: column;
  height: 100%;
  border-right: 1px solid rgba(255, 255, 255, 0.04);
}

/* ── Logo ── */
.sidebar-logo {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 20px 20px 16px;
  cursor: pointer;
  transition: opacity 0.15s;
}

.sidebar-logo:hover {
  opacity: 0.85;
}

.logo-icon {
  width: 32px;
  height: 32px;
  border-radius: 8px;
  background: linear-gradient(135deg, var(--color-accent), #15843e);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.logo-icon svg {
  width: 18px;
  height: 18px;
  color: white;
}

.logo-text {
  font-size: 20px;
  font-weight: 700;
  letter-spacing: -0.5px;
}

/* ── Navigation ── */
.sidebar-nav {
  display: flex;
  flex-direction: column;
  gap: 2px;
  padding: 0 8px;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 14px;
  padding: 10px 14px;
  border: none;
  background: transparent;
  color: var(--color-text-secondary);
  font-size: 14px;
  font-weight: 500;
  font-family: var(--font-family);
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.15s ease;
  position: relative;
}

.nav-item:hover {
  color: var(--color-text-primary);
  background: var(--color-surface-glass-hover);
}

.nav-item.active {
  color: var(--color-text-primary);
  background: var(--color-bg-elevated);
}

.nav-item.active::before {
  content: '';
  position: absolute;
  left: 0;
  top: 50%;
  transform: translateY(-50%);
  width: 3px;
  height: 20px;
  background: var(--color-accent);
  border-radius: 0 3px 3px 0;
}

.nav-icon {
  width: 22px;
  height: 22px;
  flex-shrink: 0;
}

.nav-label {
  flex: 1;
  text-align: left;
}

.nav-badge {
  font-size: 10px;
  font-weight: 700;
  background: var(--color-accent);
  color: black;
  padding: 1px 6px;
  border-radius: 10px;
  min-width: 18px;
  text-align: center;
}

/* ── Divider ── */
.sidebar-divider {
  height: 1px;
  background: var(--color-border);
  margin: 12px 20px;
}

/* ── Section ── */
.sidebar-section {
  padding: 0 8px;
}

.section-header {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 8px 14px;
  font-size: 13px;
  font-weight: 600;
  color: var(--color-text-secondary);
  letter-spacing: 0.01em;
}

.section-icon {
  width: 20px;
  height: 20px;
  flex-shrink: 0;
}

/* ── Spacer ── */
.sidebar-spacer {
  flex: 1;
}

/* ── Now Playing Card ── */
.now-playing-card {
  display: flex;
  align-items: center;
  gap: 10px;
  margin: 0 8px 8px;
  padding: 10px;
  border-radius: 8px;
  background: var(--color-bg-tertiary);
  cursor: pointer;
  transition: all 0.2s ease;
}

.now-playing-card:hover {
  background: var(--color-bg-elevated);
}

.np-art {
  width: 40px;
  height: 40px;
  border-radius: 6px;
  overflow: hidden;
  flex-shrink: 0;
  position: relative;
  background: var(--color-bg-elevated);
}

.np-art-img {
  width: 100%;
  height: 100%;
  background-size: cover;
  background-position: center;
}

.np-art-placeholder {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.np-art-placeholder svg {
  width: 18px;
  height: 18px;
  color: var(--color-text-tertiary);
}

.np-eq {
  position: absolute;
  bottom: 3px;
  right: 3px;
}

.np-info {
  min-width: 0;
  flex: 1;
}

.np-title {
  font-size: 12px;
  font-weight: 500;
  color: var(--color-text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.np-artist {
  font-size: 11px;
  color: var(--color-text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

/* ── Source Indicator ── */
.source-indicator {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 20px;
  font-size: 11px;
  color: var(--color-text-tertiary);
  border-top: 1px solid var(--color-border);
}

.source-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: var(--color-accent);
  box-shadow: 0 0 6px var(--color-accent-glow);
}
</style>
