import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'
import { openUrl } from '@tauri-apps/plugin-opener'

export const useSpotifyStore = defineStore('spotify', {
  state: () => ({
    clientId: '',
    accessToken: '',
    refreshToken: '',
    deviceId: '',
    player: null as any,
    ready: false,
    isActive: false,
    playbackState: null as any,
  }),
  actions: {
    async init() {
      this.clientId = await invoke('get_setting', { key: 'spotify_client_id' }) || ''
      this.accessToken = await invoke('get_setting', { key: 'spotify_access_token' }) || ''
      this.refreshToken = await invoke('get_setting', { key: 'spotify_refresh_token' }) || ''
      if (this.accessToken) {
        this.initPlayer()
      }
    },
    async saveClientId(id: string) {
      this.clientId = id
      await invoke('set_setting', { key: 'spotify_client_id', value: id })
    },
    async authenticate() {
      if (!this.clientId) throw new Error('No client ID')
      
      const verifier = generateRandomString(64)
      const challenge = await generateCodeChallenge(verifier)
      
      const scope = encodeURIComponent('streaming user-read-email user-read-private user-modify-playback-state user-read-playback-state')
      const redirectUri = encodeURIComponent('http://localhost:1424/callback')
      const authUrl = `https://accounts.spotify.com/authorize?client_id=${this.clientId}&response_type=code&redirect_uri=${redirectUri}&code_challenge_method=S256&code_challenge=${challenge}&scope=${scope}`
      
      await openUrl(authUrl)
      
      const code = await invoke<string>('start_oauth_server')
      
      const body = new URLSearchParams({
        client_id: this.clientId,
        grant_type: 'authorization_code',
        code,
        redirect_uri: 'http://localhost:1424/callback',
        code_verifier: verifier,
      })
      
      const res = await fetch('https://accounts.spotify.com/api/token', {
        method: 'POST',
        headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
        body
      })
      
      const data = await res.json()
      if (data.access_token) {
        this.accessToken = data.access_token
        this.refreshToken = data.refresh_token
        await invoke('set_setting', { key: 'spotify_access_token', value: this.accessToken })
        await invoke('set_setting', { key: 'spotify_refresh_token', value: this.refreshToken })
        this.initPlayer()
      }
    },
    
    initPlayer() {
      const start = () => {
        const player = new (window as any).Spotify.Player({
          name: '// rifly',
          getOAuthToken: (cb: any) => { cb(this.accessToken) },
          volume: 0.5
        })
        
        player.addListener('ready', ({ device_id }: any) => {
          this.deviceId = device_id
          this.ready = true
        })
        
        player.addListener('not_ready', () => {
          this.ready = false
        })
        
        player.addListener('player_state_changed', (state: any) => {
          this.playbackState = state
        })
        
        player.connect()
        this.player = player
      }

      if ((window as any).Spotify) {
        start()
      } else {
        ;(window as any).onSpotifyWebPlaybackSDKReady = start
      }
    },
    
    async getPlaylists() {
       const res = await fetch(`https://api.spotify.com/v1/me/playlists?limit=50`, {
         headers: { 'Authorization': `Bearer ${this.accessToken}` }
       })
       return res.json()
    },
    async getPlaylistTracks(id: string) {
       const res = await fetch(`https://api.spotify.com/v1/playlists/${id}/tracks?limit=50`, {
         headers: { 'Authorization': `Bearer ${this.accessToken}` }
       })
       return res.json()
    },
    async getLikedSongs() {
       const res = await fetch(`https://api.spotify.com/v1/me/tracks?limit=50`, {
         headers: { 'Authorization': `Bearer ${this.accessToken}` }
       })
       return res.json()
    },
    async search(query: string) {
       const res = await fetch(`https://api.spotify.com/v1/search?q=${encodeURIComponent(query)}&type=track,album,artist&limit=20`, {
         headers: { 'Authorization': `Bearer ${this.accessToken}` }
       })
       return res.json()
    },
    
    async playUris(uris: string[], offsetIndex = 0) {
       await fetch(`https://api.spotify.com/v1/me/player/play?device_id=${this.deviceId}`, {
         method: 'PUT',
         headers: { 
           'Authorization': `Bearer ${this.accessToken}`,
           'Content-Type': 'application/json'
         },
         body: JSON.stringify({ 
           uris,
           offset: { position: offsetIndex }
         })
       })
    },

    async pause() {
       this.player?.pause()
    },
    async resume() {
       this.player?.resume()
    },
    async seek(pos_ms: number) {
       this.player?.seek(pos_ms)
    },
    async next() {
       this.player?.nextTrack()
    },
    async prev() {
       this.player?.previousTrack()
    },
    async setVolume(vol: number) {
       this.player?.setVolume(vol)
    }
  }
})

export function mapSpotifyTrack(s: any): any {
  return {
    id: s.id,
    path: s.uri,
    title: s.name,
    artist: s.artists.map((a: any) => a.name).join(', '),
    album: s.album.name,
    album_artist: s.artists[0]?.name,
    composers: [],
    genres: [],
    track_number: s.track_number,
    disc_number: s.disc_number,
    duration: s.duration_ms / 1000,
    sample_rate: 44100,
    bit_depth: 16,
    channels: 2,
    format: 'spotify',
    year: parseInt(s.album.release_date?.substring(0, 4)) || 0,
    has_artwork: true,
    coverArt: s.album.images[0]?.url,
    replaygain_track: 0,
    replaygain_album: 0,
  }
}

function generateRandomString(length: number) {
  const possible = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789'
  const values = crypto.getRandomValues(new Uint8Array(length))
  return values.reduce((acc, x) => acc + possible[x % possible.length], "")
}

async function generateCodeChallenge(codeVerifier: string) {
  const encoder = new TextEncoder()
  const data = encoder.encode(codeVerifier)
  const digest = await window.crypto.subtle.digest('SHA-256', data)
  return btoa(String.fromCharCode(...new Uint8Array(digest)))
    .replace(/=/g, '')
    .replace(/\+/g, '-')
    .replace(/\//g, '_')
}
