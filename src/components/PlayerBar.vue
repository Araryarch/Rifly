<script setup lang="ts">
import { usePlayerStore } from '../stores/player'
import { formatTime, isHiRes } from '../types'
import EqBars from './EqBars.vue'

const player = usePlayerStore()

function seekTo(e: MouseEvent) {
  const bar = e.currentTarget as HTMLElement
  const rect = bar.getBoundingClientRect()
  const pct = (e.clientX - rect.left) / rect.width
  player.seek(pct * player.duration)
}

function setVol(e: MouseEvent) {
  const bar = e.currentTarget as HTMLElement
  const rect = bar.getBoundingClientRect()
  const pct = (e.clientX - rect.left) / rect.width
  player.setVolume(pct)
}
</script>

<template>
  <footer class="player-bar">
    <!-- ─── Left: Track Info ─── -->
    <div class="player-left">
      <div class="player-art">
        <div v-if="player.coverArt" class="player-art-img" :style="{ backgroundImage: `url(${player.coverArt})` }" />
        <div v-else class="player-art-placeholder">
          <svg viewBox="0 0 24 24" fill="currentColor"><circle cx="7" cy="17" r="3"/><circle cx="17" cy="15" r="3"/><polyline points="10 17 10 5 20 3 20 15"/></svg>
        </div>
      </div>
      <div v-if="player.currentTrack" class="player-track-info">
        <div class="player-track-title">{{ player.currentTrack.title }}</div>
        <div class="player-track-artist">{{ player.currentTrack.artist }}</div>
      </div>
      <div v-else class="player-track-info">
        <div class="player-track-title empty">No track playing</div>
      </div>
      <!-- Hi-Res badge -->
      <div v-if="player.currentTrack && isHiRes(player.currentTrack)" class="player-hires-badge">
        <svg viewBox="0 0 24 24" fill="currentColor" class="hires-star">
          <path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"/>
        </svg>
        Hi-Res
      </div>
    </div>

    <!-- ─── Center: Controls + Progress ─── -->
    <div class="player-center">
      <div class="player-controls">
        <button class="ctrl-btn" :class="{ active: player.shuffle }" @click="player.toggleShuffle()">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M16 3h5v5M4 20L21 3M21 16v5h-5M15 15l6 6M4 4l5 5"/>
          </svg>
        </button>
        <button class="ctrl-btn" @click="player.prev()">
          <svg viewBox="0 0 24 24" fill="currentColor"><path d="M6 6h2v12H6zm3.5 6l8.5 6V6z"/></svg>
        </button>
        <button class="ctrl-btn-play" @click="player.isPlaying ? player.pause() : player.resume()">
          <svg v-if="player.isPlaying" viewBox="0 0 24 24" fill="currentColor"><path d="M6 4h4v16H6zM14 4h4v16h-4z"/></svg>
          <svg v-else viewBox="0 0 24 24" fill="currentColor" class="play-icon"><path d="M8 5v14l11-7z"/></svg>
        </button>
        <button class="ctrl-btn" @click="player.next()">
          <svg viewBox="0 0 24 24" fill="currentColor"><path d="M6 18l8.5-6L6 6v12zM16 6v12h2V6h-2z"/></svg>
        </button>
        <button class="ctrl-btn" :class="{ active: player.repeat !== 'off' }" @click="player.cycleRepeat()">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M17 1l4 4-4 4"/><path d="M3 11V9a4 4 0 0 1 4-4h14"/><path d="M7 23l-4-4 4-4"/><path d="M21 13v2a4 4 0 0 1-4 4H3"/>
          </svg>
          <span v-if="player.repeat === 'one'" class="repeat-one-dot">1</span>
        </button>
      </div>

      <div v-if="player.currentTrack" class="player-progress">
        <span class="progress-time">{{ formatTime(player.position) }}</span>
        <div class="progress-bar-wrapper" @click="seekTo">
          <div class="progress-bar-bg">
            <div class="progress-bar-fill" :style="{ width: (player.duration > 0 ? (player.position / player.duration) * 100 : 0) + '%' }">
              <div class="progress-bar-dot" />
            </div>
          </div>
        </div>
        <span class="progress-time">{{ formatTime(player.duration) }}</span>
      </div>
    </div>

    <!-- ─── Right: Volume + Extras ─── -->
    <div class="player-right">
      <div v-if="player.currentTrack && player.isPlaying" class="player-eq-indicator">
        <EqBars :playing="player.isPlaying" size="sm" />
      </div>
      <div class="volume-control">
        <button class="ctrl-btn-sm">
          <svg v-if="player.volume > 0.5" viewBox="0 0 24 24" fill="currentColor"><path d="M3 9v6h4l5 5V4L7 9H3zm13.5 3A4.5 4.5 0 0 0 14 8.5v7a4.49 4.49 0 0 0 2.5-3.5zM14 3.23v2.06a7.008 7.008 0 0 1 0 13.42v2.06A9.02 9.02 0 0 0 21 12a9.02 9.02 0 0 0-7-8.77z"/></svg>
          <svg v-else-if="player.volume > 0" viewBox="0 0 24 24" fill="currentColor"><path d="M3 9v6h4l5 5V4L7 9H3zm13.5 3A4.5 4.5 0 0 0 14 8.5v7a4.49 4.49 0 0 0 2.5-3.5z"/></svg>
          <svg v-else viewBox="0 0 24 24" fill="currentColor"><path d="M16.5 12A4.5 4.5 0 0 0 14 8.5v2.14L16.45 13 16.5 12zM19 12c0 .94-.2 1.82-.54 2.64l1.51 1.51A8.796 8.796 0 0 0 21 12c0-4.28-2.99-7.86-7-8.77v2.06c2.89.86 5 3.54 5 6.71zM4.27 3L3 4.27 7.73 9H3v6h4l5 5v-6.73l4.25 4.25c-.67.52-1.42.93-2.25 1.18v2.06a8.99 8.99 0 0 0 3.69-1.81L19.73 21 21 19.73l-9-9L4.27 3zM12 4L9.91 6.09 12 8.18V4z"/></svg>
        </button>
        <div class="volume-bar-wrapper" @click="setVol">
          <div class="volume-bar-bg">
            <div class="volume-bar-fill" :style="{ width: player.volume * 100 + '%' }">
              <div class="volume-bar-dot" />
            </div>
          </div>
        </div>
      </div>
    </div>
  </footer>
</template>

<style scoped>
.player-bar {
  height: var(--player-height);
  background: var(--color-bg-secondary);
  border-top: 1px solid rgba(255, 255, 255, 0.04);
  display: grid;
  grid-template-columns: 1fr 2fr 1fr;
  align-items: center;
  padding: 0 16px;
}

/* ── Left: Track Info ── */
.player-left {
  display: flex;
  align-items: center;
  gap: 12px;
  min-width: 0;
}

.player-art {
  width: 56px;
  height: 56px;
  border-radius: 6px;
  overflow: hidden;
  flex-shrink: 0;
  background: var(--color-bg-tertiary);
  transition: all 0.3s ease;
}

.player-art:hover {
  transform: scale(1.04);
}

.player-art-img {
  width: 100%;
  height: 100%;
  background-size: cover;
  background-position: center;
}

.player-art-placeholder {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.player-art-placeholder svg {
  width: 24px;
  height: 24px;
  color: var(--color-text-tertiary);
}

.player-track-info {
  min-width: 0;
}

.player-track-title {
  font-size: 13px;
  font-weight: 500;
  color: var(--color-text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 200px;
}

.player-track-title.empty {
  color: var(--color-text-tertiary);
}

.player-track-title:hover {
  text-decoration: underline;
  cursor: pointer;
}

.player-track-artist {
  font-size: 11px;
  color: var(--color-text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 200px;
}

.player-track-artist:hover {
  text-decoration: underline;
  color: var(--color-text-primary);
  cursor: pointer;
}

.player-hires-badge {
  display: inline-flex;
  align-items: center;
  gap: 3px;
  font-size: 9px;
  font-weight: 700;
  color: var(--color-gold);
  background: var(--color-gold-dim);
  padding: 2px 8px;
  border-radius: 3px;
  white-space: nowrap;
  letter-spacing: 0.04em;
  flex-shrink: 0;
}

.hires-star {
  width: 9px;
  height: 9px;
}

/* ── Center: Controls ── */
.player-center {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
}

.player-controls {
  display: flex;
  align-items: center;
  gap: 8px;
}

.ctrl-btn {
  background: transparent;
  border: none;
  color: var(--color-text-secondary);
  cursor: pointer;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  transition: all 0.15s;
  position: relative;
}

.ctrl-btn svg {
  width: 16px;
  height: 16px;
}

.ctrl-btn:hover {
  color: var(--color-text-primary);
  transform: scale(1.05);
}

.ctrl-btn.active {
  color: var(--color-accent);
}

.ctrl-btn.active:hover {
  color: var(--color-accent-hover);
}

.repeat-one-dot {
  position: absolute;
  bottom: 2px;
  right: 4px;
  font-size: 7px;
  font-weight: 800;
  color: var(--color-accent);
}

.ctrl-btn-play {
  width: 36px;
  height: 36px;
  border-radius: 50%;
  border: none;
  background: var(--color-text-primary);
  color: var(--color-bg-secondary);
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.15s;
}

.ctrl-btn-play:hover {
  transform: scale(1.06);
  background: #ffffff;
}

.ctrl-btn-play svg {
  width: 18px;
  height: 18px;
}

.play-icon {
  margin-left: 2px;
}

/* ── Progress Bar ── */
.player-progress {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 100%;
  max-width: 540px;
}

.progress-time {
  font-size: 11px;
  color: var(--color-text-tertiary);
  font-variant-numeric: tabular-nums;
  min-width: 36px;
  text-align: center;
  user-select: none;
}

.progress-bar-wrapper {
  flex: 1;
  height: 16px;
  display: flex;
  align-items: center;
  cursor: pointer;
}

.progress-bar-bg {
  width: 100%;
  height: 4px;
  background: var(--color-bg-hover);
  border-radius: 2px;
  position: relative;
  overflow: visible;
}

.progress-bar-fill {
  height: 100%;
  background: var(--color-text-primary);
  border-radius: 2px;
  position: relative;
  transition: background 0.15s;
}

.progress-bar-dot {
  position: absolute;
  right: -6px;
  top: 50%;
  transform: translateY(-50%);
  width: 12px;
  height: 12px;
  border-radius: 50%;
  background: var(--color-text-primary);
  opacity: 0;
  transition: opacity 0.15s;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
}

.progress-bar-wrapper:hover .progress-bar-fill {
  background: var(--color-accent);
}

.progress-bar-wrapper:hover .progress-bar-dot {
  opacity: 1;
}

/* ── Right: Volume ── */
.player-right {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 12px;
}

.player-eq-indicator {
  flex-shrink: 0;
}

.volume-control {
  display: flex;
  align-items: center;
  gap: 6px;
}

.ctrl-btn-sm {
  background: transparent;
  border: none;
  color: var(--color-text-secondary);
  cursor: pointer;
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: color 0.15s;
}

.ctrl-btn-sm svg {
  width: 16px;
  height: 16px;
}

.ctrl-btn-sm:hover {
  color: var(--color-text-primary);
}

.volume-bar-wrapper {
  width: 93px;
  height: 16px;
  display: flex;
  align-items: center;
  cursor: pointer;
}

.volume-bar-bg {
  width: 100%;
  height: 4px;
  background: var(--color-bg-hover);
  border-radius: 2px;
  position: relative;
}

.volume-bar-fill {
  height: 100%;
  background: var(--color-text-primary);
  border-radius: 2px;
  position: relative;
  transition: background 0.15s;
}

.volume-bar-dot {
  position: absolute;
  right: -6px;
  top: 50%;
  transform: translateY(-50%);
  width: 12px;
  height: 12px;
  border-radius: 50%;
  background: var(--color-text-primary);
  opacity: 0;
  transition: opacity 0.15s;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
}

.volume-bar-wrapper:hover .volume-bar-fill {
  background: var(--color-accent);
}

.volume-bar-wrapper:hover .volume-bar-dot {
  opacity: 1;
}
</style>
