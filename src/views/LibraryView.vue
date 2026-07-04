<script setup lang="ts">
import { ref, onMounted, computed, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useLibraryStore } from '../stores/library'
import { usePlayerStore } from '../stores/player'
import { useFavoritesStore } from '../stores/favorites'
import { useSpotifyStore, mapSpotifyTrack } from '../stores/spotify'
import { useUiStore } from '../stores/ui'
import AlbumCard from '../components/AlbumCard.vue'
import TrackList from '../components/TrackList.vue'
import MetadataEditor from '../components/MetadataEditor.vue'
import AddMusicForm from '../components/AddMusicForm.vue'
import type { LibraryFilter, AlbumGroup, Track } from '../types'

const lib = useLibraryStore()
const player = usePlayerStore()
const favorites = useFavoritesStore()
const spotifyStore = useSpotifyStore()
const ui = useUiStore()

const selectedAlbum = ref<{ artist: string; album: string } | null>(null)
const filter = ref<LibraryFilter | 'playlists' | 'liked' | 'favorites' | 'recent' | 'top'>('albums')

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
      lib.scanFolder(saved)
    }
  } catch {}
})

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
      filter.value = 'tracks'
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

function loadFavorites() {
  filter.value = 'favorites'
  ui.triggerSearch('')
  lib.loadRecentlyPlayed()
  lib.loadMostPlayed()
}

function loadRecent() {
  filter.value = 'recent'
  ui.triggerSearch('')
  lib.loadRecentlyPlayed()
}

function loadTop() {
  filter.value = 'top'
  ui.triggerSearch('')
  lib.loadMostPlayed()
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

// Pagination
const ALBUMS_PER_PAGE = 24
const TRACKS_PER_PAGE = 50
const ARTISTS_PER_PAGE = 50

const albumPage = ref(1)
const trackPage = ref(1)
const artistPage = ref(1)

const scrollContainer = ref<HTMLElement | null>(null)

function scrollToTop() {
  if (scrollContainer.value) {
    scrollContainer.value.scrollTop = 0
  }
}

const paginatedAlbums = computed(() => filteredAlbums.value.slice((albumPage.value - 1) * ALBUMS_PER_PAGE, albumPage.value * ALBUMS_PER_PAGE))
const paginatedTracks = computed(() => filteredTracks.value.slice((trackPage.value - 1) * TRACKS_PER_PAGE, trackPage.value * TRACKS_PER_PAGE))
const paginatedArtists = computed(() => lib.artists.slice((artistPage.value - 1) * ARTISTS_PER_PAGE, artistPage.value * ARTISTS_PER_PAGE))

function resetPagination() {
  albumPage.value = 1
  trackPage.value = 1
  artistPage.value = 1
}

watch(filter, () => resetPagination())
watch(() => ui.searchQuery, () => resetPagination())

// Metadata editor
const editingTrack = ref<Track | null>(null)
const showEditor = ref(false)
const showAddForm = ref(false)

function editTrackMetadata(track: Track) {
  editingTrack.value = track
  showEditor.value = true
}

function onMetadataSaved(updated: Track) {
  lib.replaceTrack(updated)
}

function addMusic() {
  showAddForm.value = true
}

function onMusicCreated(track: Track) {
  lib.addTrack(track)
}

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

// Favorite tracks from library
const favoriteTracks = computed<Track[]>(() => {
  return lib.tracks.filter(t => favorites.isFavorite(t.path))
})

// Recently played tracks (resolved from paths)
const recentTracks = computed<Track[]>(() => {
  return lib.resolveRecords(lib.recentlyPlayed)
})

// Most played tracks
const topTracks = computed<Track[]>(() => {
  return lib.resolveRecords(lib.mostPlayed)
})

// Favorite albums grouping
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

</script>

<template>
  <div class="lib-wrapper" style="height: 100%;">
    <div class="lib">
      <div v-if="!lib.scanning && lib.tracks.length === 0" class="center-state">
        <div class="empty-hero animate-fade-in-up">
          <div class="empty-icon">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M9 18V5l12-2v13"/><circle cx="6" cy="18" r="3"/><circle cx="18" cy="16" r="3"/></svg>
          </div>
          <h2>Your library is empty</h2>
          <p>Add music files or go to <strong>Settings</strong> to connect a folder.</p>
          <button class="btn-primary add-btn" @click="addMusic">+ add music</button>
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
          <div class="detail-toolbar">
            <button class="btn-edit" @click="albumDetail && editTrackMetadata(albumDetail.tracks[0])">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/><path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/></svg>
              edit metadata
            </button>
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
                <button class="btn-add-music" @click="addMusic">+ add music</button>
              </div>
            </div>
            <div class="filters">
              <button :class="['filter-chip', { on: filter === 'albums' }]" @click="filter = 'albums'; ui.triggerSearch('')">ALBUMS</button>
              <button :class="['filter-chip', { on: filter === 'artists' }]" @click="filter = 'artists'; ui.triggerSearch('')">ARTISTS</button>
              <button :class="['filter-chip', { on: filter === 'tracks' }]" @click="filter = 'tracks'; ui.triggerSearch('')">TRACKS</button>
              <button :class="['filter-chip', { on: filter === 'favorites' }]" @click="loadFavorites">❤️ LIKED</button>
              <button :class="['filter-chip', { on: filter === 'recent' }]" @click="loadRecent">🕐 RECENT</button>
              <button :class="['filter-chip', { on: filter === 'top' }]" @click="loadTop">🔥 TOP</button>
              <div class="v-divider" />
              <button :class="['filter-chip', { on: filter === 'playlists' }]" @click="loadPlaylists">PLAYLISTS</button>
              <button :class="['filter-chip', { on: filter === 'liked' }]" @click="loadLikedSongs">LIKED SONGS</button>
            </div>
          </div>

          <div v-if="filter === 'albums'" class="main-scroll" ref="scrollContainer">
            <div class="grid-albums">
              <div v-for="a in paginatedAlbums" :key="a.artist + a.album" @click="viewAlbum(a.artist, a.album)">
                <AlbumCard :artist="a.artist" :album="a.album" :tracks="a.tracks" :year="a.year" :coverArt="a.coverArt" @playAll="playAlbum" />
              </div>
            </div>
            
            <div v-if="filteredAlbums.length === 0 && ui.searchQuery" class="empty">no results</div>
            <div v-else-if="filteredAlbums.length > ALBUMS_PER_PAGE" class="pagination">
              <button class="btn-page" :disabled="albumPage === 1" @click="albumPage--; scrollToTop()">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M15 18l-6-6 6-6"/></svg>
                PREV
              </button>
              <span class="page-info">PAGE {{ albumPage }} OF {{ Math.ceil(filteredAlbums.length / ALBUMS_PER_PAGE) }}</span>
              <button class="btn-page" :disabled="albumPage >= Math.ceil(filteredAlbums.length / ALBUMS_PER_PAGE)" @click="albumPage++; scrollToTop()">
                NEXT
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M9 18l6-6-6-6"/></svg>
              </button>
            </div>
          </div>

          <div v-else-if="filter === 'artists'" class="main-scroll" ref="scrollContainer">
            <div v-for="artist in paginatedArtists" :key="artist" class="artist-row">{{ artist }}</div>
            
            <div v-if="lib.artists.length > ARTISTS_PER_PAGE" class="pagination">
              <button class="btn-page" :disabled="artistPage === 1" @click="artistPage--; scrollToTop()">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M15 18l-6-6 6-6"/></svg>
                PREV
              </button>
              <span class="page-info">PAGE {{ artistPage }} OF {{ Math.ceil(lib.artists.length / ARTISTS_PER_PAGE) }}</span>
              <button class="btn-page" :disabled="artistPage >= Math.ceil(lib.artists.length / ARTISTS_PER_PAGE)" @click="artistPage++; scrollToTop()">
                NEXT
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M9 18l6-6-6-6"/></svg>
              </button>
            </div>
          </div>

          <div v-else-if="filter === 'tracks'" class="main-scroll" ref="scrollContainer">
            <div v-if="ui.searchQuery" class="search-header">LOCAL RESULTS</div>
            <TrackList :tracks="paginatedTracks" :showArtist="true" :showAlbum="true" compact @play="playTrack" />
            
            <div v-if="filteredTracks.length > TRACKS_PER_PAGE" class="pagination">
              <button class="btn-page" :disabled="trackPage === 1" @click="trackPage--; scrollToTop()">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M15 18l-6-6 6-6"/></svg>
                PREV
              </button>
              <span class="page-info">PAGE {{ trackPage }} OF {{ Math.ceil(filteredTracks.length / TRACKS_PER_PAGE) }}</span>
              <button class="btn-page" :disabled="trackPage >= Math.ceil(filteredTracks.length / TRACKS_PER_PAGE)" @click="trackPage++; scrollToTop()">
                NEXT
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M9 18l6-6-6-6"/></svg>
              </button>
            </div>

            <div v-if="ui.searchQuery" class="search-header" style="margin-top:24px">SPOTIFY RESULTS</div>
            <div v-if="spotifySearching" class="empty" style="padding:10px">searching spotify...</div>
            <TrackList v-if="ui.searchQuery && spotifyResults.length > 0" :tracks="spotifyResults" :showArtist="true" :showAlbum="true" compact @play="playTrack" />
          </div>

          <!-- Favorites -->
          <div v-else-if="filter === 'favorites'" class="main-scroll">
            <div v-if="favoriteAlbums.length > 0">
              <h3 class="section-title">❤️ Liked Albums</h3>
              <div class="grid-albums">
                <div v-for="a in favoriteAlbums" :key="'fav-' + a.artist + a.album" @click="viewAlbum(a.artist, a.album)">
                  <AlbumCard :artist="a.artist" :album="a.album" :tracks="a.tracks" :year="a.year" :coverArt="a.coverArt" @playAll="playAlbum" />
                </div>
              </div>
              <h3 class="section-title" style="margin-top:24px">❤️ Liked Tracks</h3>
              <TrackList :tracks="favoriteTracks" :showArtist="true" :showAlbum="true" compact @play="playTrack" />
            </div>
            <div v-else class="empty">no favorites yet. heart a track to add it.</div>
          </div>

          <!-- Recently Played -->
          <div v-else-if="filter === 'recent'" class="main-scroll">
            <h3 class="section-title">🕐 Recently Played</h3>
            <TrackList v-if="recentTracks.length > 0" :tracks="recentTracks" :showArtist="true" :showAlbum="true" compact @play="playTrack" />
            <div v-else class="empty">no tracks played yet.</div>
          </div>

          <!-- Most Played -->
          <div v-else-if="filter === 'top'" class="main-scroll">
            <h3 class="section-title">🔥 Most Played</h3>
            <TrackList v-if="topTracks.length > 0" :tracks="topTracks" :showArtist="true" :showAlbum="true" compact @play="playTrack" />
            <div v-else class="empty">no tracks played yet.</div>
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
    </div>

    <MetadataEditor :track="editingTrack" :show="showEditor" @close="showEditor = false" @saved="onMetadataSaved" />
    <AddMusicForm v-if="showAddForm" @close="showAddForm = false" @created="onMusicCreated" />
  </div>
</template>

<style scoped>
.lib {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.empty-hero {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
  max-width: 400px;
}
.empty-hero h2 { font-size: 24px; font-weight: 800; color: var(--foreground); }
.empty-hero p { font-size: 14px; color: var(--text-muted); }
.empty-hero p strong { color: var(--main); }
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

.center-state {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  text-align: center;
  color: color-mix(in srgb, var(--foreground) 50%, transparent);
}
.spinner {
  width: 24px; height: 24px;
  color: var(--main);
  animation: spin 1s linear infinite;
  margin: 0 auto 12px;
}
@keyframes spin { 100% { transform: rotate(360deg); } }

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
.head-actions { display: flex; align-items: center; gap: 12px; }

.filters {
  display: flex;
  gap: 8px;
  padding-bottom: 16px;
  flex-wrap: wrap;
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
.filter-chip:hover { box-shadow: none; transform: translate(4px, 4px); }
.filter-chip.on { background: var(--main); color: var(--main-foreground); box-shadow: none; transform: translate(4px, 4px); }

.main-scroll { flex: 1; overflow-y: auto; padding: 0 24px 24px; }
.grid-albums { display: grid; grid-template-columns: repeat(auto-fill, minmax(170px, 1fr)); gap: 16px; }
.empty { text-align: center; padding: 40px; color: color-mix(in srgb, var(--foreground) 40%, transparent); }

.section-title {
  font-size: 14px;
  font-weight: 800;
  margin-bottom: 12px;
  color: var(--foreground);
}

.artist-row {
  padding: 12px 16px;
  border-bottom: 2px solid var(--border);
  cursor: pointer;
  font-weight: 600;
}
.artist-row:hover { background: var(--secondary-background); }

.detail { height: 100%; display: flex; flex-direction: column; }
.detail-hero { padding: 24px; border-bottom: 2px solid var(--border); }
.btn-back {
  display: inline-flex; align-items: center; gap: 8px;
  font-family: var(--font); font-size: 11px; font-weight: 700;
  background: transparent; border: none; color: var(--foreground);
  cursor: pointer; margin-bottom: 20px;
}
.btn-back svg { width: 16px; height: 16px; }
.btn-back:hover { color: var(--main); }

.detail-flex { display: flex; gap: 24px; align-items: flex-end; }
.detail-art {
  width: 180px; height: 180px;
  border: 2px solid var(--border); border-radius: var(--radius-base);
  box-shadow: var(--shadow); background: var(--secondary-background);
  overflow: hidden; flex-shrink: 0;
}
.detail-art-img { width: 100%; height: 100%; background-size: cover; background-position: center; }
.detail-art-empty { width: 100%; height: 100%; display: flex; align-items: center; justify-content: center; }
.detail-art-empty svg { width: 48px; height: 48px; color: color-mix(in srgb, var(--foreground) 30%, transparent); }

.detail-info { min-width: 0; }
.detail-type { font-size: 10px; font-weight: 700; color: color-mix(in srgb, var(--foreground) 40%, transparent); }
.detail-title { font-size: 32px; font-weight: 800; line-height: 1.1; margin: 8px 0; }
.detail-meta { font-size: 13px; color: color-mix(in srgb, var(--foreground) 60%, transparent); margin-bottom: 16px; }
.detail-artist { color: var(--foreground); font-weight: 700; }
.detail-play { padding: 10px 24px; font-size: 12px; }
.detail-play svg { width: 14px; height: 14px; }
.detail-tracks { flex: 1; overflow-y: auto; }

.detail-toolbar {
  display: flex; gap: 8px;
  padding: 12px 24px;
  border-bottom: 2px solid var(--border);
}
.btn-edit {
  display: inline-flex; align-items: center; gap: 6px;
  padding: 6px 14px;
  font-family: var(--font); font-size: 10px; font-weight: 700; text-transform: uppercase;
  background: var(--secondary-background);
  color: var(--foreground);
  border: 2px solid var(--border);
  border-radius: calc(var(--radius-base) - 2px);
  cursor: pointer;
  box-shadow: 2px 2px 0px 0px var(--border);
  transition: all 0.15s;
}
.btn-edit:hover { transform: translate(2px, 2px); box-shadow: none; }
.btn-edit svg { width: 14px; height: 14px; }
.btn-add-music {
  padding: 6px 14px;
  font-family: var(--font); font-size: 11px; font-weight: 700;
  background: var(--main);
  color: var(--main-foreground);
  border: 2px solid var(--main);
  border-radius: calc(var(--radius-base) - 2px);
  cursor: pointer;
  box-shadow: 2px 2px 0px 0px var(--border);
  transition: all 0.15s;
}
.btn-add-music:hover { transform: translate(2px, 2px); box-shadow: none; }
.add-btn { margin-top: 8px; }

.v-divider { width: 2px; background: var(--border); margin: 0 4px; }
.search-header { font-size: 14px; font-weight: 800; color: var(--main); margin-bottom: 12px; }

.pagination {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 16px;
  padding: 32px 0;
}
.btn-page {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 20px;
  font-family: var(--font); font-size: 11px; font-weight: 800; text-transform: uppercase;
  background: var(--secondary-background);
  color: var(--foreground);
  border: 2px solid var(--border);
  border-radius: var(--radius-base);
  cursor: pointer;
  transition: all 0.15s;
  box-shadow: 2px 2px 0px 0px var(--border);
}
.btn-page:hover:not(:disabled) { box-shadow: none; transform: translate(2px, 2px); }
.btn-page:disabled { opacity: 0.5; cursor: not-allowed; }
.btn-page svg { width: 14px; height: 14px; }
.page-info { text-align: center; font-size: 11px; font-weight: 700; color: color-mix(in srgb, var(--foreground) 50%, transparent); }

.playlist-card {
  background: var(--secondary-background);
  border: 2px solid var(--border);
  border-radius: var(--radius-base);
  padding: 12px;
  cursor: pointer;
  box-shadow: 2px 2px 0px 0px var(--border);
  transition: all 0.15s;
}
.playlist-card:hover { transform: translate(2px, 2px); box-shadow: none; }
.p-cover {
  width: 100%; aspect-ratio: 1;
  background-size: cover; background-position: center;
  border: 2px solid var(--border);
  border-radius: calc(var(--radius-base) - 2px);
  margin-bottom: 12px;
  background-color: var(--background);
}
.p-title { font-weight: 700; font-size: 13px; margin-bottom: 4px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
.p-owner { font-size: 11px; color: color-mix(in srgb, var(--foreground) 60%, transparent); }
</style>
