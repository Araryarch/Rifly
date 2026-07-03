<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { useSpotifyStore } from '../stores/spotify'
import { useLibraryStore } from '../stores/library'

const spotify = useSpotifyStore()
const lib = useLibraryStore()
const clientIdInput = ref('')
const pathInput = ref('')
const loading = ref(false)
const error = ref('')

// Discord Rich Presence settings
const discordEnabled = ref(true)
const discordHideAll = ref(false)
const discordShowTitle = ref(true)
const discordShowArtist = ref(true)
const discordShowAlbum = ref(true)
const discordShowCoverArt = ref(true)
const discordShowQuality = ref(true)
const discordShowProgress = ref(true)
const discordAudiophile = ref(false)
const discordClientId = ref('')
const discordBtn1Label = ref('')
const discordBtn1Url = ref('')
const discordBtn2Label = ref('')
const discordBtn2Url = ref('')

const themeMode = ref('dark')

async function loadTheme() {
  try {
    const saved = await invoke<string | null>('get_setting', { key: 'theme' })
    themeMode.value = saved || 'dark'
  } catch { themeMode.value = 'dark' }
}

function applyTheme(mode: string) {
  if (mode === 'light') {
    document.documentElement.style.setProperty('--background', '#f5f5f5')
    document.documentElement.style.setProperty('--panel-bg', '#ffffff')
    document.documentElement.style.setProperty('--panel-bg-hover', '#eeeeee')
    document.documentElement.style.setProperty('--foreground', '#000000')
    document.documentElement.style.setProperty('--text-muted', '#666666')
    document.documentElement.style.setProperty('--border', '#dddddd')
    document.documentElement.style.setProperty('--ring', '#000000')
    document.documentElement.style.setProperty('--overlay', 'rgba(255,255,255,0.8)')
  } else {
    document.documentElement.style.setProperty('--background', '#000000')
    document.documentElement.style.setProperty('--panel-bg', '#121212')
    document.documentElement.style.setProperty('--panel-bg-hover', '#1f1f1f')
    document.documentElement.style.setProperty('--foreground', '#ffffff')
    document.documentElement.style.setProperty('--text-muted', '#b3b3b3')
    document.documentElement.style.setProperty('--border', 'transparent')
    document.documentElement.style.setProperty('--ring', '#ffffff')
    document.documentElement.style.setProperty('--overlay', 'rgba(0,0,0,0.8)')
  }
}

watch(themeMode, (v) => {
  applyTheme(v)
  invoke('set_setting', { key: 'theme', value: v })
})

const dcKeys = {
  enabled: 'discord_enabled',
  hideAll: 'discord_hide_everything',
  showTitle: 'discord_show_title',
  showArtist: 'discord_show_artist',
  showAlbum: 'discord_show_album',
  showCoverArt: 'discord_show_cover_art',
  showQuality: 'discord_show_audio_quality',
  showProgress: 'discord_show_playback_progress',
  audiophile: 'discord_audiophile_mode',
  clientId: 'discord_client_id',
  btn1Label: 'discord_button1_label',
  btn1Url: 'discord_button1_url',
  btn2Label: 'discord_button2_label',
  btn2Url: 'discord_button2_url',
}

async function loadDiscordSettings() {
  const get = async (k: string) => { try { return await invoke<string | null>('get_setting', { key: k }) } catch { return null } }
  discordEnabled.value = (await get(dcKeys.enabled)) !== 'false'
  discordHideAll.value = (await get(dcKeys.hideAll)) === 'true'
  discordShowTitle.value = (await get(dcKeys.showTitle)) !== 'false'
  discordShowArtist.value = (await get(dcKeys.showArtist)) !== 'false'
  discordShowAlbum.value = (await get(dcKeys.showAlbum)) !== 'false'
  discordShowCoverArt.value = (await get(dcKeys.showCoverArt)) !== 'false'
  discordShowQuality.value = (await get(dcKeys.showQuality)) !== 'false'
  discordShowProgress.value = (await get(dcKeys.showProgress)) !== 'false'
  discordAudiophile.value = (await get(dcKeys.audiophile)) === 'true'
  discordClientId.value = (await get(dcKeys.clientId)) || ''
  discordBtn1Label.value = (await get(dcKeys.btn1Label)) || ''
  discordBtn1Url.value = (await get(dcKeys.btn1Url)) || ''
  discordBtn2Label.value = (await get(dcKeys.btn2Label)) || ''
  discordBtn2Url.value = (await get(dcKeys.btn2Url)) || ''
}

function saveDiscordSetting(key: string, value: string) {
  invoke('set_setting', { key, value })
}

watch(discordEnabled, v => saveDiscordSetting(dcKeys.enabled, String(v)))
watch(discordHideAll, v => saveDiscordSetting(dcKeys.hideAll, String(v)))
watch(discordShowTitle, v => saveDiscordSetting(dcKeys.showTitle, String(v)))
watch(discordShowArtist, v => saveDiscordSetting(dcKeys.showArtist, String(v)))
watch(discordShowAlbum, v => saveDiscordSetting(dcKeys.showAlbum, String(v)))
watch(discordShowCoverArt, v => saveDiscordSetting(dcKeys.showCoverArt, String(v)))
watch(discordShowQuality, v => saveDiscordSetting(dcKeys.showQuality, String(v)))
watch(discordShowProgress, v => saveDiscordSetting(dcKeys.showProgress, String(v)))
watch(discordAudiophile, v => saveDiscordSetting(dcKeys.audiophile, String(v)))
watch(discordClientId, v => saveDiscordSetting(dcKeys.clientId, v))
watch(discordBtn1Label, v => saveDiscordSetting(dcKeys.btn1Label, v))
watch(discordBtn1Url, v => saveDiscordSetting(dcKeys.btn1Url, v))
watch(discordBtn2Label, v => saveDiscordSetting(dcKeys.btn2Label, v))
watch(discordBtn2Url, v => saveDiscordSetting(dcKeys.btn2Url, v))

onMounted(async () => {
  await spotify.init()
  clientIdInput.value = spotify.clientId
  pathInput.value = lib.musicFolder
  await loadDiscordSettings()
  await loadTheme()
})

async function pickFolder() {
  const selected = await open({
    directory: true,
    title: 'Select Music Folder',
  })
  if (selected && typeof selected === 'string') {
    lib.scanFolder(selected)
    invoke('set_setting', { key: 'music_folder', value: selected })
    pathInput.value = selected
  }
}

async function setFolderManual() {
  const p = pathInput.value.trim()
  if (!p) return
  lib.scanFolder(p)
  invoke('set_setting', { key: 'music_folder', value: p })
}

async function connect() {
  if (!clientIdInput.value.trim()) {
    error.value = 'Client ID is required'
    return
  }
  
  error.value = ''
  loading.value = true
  try {
    await spotify.saveClientId(clientIdInput.value.trim())
    await spotify.authenticate()
  } catch (err: any) {
    error.value = err.message || 'Authentication failed'
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="settings">
    <div class="s-header">
      <h1 class="s-title">settings</h1>
    </div>

    <div class="s-content">
      <div class="card animate-fade-in" style="margin-bottom: 24px">
        <h2 class="card-title">Local Library</h2>
        <p class="card-desc">
          Set the folder where your local music files (FLAC, MP3, WAV, etc.) are stored.
        </p>

        <div v-if="lib.musicFolder" class="connected-badge" style="margin-bottom: 16px; border-color: var(--border); background: transparent; color: var(--foreground); font-weight: normal; font-size: 11px;">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="color: var(--text-muted)"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/></svg>
          {{ lib.musicFolder }}
        </div>
        
        <button class="btn-primary" @click="pickFolder" style="margin-bottom: 12px">
          BROWSE FOLDER
        </button>

        <div class="form-group" style="margin-bottom: 0;">
          <label>OR TYPE PATH MANUALLY</label>
          <div style="display: flex; gap: 8px">
            <input 
              v-model="pathInput" 
              type="text" 
              class="input" 
              placeholder="C:\Music" 
              @keydown.enter="setFolderManual"
            />
            <button class="btn-primary" @click="setFolderManual" style="width: auto; padding: 0 24px;">SET</button>
          </div>
        </div>
      </div>

      <div class="card animate-fade-in">
        <h2 class="card-title">Spotify Integration</h2>
        <p class="card-desc">
          Connect your Spotify Premium account to search and stream directly inside Rifly.
          You'll need a Developer Client ID with the redirect URI set to <code>http://localhost:1424/callback</code>.
        </p>

        <div class="form-group">
          <label>CLIENT ID</label>
          <input 
            v-model="clientIdInput" 
            type="text" 
            class="input" 
            placeholder="Paste your Spotify Client ID here" 
          />
        </div>

        <button 
          v-if="!spotify.accessToken" 
          class="btn-primary" 
          :disabled="loading"
          @click="connect"
        >
          {{ loading ? 'CONNECTING...' : 'CONNECT SPOTIFY' }}
        </button>

        <div v-else class="connected-badge">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M20 6L9 17l-5-5"/></svg>
          SPOTIFY CONNECTED
        </div>

        <p v-if="error" class="error">{{ error }}</p>

        <div v-if="spotify.accessToken" class="status-box">
          <div class="status-row">
            <span>DEVICE SDK</span>
            <span :class="['status-dot', { on: spotify.ready }]" />
          </div>
          <div class="status-desc">
            {{ spotify.ready ? 'Device "Rifly" is ready.' : 'Initializing virtual device...' }}
          </div>
        </div>
      </div>

      <div class="card animate-fade-in" style="margin-top: 24px">
        <h2 class="card-title">Appearance</h2>
        <p class="card-desc">
          Switch between dark and light mode.
        </p>
        <div class="form-group" style="margin-bottom: 0;">
          <label>THEME</label>
          <div style="display: flex; gap: 8px">
            <button :class="['btn-half', { on: themeMode === 'dark' }]" @click="themeMode = 'dark'">DARK</button>
            <button :class="['btn-half', { on: themeMode === 'light' }]" @click="themeMode = 'light'">LIGHT</button>
          </div>
        </div>
      </div>

      <div class="card animate-fade-in" style="margin-top: 24px">
        <h2 class="card-title">Discord Rich Presence</h2>
        <p class="card-desc">
          Show what you're listening to on your Discord profile.
          Requires a <a href="https://discord.com/developers/applications" target="_blank" style="color: var(--main)">Discord Application</a>
          with a Client ID and uploaded art assets (large image key: <code>rifly_logo</code>).
        </p>

        <div class="form-group">
          <label>CLIENT ID</label>
          <input v-model="discordClientId" type="text" class="input" placeholder="Paste your Discord App Client ID" />
        </div>

        <div class="toggle-group">
          <label class="toggle-row">
            <span>Enable Discord RPC</span>
            <input type="checkbox" v-model="discordEnabled" class="toggle" />
          </label>
          <label class="toggle-row">
            <span>Hide Everything (overrides all below)</span>
            <input type="checkbox" v-model="discordHideAll" class="toggle" />
          </label>
        </div>

        <div class="toggle-group" v-show="discordEnabled && !discordHideAll">
          <div class="sub-label">DISPLAY OPTIONS</div>
          <label class="toggle-row">
            <span>Show Song Title</span>
            <input type="checkbox" v-model="discordShowTitle" class="toggle" />
          </label>
          <label class="toggle-row">
            <span>Show Artist</span>
            <input type="checkbox" v-model="discordShowArtist" class="toggle" />
          </label>
          <label class="toggle-row">
            <span>Show Album</span>
            <input type="checkbox" v-model="discordShowAlbum" class="toggle" />
          </label>
          <label class="toggle-row">
            <span>Show Cover Art</span>
            <input type="checkbox" v-model="discordShowCoverArt" class="toggle" />
          </label>
          <label class="toggle-row">
            <span>Show Audio Quality</span>
            <input type="checkbox" v-model="discordShowQuality" class="toggle" />
          </label>
          <label class="toggle-row">
            <span>Show Playback Progress</span>
            <input type="checkbox" v-model="discordShowProgress" class="toggle" />
          </label>
          <label class="toggle-row">
            <span>Audiophile Mode</span>
            <input type="checkbox" v-model="discordAudiophile" class="toggle" />
          </label>
        </div>

        <div class="form-group" v-show="discordEnabled && !discordHideAll" style="margin-top: 16px; padding-top: 16px; border-top: 2px dashed var(--border)">
          <div class="sub-label">BUTTON 1</div>
          <input v-model="discordBtn1Label" type="text" class="input" placeholder="Label (e.g. Open Album)" style="margin-bottom: 8px" />
          <input v-model="discordBtn1Url" type="text" class="input" placeholder="URL (e.g. https://..." />
        </div>

        <div class="form-group" v-show="discordEnabled && !discordHideAll" style="margin-top: 16px">
          <div class="sub-label">BUTTON 2</div>
          <input v-model="discordBtn2Label" type="text" class="input" placeholder="Label" style="margin-bottom: 8px" />
          <input v-model="discordBtn2Url" type="text" class="input" placeholder="URL" />
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.settings {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.s-header {
  padding: 24px 24px 16px;
  border-bottom: 2px solid var(--border);
}
.s-title {
  font-size: 28px;
  font-weight: 800;
  text-transform: lowercase;
}

.s-content {
  flex: 1;
  overflow-y: auto;
  padding: 24px;
}

.card {
  background: var(--secondary-background);
  border: 2px solid var(--border);
  border-radius: var(--radius-base);
  box-shadow: var(--shadow);
  padding: 24px;
  max-width: 500px;
}

.card-title {
  font-size: 18px;
  font-weight: 800;
  margin-bottom: 8px;
  color: var(--main);
}
.card-desc {
  font-size: 13px;
  color: color-mix(in srgb, var(--foreground) 70%, transparent);
  margin-bottom: 24px;
}
.card-desc code {
  background: var(--background);
  border: 1px solid var(--border);
  border-radius: 4px;
  padding: 2px 4px;
  font-family: var(--font);
  font-weight: 700;
  color: var(--main);
}

.form-group {
  margin-bottom: 20px;
}
.form-group label {
  display: block;
  font-size: 11px;
  font-weight: 700;
  margin-bottom: 6px;
  color: color-mix(in srgb, var(--foreground) 50%, transparent);
}
.input {
  width: 100%;
  background: var(--background);
  border: 2px solid var(--border);
  border-radius: var(--radius-base);
  padding: 10px 12px;
  color: var(--foreground);
  font-family: var(--font);
  font-size: 13px;
}
.input:focus {
  background: color-mix(in srgb, var(--main) 10%, var(--background));
  outline: 2px solid var(--ring);
  outline-offset: 2px;
}

.btn-primary {
  width: 100%;
  background: var(--main);
  color: var(--main-foreground);
  border: 2px solid var(--border);
  border-radius: var(--radius-base);
  padding: 12px;
  font-family: var(--font);
  font-size: 13px;
  font-weight: 800;
  cursor: pointer;
  box-shadow: var(--shadow);
  transition: all 0.15s;
}
.btn-primary:hover {
  box-shadow: none;
  transform: translate(4px, 4px);
}
.btn-primary:disabled {
  opacity: 0.7;
  pointer-events: none;
}

.connected-badge {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 13px;
  font-weight: 800;
  color: var(--main);
  background: color-mix(in srgb, var(--main) 10%, transparent);
  border: 2px solid var(--main);
  border-radius: var(--radius-base);
  padding: 12px;
  justify-content: center;
}
.connected-badge svg {
  width: 18px;
  height: 18px;
}

.error {
  margin-top: 12px;
  font-size: 11px;
  font-weight: 700;
  color: #ff4444;
}

.btn-half {
  flex: 1;
  background: var(--secondary-background);
  color: var(--foreground);
  border: 2px solid var(--border);
  border-radius: var(--radius-base);
  padding: 10px;
  font-family: var(--font);
  font-size: 12px;
  font-weight: 800;
  cursor: pointer;
  transition: all 0.15s;
}
.btn-half:hover { box-shadow: none; transform: translate(2px, 2px); }
.btn-half.on { background: var(--main); color: var(--main-foreground); box-shadow: none; transform: translate(2px, 2px); }

.status-box {
  margin-top: 24px;
  padding-top: 16px;
  border-top: 2px dashed var(--border);
}
.status-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  font-size: 11px;
  font-weight: 700;
  margin-bottom: 4px;
}
.status-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background: var(--background);
  border: 2px solid var(--border);
}
.status-dot.on {
  background: var(--main);
}
.status-desc {
  font-size: 11px;
  color: color-mix(in srgb, var(--foreground) 50%, transparent);
}

.toggle-group {
  display: flex;
  flex-direction: column;
  gap: 4px;
}
.sub-label {
  font-size: 11px;
  font-weight: 700;
  color: color-mix(in srgb, var(--foreground) 50%, transparent);
  margin: 8px 0 4px;
}
.toggle-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  background: var(--background);
  border: 2px solid var(--border);
  border-radius: var(--radius-base);
  padding: 10px 14px;
  cursor: pointer;
  font-size: 13px;
  font-weight: 600;
  color: var(--foreground);
  transition: all 0.1s;
}
.toggle-row:hover {
  border-color: var(--main);
}
.toggle {
  appearance: none;
  width: 36px;
  height: 20px;
  background: var(--background);
  border: 2px solid var(--border);
  border-radius: 10px;
  cursor: pointer;
  position: relative;
  flex-shrink: 0;
  transition: all 0.15s;
}
.toggle::after {
  content: '';
  position: absolute;
  top: 2px;
  left: 2px;
  width: 12px;
  height: 12px;
  background: color-mix(in srgb, var(--foreground) 40%, transparent);
  border-radius: 50%;
  transition: all 0.15s;
}
.toggle:checked {
  background: var(--main);
  border-color: var(--main);
}
.toggle:checked::after {
  left: 18px;
  background: var(--main-foreground);
}
</style>
