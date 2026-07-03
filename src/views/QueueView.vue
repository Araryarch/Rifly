<script setup lang="ts">
import { usePlayerStore } from '../stores/player'
import { formatTime } from '../types'
import EqBars from '../components/EqBars.vue'

const player = usePlayerStore()
</script>

<template>
  <div class="queue-view">
    <!-- Header -->
    <div class="queue-header">
      <div class="queue-header-left">
        <h1 class="queue-title">Queue</h1>
        <span class="queue-count">{{ player.queue.length }} tracks</span>
      </div>
      <div class="queue-header-right">
        <button
          :class="['queue-chip', { active: player.shuffle }]"
          @click="player.toggleShuffle()"
        >
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="chip-icon">
            <path d="M16 3h5v5M4 20L21 3M21 16v5h-5M15 15l6 6M4 4l5 5"/>
          </svg>
          Shuffle
        </button>
        <button
          :class="['queue-chip', { active: player.repeat !== 'off' }]"
          @click="player.cycleRepeat()"
        >
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="chip-icon">
            <path d="M17 1l4 4-4 4"/><path d="M3 11V9a4 4 0 0 1 4-4h14"/><path d="M7 23l-4-4 4-4"/><path d="M21 13v2a4 4 0 0 1-4 4H3"/>
          </svg>
          {{ player.repeat === 'all' ? 'Repeat All' : player.repeat === 'one' ? 'Repeat One' : 'Repeat' }}
        </button>
        <button
          v-if="player.queue.length > 0"
          class="queue-clear-btn"
          @click="player.clearQueue()"
        >
          Clear all
        </button>
      </div>
    </div>

    <!-- Queue content -->
    <div class="queue-scroll">
      <!-- Now Playing section -->
      <div v-if="player.currentTrack && player.queueIndex >= 0" class="queue-section">
        <h3 class="section-label">Now Playing</h3>
        <div class="queue-item queue-item-active">
          <div class="qi-num-active">
            <EqBars :playing="player.isPlaying" size="sm" />
          </div>
          <div class="qi-art">
            <div v-if="player.coverArt" class="qi-art-img" :style="{ backgroundImage: `url(${player.coverArt})` }" />
            <div v-else class="qi-art-placeholder">
              <svg viewBox="0 0 24 24" fill="currentColor"><circle cx="7" cy="17" r="3"/><circle cx="17" cy="15" r="3"/><polyline points="10 17 10 5 20 3 20 15"/></svg>
            </div>
          </div>
          <div class="qi-info">
            <div class="qi-title active">{{ player.currentTrack.title }}</div>
            <div class="qi-subtitle">{{ player.currentTrack.artist }} — {{ player.currentTrack.album }}</div>
          </div>
          <span class="qi-duration">{{ formatTime(player.currentTrack.duration) }}</span>
        </div>
      </div>

      <!-- Next in Queue section -->
      <div v-if="player.queue.length > 0" class="queue-section">
        <h3 class="section-label">
          {{ player.queueIndex >= 0 ? 'Next in Queue' : 'Queue' }}
        </h3>
        <div
          v-for="(track, idx) in player.queue"
          :key="track.path"
          :class="['queue-item', { 'queue-item-active': idx === player.queueIndex }]"
          v-show="idx !== player.queueIndex"
          @dblclick="player.playIndex(idx)"
        >
          <!-- Number / grip -->
          <div class="qi-num">
            <span class="qi-num-text">{{ idx + 1 }}</span>
            <span class="qi-grip">
              <svg viewBox="0 0 24 24" fill="currentColor"><circle cx="8" cy="4" r="1.5"/><circle cx="16" cy="4" r="1.5"/><circle cx="8" cy="10" r="1.5"/><circle cx="16" cy="10" r="1.5"/><circle cx="8" cy="16" r="1.5"/><circle cx="16" cy="16" r="1.5"/></svg>
            </span>
          </div>

          <!-- Track info -->
          <div class="qi-info">
            <div class="qi-title">{{ track.title }}</div>
            <div class="qi-subtitle">{{ track.artist }} — {{ track.album }}</div>
          </div>

          <!-- Duration -->
          <span class="qi-duration">{{ formatTime(track.duration) }}</span>

          <!-- Remove button -->
          <button class="qi-remove" @click="player.removeFromQueue(idx)">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M18 6 6 18M6 6l12 12"/></svg>
          </button>
        </div>
      </div>

      <!-- Empty state -->
      <div v-if="player.queue.length === 0" class="queue-empty animate-fade-in">
        <div class="queue-empty-icon">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <path d="M9 18V5l12-2v13"/><circle cx="6" cy="18" r="3"/><circle cx="18" cy="16" r="3"/>
          </svg>
        </div>
        <p class="queue-empty-title">Queue is empty</p>
        <p class="queue-empty-sub">Double-click tracks in your library to start playing</p>
      </div>
    </div>
  </div>
</template>

<style scoped>
.queue-view {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  background: var(--color-bg-primary);
}

/* ── Header ── */
.queue-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 24px 28px 16px;
  flex-shrink: 0;
}

.queue-header-left {
  display: flex;
  align-items: baseline;
  gap: 10px;
}

.queue-title {
  font-size: 24px;
  font-weight: 800;
  letter-spacing: -0.3px;
}

.queue-count {
  font-size: 13px;
  color: var(--color-text-tertiary);
}

.queue-header-right {
  display: flex;
  align-items: center;
  gap: 8px;
}

.queue-chip {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 6px 14px;
  font-family: var(--font-family);
  font-size: 12px;
  font-weight: 500;
  border: none;
  border-radius: 500px;
  cursor: pointer;
  transition: all 0.15s;
  background: var(--color-bg-elevated);
  color: var(--color-text-secondary);
}

.queue-chip:hover {
  color: var(--color-text-primary);
  background: var(--color-bg-hover);
}

.queue-chip.active {
  background: var(--color-accent);
  color: black;
}

.chip-icon {
  width: 14px;
  height: 14px;
}

.queue-clear-btn {
  font-family: var(--font-family);
  font-size: 12px;
  color: var(--color-text-tertiary);
  background: transparent;
  border: none;
  cursor: pointer;
  padding: 6px 12px;
  border-radius: 500px;
  transition: all 0.15s;
}

.queue-clear-btn:hover {
  color: var(--color-danger);
  background: rgba(231, 76, 60, 0.08);
}

/* ── Scroll ── */
.queue-scroll {
  flex: 1;
  overflow-y: auto;
  padding: 0 16px 24px;
}

/* ── Section ── */
.queue-section {
  margin-bottom: 24px;
}

.section-label {
  font-size: 12px;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: var(--color-text-tertiary);
  padding: 8px 12px;
}

/* ── Queue Item ── */
.queue-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px 12px;
  border-radius: 8px;
  cursor: pointer;
  transition: background 0.1s;
}

.queue-item:hover {
  background: var(--color-surface-glass-hover);
}

.queue-item-active {
  background: rgba(29, 185, 84, 0.06);
}

/* ── Number / Grip ── */
.qi-num {
  width: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  position: relative;
}

.qi-num-text {
  font-size: 13px;
  font-variant-numeric: tabular-nums;
  color: var(--color-text-tertiary);
}

.qi-grip {
  display: none;
  color: var(--color-text-tertiary);
}

.qi-grip svg {
  width: 14px;
  height: 14px;
}

.queue-item:hover .qi-num-text {
  display: none;
}

.queue-item:hover .qi-grip {
  display: flex;
}

.qi-num-active {
  width: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

/* ── Art ── */
.qi-art {
  width: 40px;
  height: 40px;
  border-radius: 4px;
  overflow: hidden;
  flex-shrink: 0;
  background: var(--color-bg-elevated);
}

.qi-art-img {
  width: 100%;
  height: 100%;
  background-size: cover;
  background-position: center;
}

.qi-art-placeholder {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.qi-art-placeholder svg {
  width: 16px;
  height: 16px;
  color: var(--color-text-tertiary);
}

/* ── Info ── */
.qi-info {
  flex: 1;
  min-width: 0;
}

.qi-title {
  font-size: 14px;
  font-weight: 500;
  color: var(--color-text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.qi-title.active {
  color: var(--color-accent);
}

.qi-subtitle {
  font-size: 12px;
  color: var(--color-text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

/* ── Duration ── */
.qi-duration {
  font-size: 12px;
  color: var(--color-text-tertiary);
  font-variant-numeric: tabular-nums;
  flex-shrink: 0;
}

/* ── Remove Button ── */
.qi-remove {
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  color: var(--color-text-tertiary);
  cursor: pointer;
  border-radius: 50%;
  opacity: 0;
  transition: all 0.15s;
  flex-shrink: 0;
}

.qi-remove svg {
  width: 14px;
  height: 14px;
}

.queue-item:hover .qi-remove {
  opacity: 1;
}

.qi-remove:hover {
  color: var(--color-danger);
  background: rgba(231, 76, 60, 0.1);
}

/* ── Empty State ── */
.queue-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  padding: 80px 0;
}

.queue-empty-icon {
  width: 80px;
  height: 80px;
  border-radius: 50%;
  background: var(--color-bg-tertiary);
  display: flex;
  align-items: center;
  justify-content: center;
}

.queue-empty-icon svg {
  width: 36px;
  height: 36px;
  color: var(--color-text-tertiary);
  opacity: 0.4;
}

.queue-empty-title {
  font-size: 16px;
  font-weight: 500;
  color: var(--color-text-secondary);
}

.queue-empty-sub {
  font-size: 13px;
  color: var(--color-text-tertiary);
}
</style>
