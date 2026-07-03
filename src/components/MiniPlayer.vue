<script setup lang="ts">
import { onMounted } from 'vue'
import { usePlayerStore } from '../stores/player'
import { useFavoritesStore } from '../stores/favorites'
import { formatTime } from '../types'

const player = usePlayerStore()
const favorites = useFavoritesStore()

onMounted(async () => {
  await player.init()
  await favorites.load()
})
</script>

<template>
  <div class="mini">
    <div class="mini-cover" :style="{ backgroundImage: player.coverArt ? `url(${player.coverArt})` : undefined }">
      <div v-if="!player.coverArt" class="mini-cover-empty">
        <svg viewBox="0 0 24 24" fill="currentColor"><circle cx="7" cy="17" r="3"/><circle cx="17" cy="15" r="3"/><polyline points="10 17 10 5 20 3 20 15"/></svg>
      </div>
    </div>

    <div class="mini-info" v-if="player.currentTrack">
      <div class="mini-title">{{ player.currentTrack.title }}</div>
      <div class="mini-artist">{{ player.currentTrack.artist }}</div>
    </div>

    <div class="mini-controls">
      <button @click="player.prev">
        <svg viewBox="0 0 24 24" fill="currentColor"><path d="M6 6h2v12H6zm3.5 6l8.5 6V6z"/></svg>
      </button>
      <button class="mini-play" @click="player.isPlaying ? player.pause() : player.resume()">
        <svg v-if="!player.isPlaying" viewBox="0 0 24 24" fill="currentColor"><path d="M8 5v14l11-7z"/></svg>
        <svg v-else viewBox="0 0 24 24" fill="currentColor"><path d="M6 4h4v16H6zM14 4h4v16h-4z"/></svg>
      </button>
      <button @click="player.next">
        <svg viewBox="0 0 24 24" fill="currentColor"><path d="M6 18l8.5-6L6 6v12zM16 6v12h2V6h-2z"/></svg>
      </button>
    </div>

    <div class="mini-progress">
      <div class="mini-bar">
        <div class="mini-fill" :style="{ width: player.duration > 0 ? (player.position / player.duration) * 100 + '%' : '0%' }" />
      </div>
    </div>

    <button
      class="mini-heart"
      :class="{ liked: favorites.isFavorite(player.currentTrack?.path || '') }"
      @click="player.currentTrack && favorites.toggle(player.currentTrack.path)"
    >
      <svg viewBox="0 0 24 24" :fill="favorites.isFavorite(player.currentTrack?.path || '') ? 'currentColor' : 'none'" stroke="currentColor" stroke-width="2">
        <path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"/>
      </svg>
    </button>

    <div class="mini-time">{{ formatTime(player.position) }} / {{ formatTime(player.duration) }}</div>
  </div>
</template>

<style scoped>
.mini {
  height: 100vh;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 24px;
  gap: 16px;
  background: var(--background);
  color: var(--foreground);
  font-family: var(--font);
}

.mini-cover {
  width: 200px;
  height: 200px;
  border-radius: 8px;
  background-size: cover;
  background-position: center;
  background-color: var(--panel-bg-hover);
  overflow: hidden;
}
.mini-cover-empty {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}
.mini-cover-empty svg { width: 64px; height: 64px; color: var(--text-muted); }

.mini-info { text-align: center; }
.mini-title { font-size: 18px; font-weight: 700; }
.mini-artist { font-size: 13px; color: var(--text-muted); }

.mini-controls {
  display: flex;
  align-items: center;
  gap: 24px;
}
.mini-controls button {
  background: transparent;
  border: none;
  color: var(--text-muted);
  cursor: pointer;
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
}
.mini-controls button svg { width: 24px; height: 24px; }
.mini-controls button:hover { color: var(--foreground); }
.mini-play {
  width: 56px;
  height: 56px;
  background: var(--main);
  color: var(--main-foreground) !important;
  border-radius: 50%;
}
.mini-play svg { width: 24px; height: 24px; }

.mini-progress { width: 100%; max-width: 300px; }
.mini-bar {
  height: 4px;
  background: var(--panel-bg-hover);
  border-radius: 2px;
  overflow: hidden;
}
.mini-fill {
  height: 100%;
  background: var(--main);
  transition: width 0.2s linear;
}

.mini-heart {
  background: transparent;
  border: none;
  cursor: pointer;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-muted);
  transition: color 0.15s;
}
.mini-heart:hover { color: oklch(65% 0.2 20); }
.mini-heart.liked { color: oklch(65% 0.2 20); }
.mini-heart svg { width: 20px; height: 20px; }

.mini-time {
  font-size: 11px;
  color: var(--text-muted);
  font-variant-numeric: tabular-nums;
}
</style>
