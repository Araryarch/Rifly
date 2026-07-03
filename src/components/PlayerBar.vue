<script setup lang="ts">
import { ref } from 'vue'
import { usePlayerStore } from '../stores/player'
import { useFavoritesStore } from '../stores/favorites'
import { invoke } from '@tauri-apps/api/core'

const player = usePlayerStore()
const favorites = useFavoritesStore()
const seekRef = ref<HTMLElement | null>(null)
const volRef = ref<HTMLElement | null>(null)

function formatTime(secs: number) {
  if (!secs) return '0:00'
  const m = Math.floor(secs / 60)
  const s = Math.floor(secs % 60).toString().padStart(2, '0')
  return `${m}:${s}`
}

function onSeekClick(e: MouseEvent) {
  if (!seekRef.value || player.duration === 0) return
  const rect = seekRef.value.getBoundingClientRect()
  const pos = (e.clientX - rect.left) / rect.width
  player.seek(pos * player.duration)
}

function onVolClick(e: MouseEvent) {
  if (!volRef.value) return
  const rect = volRef.value.getBoundingClientRect()
  const pos = Math.max(0, Math.min(1, (e.clientX - rect.left) / rect.width))
  invoke('set_volume', { volume: pos })
}
</script>

<template>
  <div class="player-bar">
    <!-- LEFT: Info + Heart -->
    <div class="pb-left">
      <div v-if="player.currentTrack" class="pb-art" :style="{ backgroundImage: `url(${player.coverArt})` }" />
      <div v-else class="pb-art empty">
        <svg viewBox="0 0 24 24" fill="currentColor"><circle cx="7" cy="17" r="3"/><circle cx="17" cy="15" r="3"/><polyline points="10 17 10 5 20 3 20 15"/></svg>
      </div>

      <div class="pb-meta" v-if="player.currentTrack">
        <div class="pb-title">{{ player.currentTrack.title }}</div>
        <div class="pb-artist">{{ player.currentTrack.artist }}</div>
      </div>
      <div class="pb-meta" v-else>
        <div class="pb-title" style="color: var(--text-muted)">No track selected</div>
      </div>

      <button
        v-if="player.currentTrack"
        class="pb-heart"
        :class="{ liked: favorites.isFavorite(player.currentTrack.path) }"
        @click="favorites.toggle(player.currentTrack!.path)"
        title="Favorite"
      >
        <svg viewBox="0 0 24 24" :fill="favorites.isFavorite(player.currentTrack.path) ? 'currentColor' : 'none'" stroke="currentColor" stroke-width="2">
          <path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"/>
        </svg>
      </button>
    </div>

    <!-- CENTER: Controls -->
    <div class="pb-center">
      <div class="pb-controls">
        <button :class="['pb-btn', { active: player.shuffle }]" @click="player.toggleShuffle" title="Shuffle">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="16 3 21 3 21 8"/><line x1="4" y1="20" x2="21" y2="3"/><polyline points="21 16 21 21 16 21"/><line x1="15" y1="15" x2="21" y2="21"/><line x1="4" y1="4" x2="9" y2="9"/></svg>
        </button>
        <button class="pb-btn" @click="player.prev" title="Previous">
          <svg viewBox="0 0 24 24" fill="currentColor"><polygon points="19 20 9 12 19 4 19 20"/><line x1="5" y1="19" x2="5" y2="5" stroke="currentColor" stroke-width="2"/></svg>
        </button>

        <button class="pb-play" @click="player.status === 'playing' ? player.pause() : player.resume()">
          <svg v-if="player.status !== 'playing'" viewBox="0 0 24 24" fill="currentColor"><polygon points="5 3 19 12 5 21 5 3"/></svg>
          <svg v-else viewBox="0 0 24 24" fill="currentColor"><rect x="6" y="4" width="4" height="16"/><rect x="14" y="4" width="4" height="16"/></svg>
        </button>

        <button class="pb-btn" @click="player.next" title="Next">
          <svg viewBox="0 0 24 24" fill="currentColor"><polygon points="5 4 15 12 5 20 5 4"/><line x1="19" y1="5" x2="19" y2="19" stroke="currentColor" stroke-width="2"/></svg>
        </button>
        <button :class="['pb-btn', { active: player.repeat !== 'off' }]" @click="player.cycleRepeat" title="Repeat">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="17 1 21 5 17 9"/><path d="M3 11V9a4 4 0 0 1 4-4h14"/><polyline points="7 23 3 19 7 15"/><path d="M21 13v2a4 4 0 0 1-4 4H3"/></svg>
          <span v-if="player.repeat === 'one'" class="pb-badge">1</span>
        </button>
      </div>

      <div class="pb-progress">
        <span>{{ formatTime(player.position) }}</span>
        <div class="seek-bar" ref="seekRef" @mousedown="onSeekClick">
          <div class="seek-fill" :style="{ width: player.duration ? `${(player.position / player.duration) * 100}%` : '0%' }" />
        </div>
        <span>{{ formatTime(player.duration) }}</span>
      </div>
    </div>

    <!-- RIGHT: Volume -->
    <div class="pb-right">
      <button class="pb-btn" @click="$emit('toggleMini')" title="Mini Player">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="2" y="3" width="20" height="14" rx="2"/><path d="M12 17v4M8 21h8"/><rect x="12" y="11" width="6" height="4" rx="1"/></svg>
      </button>
      <button class="pb-btn">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M11 5L6 9H2v6h4l5 4V5z"/><path d="M15.54 8.46a5 5 0 0 1 0 7.07M19.07 4.93a10 10 0 0 1 0 14.14"/></svg>
      </button>
      <div class="vol-bar" ref="volRef" @mousedown="onVolClick">
        <div class="vol-fill" :style="{ width: `${player.volume * 100}%` }" />
      </div>
    </div>
  </div>
</template>

<style scoped>
.player-bar {
  display: grid;
  grid-template-columns: 300px 1fr 300px;
  align-items: center;
  height: 100%;
  padding: 0 16px;
  background: var(--background);
}

.pb-left {
  display: flex;
  align-items: center;
  gap: 12px;
  min-width: 0;
}

.pb-art {
  width: 56px;
  height: 56px;
  border-radius: 4px;
  background-size: cover;
  background-position: center;
  background-color: var(--panel-bg-hover);
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
}
.pb-art.empty svg {
  width: 24px;
  height: 24px;
  color: var(--text-muted);
}

.pb-meta {
  display: flex;
  flex-direction: column;
  min-width: 0;
}

.pb-title {
  font-size: 14px;
  font-weight: 500;
  color: var(--foreground);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  margin-bottom: 2px;
}

.pb-artist {
  font-size: 12px;
  color: var(--text-muted);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.pb-heart {
  background: transparent;
  border: none;
  cursor: pointer;
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-muted);
  flex-shrink: 0;
  transition: color 0.15s;
}
.pb-heart:hover { color: oklch(65% 0.2 20); }
.pb-heart.liked { color: oklch(65% 0.2 20); }
.pb-heart svg { width: 16px; height: 16px; }

.pb-center {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  width: 100%;
  max-width: 722px;
  margin: 0 auto;
}

.pb-controls {
  display: flex;
  align-items: center;
  gap: 24px;
  position: relative;
}

.pb-btn {
  background: transparent;
  border: none;
  color: var(--text-muted);
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.2s;
  position: relative;
}
.pb-btn:hover { color: var(--foreground); }
.pb-btn.active { color: var(--main); }
.pb-btn svg { width: 16px; height: 16px; }

.pb-badge {
  position: absolute;
  bottom: 0;
  right: 0;
  font-size: 9px;
  font-weight: 800;
  color: var(--main);
}

.pb-play {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  background: #fff;
  color: #000;
  border: none;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: transform 0.1s;
}
.pb-play:hover { transform: scale(1.05); }
.pb-play svg { width: 16px; height: 16px; }

.pb-progress {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 100%;
  font-size: 11px;
  color: var(--text-muted);
  font-variant-numeric: tabular-nums;
}

.seek-bar {
  flex: 1;
  height: 4px;
  background: rgba(255,255,255,0.1);
  border-radius: 2px;
  position: relative;
  cursor: pointer;
  overflow: hidden;
}

.seek-fill {
  height: 100%;
  background: #fff;
  border-radius: 2px;
  transition: width 0.1s linear;
}
.seek-bar:hover .seek-fill { background: var(--main); }

.pb-right {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 12px;
}

.vol-bar {
  width: 100px;
  height: 4px;
  background: rgba(255,255,255,0.1);
  border-radius: 2px;
  cursor: pointer;
}
.vol-fill {
  height: 100%;
  background: #fff;
  border-radius: 2px;
}
.vol-bar:hover .vol-fill { background: var(--main); }
</style>
