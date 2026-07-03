<script setup lang="ts">
import type { Track } from '../types'
import { hiResBadge, formatTime } from '../types'
import { usePlayerStore } from '../stores/player'
import EqBars from './EqBars.vue'
import AudioBadge from './AudioBadge.vue'

defineProps<{
  tracks: Track[]
  showAlbum?: boolean
  showArtist?: boolean
  trackNumbers?: boolean
  compact?: boolean
}>()

const emit = defineEmits<{ play: [track: Track, index: number] }>()
const player = usePlayerStore()

function isCurrentTrack(track: Track): boolean {
  return player.currentTrack?.path === track.path
}
</script>

<template>
  <div class="tracklist">
    <!-- Header row -->
    <div class="tracklist-header">
      <span v-if="trackNumbers !== false" class="col-num">#</span>
      <span class="col-title">Title</span>
      <span v-if="showArtist !== false" class="col-artist">Artist</span>
      <span v-if="showAlbum !== false" class="col-album">Album</span>
      <span class="col-quality">Quality</span>
      <span class="col-duration">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="duration-icon">
          <circle cx="12" cy="12" r="10"/><path d="M12 6v6l4 2"/>
        </svg>
      </span>
    </div>

    <!-- Track rows -->
    <div
      v-for="(track, idx) in tracks"
      :key="track.path"
      :class="['track-row', { 'track-active': isCurrentTrack(track) }]"
      @dblclick="emit('play', track, idx)"
    >
      <!-- Track number / playing indicator -->
      <span v-if="trackNumbers !== false" class="col-num">
        <span v-if="isCurrentTrack(track) && player.isPlaying" class="track-eq">
          <EqBars :playing="player.isPlaying" size="sm" />
        </span>
        <span v-else-if="isCurrentTrack(track)" class="track-num active">▶</span>
        <span v-else class="track-num">{{ track.track_number || idx + 1 }}</span>
        <!-- Hover play icon -->
        <span class="track-play-hover">
          <svg viewBox="0 0 24 24" fill="currentColor"><path d="M8 5v14l11-7z"/></svg>
        </span>
      </span>

      <!-- Title + artist subtitle -->
      <div class="col-title">
        <span :class="['track-title', { active: isCurrentTrack(track) }]">{{ track.title }}</span>
        <span v-if="compact" class="track-artist-sub">{{ track.artist }}</span>
      </div>

      <!-- Artist -->
      <span v-if="showArtist !== false" class="col-artist track-text-secondary">{{ track.artist }}</span>

      <!-- Album -->
      <span v-if="showAlbum !== false" class="col-album track-text-secondary">{{ track.album }}</span>

      <!-- Quality badge -->
      <div class="col-quality">
        <AudioBadge :track="track" compact />
      </div>

      <!-- Duration -->
      <span class="col-duration track-duration">{{ formatTime(track.duration) }}</span>
    </div>
  </div>
</template>

<style scoped>
.tracklist {
  display: flex;
  flex-direction: column;
}

/* ── Header ── */
.tracklist-header {
  display: grid;
  grid-template-columns: 40px 1fr 1.2fr 1.2fr 80px 56px;
  gap: 12px;
  padding: 8px 16px;
  font-size: 11px;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: var(--color-text-tertiary);
  border-bottom: 1px solid var(--color-border);
  position: sticky;
  top: 0;
  background: rgba(18, 18, 18, 0.92);
  backdrop-filter: blur(12px);
  z-index: 10;
}

.tracklist-header .col-num {
  text-align: center;
}

.tracklist-header .col-quality {
  text-align: center;
}

.tracklist-header .col-duration {
  text-align: right;
  display: flex;
  justify-content: flex-end;
  align-items: center;
}

.duration-icon {
  width: 14px;
  height: 14px;
}

/* ── Track Row ── */
.track-row {
  display: grid;
  grid-template-columns: 40px 1fr 1.2fr 1.2fr 80px 56px;
  gap: 12px;
  padding: 8px 16px;
  font-size: 13px;
  border-radius: 6px;
  cursor: pointer;
  transition: background 0.1s;
  align-items: center;
  margin: 0 4px;
}

.track-row:hover {
  background: var(--color-surface-glass-hover);
}

.track-active {
  background: rgba(29, 185, 84, 0.06);
}

/* ── Number Column ── */
.col-num {
  text-align: center;
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
  min-width: 0;
}

.track-num {
  font-variant-numeric: tabular-nums;
  color: var(--color-text-tertiary);
  font-size: 13px;
}

.track-num.active {
  color: var(--color-accent);
  font-size: 11px;
}

.track-eq {
  display: flex;
  align-items: center;
  justify-content: center;
}

.track-play-hover {
  display: none;
  align-items: center;
  justify-content: center;
}

.track-play-hover svg {
  width: 14px;
  height: 14px;
  color: var(--color-text-primary);
}

.track-row:hover .track-num,
.track-row:hover .track-eq {
  display: none;
}

.track-row:hover .track-play-hover {
  display: flex;
}

/* ── Title Column ── */
.col-title {
  min-width: 0;
  display: flex;
  flex-direction: column;
}

.track-title {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  color: var(--color-text-primary);
}

.track-title.active {
  color: var(--color-accent);
  font-weight: 500;
}

.track-artist-sub {
  font-size: 11px;
  color: var(--color-text-tertiary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

/* ── Other Columns ── */
.track-text-secondary {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  color: var(--color-text-secondary);
  font-size: 13px;
}

.col-quality {
  display: flex;
  align-items: center;
  justify-content: center;
}

.col-duration {
  text-align: right;
}

.track-duration {
  font-variant-numeric: tabular-nums;
  color: var(--color-text-secondary);
  font-size: 13px;
}
</style>
