<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { useLibraryStore } from '../stores/library'
import { useSpotifyStore } from '../stores/spotify'

const emit = defineEmits<{ done: [] }>()

const lib = useLibraryStore()
const spotify = useSpotifyStore()

const steps = ['welcome', 'music', 'spotify', 'discord', 'done']
const stepIndex = ref(0)
const step = computed(() => steps[stepIndex.value])

// Music step
const selectedFolder = ref('')
const musicDone = ref(false)
const creating = ref(false)

// Spotify step
const spotifyClientId = ref('')
const spotifyDone = ref(false)

// Discord step
const discordClientId = ref('')
const discordDone = ref(false)

onMounted(async () => {
  const saved = await invoke('get_setting', { key: 'music_folder' }).catch(() => null)
  if (saved) { selectedFolder.value = String(saved); musicDone.value = true }
  const scid = await invoke('get_setting', { key: 'spotify_client_id' }).catch(() => null)
  if (scid) { spotifyClientId.value = String(scid); spotifyDone.value = true }
  const dcid = await invoke('get_setting', { key: 'discord_client_id' }).catch(() => null)
  if (dcid) { discordClientId.value = String(dcid); discordDone.value = true }
})

async function selectFolder() {
  const folder = await open({ directory: true, multiple: false })
  if (folder) {
    selectedFolder.value = String(folder)
    musicDone.value = true
  }
}

async function createRiflyFolder() {
  creating.value = true
  try {
    const folder = await open({ directory: true, multiple: false })
    if (folder) {
      const path: string = await invoke('create_rifly_folder', { base: String(folder) })
      selectedFolder.value = path
      musicDone.value = true
    }
  } finally {
    creating.value = false
  }
}

async function connectSpotify() {
  if (!spotifyClientId.value.trim()) return
  await spotify.saveClientId(spotifyClientId.value.trim())
  try { await spotify.authenticate(); spotifyDone.value = true } catch {}
}

async function saveDiscord() {
  if (discordClientId.value.trim()) {
    await invoke('set_setting', { key: 'discord_client_id', value: discordClientId.value.trim() })
  }
  discordDone.value = true
}

async function finish() {
  if (selectedFolder.value) {
    lib.scanFolder(selectedFolder.value)
    await invoke('set_setting', { key: 'music_folder', value: selectedFolder.value })
  }
  await invoke('set_setting', { key: 'setup_done', value: 'true' })
  emit('done')
}

function prev() { stepIndex.value-- }
function next() { stepIndex.value++ }

const canNext = computed(() => {
  switch (step.value) {
    case 'welcome': return true
    case 'music': return musicDone.value
    case 'spotify': return true
    case 'discord': return true
    default: return false
  }
})

const isLast = computed(() => step.value === 'done')
</script>

<template>
  <div class="setup-overlay">
    <div class="setup-window">
      <aside class="setup-sidebar">
        <div class="setup-brand">
          <img src="/logo/logos.png" alt="Rifly" class="setup-logo" />
          <span class="setup-brand-name">Rifly</span>
        </div>
        <nav class="setup-steps">
          <div
            v-for="(s, i) in steps"
            :key="s"
            class="step-item"
            :class="{ active: stepIndex === i, done: stepIndex > i }"
          >
            <span class="step-num">
              <svg v-if="stepIndex > i" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3"><path d="M20 6L9 17l-5-5"/></svg>
              <span v-else>{{ i + 1 }}</span>
            </span>
            <span class="step-label">{{ s }}</span>
          </div>
        </nav>
      </aside>

      <div class="setup-main">
        <div class="setup-step">
          <!-- Welcome -->
          <div v-if="step === 'welcome'" class="step-body center">
            <img src="/logo/logos.png" alt="" class="welcome-logo" />
            <h1>Welcome to Rifly</h1>
            <p class="welcome-desc">Audiophile-grade desktop music player with high-resolution audio support and Spotify integration.</p>
            <p class="welcome-hint">Let's get you set up in a few quick steps.</p>
          </div>

          <!-- Music -->
          <div v-else-if="step === 'music'" class="step-body">
            <h2>Music Library</h2>
            <p>Choose an existing music folder or let Rifly create a managed library structure.</p>
            <div class="field-box">
              <div class="field-row">
                <input :value="selectedFolder" type="text" placeholder="no folder selected" readonly class="field-input" />
                <button class="btn-action" @click="selectFolder">browse</button>
              </div>
              <p v-if="musicDone" class="field-ok">folder selected</p>
            </div>
            <div class="divider">
              <span>or</span>
            </div>
            <button class="btn-create-folder" @click="createRiflyFolder" :disabled="creating">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/></svg>
              <span>{{ creating ? 'creating...' : 'auto-create Rifly folder' }}</span>
            </button>
            <p class="field-help">Creates <strong>Rifly/Albums</strong>, <strong>Rifly/Playlists</strong>, and <strong>Rifly/Imports</strong> — drop your music into <strong>Albums</strong>.</p>
          </div>

          <!-- Spotify -->
          <div v-else-if="step === 'spotify'" class="step-body">
            <h2>Spotify Integration</h2>
            <p>Optional. Connect your Spotify Premium account to search and stream millions of tracks directly in Rifly.</p>
            <div class="field-box">
              <label class="field-label">Spotify Client ID</label>
              <input v-model="spotifyClientId" type="text" placeholder="paste your client id" class="field-input" />
              <div class="field-actions">
                <button class="btn-action" @click="connectSpotify" :disabled="!spotifyClientId.trim()">connect</button>
                <button class="btn-skip" @click="spotifyDone = true">skip</button>
              </div>
              <p v-if="spotifyDone" class="field-ok">done</p>
            </div>
            <p class="field-help">Go to <a href="https://developer.spotify.com/dashboard" target="_blank">Spotify Developer Dashboard</a> to create an app and get your Client ID.</p>
          </div>

          <!-- Discord -->
          <div v-else-if="step === 'discord'" class="step-body">
            <h2>Discord Rich Presence</h2>
            <p>Optional. Show what you're listening to on your Discord profile, including hi-res audio badges.</p>
            <div class="field-box">
              <label class="field-label">Discord Application ID (Client ID)</label>
              <input v-model="discordClientId" type="text" placeholder="paste your application id" class="field-input" />
              <div class="field-actions">
                <button class="btn-action" @click="saveDiscord" :disabled="!discordClientId.trim()">save</button>
                <button class="btn-skip" @click="discordDone = true">skip</button>
              </div>
              <p v-if="discordDone" class="field-ok">done</p>
            </div>
            <p class="field-help">Create an application at <a href="https://discord.com/developers/applications" target="_blank">Discord Developer Portal</a> and copy the Client ID.</p>
          </div>

          <!-- Done -->
          <div v-else-if="step === 'done'" class="step-body center">
            <div class="done-icon">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><path d="M22 4L12 14.01l-3-3"/></svg>
            </div>
            <h1>All set!</h1>
            <p class="done-summary">
              <span v-if="musicDone">music library configured</span>
              <span v-if="spotifyDone"> / spotify connected</span>
              <span v-if="discordDone"> / discord rpc ready</span>
              <span v-if="!musicDone && !spotifyDone && !discordDone">nothing configured — you can change this later in settings.</span>
            </p>
            <p class="welcome-hint">You can always change these settings later.</p>
            <button class="btn-finish" @click="finish">start listening</button>
          </div>
        </div>

        <div class="setup-footer">
          <button v-if="stepIndex > 0" class="btn-prev" @click="prev">back</button>
          <div class="footer-spacer" />
          <button v-if="!isLast" class="btn-next" :disabled="!canNext" @click="next">next</button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.setup-overlay {
  position: fixed; inset: 0; z-index: 9999;
  background: #000;
  display: flex; align-items: center; justify-content: center;
}
.setup-window {
  width: 720px; height: 480px;
  display: flex;
  background: var(--panel-bg);
  border: 2px solid var(--border);
  border-radius: var(--radius-base);
  box-shadow: var(--shadow), 0 16px 48px rgba(0,0,0,0.5);
  overflow: hidden;
}

.setup-sidebar {
  width: 180px; flex-shrink: 0;
  background: var(--secondary-background);
  padding: 24px 16px;
  display: flex; flex-direction: column;
  gap: 32px;
}
.setup-brand { display: flex; align-items: center; gap: 10px; }
.setup-logo { width: 28px; height: 28px; object-fit: contain; }
.setup-brand-name { font-size: 16px; font-weight: 800; text-transform: lowercase; }
.setup-steps { display: flex; flex-direction: column; gap: 6px; }
.step-item {
  display: flex; align-items: center; gap: 10px; padding: 6px 8px;
  border-radius: calc(var(--radius-base) - 2px);
  font-size: 11px; font-weight: 600; text-transform: uppercase;
  color: var(--text-muted);
}
.step-item.active { background: var(--panel-bg); color: var(--foreground); }
.step-item.done { color: var(--main); }
.step-num {
  width: 20px; height: 20px; flex-shrink: 0;
  display: flex; align-items: center; justify-content: center;
  background: var(--panel-bg);
  border-radius: 50%;
  font-size: 10px; font-weight: 700;
}
.step-item.done .step-num { background: var(--main); color: var(--main-foreground); }
.step-item.active .step-num { background: var(--main); color: var(--main-foreground); }
.step-num svg { width: 10px; height: 10px; }

.setup-main {
  flex: 1; display: flex; flex-direction: column;
  min-width: 0;
}
.setup-step { flex: 1; padding: 40px 32px; overflow-y: auto; }
.step-body { max-width: 420px; }
.step-body.center {
  display: flex; flex-direction: column; align-items: center; text-align: center;
  max-width: 100%; padding-top: 32px;
}
.step-body h1 { font-size: 22px; font-weight: 800; margin-bottom: 8px; }
.step-body h2 { font-size: 16px; font-weight: 800; margin-bottom: 8px; }
.step-body p { font-size: 13px; color: var(--text-muted); line-height: 1.6; margin-bottom: 16px; }
.welcome-logo { width: 64px; height: 64px; object-fit: contain; margin-bottom: 16px; }
.welcome-desc { max-width: 340px; }
.welcome-hint { font-size: 12px; color: var(--text-muted); margin-top: 16px; }

.field-box { background: var(--background); border-radius: var(--radius-base); padding: 16px; margin-bottom: 8px; }
.field-label { display: block; font-size: 10px; font-weight: 700; text-transform: uppercase; color: var(--text-muted); margin-bottom: 6px; }
.field-input {
  width: 100%; padding: 8px 10px;
  font-family: var(--font); font-size: 13px;
  background: var(--secondary-background); color: var(--foreground);
  border: 2px solid var(--border); border-radius: calc(var(--radius-base) - 2px);
  outline: none;
}
.field-input:focus { border-color: var(--main); }
.field-row { display: flex; gap: 8px; }
.field-row .field-input { flex: 1; }
.field-actions { display: flex; gap: 8px; margin-top: 10px; }
.field-ok { font-size: 11px; color: var(--main); font-weight: 700; margin-top: 8px; }
.field-help { font-size: 11px; color: var(--text-muted); }
.field-help a { color: var(--main); text-decoration: none; }

.btn-action {
  padding: 8px 18px; font-family: var(--font); font-size: 10px; font-weight: 700; text-transform: uppercase;
  background: var(--main); color: var(--main-foreground);
  border: 2px solid var(--main); border-radius: calc(var(--radius-base) - 2px);
  cursor: pointer; white-space: nowrap;
}
.btn-action:disabled { opacity: 0.4; cursor: default; }
.divider { display: flex; align-items: center; gap: 12px; margin: 16px 0; font-size: 10px; font-weight: 700; text-transform: uppercase; color: var(--text-muted); }
.divider::before, .divider::after { content: ''; flex: 1; height: 2px; background: var(--border); }
.btn-create-folder {
  display: flex; align-items: center; justify-content: center; gap: 10px;
  width: 100%; padding: 12px;
  font-family: var(--font); font-size: 11px; font-weight: 700; text-transform: uppercase;
  background: var(--secondary-background); color: var(--foreground);
  border: 2px dashed var(--border);
  border-radius: var(--radius-base);
  cursor: pointer; transition: all 0.15s;
}
.btn-create-folder:hover { border-color: var(--main); color: var(--main); }
.btn-create-folder:disabled { opacity: 0.5; cursor: default; }
.btn-create-folder svg { width: 18px; height: 18px; }

.btn-skip {
  padding: 8px 18px; font-family: var(--font); font-size: 10px; font-weight: 700; text-transform: uppercase;
  background: transparent; color: var(--text-muted);
  border: 2px solid var(--border); border-radius: calc(var(--radius-base) - 2px);
  cursor: pointer;
}

.done-icon {
  width: 56px; height: 56px; margin-bottom: 16px;
  background: var(--main); border-radius: 50%;
  display: flex; align-items: center; justify-content: center;
  color: var(--main-foreground);
}
.done-icon svg { width: 28px; height: 28px; }
.done-summary { font-size: 13px; color: var(--text-muted); }
.btn-finish {
  padding: 12px 32px; font-family: var(--font); font-size: 12px; font-weight: 700; text-transform: uppercase;
  background: var(--main); color: var(--main-foreground);
  border: 2px solid var(--main); border-radius: calc(var(--radius-base) - 2px);
  cursor: pointer; margin-top: 8px;
}

.setup-footer {
  display: flex; align-items: center; padding: 12px 24px;
  border-top: 2px solid var(--border);
}
.footer-spacer { flex: 1; }
.btn-prev, .btn-next {
  padding: 8px 20px; font-family: var(--font); font-size: 10px; font-weight: 700; text-transform: uppercase;
  border-radius: calc(var(--radius-base) - 2px); cursor: pointer;
}
.btn-prev {
  background: var(--secondary-background); color: var(--foreground);
  border: 2px solid var(--border);
}
.btn-next {
  background: var(--main); color: var(--main-foreground);
  border: 2px solid var(--main);
}
.btn-next:disabled { opacity: 0.4; cursor: default; }
</style>
