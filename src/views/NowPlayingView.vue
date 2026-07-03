<script setup lang="ts">
import { usePlayerStore } from '../stores/player'
import { formatTime, isHiRes } from '../types'
import EqBars from '../components/EqBars.vue'

const player = usePlayerStore()

function seekTo(e: MouseEvent) {
  const bar = e.currentTarget as HTMLElement
  const rect = bar.getBoundingClientRect()
  const pct = (e.clientX - rect.left) / rect.width
  player.seek(pct * player.duration)
}
</script>

<template>
  <div class="now-playing">
    <template v-if="player.currentTrack">
      <!-- Background ambient gradient -->
      <div class="np-ambient">
        <div class="np-gradient-1" />
        <div class="np-gradient-2" />
      </div>

      <div class="np-content animate-fade-in-up">
        <!-- Cover art -->
        <div class="np-cover-container">
          <div class="np-cover-shadow" />
          <div class="np-cover">
            <div v-if="player.coverArt" class="np-cover-img" :style="{ backgroundImage: `url(${player.coverArt})` }" />
            <div v-else class="np-cover-placeholder">
              <svg viewBox="0 0 24 24" fill="currentColor"><circle cx="7" cy="17" r="3"/><circle cx="17" cy="15" r="3"/><polyline points="10 17 10 5 20 3 20 15"/></svg>
            </div>
          </div>
        </div>

        <!-- Track info -->
        <div class="np-info">
          <h1 class="np-title">{{ player.currentTrack.title }}</h1>
          <p class="np-artist">{{ player.currentTrack.artist }}</p>
          <p class="np-album">{{ player.currentTrack.album }}</p>
        </div>

        <!-- Progress bar -->
        <div class="np-progress">
          <div class="np-seek-wrapper" @click="seekTo">
            <div class="np-seek-bg">
              <div class="np-seek-fill" :style="{ width: (player.duration > 0 ? (player.position / player.duration) * 100 : 0) + '%' }">
                <div class="np-seek-dot" />
              </div>
            </div>
          </div>
          <div class="np-times">
            <span>{{ formatTime(player.position) }}</span>
            <span>{{ formatTime(player.duration) }}</span>
          </div>
        </div>

        <!-- Controls -->
        <div class="np-controls">
          <button class="np-ctrl" :class="{ active: player.shuffle }" @click="player.toggleShuffle()">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M16 3h5v5M4 20L21 3M21 16v5h-5M15 15l6 6M4 4l5 5"/></svg>
          </button>
          <button class="np-ctrl" @click="player.prev()">
            <svg viewBox="0 0 24 24" fill="currentColor" class="ctrl-icon-lg"><path d="M6 6h2v12H6zm3.5 6l8.5 6V6z"/></svg>
          </button>
          <button class="np-ctrl-play" @click="player.isPlaying ? player.pause() : player.resume()">
            <svg v-if="player.isPlaying" viewBox="0 0 24 24" fill="currentColor"><path d="M6 4h4v16H6zM14 4h4v16h-4z"/></svg>
            <svg v-else viewBox="0 0 24 24" fill="currentColor" class="play-offset"><path d="M8 5v14l11-7z"/></svg>
          </button>
          <button class="np-ctrl" @click="player.next()">
            <svg viewBox="0 0 24 24" fill="currentColor" class="ctrl-icon-lg"><path d="M6 18l8.5-6L6 6v12zM16 6v12h2V6h-2z"/></svg>
          </button>
          <button class="np-ctrl" :class="{ active: player.repeat !== 'off' }" @click="player.cycleRepeat()">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M17 1l4 4-4 4"/><path d="M3 11V9a4 4 0 0 1 4-4h14"/><path d="M7 23l-4-4 4-4"/><path d="M21 13v2a4 4 0 0 1-4 4H3"/>
            </svg>
            <span v-if="player.repeat === 'one'" class="repeat-dot">1</span>
          </button>
        </div>

        <!-- Audio info card -->
        <div class="np-audio-card glass">
          <div v-if="player.isPlaying" class="audio-card-eq">
            <EqBars :playing="true" size="md" />
          </div>
          <div class="audio-specs">
            <div class="spec-item">
              <span class="spec-label">Sample Rate</span>
              <span class="spec-value">{{ player.currentTrack.sample_rate / 1000 }}kHz</span>
            </div>
            <div class="spec-divider" />
            <div class="spec-item">
              <span class="spec-label">Bit Depth</span>
              <span class="spec-value">{{ player.currentTrack.bit_depth }}bit</span>
            </div>
            <div class="spec-divider" />
            <div class="spec-item">
              <span class="spec-label">Channels</span>
              <span class="spec-value">{{ player.currentTrack.channels }}ch</span>
            </div>
            <div class="spec-divider" />
            <div class="spec-item">
              <span class="spec-label">Format</span>
              <span class="spec-value">{{ player.currentTrack.format.toUpperCase() }}</span>
            </div>
          </div>
          <div v-if="isHiRes(player.currentTrack)" class="audio-hires-badge">
            <svg viewBox="0 0 24 24" fill="currentColor"><path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"/></svg>
            Hi-Res Audio
          </div>
        </div>
      </div>
    </template>

    <!-- Empty state -->
    <template v-else>
      <div class="np-empty animate-fade-in">
        <div class="np-empty-icon">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1">
            <circle cx="7" cy="17" r="3"/><circle cx="17" cy="15" r="3"/><polyline points="10 17 10 5 20 3 20 15"/>
          </svg>
        </div>
        <p class="np-empty-title">No track playing</p>
        <p class="np-empty-sub">Double-click a track from your library to start playing</p>
      </div>
    </template>
  </div>
</template>

<style scoped>
.now-playing {
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  position: relative;
  overflow: hidden;
  padding: 24px;
}

/* ── Ambient Background ── */
.np-ambient {
  position: absolute;
  inset: 0;
  overflow: hidden;
  pointer-events: none;
}

.np-gradient-1 {
  position: absolute;
  top: -20%;
  left: -10%;
  width: 60%;
  height: 60%;
  background: radial-gradient(circle, rgba(29, 185, 84, 0.15), transparent 70%);
  animation: gradientShift 8s ease-in-out infinite;
  background-size: 200% 200%;
}

.np-gradient-2 {
  position: absolute;
  bottom: -10%;
  right: -10%;
  width: 50%;
  height: 50%;
  background: radial-gradient(circle, rgba(29, 185, 84, 0.08), transparent 70%);
  animation: gradientShift 10s ease-in-out infinite reverse;
  background-size: 200% 200%;
}

/* ── Content ── */
.np-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  max-width: 420px;
  width: 100%;
  position: relative;
  z-index: 1;
}

/* ── Cover Art ── */
.np-cover-container {
  position: relative;
  margin-bottom: 32px;
}

.np-cover-shadow {
  position: absolute;
  inset: 8px;
  border-radius: 16px;
  background: inherit;
  filter: blur(40px);
  opacity: 0.3;
  z-index: -1;
}

.np-cover {
  width: 300px;
  height: 300px;
  border-radius: 12px;
  overflow: hidden;
  box-shadow: 0 16px 48px rgba(0, 0, 0, 0.5), 0 4px 12px rgba(0, 0, 0, 0.3);
  transition: transform 0.4s ease;
}

.np-cover:hover {
  transform: scale(1.02);
}

.np-cover-img {
  width: 100%;
  height: 100%;
  background-size: cover;
  background-position: center;
}

.np-cover-placeholder {
  width: 100%;
  height: 100%;
  background: linear-gradient(135deg, var(--color-bg-tertiary), var(--color-bg-elevated));
  display: flex;
  align-items: center;
  justify-content: center;
}

.np-cover-placeholder svg {
  width: 80px;
  height: 80px;
  color: var(--color-text-tertiary);
  opacity: 0.25;
}

/* ── Track Info ── */
.np-info {
  text-align: center;
  margin-bottom: 28px;
  max-width: 100%;
}

.np-title {
  font-size: 24px;
  font-weight: 700;
  letter-spacing: -0.3px;
  margin-bottom: 4px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.np-artist {
  font-size: 15px;
  color: var(--color-text-secondary);
  margin-bottom: 2px;
}

.np-artist:hover {
  color: var(--color-text-primary);
  text-decoration: underline;
  cursor: pointer;
}

.np-album {
  font-size: 13px;
  color: var(--color-text-tertiary);
}

/* ── Progress ── */
.np-progress {
  width: 100%;
  margin-bottom: 20px;
}

.np-seek-wrapper {
  width: 100%;
  height: 20px;
  display: flex;
  align-items: center;
  cursor: pointer;
}

.np-seek-bg {
  width: 100%;
  height: 4px;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 2px;
  position: relative;
}

.np-seek-fill {
  height: 100%;
  background: var(--color-text-primary);
  border-radius: 2px;
  position: relative;
  transition: background 0.15s;
}

.np-seek-dot {
  position: absolute;
  right: -6px;
  top: 50%;
  transform: translateY(-50%);
  width: 12px;
  height: 12px;
  border-radius: 50%;
  background: white;
  opacity: 0;
  transition: opacity 0.15s;
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.3);
}

.np-seek-wrapper:hover .np-seek-fill {
  background: var(--color-accent);
}

.np-seek-wrapper:hover .np-seek-dot {
  opacity: 1;
}

.np-times {
  display: flex;
  justify-content: space-between;
  font-size: 11px;
  color: var(--color-text-tertiary);
  font-variant-numeric: tabular-nums;
  margin-top: 6px;
}

/* ── Controls ── */
.np-controls {
  display: flex;
  align-items: center;
  gap: 16px;
  margin-bottom: 28px;
}

.np-ctrl {
  background: transparent;
  border: none;
  color: var(--color-text-secondary);
  cursor: pointer;
  width: 36px;
  height: 36px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  transition: all 0.15s;
  position: relative;
}

.np-ctrl svg {
  width: 18px;
  height: 18px;
}

.ctrl-icon-lg {
  width: 24px !important;
  height: 24px !important;
}

.np-ctrl:hover {
  color: var(--color-text-primary);
  transform: scale(1.1);
}

.np-ctrl.active {
  color: var(--color-accent);
}

.repeat-dot {
  position: absolute;
  bottom: 2px;
  right: 4px;
  font-size: 8px;
  font-weight: 800;
  color: var(--color-accent);
}

.np-ctrl-play {
  width: 56px;
  height: 56px;
  border-radius: 50%;
  border: none;
  background: var(--color-text-primary);
  color: var(--color-bg-primary);
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.15s;
}

.np-ctrl-play:hover {
  transform: scale(1.06);
}

.np-ctrl-play svg {
  width: 24px;
  height: 24px;
}

.play-offset {
  margin-left: 3px;
}

/* ── Audio Info Card ── */
.np-audio-card {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 12px 20px;
  border-radius: 12px;
  border: 1px solid var(--color-border);
}

.audio-card-eq {
  flex-shrink: 0;
}

.audio-specs {
  display: flex;
  align-items: center;
  gap: 12px;
}

.spec-item {
  display: flex;
  flex-direction: column;
  align-items: center;
}

.spec-label {
  font-size: 9px;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.06em;
  color: var(--color-text-tertiary);
  margin-bottom: 2px;
}

.spec-value {
  font-size: 13px;
  font-weight: 600;
  color: var(--color-text-primary);
  font-variant-numeric: tabular-nums;
}

.spec-divider {
  width: 1px;
  height: 24px;
  background: var(--color-border);
}

.audio-hires-badge {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  font-size: 10px;
  font-weight: 700;
  color: var(--color-gold);
  background: var(--color-gold-dim);
  padding: 4px 10px;
  border-radius: 6px;
  letter-spacing: 0.04em;
  white-space: nowrap;
  flex-shrink: 0;
}

.audio-hires-badge svg {
  width: 11px;
  height: 11px;
}

/* ── Empty State ── */
.np-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
}

.np-empty-icon {
  width: 120px;
  height: 120px;
  border-radius: 50%;
  background: var(--color-bg-tertiary);
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 8px;
}

.np-empty-icon svg {
  width: 52px;
  height: 52px;
  color: var(--color-text-tertiary);
  opacity: 0.3;
}

.np-empty-title {
  font-size: 20px;
  font-weight: 600;
  color: var(--color-text-secondary);
}

.np-empty-sub {
  font-size: 13px;
  color: var(--color-text-tertiary);
}
</style>
