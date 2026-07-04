<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import type { Track } from '../types'

const emit = defineEmits<{
  close: []
  created: [track: Track]
}>()

const title = ref('')
const artist = ref('')
const album = ref('')
const album_artist = ref('')
const genre = ref('')
const year = ref<number | null>(null)
const track_number = ref<number | null>(null)
const audioPath = ref('')
const saving = ref(false)
const error = ref('')

async function pickAudio() {
  const file = await open({
    multiple: false,
    filters: [{
      name: 'Audio',
      extensions: ['flac', 'wav', 'aiff', 'aif', 'aifc', 'alac', 'm4a', 'mp3', 'aac', 'ogg', 'opus', 'wma', 'dsf', 'dff', 'ape', 'wv', 'tak'],
    }],
  })
  if (file) {
    audioPath.value = String(file)
  }
}

async function submit() {
  if (!title.value.trim()) {
    error.value = 'title is required'
    return
  }
  saving.value = true
  error.value = ''
  try {
    const track: Track = await invoke('create_music_entry', {
      title: title.value.trim(),
      artist: artist.value.trim() || 'Unknown Artist',
      album: album.value.trim() || 'Unknown Album',
      albumArtist: album_artist.value.trim() || null,
      genre: genre.value.trim() || null,
      year: year.value || null,
      trackNumber: track_number.value || null,
      audioPath: audioPath.value || null,
    })
    emit('created', track)
    emit('close')
  } catch (e: any) {
    error.value = String(e)
  } finally {
    saving.value = false
  }
}

function close() {
  if (saving.value) return
  emit('close')
}
</script>

<template>
  <Teleport to="body">
    <div class="modal-overlay" @click.self="close">
      <div class="modal">
        <div class="modal-head">
          <h2>new music entry</h2>
          <button class="btn-close" @click="close">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3"><path d="M18 6L6 18M6 6l12 12"/></svg>
          </button>
        </div>
        <div class="modal-body">
          <label class="field">
            <span class="req">Title</span>
            <input v-model="title" type="text" placeholder="song title" />
          </label>
          <div class="field-row">
            <label class="field">
              <span>Artist</span>
              <input v-model="artist" type="text" placeholder="artist" />
            </label>
            <label class="field">
              <span>Album</span>
              <input v-model="album" type="text" placeholder="album" />
            </label>
          </div>
          <div class="field-row">
            <label class="field">
              <span>Album Artist</span>
              <input v-model="album_artist" type="text" placeholder="album artist" />
            </label>
            <label class="field">
              <span>Genre</span>
              <input v-model="genre" type="text" placeholder="genre" />
            </label>
          </div>
          <div class="field-row">
            <label class="field">
              <span>Year</span>
              <input v-model.number="year" type="number" placeholder="year" />
            </label>
            <label class="field">
              <span>Track #</span>
              <input v-model.number="track_number" type="number" placeholder="track" />
            </label>
          </div>
          <label class="field audio-field">
            <span>Audio File</span>
            <div class="audio-row">
              <input :value="audioPath" type="text" placeholder="optional — pick an audio file" readonly />
              <button class="btn-browse" @click="pickAudio">browse</button>
              <button v-if="audioPath" class="btn-clear" @click="audioPath = ''">x</button>
            </div>
          </label>
          <div v-if="error" class="error">{{ error }}</div>
        </div>
        <div class="modal-foot">
          <button class="btn-cancel" @click="close" :disabled="saving">cancel</button>
          <button class="btn-create" @click="submit" :disabled="saving || !title.trim()">
            <span v-if="saving">creating...</span>
            <span v-else>create</span>
          </button>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<style scoped>
.modal-overlay {
  position: fixed; inset: 0;
  background: rgba(0,0,0,0.6);
  display: flex; align-items: center; justify-content: center;
  z-index: 1000;
}
.modal {
  background: var(--background);
  border: 2px solid var(--border);
  border-radius: var(--radius-base);
  box-shadow: var(--shadow), 0 8px 32px rgba(0,0,0,0.3);
  width: 480px;
  max-width: 90vw;
}
.modal-head {
  display: flex; justify-content: space-between; align-items: center;
  padding: 16px 20px;
  border-bottom: 2px solid var(--border);
}
.modal-head h2 {
  font-size: 14px; font-weight: 800; text-transform: lowercase;
}
.btn-close {
  background: none; border: none; color: var(--foreground);
  cursor: pointer; padding: 4px;
}
.btn-close svg { width: 16px; height: 16px; }
.modal-body { padding: 20px; display: flex; flex-direction: column; gap: 14px; }
.field { display: flex; flex-direction: column; gap: 4px; }
.field span { font-size: 10px; font-weight: 700; text-transform: uppercase; color: var(--text-muted); }
.field .req::after { content: ' *'; color: var(--main); }
.field input, .audio-row input {
  background: var(--secondary-background);
  border: 2px solid var(--border);
  border-radius: calc(var(--radius-base) - 2px);
  padding: 8px 10px;
  font-family: var(--font);
  font-size: 13px;
  color: var(--foreground);
  outline: none;
  transition: border-color 0.15s;
}
.field input:focus { border-color: var(--main); }
.field-row { display: flex; gap: 12px; }
.field-row .field { flex: 1; }
.audio-row { display: flex; gap: 6px; }
.audio-row input { flex: 1; cursor: default; }
.btn-browse, .btn-clear {
  padding: 8px 14px;
  font-family: var(--font); font-size: 10px; font-weight: 700; text-transform: uppercase;
  border: 2px solid var(--border);
  border-radius: calc(var(--radius-base) - 2px);
  cursor: pointer;
}
.btn-browse { background: var(--secondary-background); color: var(--foreground); }
.btn-clear { background: transparent; color: var(--foreground); border-color: transparent; }
.error { font-size: 12px; color: #ff6b6b; padding: 4px 0; }
.modal-foot {
  display: flex; justify-content: flex-end; gap: 8px;
  padding: 12px 20px;
  border-top: 2px solid var(--border);
}
.btn-cancel, .btn-create {
  padding: 8px 20px;
  font-family: var(--font);
  font-size: 11px;
  font-weight: 700;
  border: 2px solid var(--border);
  border-radius: calc(var(--radius-base) - 2px);
  cursor: pointer;
}
.btn-cancel {
  background: var(--secondary-background);
  color: var(--foreground);
}
.btn-create {
  background: var(--main);
  color: var(--main-foreground);
  border-color: var(--main);
}
.btn-cancel:disabled, .btn-create:disabled { opacity: 0.5; cursor: not-allowed; }
</style>
