<script setup lang="ts">
import type { Track } from '../types'
import { isHiRes, formatTime } from '../types'

const props = defineProps<{
  artist: string
  album: string
  tracks: Track[]
  year: number
  coverArt?: string | null
}>()

const emit = defineEmits<{ playAll: [tracks: Track[]] }>()

const totalDuration = props.tracks.reduce((s, t) => s + t.duration, 0)
const maxSR = Math.max(...props.tracks.map(t => t.sample_rate))
const hiRes = isHiRes(props.tracks[0])
</script>

<template>
  <div class="album-card group">
    <div class="card-art-wrapper">
      <div v-if="coverArt" class="card-art" :style="{ backgroundImage: `url(${coverArt})` }" />
      <div v-else class="card-art-placeholder">
        <svg viewBox="0 0 24 24" fill="currentColor">
          <circle cx="7" cy="17" r="3"/><circle cx="17" cy="15" r="3"/><polyline points="10 17 10 5 20 3 20 15"/>
        </svg>
      </div>
      <!-- Hi-Res corner badge -->
      <div v-if="hiRes" class="card-hires-tag">
        <svg viewBox="0 0 24 24" fill="currentColor"><path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"/></svg>
        Hi-Res
      </div>
      <!-- Play button overlay -->
      <button class="card-play-btn" @click.stop="emit('playAll', tracks)">
        <svg viewBox="0 0 24 24" fill="currentColor"><path d="M8 5v14l11-7z"/></svg>
      </button>
      <!-- Shadow overlay -->
      <div class="card-art-shadow" />
    </div>
    <div class="card-body">
      <div class="card-title">{{ album }}</div>
      <div class="card-subtitle">{{ year ? `${year} · ` : '' }}{{ artist }}</div>
      <div class="card-meta">
        <span>{{ tracks.length }} track{{ tracks.length > 1 ? 's' : '' }}</span>
        <span class="meta-dot">·</span>
        <span>{{ formatTime(totalDuration) }}</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.album-card {
  position: relative;
  background: var(--color-bg-card);
  border-radius: 8px;
  overflow: hidden;
  cursor: pointer;
  transition: all 0.25s ease;
  padding: 12px;
}

.album-card:hover {
  background: var(--color-bg-elevated);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.35);
  transform: translateY(-2px);
}

/* ── Art ── */
.card-art-wrapper {
  position: relative;
  border-radius: 6px;
  overflow: hidden;
  margin-bottom: 12px;
  aspect-ratio: 1;
}

.card-art {
  width: 100%;
  height: 100%;
  background-size: cover;
  background-position: center;
}

.card-art-placeholder {
  width: 100%;
  height: 100%;
  background: linear-gradient(135deg, var(--color-bg-tertiary), var(--color-bg-elevated));
  display: flex;
  align-items: center;
  justify-content: center;
}

.card-art-placeholder svg {
  width: 48px;
  height: 48px;
  color: var(--color-text-tertiary);
  opacity: 0.35;
}

.card-art-shadow {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  height: 40%;
  background: linear-gradient(transparent, rgba(0, 0, 0, 0.2));
  pointer-events: none;
  opacity: 0;
  transition: opacity 0.25s;
}

.album-card:hover .card-art-shadow {
  opacity: 1;
}

/* ── Hi-Res tag ── */
.card-hires-tag {
  position: absolute;
  top: 6px;
  right: 6px;
  display: inline-flex;
  align-items: center;
  gap: 2px;
  font-size: 9px;
  font-weight: 700;
  color: var(--color-gold);
  background: rgba(0, 0, 0, 0.7);
  backdrop-filter: blur(8px);
  padding: 2px 6px;
  border-radius: 4px;
  z-index: 2;
  letter-spacing: 0.04em;
}

.card-hires-tag svg {
  width: 8px;
  height: 8px;
}

/* ── Play Button ── */
.card-play-btn {
  position: absolute;
  bottom: 8px;
  right: 8px;
  width: 44px;
  height: 44px;
  border-radius: 50%;
  background: var(--color-accent);
  color: black;
  border: none;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  opacity: 0;
  transform: translateY(8px);
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  z-index: 3;
  box-shadow: 0 8px 16px rgba(0, 0, 0, 0.4);
}

.card-play-btn svg {
  width: 20px;
  height: 20px;
  margin-left: 2px;
}

.card-play-btn:hover {
  background: var(--color-accent-hover);
  transform: translateY(8px) scale(1.06);
}

.album-card:hover .card-play-btn {
  opacity: 1;
  transform: translateY(0);
}

/* ── Body ── */
.card-body {
  min-width: 0;
}

.card-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--color-text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  margin-bottom: 2px;
}

.card-subtitle {
  font-size: 12px;
  color: var(--color-text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  margin-bottom: 4px;
}

.card-meta {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 11px;
  color: var(--color-text-tertiary);
}

.meta-dot {
  color: var(--color-text-subdued);
}
</style>
