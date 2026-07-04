import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { Track, AlbumGroup, PlayRecord } from '../types'

export const useLibraryStore = defineStore('library', () => {
  const tracks = ref<Track[]>([])
  const musicFolder = ref('')
  const scanning = ref(false)
  const coverArtCache = ref<Map<string, string | null>>(new Map())
  const recentlyPlayed = ref<PlayRecord[]>([])
  const mostPlayed = ref<PlayRecord[]>([])

  const albums = computed<AlbumGroup[]>(() => {
    const map = new Map<string, Track[]>()
    for (const t of tracks.value) {
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

  const artists = computed(() => {
    const s = new Set(tracks.value.map(t => t.artist).filter(Boolean))
    return Array.from(s).sort()
  })

  async function loadCoverArt(trackPath: string): Promise<string | null> {
    if (coverArtCache.value.has(trackPath)) return coverArtCache.value.get(trackPath) || null
    try {
      const b64 = await invoke<string | null>('get_cover_art', { path: trackPath })
      coverArtCache.value.set(trackPath, b64)
      return b64
    } catch { return null }
  }

  async function loadCoverArtForAlbum(album: AlbumGroup) {
    if (!album.coverArt && album.tracks.length > 0) {
      album.coverArt = await loadCoverArt(album.tracks[0].path)
    }
  }

  async function scanFolder(dir: string) {
    scanning.value = true
    musicFolder.value = dir
    try {
      tracks.value = await invoke('scan_folder', { dir })
    } catch (e) {
      console.error('Scan error:', e)
    } finally {
      scanning.value = false
    }
  }

  function tracksForAlbum(artist: string, album: string): Track[] {
    return tracks.value
      .filter(t => (t.album_artist || t.artist) === artist && t.album === album)
      .sort((a, b) => a.disc_number - b.disc_number || a.track_number - b.track_number)
  }

  function tracksForArtist(artistName: string): Track[] {
    return tracks.value
      .filter(t => t.artist === artistName || t.album_artist === artistName)
      .sort((a, b) => a.album.localeCompare(b.album) || a.disc_number - b.disc_number || a.track_number - b.track_number)
  }

  /** Returns recently played tracks (finds matching Track from loaded tracks) */
  async function loadRecentlyPlayed() {
    try {
      recentlyPlayed.value = await invoke('get_recently_played', { limit: 50 })
    } catch { recentlyPlayed.value = [] }
  }

  async function loadMostPlayed() {
    try {
      mostPlayed.value = await invoke('get_most_played', { limit: 50 })
    } catch { mostPlayed.value = [] }
  }

  function trackByPath(path: string): Track | undefined {
    return tracks.value.find(t => t.path === path)
  }

  /** Resolve PlayRecord paths to actual Track objects that exist in library */
  function resolveRecords(records: PlayRecord[]): Track[] {
    return records
      .map(r => trackByPath(r.path))
      .filter((t): t is Track => t !== undefined)
  }

  function replaceTrack(updated: Track) {
    const idx = tracks.value.findIndex(t => t.path === updated.path)
    if (idx !== -1) {
      tracks.value[idx] = updated
    }
  }

  function addTrack(t: Track) {
    if (!tracks.value.find(t2 => t2.path === t.path)) {
      tracks.value.push(t)
    }
  }

  return {
    tracks, musicFolder, scanning, coverArtCache, albums, artists,
    recentlyPlayed, mostPlayed,
    scanFolder, loadCoverArt, loadCoverArtForAlbum,
    tracksForAlbum, tracksForArtist,
    loadRecentlyPlayed, loadMostPlayed, trackByPath, resolveRecords,
    replaceTrack, addTrack,
  }
})
