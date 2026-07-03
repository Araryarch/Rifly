<script setup lang="ts">
import { usePlayerStore } from '../stores/player'
import { useFavoritesStore } from '../stores/favorites'
import { formatTime, isHiRes } from '../types'
import EqBars from '../components/EqBars.vue'

const player = usePlayerStore()
const favorites = useFavoritesStore()

function seekTo(e: MouseEvent) {
  const el = e.currentTarget as HTMLElement
  const r = el.getBoundingClientRect()
  player.seek(((e.clientX - r.left) / r.width) * player.duration)
}
</script>

<template>
  <div class="np-view">
    <template v-if="player.currentTrack">
      <div class="np-content animate-fade-in-up">

        <div class="np-cover">
          <div v-if="player.coverArt" class="np-cover-img" :style="{ backgroundImage: `url(${player.coverArt})` }" />
          <div v-else class="np-cover-empty">
            <svg viewBox="0 0 24 24" fill="currentColor"><circle cx="7" cy="17" r="3"/><circle cx="17" cy="15" r="3"/><polyline points="10 17 10 5 20 3 20 15"/></svg>
          </div>
        </div>

        <div class="np-info">
          <h1 class="np-title">{{ player.currentTrack.title }}</h1>
          <p class="np-artist">{{ player.currentTrack.artist }}</p>
          <p class="np-album">{{ player.currentTrack.album }}</p>
          <button
            class="np-heart"
            :class="{ liked: favorites.isFavorite(player.currentTrack.path) }"
            @click="favorites.toggle(player.currentTrack!.path)"
          >
            <svg viewBox="0 0 24 24" :fill="favorites.isFavorite(player.currentTrack.path) ? 'currentColor' : 'none'" stroke="currentColor" stroke-width="2">
              <path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"/>
            </svg>
            <span>{{ favorites.isFavorite(player.currentTrack.path) ? 'Liked' : 'Like' }}</span>
          </button>
        </div>

        <div class="np-seek">
          <div class="seek-track" @click="seekTo">
            <div class="seek-bg">
              <div class="seek-fill" :style="{ width: (player.duration > 0 ? (player.position / player.duration) * 100 : 0) + '%' }" />
            </div>
          </div>
          <div class="seek-times">
            <span>{{ formatTime(player.position) }}</span>
            <span>{{ formatTime(player.duration) }}</span>
          </div>
        </div>

        <div class="np-controls">
          <button :class="['ctrl', { on: player.shuffle }]" @click="player.toggleShuffle()">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M16 3h5v5M4 20L21 3M21 16v5h-5M15 15l6 6M4 4l5 5"/></svg>
          </button>
          <button class="ctrl" @click="player.prev()">
            <svg viewBox="0 0 24 24" fill="currentColor"><path d="M6 6h2v12H6zm3.5 6l8.5 6V6z"/></svg>
          </button>
          <button class="ctrl-play" @click="player.isPlaying ? player.pause() : player.resume()">
            <svg v-if="player.isPlaying" viewBox="0 0 24 24" fill="currentColor"><path d="M6 4h4v16H6zM14 4h4v16h-4z"/></svg>
            <svg v-else viewBox="0 0 24 24" fill="currentColor" style="margin-left:4px"><path d="M8 5v14l11-7z"/></svg>
          </button>
          <button class="ctrl" @click="player.next()">
            <svg viewBox="0 0 24 24" fill="currentColor"><path d="M6 18l8.5-6L6 6v12zM16 6v12h2V6h-2z"/></svg>
          </button>
          <button :class="['ctrl', { on: player.repeat !== 'off' }]" @click="player.cycleRepeat()">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
              <path d="M17 1l4 4-4 4"/><path d="M3 11V9a4 4 0 0 1 4-4h14"/><path d="M7 23l-4-4 4-4"/><path d="M21 13v2a4 4 0 0 1-4 4H3"/>
            </svg>
            <span v-if="player.repeat === 'one'" class="ctrl-badge">1</span>
          </button>
        </div>

        <div class="np-specs">
          <div v-if="player.isPlaying" class="specs-eq"><EqBars :playing="true" size="md" /></div>
          <div class="specs-data">
            <div class="spec">
              <span class="spec-k">SR</span>
              <span class="spec-v">{{ player.currentTrack.sample_rate / 1000 }}kHz</span>
            </div>
            <div class="spec">
              <span class="spec-k">BD</span>
              <span class="spec-v">{{ player.currentTrack.bit_depth }}bit</span>
            </div>
            <div class="spec">
              <span class="spec-k">CH</span>
              <span class="spec-v">{{ player.currentTrack.channels }}</span>
            </div>
            <div class="spec">
              <span class="spec-v">{{ player.currentTrack.format.toUpperCase() }}</span>
            </div>
          </div>
          <span v-if="isHiRes(player.currentTrack)" class="specs-hr">HI-RES</span>
        </div>

      </div>
    </template>
    <div v-else class="empty-state animate-fade-in">
      <p>no track playing</p>
    </div>
  </div>
</template>

<style scoped>
.np-view {
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 32px;
}

.np-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  max-width: 420px;
  width: 100%;
}

.np-cover {
  width: 280px;
  height: 280px;
  border: 2px solid var(--border);
  border-radius: var(--radius-base);
  box-shadow: var(--shadow);
  background: var(--secondary-background);
  overflow: hidden;
  margin-bottom: 32px;
}
.np-cover-img {
  width: 100%;
  height: 100%;
  background-size: cover;
  background-position: center;
}
.np-cover-empty {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}
.np-cover-empty svg { width: 64px; height: 64px; color: color-mix(in srgb, var(--foreground) 30%, transparent); }

.np-info {
  text-align: center;
  margin-bottom: 24px;
  width: 100%;
}
.np-title {
  font-size: 24px;
  font-weight: 800;
  margin-bottom: 4px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.np-artist {
  font-size: 14px;
  color: color-mix(in srgb, var(--foreground) 70%, transparent);
  margin-bottom: 2px;
}
.np-album {
  font-size: 12px;
  color: color-mix(in srgb, var(--foreground) 40%, transparent);
  margin-bottom: 8px;
}

.np-heart {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  background: transparent;
  border: 2px solid var(--border);
  border-radius: var(--radius-base);
  color: var(--text-muted);
  font-family: var(--font);
  font-size: 11px;
  font-weight: 700;
  padding: 6px 12px;
  cursor: pointer;
  transition: all 0.15s;
}
.np-heart:hover { color: oklch(65% 0.2 20); border-color: oklch(65% 0.2 20); }
.np-heart.liked { color: oklch(65% 0.2 20); border-color: oklch(65% 0.2 20); }
.np-heart svg { width: 14px; height: 14px; }

.np-seek {
  width: 100%;
  margin-bottom: 24px;
}
.seek-track {
  width: 100%;
  height: 20px;
  display: flex;
  align-items: center;
  cursor: pointer;
}
.seek-bg {
  width: 100%;
  height: 6px;
  background: var(--secondary-background);
  border: 2px solid var(--border);
  border-radius: var(--radius-base);
  overflow: hidden;
}
.seek-fill {
  height: 100%;
  background: var(--foreground);
  transition: background 0.1s;
}
.seek-track:hover .seek-fill { background: var(--main); }
.seek-times {
  display: flex;
  justify-content: space-between;
  font-size: 11px;
  color: color-mix(in srgb, var(--foreground) 50%, transparent);
  font-variant-numeric: tabular-nums;
  margin-top: 6px;
}

.np-controls {
  display: flex;
  align-items: center;
  gap: 16px;
  margin-bottom: 32px;
}
.ctrl {
  background: transparent;
  border: none;
  color: color-mix(in srgb, var(--foreground) 50%, transparent);
  cursor: pointer;
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: color 0.1s;
  position: relative;
}
.ctrl svg { width: 24px; height: 24px; }
.ctrl:hover { color: var(--foreground); }
.ctrl.on { color: var(--main); }
.ctrl-badge {
  position: absolute;
  bottom: 0;
  right: 2px;
  font-size: 10px;
  font-weight: 800;
  color: var(--main);
}

.ctrl-play {
  width: 56px;
  height: 56px;
  background: var(--main);
  color: var(--main-foreground);
  border: 2px solid var(--border);
  border-radius: var(--radius-base);
  box-shadow: var(--shadow);
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.15s;
}
.ctrl-play svg { width: 24px; height: 24px; }
.ctrl-play:hover {
  box-shadow: none;
  transform: translate(4px, 4px);
}

.np-specs {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 12px 20px;
  background: var(--secondary-background);
  border: 2px solid var(--border);
  border-radius: var(--radius-base);
  width: 100%;
}
.specs-eq { flex-shrink: 0; }
.specs-data {
  display: flex;
  align-items: center;
  gap: 12px;
  flex: 1;
}
.spec { display: flex; align-items: baseline; gap: 4px; }
.spec-k {
  font-size: 9px;
  font-weight: 700;
  color: color-mix(in srgb, var(--foreground) 40%, transparent);
}
.spec-v {
  font-size: 12px;
  font-weight: 700;
  color: var(--foreground);
}
.specs-hr {
  font-size: 10px;
  font-weight: 800;
  color: var(--gold);
  border: 2px solid var(--gold);
  border-radius: var(--radius-base);
  padding: 2px 6px;
}

.empty-state {
  color: color-mix(in srgb, var(--foreground) 50%, transparent);
}
</style>
