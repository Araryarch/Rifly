<script setup lang="ts">
import { computed } from 'vue'
import { useLibraryStore } from '../stores/library'
import { usePlayerStore } from '../stores/player'
import { useFavoritesStore } from '../stores/favorites'
import AlbumCard from '../components/AlbumCard.vue'
import TrackList from '../components/TrackList.vue'
import type { Track, AlbumGroup } from '../types'

const lib = useLibraryStore()
const player = usePlayerStore()
const favorites = useFavoritesStore()

const favoriteTracks = computed<Track[]>(() => {
  return lib.tracks.filter(t => favorites.isFavorite(t.path))
})

const favoriteAlbums = computed<AlbumGroup[]>(() => {
  const favTracks = favoriteTracks.value
  const map = new Map<string, Track[]>()
  for (const t of favTracks) {
    const key = `${t.album_artist || t.artist}||${t.album || '(Unknown)'}`
    if (!map.has(key)) map.set(key, [])
    map.get(key)!.push(t)
  }
  return Array.from(map.entries()).map(([key, items]) => {
    const [artist, album] = key.split('||')
    items.sort((a, b) => a.disc_number - b.disc_number || a.track_number - b.track_number)
    return { artist, album, tracks: items, year: items[0]?.year || 0, coverArt: null }
  }).sort((a, b) => a.album.localeCompare(b.album))
})

function playTrack(_track: any, idx: number) {
  player.playAll(favoriteTracks.value, idx)
}

function playAlbum(tracks: any[]) {
  player.playAll(tracks)
}
</script>

<template>
  <div class="liked">
    <div class="liked-head">
      <h1 class="liked-title">liked songs</h1>
      <p class="liked-sub" v-if="favoriteTracks.length > 0">{{ favoriteTracks.length }} tracks / {{ favoriteAlbums.length }} albums</p>
    </div>

    <div class="liked-scroll">
      <div v-if="favoriteTracks.length === 0" class="empty-state animate-fade-in-up">
        <div class="empty-icon">
          <svg viewBox="0 0 24 24" fill="currentColor" style="color: oklch(65% 0.2 20)"><path d="M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z"/></svg>
        </div>
        <h2>No liked songs yet</h2>
        <p>Heart a track to add it here.</p>
      </div>

      <template v-else>
        <div v-if="favoriteAlbums.length > 0" class="section animate-fade-in">
          <h2 class="section-title">❤️ Albums</h2>
          <div class="grid-albums">
            <div v-for="a in favoriteAlbums" :key="'fav-' + a.artist + a.album">
              <AlbumCard :artist="a.artist" :album="a.album" :tracks="a.tracks" :year="a.year" :coverArt="a.coverArt" @playAll="playAlbum" />
            </div>
          </div>
        </div>

        <div class="section animate-fade-in">
          <h2 class="section-title">❤️ All Tracks</h2>
          <TrackList :tracks="favoriteTracks" :showArtist="true" :showAlbum="true" compact @play="playTrack" />
        </div>
      </template>
    </div>
  </div>
</template>

<style scoped>
.liked {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.liked-head {
  padding: 24px 24px 16px;
  flex-shrink: 0;
}
.liked-title {
  font-size: 28px;
  font-weight: 800;
  text-transform: lowercase;
  margin-bottom: 2px;
}
.liked-sub {
  font-size: 11px;
  color: color-mix(in srgb, var(--foreground) 40%, transparent);
}

.liked-scroll {
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
}

.grid-albums {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(170px, 1fr));
  gap: 16px;
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
.empty-state p { font-size: 14px; color: var(--text-muted); }
.empty-icon {
  width: 64px; height: 64px;
  background: var(--panel-bg-hover);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 8px;
}
.empty-icon svg { width: 32px; height: 32px; }
</style>
