import { defineStore } from 'pinia'
import { ref, computed, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { useSpotifyStore } from './spotify'
import type { Track, PlaybackStatus, RepeatMode, AudioSource } from '../types'

export const usePlayerStore = defineStore('player', () => {
  const source = ref<AudioSource>('local')
  const currentTrack = ref<Track | null>(null)
  const status = ref<PlaybackStatus>('stopped')
  const position = ref(0)
  const duration = ref(0)
  const volume = ref(0.7)
  const shuffle = ref(false)
  const repeat = ref<RepeatMode>('off')
  const queue = ref<Track[]>([])
  const queueIndex = ref(-1)
  const coverArt = ref<string | null>(null)

  const isPlaying = computed(() => status.value === 'playing')
  let unlisten: (() => void) | null = null
  let posInterval: any = null

  const spotify = useSpotifyStore()

  async function init() {
    // Local player state sync
    unlisten = await listen<{ status: string; position: number; duration: number }>('player:state', (e) => {
      if (source.value !== 'local') return
      status.value = e.payload.status as PlaybackStatus
      position.value = e.payload.position
      duration.value = e.payload.duration
    })

    // Spotify player state sync
    watch(() => spotify.playbackState, (st: any) => {
      if (!st || source.value !== 'spotify') return
      
      const t = st.track_window.current_track
      if (t) {
        currentTrack.value = {
          id: 0, path: t.uri, title: t.name,
          artist: t.artists.map((a: any) => a.name).join(', '),
          album: t.album.name, album_artist: '', composers: [], genres: [],
          track_number: 1, disc_number: 1, duration: st.duration / 1000,
          sample_rate: 44100, bit_depth: 16, channels: 2, format: 'spotify',
          year: 0, has_artwork: true, replaygain_track: 0, replaygain_album: 0
        }
        coverArt.value = t.album.images[0]?.url || null
        duration.value = st.duration / 1000
      }
      
      status.value = st.paused ? 'paused' : 'playing'
      position.value = st.position / 1000
      shuffle.value = st.shuffle
      
      if (!st.paused) {
        if (!posInterval) {
          posInterval = setInterval(() => { position.value += 1 }, 1000)
        }
      } else {
        clearInterval(posInterval)
        posInterval = null
      }
    }, { deep: true })
  }

  function cleanup() {
    unlisten?.()
    clearInterval(posInterval)
  }

  function toggleSource() {
    // Internal auto-switch only now
  }

  async function loadCoverArt(path: string) {
    try {
      coverArt.value = await invoke<string | null>('get_cover_art', { path })
    } catch { coverArt.value = null }
  }

  async function play(track: Track) {
    if (track.format === 'spotify') {
      source.value = 'spotify'
      await spotify.playUris([track.path])
      return
    }

    source.value = 'local'
    try {
      await invoke('play_track', { path: track.path })
      currentTrack.value = track
      status.value = 'playing'
      position.value = 0
      duration.value = track.duration
      if (track.has_artwork) loadCoverArt(track.path)
      if (!queue.value.some(t => t.path === track.path)) {
        queue.value.push(track)
        queueIndex.value = queue.value.length - 1
      } else {
        queueIndex.value = queue.value.findIndex(t => t.path === track.path)
      }
    } catch (e) { console.error('Playback error:', e) }
  }

  async function playAll(tracks: Track[], startIndex = 0) {
    if (tracks.length === 0) return
    if (tracks[0].format === 'spotify') {
      source.value = 'spotify'
      const uris = tracks.map(t => t.path)
      await spotify.playUris(uris, startIndex)
      return
    }
    source.value = 'local'
    queue.value = [...tracks]
    queueIndex.value = startIndex
    play(tracks[startIndex])
  }

  async function playIndex(idx: number) {
    if (source.value === 'spotify') return // Spotify handles internal queue
    if (idx >= 0 && idx < queue.value.length) {
      await play(queue.value[idx])
      queueIndex.value = idx
    }
  }

  async function pause() {
    if (source.value === 'spotify') { await spotify.pause(); return }
    await invoke('pause_track')
    status.value = 'paused'
  }

  async function resume() {
    if (source.value === 'spotify') { await spotify.resume(); return }
    if (!currentTrack.value) return
    await invoke('resume_track')
    status.value = 'playing'
  }

  async function next() {
    if (source.value === 'spotify') { await spotify.next(); return }
    if (queue.value.length === 0) return
    let nextIdx = queueIndex.value + 1
    if (nextIdx >= queue.value.length) {
      if (repeat.value !== 'off') nextIdx = 0
      else return
    }
    await playIndex(nextIdx)
  }

  async function prev() {
    if (source.value === 'spotify') { await spotify.prev(); return }
    if (queue.value.length === 0) return
    if (position.value > 3) { position.value = 0; return }
    let prevIdx = queueIndex.value - 1
    if (prevIdx < 0) {
      if (repeat.value !== 'off') prevIdx = queue.value.length - 1
      else return
    }
    await playIndex(prevIdx)
  }

  async function toggleShuffle() { 
    shuffle.value = !shuffle.value 
  }
  
  function cycleRepeat() {
    const m: RepeatMode[] = ['off', 'all', 'one']
    repeat.value = m[(m.indexOf(repeat.value) + 1) % m.length]
  }

  function removeFromQueue(idx: number) {
    if (source.value === 'spotify') return
    if (idx < queueIndex.value) queueIndex.value--
    else if (idx === queueIndex.value) { queueIndex.value = -1; currentTrack.value = null }
    queue.value.splice(idx, 1)
  }

  function clearQueue() { queue.value = []; queueIndex.value = -1; currentTrack.value = null }
  
  async function seek(pos: number) { 
    if (source.value === 'spotify') { await spotify.seek(pos * 1000); return }
    position.value = pos 
  }
  
  async function setVolume(v: number) { 
    volume.value = Math.max(0, Math.min(1, v))
    if (source.value === 'spotify') { await spotify.setVolume(volume.value) }
  }

  return {
    source, currentTrack, status, position, duration, volume, shuffle, repeat,
    queue, queueIndex, coverArt, isPlaying,
    init, cleanup, toggleSource, play, playAll, playIndex, pause, resume, seek, next, prev,
    toggleShuffle, cycleRepeat, removeFromQueue, clearQueue, setVolume, loadCoverArt,
  }
})
