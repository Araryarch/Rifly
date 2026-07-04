export interface Track {
  id: number
  path: string
  title: string
  artist: string
  album: string
  album_artist: string
  composers: string[]
  genres: string[]
  track_number: number
  disc_number: number
  duration: number
  sample_rate: number
  bit_depth: number
  channels: number
  format: string
  year: number
  has_artwork: boolean
  replaygain_track: number
  replaygain_album: number
}

export interface DeviceInfo {
  name: string
  api: string
  sample_rate: number
  is_default: boolean
}

export interface PlayRecord {
  path: string
  timestamp: number
  play_count: number
}

export interface SessionData {
  track_path: string
  position: number
  queue: string[]
  queue_index: number
}

export type PlaybackStatus = 'stopped' | 'playing' | 'paused' | 'loading'
export type RepeatMode = 'off' | 'one' | 'all'
export type ViewName = 'library' | 'your-library' | 'liked-songs' | 'now-playing' | 'queue' | 'search' | 'settings'
export type AudioSource = 'local' | 'spotify'
export type LibraryFilter = 'albums' | 'artists' | 'tracks'

export interface AlbumGroup {
  artist: string
  album: string
  tracks: Track[]
  year: number
  coverArt: string | null
}

export const isHiRes = (t: Track) => t.sample_rate > 48000 || t.bit_depth > 16
export const hiResBadge = (t: Track): string | null => {
  if (t.sample_rate > 48000) return `${t.sample_rate / 1000}kHz`
  if (t.bit_depth > 16) return `${t.bit_depth}bit`
  return null
}
export const formatTime = (sec: number): string => {
  const m = Math.floor(sec / 60)
  const s = Math.floor(sec % 60)
  return `${m}:${s.toString().padStart(2, '0')}`
}
export const formatBitrate = (track: Track): string => {
  return `${track.sample_rate / 1000}kHz / ${track.bit_depth}bit`
}
