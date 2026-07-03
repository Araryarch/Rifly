<script setup lang="ts">
import { ref, onMounted, computed, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { useLibraryStore } from '../stores/library'
import { usePlayerStore } from '../stores/player'
import { useSpotifyStore, mapSpotifyTrack } from '../stores/spotify'
import { useUiStore } from '../stores/ui'
import AlbumCard from '../components/AlbumCard.vue'
import TrackList from '../components/TrackList.vue'
import type { LibraryFilter, AlbumGroup } from '../types'

const lib = useLibraryStore()
const player = usePlayerStore()
const spotifyStore = useSpotifyStore()
const ui = useUiStore()

const selectedAlbum = ref<{ artist: string; album: string } | null>(null)
const showPathInput = ref(false)
const pathInput = ref('')
const filter = ref<LibraryFilter | 'playlists' | 'liked'>('albums')

const spotifyResults = ref<any[]>([])
const spotifyPlaylists = ref<any[]>([])
const spotifyLiked = ref<any[]>([])
const spotifySearching = ref(false)

watch(() => ui.searchTrigger, () => {
  onSearch(ui.searchQuery)
})

onMounted(async () => {
  try {
    const saved: string | null = await invoke('get_setting', { key: 'music_folder' })
    if (saved) {
      showPathInput.value = false
      lib.scanFolder(saved)
    } else {
      showPathInput.value = true
    }
  } catch {
    showPathInput.value = true
  }
})

async function pickFolder() {
  const selected = await open({
    directory: true,
    title: 'Select Music Folder',
  })
  if (selected && typeof selected === 'string') {
    lib.scanFolder(selected)
    invoke('set_setting', { key: 'music_folder', value: selected })
    showPathInput.value = false
  }
}

async function setFolderManual() {
  const p = pathInput.value.trim()
  if (!p) return
  lib.scanFolder(p)
  invoke('set_setting', { key: 'music_folder', value: p })
  showPathInput.value = false
}

function playTrack(_track: any, idx: number) {
  if (player.source === 'spotify') {
    player.playAll(spotifyResults.value, idx)
    return
  }
  const album = selectedAlbum.value
  if (album) {
    const tracks = lib.tracksForAlbum(album.artist, album.album)
    player.playAll(tracks, idx)
  } else {
    player.playAll(filteredTracks.value, idx)
  }
}

function playAlbum(tracks: any[]) {
  player.playAll(tracks)
}

function viewAlbum(artist: string, album: string) {
  selectedAlbum.value = { artist, album }
}

async function onSearch(query: string) {
  if (query) {
    spotifySearching.value = true
    try {
      const res = await spotifyStore.search(query)
      spotifyResults.value = res.tracks.items.map(mapSpotifyTrack)
      filter.value = 'tracks' // show local tracks, or we could have a combined view
    } catch (e) {
      console.error(e)
    } finally {
      spotifySearching.value = false
    }
    selectedAlbum.value = null
  }
}

async function loadPlaylists() {
  filter.value = 'playlists'
  ui.triggerSearch('')
  if (!spotifyStore.accessToken) return
  const res = await spotifyStore.getPlaylists()
  spotifyPlaylists.value = res.items
}

async function loadLikedSongs() {
  filter.value = 'liked'
  ui.triggerSearch('')
  if (!spotifyStore.accessToken) return
  const res = await spotifyStore.getLikedSongs()
  spotifyLiked.value = res.items.map((i: any) => mapSpotifyTrack(i.track))
}

async function playPlaylist(id: string) {
  if (!spotifyStore.accessToken) return
  const res = await spotifyStore.getPlaylistTracks(id)
  const tracks = res.items.map((i: any) => mapSpotifyTrack(i.track))
  player.playAll(tracks)
}

const filteredAlbums = computed(() => {
  if (!ui.searchQuery) return lib.albums
  const q = ui.searchQuery.toLowerCase()
  return lib.albums.filter((a: AlbumGroup) =>
    a.album.toLowerCase().includes(q) || a.artist.toLowerCase().includes(q)
  )
})

const filteredTracks = computed(() => {
  if (!ui.searchQuery) return lib.tracks
  const q = ui.searchQuery.toLowerCase()
  return lib.tracks.filter(t =>
    t.title.toLowerCase().includes(q) || t.artist.toLowerCase().includes(q) || t.album.toLowerCase().includes(q)
  )
})

const totalDuration = computed(() => {
  const total = lib.tracks.reduce((s, t) => s + t.duration, 0)
  const h = Math.floor(total / 3600)
  const m = Math.floor((total % 3600) / 60)
  return h > 0 ? `${h}h ${m}m` : `${m}m`
})

const albumDetail = computed(() => {
  if (!selectedAlbum.value) return null
  return lib.albums.find(
    (a: AlbumGroup) => a.artist === selectedAlbum.value!.artist && a.album === selectedAlbum.value!.album
  )
})
</script>

<template>
  <div class="lib">
    <div v-if="showPathInput" class="welcome">
      <div class="welcome-card animate-fade-in-up">
        <div class="w-head">// rifly</div>
        <p class="w-desc">where is your music?</p>
        
        <button class="btn-primary w-btn" @click="pickFolder">OPEN FOLDER</button>

        <div class="w-manual">
          <p class="w-hint">or type path manually:</p>
          <div class="w-row">
            <input
              v-model="pathInput"
              type="text"
              placeholder="C:\Music"
              class="w-input"
              @keydown.enter="setFolderManual"
            />
            <button class="w-set" @click="setFolderManual">SET</button>
          </div>
        </div>
      </div>
    </div>

    <div v-else-if="lib.scanning" class="center-state">
      <div class="animate-fade-in">
        <svg class="spinner" viewBox="0 0 24 24" fill="none">
          <circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="3" stroke-dasharray="31.4 31.4" stroke-linecap="round"/>
        </svg>
        <p>scanning...</p>
      </div>
    </div>

    <template v-else-if="lib.tracks.length > 0">
      <div v-if="selectedAlbum" class="detail">
        <div class="detail-hero">
          <button class="btn-back" @click="selectedAlbum = null">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M19 12H5m7-7l-7 7 7 7"/></svg>
            BACK
          </button>
          <div class="detail-flex">
            <div class="detail-art">
              <div v-if="albumDetail?.coverArt" class="detail-art-img" :style="{ backgroundImage: `url(${albumDetail.coverArt})` }" />
              <div v-else class="detail-art-empty">
                <svg viewBox="0 0 24 24" fill="currentColor"><circle cx="7" cy="17" r="3"/><circle cx="17" cy="15" r="3"/><polyline points="10 17 10 5 20 3 20 15"/></svg>
              </div>
            </div>
            <div class="detail-info">
              <span class="detail-type">ALBUM</span>
              <h1 class="detail-title">{{ selectedAlbum.album }}</h1>
              <div class="detail-meta">
                <span class="detail-artist">{{ selectedAlbum.artist }}</span>
                <span v-if="albumDetail?.year"> / {{ albumDetail.year }}</span>
                <span> / {{ albumDetail?.tracks.length || 0 }} trk</span>
              </div>
              <button class="btn-primary detail-play" @click="albumDetail && playAlbum(albumDetail.tracks)">
                <svg viewBox="0 0 24 24" fill="currentColor"><path d="M8 5v14l11-7z"/></svg>
                PLAY ALL
              </button>
            </div>
          </div>
        </div>
        <div class="detail-tracks">
          <TrackList
            :tracks="lib.tracksForAlbum(selectedAlbum.artist, selectedAlbum.album)"
            :showArtist="false"
            :showAlbum="false"
            @play="playTrack"
          />
        </div>
      </div>

      <div v-else class="main">
        <div class="main-head">
          <div class="head-top">
            <div>
              <h1 class="main-title">library</h1>
              <div class="main-stats">{{ lib.tracks.length }} tracks / {{ lib.albums.length }} albums / {{ totalDuration }}</div>
            </div>
            <div class="head-actions">
              <button class="btn-icon" @click="showPathInput = true; pathInput = lib.musicFolder">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/></svg>
              </button>
            </div>
          </div>
          <div class="filters">
            <button :class="['filter-chip', { on: filter === 'albums' }]" @click="filter = 'albums'; ui.triggerSearch('')">ALBUMS</button>
            <button :class="['filter-chip', { on: filter === 'artists' }]" @click="filter = 'artists'; ui.triggerSearch('')">ARTISTS</button>
            <button :class="['filter-chip', { on: filter === 'tracks' }]" @click="filter = 'tracks'; ui.triggerSearch('')">TRACKS</button>
            <div class="v-divider" />
            <button :class="['filter-chip', { on: filter === 'playlists' }]" @click="loadPlaylists">PLAYLISTS</button>
            <button :class="['filter-chip', { on: filter === 'liked' }]" @click="loadLikedSongs">LIKED SONGS</button>
          </div>
        </div>

        <div v-if="filter === 'albums'" class="main-scroll">
            <div class="grid-albums">
              <div v-for="a in filteredAlbums" :key="a.artist + a.album" @click="viewAlbum(a.artist, a.album)">
                <AlbumCard :artist="a.artist" :album="a.album" :tracks="a.tracks" :year="a.year" :coverArt="a.coverArt" @playAll="playAlbum" />
              </div>
            </div>
            <div v-if="filteredAlbums.length === 0 && ui.searchQuery" class="empty">no results</div>
          </div>
  
          <div v-else-if="filter === 'artists'" class="main-scroll">
            <div v-for="artist in lib.artists" :key="artist" class="artist-row">
              {{ artist }}
            </div>
          </div>
  
          <div v-else-if="filter === 'tracks'" class="main-scroll">
            <div v-if="ui.searchQuery" class="search-header">LOCAL RESULTS</div>
            <TrackList :tracks="filteredTracks" :showArtist="true" :showAlbum="true" compact @play="playTrack" />
            
            <div v-if="ui.searchQuery" class="search-header" style="margin-top: 24px">SPOTIFY RESULTS</div>
            <div v-if="spotifySearching" class="empty" style="padding: 10px">searching spotify...</div>
            <TrackList v-if="ui.searchQuery && spotifyResults.length > 0" :tracks="spotifyResults" :showArtist="true" :showAlbum="true" compact @play="playTrack" />
          </div>

          <div v-else-if="filter === 'playlists'" class="main-scroll">
            <div v-if="!spotifyStore.accessToken" class="empty">spotify is not connected. go to settings.</div>
            <div class="grid-albums">
              <div v-for="p in spotifyPlaylists" :key="p.id" class="playlist-card" @click="playPlaylist(p.id)">
                <div class="p-cover" :style="{ backgroundImage: `url(${p.images?.[0]?.url})` }" />
                <div class="p-info">
                  <div class="p-title">{{ p.name }}</div>
                  <div class="p-owner">{{ p.owner.display_name }}</div>
                </div>
              </div>
            </div>
          </div>

          <div v-else-if="filter === 'liked'" class="main-scroll">
            <div v-if="!spotifyStore.accessToken" class="empty">spotify is not connected. go to settings.</div>
            <TrackList v-else :tracks="spotifyLiked" :showArtist="true" :showAlbum="true" compact @play="playTrack" />
          </div>
        </div>
      </template>

    <div v-else class="center-state">
      <div class="animate-fade-in-up">
        <p>no music found</p>
        <button class="btn-link" @click="showPathInput = true">change folder</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.lib {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.welcome {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 24px;
}

.welcome-card {
  display: flex;
  flex-direction: column;
  max-width: 420px;
  width: 100%;
  padding: 32px;
  background: var(--panel-bg-hover);
  border-radius: var(--radius-base);
}

.w-head {
  font-size: 24px;
  font-weight: 700;
  color: var(--foreground);
  margin-bottom: 4px;
}
.w-desc {
  font-size: 14px;
  color: var(--text-muted);
  margin-bottom: 24px;
}

.btn-primary {
  background: var(--foreground);
  color: #000;
  border: none;
  border-radius: 24px;
  font-weight: 700;
  cursor: pointer;
  transition: transform 0.1s;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
}
.btn-primary:hover {
  transform: scale(1.04);
}

.w-btn {
  padding: 16px;
  font-size: 16px;
  width: 100%;
  margin-bottom: 24px;
}

.w-manual {
  border-top: 1px solid rgba(255,255,255,0.1);
  padding-top: 16px;
}
.w-hint {
  font-size: 12px;
  color: var(--text-muted);
  margin-bottom: 8px;
}
.w-row {
  display: flex;
}
.w-input {
  flex: 1;
  background: rgba(255,255,255,0.05);
  border: none;
  border-radius: 4px 0 0 4px;
  padding: 10px 12px;
  color: var(--foreground);
  outline: none;
}
.w-input:focus {
  background: rgba(255,255,255,0.1);
}
.w-set {
  background: rgba(255,255,255,0.1);
  color: var(--foreground);
  border: none;
  border-radius: 0 4px 4px 0;
  padding: 0 16px;
  font-weight: 600;
  cursor: pointer;
  transition: background 0.1s;
}
.w-set:hover {
  background: rgba(255,255,255,0.15);
}

.center-state {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  text-align: center;
  color: color-mix(in srgb, var(--foreground) 50%, transparent);
}
.spinner {
  width: 24px;
  height: 24px;
  color: var(--main);
  animation: spin 1s linear infinite;
  margin: 0 auto 12px;
}
@keyframes spin { 100% { transform: rotate(360deg); } }

.btn-link {
  background: transparent;
  border: none;
  color: var(--main);
  text-decoration: underline;
  cursor: pointer;
  font-family: var(--font);
  margin-top: 8px;
}

.main {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.main-head {
  padding: 24px 24px 0;
  flex-shrink: 0;
}
.head-top {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 20px;
}
.main-title {
  font-size: 28px;
  font-weight: 800;
  text-transform: lowercase;
  margin-bottom: 2px;
}
.main-stats {
  font-size: 11px;
  color: color-mix(in srgb, var(--foreground) 40%, transparent);
}

.head-actions {
  display: flex;
  align-items: center;
  gap: 12px;
}
.btn-icon {
  width: 36px;
  height: 36px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--secondary-background);
  border: 2px solid var(--border);
  border-radius: var(--radius-base);
  color: var(--foreground);
  cursor: pointer;
  box-shadow: 2px 2px 0px 0px var(--border);
  transition: all 0.15s;
}
.btn-icon svg { width: 16px; height: 16px; }
.btn-icon:hover {
  box-shadow: none;
  transform: translate(2px, 2px);
}

.filters {
  display: flex;
  gap: 8px;
  padding-bottom: 16px;
}
.filter-chip {
  padding: 6px 16px;
  font-family: var(--font);
  font-size: 11px;
  font-weight: 700;
  background: var(--secondary-background);
  color: var(--foreground);
  border: 2px solid var(--border);
  border-radius: var(--radius-base);
  cursor: pointer;
  box-shadow: var(--shadow);
  transition: all 0.15s;
}
.filter-chip:hover {
  box-shadow: none;
  transform: translate(4px, 4px);
}
.filter-chip.on {
  background: var(--main);
  color: var(--main-foreground);
  box-shadow: none;
  transform: translate(4px, 4px);
}

.main-scroll {
  flex: 1;
  overflow-y: auto;
  padding: 0 24px 24px;
}

.grid-albums {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(170px, 1fr));
  gap: 16px;
}

.empty {
  text-align: center;
  padding: 40px;
  color: color-mix(in srgb, var(--foreground) 40%, transparent);
}

.artist-row {
  padding: 12px 16px;
  border-bottom: 2px solid var(--border);
  cursor: pointer;
  font-weight: 600;
}
.artist-row:hover {
  background: var(--secondary-background);
}

.detail {
  height: 100%;
  display: flex;
  flex-direction: column;
}
.detail-hero {
  padding: 24px;
  border-bottom: 2px solid var(--border);
}
.btn-back {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  font-family: var(--font);
  font-size: 11px;
  font-weight: 700;
  background: transparent;
  border: none;
  color: var(--foreground);
  cursor: pointer;
  margin-bottom: 20px;
}
.btn-back svg { width: 16px; height: 16px; }
.btn-back:hover { color: var(--main); }

.detail-flex {
  display: flex;
  gap: 24px;
  align-items: flex-end;
}
.detail-art {
  width: 180px;
  height: 180px;
  border: 2px solid var(--border);
  border-radius: var(--radius-base);
  box-shadow: var(--shadow);
  background: var(--secondary-background);
  overflow: hidden;
  flex-shrink: 0;
}
.detail-art-img {
  width: 100%;
  height: 100%;
  background-size: cover;
  background-position: center;
}
.detail-art-empty {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}
.detail-art-empty svg {
  width: 48px;
  height: 48px;
  color: color-mix(in srgb, var(--foreground) 30%, transparent);
}

.detail-info {
  min-width: 0;
}
.detail-type {
  font-size: 10px;
  font-weight: 700;
  color: color-mix(in srgb, var(--foreground) 40%, transparent);
}
.detail-title {
  font-size: 32px;
  font-weight: 800;
  line-height: 1.1;
  margin: 8px 0;
}
.detail-meta {
  font-size: 13px;
  color: color-mix(in srgb, var(--foreground) 60%, transparent);
  margin-bottom: 16px;
}
.detail-artist {
  color: var(--foreground);
  font-weight: 700;
}
.detail-play {
  padding: 10px 24px;
  font-size: 12px;
}
.detail-play svg { width: 14px; height: 14px; }

.detail-tracks {
  flex: 1;
  overflow-y: auto;
}

.v-divider {
  width: 2px;
  background: var(--border);
  margin: 0 4px;
}

.search-header {
  font-size: 14px;
  font-weight: 800;
  color: var(--main);
  margin-bottom: 12px;
}

.playlist-card {
  background: var(--secondary-background);
  border: 2px solid var(--border);
  border-radius: var(--radius-base);
  padding: 12px;
  cursor: pointer;
  box-shadow: 2px 2px 0px 0px var(--border);
  transition: all 0.15s;
}
.playlist-card:hover {
  transform: translate(2px, 2px);
  box-shadow: none;
}
.p-cover {
  width: 100%;
  aspect-ratio: 1;
  background-size: cover;
  background-position: center;
  border: 2px solid var(--border);
  border-radius: calc(var(--radius-base) - 2px);
  margin-bottom: 12px;
  background-color: var(--background);
}
.p-title {
  font-weight: 700;
  font-size: 13px;
  margin-bottom: 4px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.p-owner {
  font-size: 11px;
  color: color-mix(in srgb, var(--foreground) 60%, transparent);
}
</style>
