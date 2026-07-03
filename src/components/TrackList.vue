<script setup lang="ts">
import type { Track } from '../types'
import { formatTime } from '../types'
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

function isActive(track: Track): boolean {
  return player.currentTrack?.path === track.path
}
</script>

<template>
  <div class="tl">
    <div class="tl-header">
      <span v-if="trackNumbers !== false" class="col-n">#</span>
      <span class="col-title">TITLE</span>
      <span v-if="showArtist !== false" class="col-artist">ARTIST</span>
      <span v-if="showAlbum !== false" class="col-album">ALBUM</span>
      <span class="col-q">QUALITY</span>
      <span class="col-dur">TIME</span>
    </div>

    <div
      v-for="(track, idx) in tracks"
      :key="track.path"
      :class="['tl-row', { active: isActive(track) }]"
      @dblclick="emit('play', track, idx)"
    >
      <span v-if="trackNumbers !== false" class="col-n">
        <span v-if="isActive(track) && player.isPlaying" class="row-eq">
          <EqBars :playing="true" size="sm" />
        </span>
        <span v-else-if="isActive(track)" class="row-num on">></span>
        <span v-else class="row-num">{{ track.track_number || idx + 1 }}</span>
        <span class="row-play-hover">
          <svg viewBox="0 0 24 24" fill="currentColor"><path d="M8 5v14l11-7z"/></svg>
        </span>
      </span>

      <div class="col-title">
        <span :class="['row-title', { on: isActive(track) }]">{{ track.title }}</span>
        <span v-if="compact" class="row-sub">{{ track.artist }}</span>
      </div>

      <span v-if="showArtist !== false" class="col-artist row-dim">{{ track.artist }}</span>
      <span v-if="showAlbum !== false" class="col-album row-dim">{{ track.album }}</span>

      <div class="col-q">
        <AudioBadge :track="track" compact />
      </div>

      <span class="col-dur row-dim">{{ formatTime(track.duration) }}</span>
    </div>
  </div>
</template>

<style scoped>
.tl {
  display: flex;
  flex-direction: column;
}

.tl-header {
  display: grid;
  grid-template-columns: 36px 1fr 1.2fr 1.2fr 80px 50px;
  gap: 12px;
  padding: 8px 16px;
  font-size: 10px;
  font-weight: 700;
  text-transform: uppercase;
  color: color-mix(in srgb, var(--foreground) 40%, transparent);
  border-bottom: 2px solid var(--border);
  position: sticky;
  top: 0;
  background: var(--background);
  z-index: 10;
}
.tl-header .col-n { text-align: center; }
.tl-header .col-q { text-align: center; }
.tl-header .col-dur { text-align: right; }

.tl-row {
  display: grid;
  grid-template-columns: 36px 1fr 1.2fr 1.2fr 80px 50px;
  gap: 12px;
  padding: 8px 16px;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.1s;
  align-items: center;
  border-bottom: 1px solid color-mix(in srgb, var(--border) 30%, transparent);
}

.tl-row:hover {
  background: color-mix(in srgb, var(--main) 10%, transparent);
}

.tl-row.active {
  background: color-mix(in srgb, var(--main) 15%, transparent);
}

.col-n {
  text-align: center;
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
}

.row-num {
  font-variant-numeric: tabular-nums;
  color: color-mix(in srgb, var(--foreground) 50%, transparent);
  font-size: 12px;
}
.row-num.on {
  color: var(--main);
  font-weight: 700;
}

.row-eq {
  display: flex;
  align-items: center;
  justify-content: center;
}

.row-play-hover {
  display: none;
  align-items: center;
  justify-content: center;
}
.row-play-hover svg {
  width: 14px;
  height: 14px;
  color: var(--foreground);
}

.tl-row:hover .row-num,
.tl-row:hover .row-eq { display: none; }
.tl-row:hover .row-play-hover { display: flex; }

.col-title {
  min-width: 0;
  display: flex;
  flex-direction: column;
}
.row-title {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  color: var(--foreground);
  font-weight: 600;
}
.row-title.on {
  color: var(--main);
}
.row-sub {
  font-size: 11px;
  color: color-mix(in srgb, var(--foreground) 50%, transparent);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.row-dim {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  color: color-mix(in srgb, var(--foreground) 60%, transparent);
  font-size: 12px;
}

.col-q {
  display: flex;
  align-items: center;
  justify-content: center;
}

.col-dur {
  text-align: right;
  font-variant-numeric: tabular-nums;
}
</style>
