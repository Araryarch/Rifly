<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useLibraryStore } from '../stores/library'
import { usePlayerStore } from '../stores/player'
import AlbumCard from '../components/AlbumCard.vue'
import TrackList from '../components/TrackList.vue'
import SearchBar from '../components/SearchBar.vue'
import type { LibraryFilter, AlbumGroup } from '../types'

const lib = useLibraryStore()
const player = usePlayerStore()

const selectedAlbum = ref<{ artist: string; album: string } | null>(null)
const showPathInput = ref(false)
const pathInput = ref('')
const searchQuery = ref('')
const filter = ref<LibraryFilter>('albums')

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

async function setFolder() {
  const p = pathInput.value.trim()
  if (!p) return
  lib.scanFolder(p)
  invoke('set_setting', { key: 'music_folder', value: p })
  showPathInput.value = false
}

function playTrack(track: any, idx: number) {
  const album = selectedAlbum.value
  if (album) {
    const tracks = lib.tracksForAlbum(album.artist, album.album)
    player.playAll(tracks, idx)
  } else {
    player.play(track)
  }
}

function playAlbum(tracks: any[]) {
  player.playAll(tracks)
}

function viewAlbum(artist: string, album: string) {
  selectedAlbum.value = { artist, album }
}

function onSearch(query: string) {
  searchQuery.value = query
  // Switch to tracks view for search results
  if (query) {
    filter.value = 'tracks'
    selectedAlbum.value = null
  }
}

const filteredAlbums = computed(() => {
  if (!searchQuery.value) return lib.albums
  const q = searchQuery.value.toLowerCase()
  return lib.albums.filter((a: AlbumGroup) =>
    a.album.toLowerCase().includes(q) ||
    a.artist.toLowerCase().includes(q)
  )
})

const filteredTracks = computed(() => {
  if (!searchQuery.value) return lib.tracks
  const q = searchQuery.value.toLowerCase()
  return lib.tracks.filter(t =>
    t.title.toLowerCase().includes(q) ||
    t.artist.toLowerCase().includes(q) ||
    t.album.toLowerCase().includes(q)
  )
})

const totalDuration = computed(() => {
  const total = lib.tracks.reduce((s, t) => s + t.duration, 0)
  const h = Math.floor(total / 3600)
  const m = Math.floor((total % 3600) / 60)
  return h > 0 ? `${h} hr ${m} min` : `${m} min`
})

const albumDetail = computed(() => {
  if (!selectedAlbum.value) return null
  return lib.albums.find(
    (a: AlbumGroup) => a.artist === selectedAlbum.value!.artist && a.album === selectedAlbum.value!.album
  )
})
</script>

<template>
  <div class="library">

    <!-- ─── Welcome / Setup ─── -->
    <div v-if="showPathInput" class="welcome-screen">
      <div class="welcome-card animate-fade-in-up">
        <div class="welcome-icon">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
          </svg>
        </div>
        <h1 class="welcome-title">Welcome to Rifly</h1>
        <p class="welcome-subtitle">
          Your premium audiophile music player.<br/>
          Point to a folder containing your music files.
        </p>
        <div class="welcome-input-row">
          <input
            v-model="pathInput"
            type="text"
            placeholder="C:\Users\You\Music"
            class="welcome-input"
            @keydown.enter="setFolder"
          />
          <button class="welcome-btn" @click="setFolder">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M5 12h14M12 5l7 7-7 7"/></svg>
            Set Folder
          </button>
        </div>
        <p class="welcome-hint">Supports FLAC, WAV, AIFF, MP3, AAC, OGG and more</p>
      </div>
    </div>

    <!-- ─── Scanning ─── -->
    <div v-else-if="lib.scanning" class="scanning-screen">
      <div class="scanning-content animate-fade-in">
        <div class="scanning-spinner">
          <svg viewBox="0 0 24 24" fill="none">
            <circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="3" stroke-dasharray="31.4 31.4" stroke-linecap="round"/>
          </svg>
        </div>
        <p class="scanning-text">Scanning your music library...</p>
        <p class="scanning-sub">This may take a moment for large collections</p>
      </div>
    </div>

    <!-- ─── Library Content ─── -->
    <template v-else-if="lib.tracks.length > 0">

      <!-- Album Detail View -->
      <div v-if="selectedAlbum" class="album-detail">
        <!-- Hero Section -->
        <div class="album-hero">
          <div class="album-hero-bg" />
          <div class="album-hero-content animate-fade-in-up">
            <button class="back-btn" @click="selectedAlbum = null">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M19 12H5m7-7l-7 7 7 7"/></svg>
            </button>
            <div class="album-hero-art">
              <div v-if="albumDetail?.coverArt" class="hero-art-img" :style="{ backgroundImage: `url(${albumDetail.coverArt})` }" />
              <div v-else class="hero-art-placeholder">
                <svg viewBox="0 0 24 24" fill="currentColor"><circle cx="7" cy="17" r="3"/><circle cx="17" cy="15" r="3"/><polyline points="10 17 10 5 20 3 20 15"/></svg>
              </div>
            </div>
            <div class="album-hero-info">
              <span class="album-type">Album</span>
              <h1 class="album-detail-title">{{ selectedAlbum.album }}</h1>
              <div class="album-detail-meta">
                <span class="album-detail-artist">{{ selectedAlbum.artist }}</span>
                <span v-if="albumDetail?.year" class="meta-dot">·</span>
                <span v-if="albumDetail?.year">{{ albumDetail.year }}</span>
                <span class="meta-dot">·</span>
                <span>{{ albumDetail?.tracks.length || 0 }} tracks</span>
              </div>
              <button class="album-play-btn" @click="albumDetail && playAlbum(albumDetail.tracks)">
                <svg viewBox="0 0 24 24" fill="currentColor"><path d="M8 5v14l11-7z"/></svg>
                Play
              </button>
            </div>
          </div>
        </div>
        <!-- Track list -->
        <div class="album-tracks">
          <TrackList
            :tracks="lib.tracksForAlbum(selectedAlbum.artist, selectedAlbum.album)"
            :showArtist="false"
            :showAlbum="false"
            @play="playTrack"
          />
        </div>
      </div>

      <!-- Main Library View -->
      <div v-else class="library-main">
        <!-- Header -->
        <div class="library-header">
          <div class="header-top">
            <div class="header-left">
              <h1 class="library-title">Your Library</h1>
              <div class="library-stats">
                <span>{{ lib.tracks.length }} tracks</span>
                <span class="meta-dot">·</span>
                <span>{{ lib.albums.length }} albums</span>
                <span class="meta-dot">·</span>
                <span>{{ totalDuration }}</span>
              </div>
            </div>
            <div class="header-right">
              <SearchBar @search="onSearch" />
              <button class="header-settings-btn" @click="showPathInput = true; pathInput = lib.musicFolder">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/></svg>
              </button>
            </div>
          </div>

          <!-- Filter chips -->
          <div class="filter-chips">
            <button :class="['filter-chip', { active: filter === 'albums' }]" @click="filter = 'albums'; searchQuery = ''">Albums</button>
            <button :class="['filter-chip', { active: filter === 'artists' }]" @click="filter = 'artists'; searchQuery = ''">Artists</button>
            <button :class="['filter-chip', { active: filter === 'tracks' }]" @click="filter = 'tracks'; searchQuery = ''">All Tracks</button>
          </div>
        </div>

        <!-- Album Grid -->
        <div v-if="filter === 'albums'" class="library-scroll">
          <div class="album-grid">
            <div
              v-for="(a, idx) in filteredAlbums"
              :key="a.artist + a.album"
              class="album-grid-item animate-fade-in-up"
              :style="{ animationDelay: Math.min(idx * 30, 300) + 'ms', animationFillMode: 'both' }"
              @click="viewAlbum(a.artist, a.album)"
            >
              <AlbumCard
                :artist="a.artist"
                :album="a.album"
                :tracks="a.tracks"
                :year="a.year"
                :coverArt="a.coverArt"
                @playAll="playAlbum"
              />
            </div>
          </div>
          <div v-if="filteredAlbums.length === 0 && searchQuery" class="no-results">
            <p>No albums found for "{{ searchQuery }}"</p>
          </div>
        </div>

        <!-- Artists View -->
        <div v-else-if="filter === 'artists'" class="library-scroll">
          <div class="artists-grid">
            <div v-for="artist in lib.artists" :key="artist" class="artist-card animate-fade-in-up">
              <div class="artist-avatar">
                <svg viewBox="0 0 24 24" fill="currentColor"><path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"/></svg>
              </div>
              <span class="artist-name">{{ artist }}</span>
              <span class="artist-label">Artist</span>
            </div>
          </div>
        </div>

        <!-- All Tracks View -->
        <div v-else-if="filter === 'tracks'" class="library-scroll">
          <TrackList
            :tracks="filteredTracks"
            :showArtist="true"
            :showAlbum="true"
            compact
            @play="(track, idx) => player.play(track)"
          />
        </div>
      </div>
    </template>

    <!-- ─── Empty Library ─── -->
    <div v-else class="empty-screen">
      <div class="empty-content animate-fade-in-up">
        <div class="empty-icon">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <circle cx="7" cy="17" r="3"/><circle cx="17" cy="15" r="3"/><polyline points="10 17 10 5 20 3 20 15"/>
          </svg>
        </div>
        <p class="empty-title">No music files found</p>
        <code class="empty-path">{{ lib.musicFolder }}</code>
        <button class="empty-btn" @click="showPathInput = true; pathInput = lib.musicFolder">
          Choose a different folder
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.library {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  background: var(--color-bg-primary);
}

/* ── Welcome Screen ── */
.welcome-screen {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 24px;
}

.welcome-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
  max-width: 460px;
  width: 100%;
  padding: 48px 40px;
  border-radius: 16px;
  background: var(--color-bg-tertiary);
  border: 1px solid var(--color-border);
}

.welcome-icon {
  width: 72px;
  height: 72px;
  border-radius: 50%;
  background: linear-gradient(135deg, var(--color-accent-dim), rgba(29, 185, 84, 0.05));
  display: flex;
  align-items: center;
  justify-content: center;
}

.welcome-icon svg {
  width: 36px;
  height: 36px;
  color: var(--color-accent);
}

.welcome-title {
  font-size: 26px;
  font-weight: 700;
  letter-spacing: -0.5px;
}

.welcome-subtitle {
  font-size: 14px;
  color: var(--color-text-secondary);
  text-align: center;
  line-height: 1.6;
}

.welcome-input-row {
  display: flex;
  gap: 8px;
  width: 100%;
}

.welcome-input {
  flex: 1;
  background: var(--color-bg-elevated);
  border: 1px solid var(--color-border);
  color: var(--color-text-primary);
  font-family: var(--font-family);
  font-size: 13px;
  padding: 10px 16px;
  border-radius: 8px;
  outline: none;
  transition: border-color 0.2s;
}

.welcome-input:focus {
  border-color: var(--color-accent);
}

.welcome-input::placeholder {
  color: var(--color-text-tertiary);
}

.welcome-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 10px 20px;
  background: var(--color-accent);
  color: black;
  font-family: var(--font-family);
  font-size: 13px;
  font-weight: 600;
  border: none;
  border-radius: 500px;
  cursor: pointer;
  white-space: nowrap;
  transition: all 0.15s;
}

.welcome-btn:hover {
  background: var(--color-accent-hover);
  transform: scale(1.02);
}

.welcome-btn svg {
  width: 16px;
  height: 16px;
}

.welcome-hint {
  font-size: 11px;
  color: var(--color-text-tertiary);
}

/* ── Scanning ── */
.scanning-screen {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
}

.scanning-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
}

.scanning-spinner svg {
  width: 32px;
  height: 32px;
  color: var(--color-accent);
  animation: vinylSpin 1s linear infinite;
}

.scanning-text {
  font-size: 15px;
  font-weight: 500;
  color: var(--color-text-primary);
}

.scanning-sub {
  font-size: 12px;
  color: var(--color-text-tertiary);
}

/* ── Library Main ── */
.library-main {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.library-header {
  padding: 24px 28px 0;
  flex-shrink: 0;
}

.header-top {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 24px;
  margin-bottom: 20px;
}

.header-left {
  min-width: 0;
}

.library-title {
  font-size: 28px;
  font-weight: 800;
  letter-spacing: -0.5px;
  margin-bottom: 4px;
}

.library-stats {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 13px;
  color: var(--color-text-secondary);
}

.meta-dot {
  color: var(--color-text-subdued);
}

.header-right {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-shrink: 0;
}

.header-settings-btn {
  width: 36px;
  height: 36px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  color: var(--color-text-secondary);
  cursor: pointer;
  border-radius: 50%;
  transition: all 0.15s;
}

.header-settings-btn svg {
  width: 18px;
  height: 18px;
}

.header-settings-btn:hover {
  color: var(--color-text-primary);
  background: var(--color-surface-glass-hover);
}

/* ── Filter Chips ── */
.filter-chips {
  display: flex;
  gap: 8px;
  padding-bottom: 16px;
}

.filter-chip {
  padding: 6px 16px;
  font-family: var(--font-family);
  font-size: 13px;
  font-weight: 500;
  border: none;
  border-radius: 500px;
  cursor: pointer;
  transition: all 0.15s;
  background: var(--color-bg-elevated);
  color: var(--color-text-primary);
}

.filter-chip:hover {
  background: var(--color-bg-hover);
}

.filter-chip.active {
  background: var(--color-text-primary);
  color: var(--color-bg-primary);
}

/* ── Scroll Area ── */
.library-scroll {
  flex: 1;
  overflow-y: auto;
  padding: 0 28px 28px;
}

/* ── Album Grid ── */
.album-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
  gap: 20px;
}

.album-grid-item {
  min-width: 0;
}

.no-results {
  text-align: center;
  padding: 60px 0;
  color: var(--color-text-tertiary);
  font-size: 14px;
}

/* ── Artists Grid ── */
.artists-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
  gap: 20px;
}

.artist-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 20px 12px;
  border-radius: 8px;
  background: var(--color-bg-card);
  cursor: pointer;
  transition: all 0.25s;
}

.artist-card:hover {
  background: var(--color-bg-elevated);
  transform: translateY(-2px);
}

.artist-avatar {
  width: 80px;
  height: 80px;
  border-radius: 50%;
  background: var(--color-bg-elevated);
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 4px;
}

.artist-avatar svg {
  width: 36px;
  height: 36px;
  color: var(--color-text-tertiary);
}

.artist-name {
  font-size: 14px;
  font-weight: 600;
  color: var(--color-text-primary);
  text-align: center;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 100%;
}

.artist-label {
  font-size: 11px;
  color: var(--color-text-tertiary);
}

/* ── Album Detail ── */
.album-detail {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.album-hero {
  position: relative;
  padding: 28px 32px 24px;
  flex-shrink: 0;
}

.album-hero-bg {
  position: absolute;
  inset: 0;
  background: linear-gradient(180deg, rgba(29, 185, 84, 0.12) 0%, var(--color-bg-primary) 100%);
  pointer-events: none;
}

.album-hero-content {
  position: relative;
  display: flex;
  align-items: flex-end;
  gap: 24px;
  z-index: 1;
}

.back-btn {
  position: absolute;
  top: 0;
  left: 0;
  background: rgba(255, 255, 255, 0.06);
  border: none;
  color: var(--color-text-primary);
  cursor: pointer;
  padding: 8px;
  border-radius: 50%;
  transition: all 0.15s;
}

.back-btn:hover {
  background: rgba(255, 255, 255, 0.12);
}

.back-btn svg {
  width: 18px;
  height: 18px;
}

.album-hero-art {
  width: 192px;
  height: 192px;
  border-radius: 8px;
  overflow: hidden;
  flex-shrink: 0;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
}

.hero-art-img {
  width: 100%;
  height: 100%;
  background-size: cover;
  background-position: center;
}

.hero-art-placeholder {
  width: 100%;
  height: 100%;
  background: linear-gradient(135deg, var(--color-bg-tertiary), var(--color-bg-elevated));
  display: flex;
  align-items: center;
  justify-content: center;
}

.hero-art-placeholder svg {
  width: 56px;
  height: 56px;
  color: var(--color-text-tertiary);
  opacity: 0.35;
}

.album-hero-info {
  min-width: 0;
  padding-bottom: 4px;
}

.album-type {
  font-size: 12px;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: var(--color-text-primary);
}

.album-detail-title {
  font-size: 42px;
  font-weight: 800;
  letter-spacing: -1px;
  line-height: 1.15;
  margin: 8px 0 12px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.album-detail-meta {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 13px;
  color: var(--color-text-secondary);
  margin-bottom: 16px;
}

.album-detail-artist {
  font-weight: 600;
  color: var(--color-text-primary);
}

.album-play-btn {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 12px 28px;
  background: var(--color-accent);
  color: black;
  font-family: var(--font-family);
  font-size: 14px;
  font-weight: 700;
  border: none;
  border-radius: 500px;
  cursor: pointer;
  transition: all 0.15s;
}

.album-play-btn:hover {
  background: var(--color-accent-hover);
  transform: scale(1.04);
}

.album-play-btn svg {
  width: 18px;
  height: 18px;
}

.album-tracks {
  flex: 1;
  overflow-y: auto;
  padding: 0 8px;
}

/* ── Empty State ── */
.empty-screen {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 24px;
}

.empty-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
}

.empty-icon {
  width: 80px;
  height: 80px;
  border-radius: 50%;
  background: var(--color-bg-tertiary);
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 8px;
}

.empty-icon svg {
  width: 36px;
  height: 36px;
  color: var(--color-text-tertiary);
  opacity: 0.5;
}

.empty-title {
  font-size: 16px;
  font-weight: 500;
  color: var(--color-text-secondary);
}

.empty-path {
  font-size: 12px;
  background: var(--color-bg-tertiary);
  color: var(--color-text-tertiary);
  padding: 4px 12px;
  border-radius: 6px;
}

.empty-btn {
  font-size: 13px;
  color: var(--color-accent);
  background: transparent;
  border: none;
  cursor: pointer;
  font-family: var(--font-family);
  transition: color 0.15s;
}

.empty-btn:hover {
  color: var(--color-accent-hover);
}
</style>
