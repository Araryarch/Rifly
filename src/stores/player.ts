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
  let sessionTimer: any = null

  const spotify = useSpotifyStore()

  async function init() {
    unlisten = await listen<{ status: string; position: number; duration: number }>('player:state', (e) => {
      if (source.value !== 'local') return
      status.value = e.payload.status as PlaybackStatus
      position.value = e.payload.position
      duration.value = e.payload.duration
    })

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

    // Session autosave every 10s while playing
    sessionTimer = setInterval(() => { saveSession() }, 10000)
  }

  function cleanup() {
    unlisten?.()
    clearInterval(posInterval)
    clearInterval(sessionTimer)
  }

  function toggleSource() {}

  async function logPlay(path: string) {
    try { await invoke('log_play', { path }) } catch {}
  }

  async function saveSession() {
    if (!currentTrack.value) return
    try {
      await invoke('save_session', {
        trackPath: currentTrack.value.path,
        position: position.value,
        queue: queue.value.map(t => t.path),
        queueIndex: queueIndex.value,
      })
    } catch {}
  }

  async function loadSession(): Promise<boolean> {
    try {
      const session: any = await invoke('load_session')
      if (!session) return false
      return true
    } catch { return false }
  }

  function updateMediaSession() {
    if ('mediaSession' in navigator && currentTrack.value) {
      navigator.mediaSession.metadata = new MediaMetadata({
        title: currentTrack.value.title,
        artist: currentTrack.value.artist,
        album: currentTrack.value.album,
        artwork: coverArt.value ? [{ src: coverArt.value, sizes: '512x512', type: 'image/png' }] : [],
      })
      navigator.mediaSession.setActionHandler('play', () => resume())
      navigator.mediaSession.setActionHandler('pause', () => pause())
      navigator.mediaSession.setActionHandler('previoustrack', () => prev())
      navigator.mediaSession.setActionHandler('nexttrack', () => next())
      navigator.mediaSession.setActionHandler('stop', () => { stop(); saveSession() })
    }
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
      logPlay(track.path)
      updateMediaSession()
      saveSession()
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
    if (source.value === 'spotify') return
    if (idx >= 0 && idx < queue.value.length) {
      await play(queue.value[idx])
      queueIndex.value = idx
    }
  }

  async function playTrackList(tracks: Track[], startIdx = 0) {
    source.value = 'local'
    queue.value = [...tracks]
    queueIndex.value = startIdx
    play(tracks[startIdx])
  }

  async function pause() {
    if (source.value === 'spotify') { await spotify.pause(); return }
    await invoke('pause_track')
    status.value = 'paused'
    updateMediaSession()
  }

  async function resume() {
    if (source.value === 'spotify') { await spotify.resume(); return }
    if (!currentTrack.value) return
    await invoke('resume_track')
    status.value = 'playing'
    updateMediaSession()
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

  async function stop() {
    if (source.value === 'spotify') return
    await invoke('stop_track')
    status.value = 'stopped'
    position.value = 0
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

  /** Reorder queue: move item from `from` to `to` */
  function moveInQueue(from: number, to: number) {
    if (from < 0 || from >= queue.value.length || to < 0 || to >= queue.value.length) return
    const item = queue.value.splice(from, 1)[0]
    queue.value.splice(to, 0, item)
    if (queueIndex.value === from) queueIndex.value = to
    else if (from < queueIndex.value && to >= queueIndex.value) queueIndex.value--
    else if (from > queueIndex.value && to <= queueIndex.value) queueIndex.value++
  }

  return {
    source, currentTrack, status, position, duration, volume, shuffle, repeat,
    queue, queueIndex, coverArt, isPlaying,
    init, cleanup, toggleSource, play, playAll, playIndex, playTrackList,
    pause, resume, seek, next, prev, stop,
    toggleShuffle, cycleRepeat, removeFromQueue, clearQueue, setVolume, loadCoverArt,
    logPlay, saveSession, loadSession, updateMediaSession, moveInQueue,
  }
})
