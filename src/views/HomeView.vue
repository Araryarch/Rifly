<script setup lang="ts">
import { onMounted, computed } from 'vue'
import { useLibraryStore } from '../stores/library'
import { usePlayerStore } from '../stores/player'
import { useFavoritesStore } from '../stores/favorites'
import TrackList from '../components/TrackList.vue'
import type { Track } from '../types'

const lib = useLibraryStore()
const player = usePlayerStore()
const favorites = useFavoritesStore()

onMounted(async () => {
  await lib.loadRecentlyPlayed()
  await lib.loadMostPlayed()
})

const recentTracks = computed<Track[]>(() => {
  return lib.resolveRecords(lib.recentlyPlayed).slice(0, 10)
})

const topTracks = computed<Track[]>(() => {
  return lib.resolveRecords(lib.mostPlayed).slice(0, 10)
})

const favoriteTracks = computed<Track[]>(() => {
  return lib.tracks.filter(t => favorites.isFavorite(t.path)).slice(0, 8)
})

const totalDuration = computed(() => {
  const total = lib.tracks.reduce((s, t) => s + t.duration, 0)
  const h = Math.floor(total / 3600)
  const m = Math.floor((total % 3600) / 60)
  return h > 0 ? `${h}h ${m}m` : `${m}m`
})

function playTrack(_track: any, idx: number, list: Track[]) {
  player.playAll(list, idx)
}

function playRecent(_track: any, idx: number) {
  playTrack(_track, idx, recentTracks.value)
}

function playTop(_track: any, idx: number) {
  playTrack(_track, idx, topTracks.value)
}

function playFav(_track: any, idx: number) {
  playTrack(_track, idx, favoriteTracks.value)
}
</script>

<template>
  <div class="home">
    <div class="home-head">
      <h1 class="home-title">home</h1>
      <p class="home-sub" v-if="lib.tracks.length > 0">
        {{ lib.tracks.length }} tracks / {{ lib.albums.length }} albums / {{ totalDuration }}
      </p>
    </div>

    <div class="home-scroll">
      <div v-if="lib.tracks.length === 0" class="empty-state animate-fade-in-up">
        <div class="empty-icon">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M9 18V5l12-2v13"/><circle cx="6" cy="18" r="3"/><circle cx="18" cy="16" r="3"/></svg>
        </div>
        <h2>Welcome to Rifly</h2>
        <p>Go to <strong>Settings</strong> to connect a music folder or Spotify account.</p>
      </div>

      <template v-else>
        <div v-if="recentTracks.length > 0" class="section animate-fade-in">
          <h2 class="section-title">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><path d="M12 6v6l4 2"/></svg>
            Recently Played
          </h2>
          <TrackList :tracks="recentTracks" :showArtist="true" :showAlbum="true" compact @play="playRecent" />
        </div>

        <div v-if="topTracks.length > 0" class="section animate-fade-in">
          <h2 class="section-title">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z"/></svg>
            Most Played
          </h2>
          <TrackList :tracks="topTracks" :showArtist="true" :showAlbum="true" compact @play="playTop" />
        </div>

        <div v-if="favoriteTracks.length > 0" class="section animate-fade-in">
          <h2 class="section-title">
            <svg viewBox="0 0 24 24" fill="currentColor" style="color: oklch(65% 0.2 20)"><path d="M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z"/></svg>
            Liked Songs
          </h2>
          <TrackList :tracks="favoriteTracks" :showArtist="true" :showAlbum="true" compact @play="playFav" />
        </div>

        <div v-if="recentTracks.length === 0 && topTracks.length === 0 && favoriteTracks.length === 0" class="empty-state animate-fade-in-up">
          <div class="empty-icon">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M9 18V5l12-2v13"/><circle cx="6" cy="18" r="3"/><circle cx="18" cy="16" r="3"/></svg>
          </div>
          <h2>Start listening</h2>
          <p>Play some tracks and they'll appear here.</p>
        </div>
      </template>
    </div>
  </div>
</template>

<style scoped>
.home {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.home-head {
  padding: 24px 24px 16px;
  flex-shrink: 0;
}
.home-title {
  font-size: 28px;
  font-weight: 800;
  text-transform: lowercase;
  margin-bottom: 2px;
}
.home-sub {
  font-size: 11px;
  color: color-mix(in srgb, var(--foreground) 40%, transparent);
}

.home-scroll {
  flex: 1;
  overflow-y: auto;
  padding: 0 24px 24px;
}

.section {
  margin-bottom: 32px;
}
.section-title {
  font-size: 16px;
  font-weight: 800;
  margin-bottom: 16px;
  color: var(--foreground);
  display: flex;
  align-items: center;
  gap: 10px;
}
.section-title svg {
  width: 20px;
  height: 20px;
  color: var(--main);
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  text-align: center;
  padding: 80px 0;
  gap: 16px;
}
.empty-state h2 { font-size: 24px; font-weight: 800; color: var(--foreground); }
.empty-state p { font-size: 14px; color: var(--text-muted); max-width: 360px; }
.empty-state p strong { color: var(--main); }
.empty-icon {
  width: 64px; height: 64px;
  background: var(--panel-bg-hover);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-muted);
  margin-bottom: 8px;
}
.empty-icon svg { width: 32px; height: 32px; }
</style>
