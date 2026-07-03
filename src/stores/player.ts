import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import type { Track, PlaybackStatus, RepeatMode } from '../types'

export const usePlayerStore = defineStore('player', () => {
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

  async function init() {
    unlisten = await listen<{ status: string; position: number; duration: number }>('player:state', (e) => {
      status.value = e.payload.status as PlaybackStatus
      position.value = e.payload.position
      duration.value = e.payload.duration
    })
  }

  function cleanup() {
    unlisten?.()
  }

  async function loadCoverArt(path: string) {
    try {
      coverArt.value = await invoke<string | null>('get_cover_art', { path })
    } catch {
      coverArt.value = null
    }
  }

  async function play(track: Track) {
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
    } catch (e) {
      console.error('Playback error:', e)
    }
  }

  function playAll(tracks: Track[], startIndex = 0) {
    queue.value = [...tracks]
    queueIndex.value = startIndex
    play(tracks[startIndex])
  }

  async function playIndex(idx: number) {
    if (idx >= 0 && idx < queue.value.length) {
      await play(queue.value[idx])
      queueIndex.value = idx
    }
  }

  async function pause() {
    await invoke('pause_track')
    status.value = 'paused'
  }

  async function resume() {
    if (!currentTrack.value) return
    await invoke('resume_track')
    status.value = 'playing'
  }

  async function next() {
    if (queue.value.length === 0) return
    let nextIdx = queueIndex.value + 1
    if (nextIdx >= queue.value.length) {
      if (repeat.value !== 'off') nextIdx = 0
      else return
    }
    await playIndex(nextIdx)
  }

  async function prev() {
    if (queue.value.length === 0) return
    if (position.value > 3) { position.value = 0; return }
    let prevIdx = queueIndex.value - 1
    if (prevIdx < 0) {
      if (repeat.value !== 'off') prevIdx = queue.value.length - 1
      else return
    }
    await playIndex(prevIdx)
  }

  function toggleShuffle() { shuffle.value = !shuffle.value }
  function cycleRepeat() {
    const m: RepeatMode[] = ['off', 'all', 'one']
    repeat.value = m[(m.indexOf(repeat.value) + 1) % m.length]
  }
  function removeFromQueue(idx: number) {
    if (idx < queueIndex.value) queueIndex.value--
    else if (idx === queueIndex.value) { queueIndex.value = -1; currentTrack.value = null }
    queue.value.splice(idx, 1)
  }
  function clearQueue() { queue.value = []; queueIndex.value = -1; currentTrack.value = null }
  function seek(pos: number) { position.value = pos }
  function setVolume(v: number) { volume.value = Math.max(0, Math.min(1, v)) }

  return {
    currentTrack, status, position, duration, volume, shuffle, repeat,
    queue, queueIndex, coverArt, isPlaying,
    init, cleanup, play, playAll, playIndex, pause, resume, seek, next, prev,
    toggleShuffle, cycleRepeat, removeFromQueue, clearQueue, setVolume, loadCoverArt,
  }
})
