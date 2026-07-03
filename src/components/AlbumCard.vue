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
const hiRes = isHiRes(props.tracks[0])
</script>

<template>
  <div class="card">
    <div class="card-art-wrap">
      <div v-if="coverArt" class="card-art" :style="{ backgroundImage: `url(${coverArt})` }" />
      <div v-else class="card-art-empty">
        <svg viewBox="0 0 24 24" fill="currentColor"><circle cx="7" cy="17" r="3"/><circle cx="17" cy="15" r="3"/><polyline points="10 17 10 5 20 3 20 15"/></svg>
      </div>
      <div v-if="hiRes" class="card-hr">HR</div>
      <button class="card-play" @click.stop="emit('playAll', tracks)">
        <svg viewBox="0 0 24 24" fill="currentColor"><path d="M8 5v14l11-7z"/></svg>
      </button>
    </div>
    <div class="card-info">
      <div class="card-title">{{ album }}</div>
      <div class="card-artist">{{ artist }}</div>
      <div class="card-meta">{{ tracks.length }} trk / {{ formatTime(totalDuration) }}</div>
    </div>
  </div>
</template>

<style scoped>
.card {
  background: var(--secondary-background);
  border: 2px solid var(--border);
  border-radius: var(--radius-base);
  padding: 12px;
  cursor: pointer;
  box-shadow: var(--shadow);
  transition: all 0.15s;
}

.card:hover {
  transform: translate(-4px, -4px);
  box-shadow: 8px 8px 0px 0px var(--border);
}

.card-art-wrap {
  position: relative;
  aspect-ratio: 1;
  border: 2px solid var(--border);
  border-radius: var(--radius-base);
  overflow: hidden;
  margin-bottom: 12px;
  background: var(--background);
}

.card-art {
  width: 100%;
  height: 100%;
  background-size: cover;
  background-position: center;
}

.card-art-empty {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}
.card-art-empty svg {
  width: 40px;
  height: 40px;
  color: color-mix(in srgb, var(--foreground) 40%, transparent);
}

.card-hr {
  position: absolute;
  top: 6px;
  right: 6px;
  background: var(--secondary-background);
  color: var(--gold);
  border: 2px solid var(--gold);
  border-radius: var(--radius-base);
  font-size: 9px;
  font-weight: 800;
  padding: 2px 6px;
}

.card-play {
  position: absolute;
  bottom: 8px;
  right: 8px;
  width: 36px;
  height: 36px;
  background: var(--main);
  color: var(--main-foreground);
  border: 2px solid var(--border);
  border-radius: var(--radius-base);
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: var(--shadow);
  cursor: pointer;
  opacity: 0;
  transition: all 0.15s;
}
.card-play svg {
  width: 16px;
  height: 16px;
  margin-left: 2px;
}

.card:hover .card-play {
  opacity: 1;
}

.card-play:hover {
  box-shadow: none;
  transform: translate(4px, 4px);
}

.card-info {
  min-width: 0;
}

.card-title {
  font-size: 14px;
  font-weight: 700;
  color: var(--foreground);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  margin-bottom: 2px;
}

.card-artist {
  font-size: 12px;
  color: color-mix(in srgb, var(--foreground) 70%, transparent);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  margin-bottom: 6px;
}

.card-meta {
  font-size: 10px;
  color: color-mix(in srgb, var(--foreground) 40%, transparent);
}
</style>
