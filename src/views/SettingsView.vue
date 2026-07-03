<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useSpotifyStore } from '../stores/spotify'

const spotify = useSpotifyStore()
const clientIdInput = ref('')
const loading = ref(false)
const error = ref('')

onMounted(async () => {
  await spotify.init()
  clientIdInput.value = spotify.clientId
})

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
</style>
