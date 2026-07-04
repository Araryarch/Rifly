<script setup lang="ts">
import { ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { Track } from '../types'

const props = defineProps<{
  track: Track | null
  show: boolean
}>()

const emit = defineEmits<{
  close: []
  saved: [track: Track]
}>()

const title = ref('')
const artist = ref('')
const album = ref('')
const album_artist = ref('')
const genre = ref('')
const year = ref<number | null>(null)
const track_number = ref<number | null>(null)
const saving = ref(false)
const error = ref('')

watch(() => props.show, (v) => {
  if (v && props.track) {
    title.value = props.track.title
    artist.value = props.track.artist
    album.value = props.track.album
    album_artist.value = props.track.album_artist
    genre.value = props.track.genres?.[0] || ''
    year.value = props.track.year || null
    track_number.value = props.track.track_number || null
    error.value = ''
  }
})

async function save() {
  if (!props.track) return
  saving.value = true
  error.value = ''
  try {
    const updated: Track = await invoke('edit_track_metadata', {
      path: props.track.path,
      title: title.value || null,
      artist: artist.value || null,
      album: album.value || null,
      albumArtist: album_artist.value || null,
      genre: genre.value || null,
      year: year.value || null,
      trackNumber: track_number.value || null,
    })
    emit('saved', updated)
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
    <div v-if="show" class="modal-overlay" @click.self="close">
      <div class="modal">
        <div class="modal-head">
          <h2>edit metadata</h2>
          <button class="btn-close" @click="close">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3"><path d="M18 6L6 18M6 6l12 12"/></svg>
          </button>
        </div>
        <div class="modal-body">
          <label class="field">
            <span>Title</span>
            <input v-model="title" type="text" placeholder="title" />
          </label>
          <label class="field">
            <span>Artist</span>
            <input v-model="artist" type="text" placeholder="artist" />
          </label>
          <label class="field">
            <span>Album</span>
            <input v-model="album" type="text" placeholder="album" />
          </label>
          <label class="field">
            <span>Album Artist</span>
            <input v-model="album_artist" type="text" placeholder="album artist" />
          </label>
          <label class="field">
            <span>Genre</span>
            <input v-model="genre" type="text" placeholder="genre" />
          </label>
          <div class="field-row">
            <label class="field">
              <span>Year</span>
              <input v-model="year" type="number" placeholder="year" />
            </label>
            <label class="field">
              <span>Track #</span>
              <input v-model="track_number" type="number" placeholder="track" />
            </label>
          </div>
          <div v-if="error" class="error">{{ error }}</div>
        </div>
        <div class="modal-foot">
          <button class="btn-cancel" @click="close" :disabled="saving">cancel</button>
          <button class="btn-save" @click="save" :disabled="saving">
            <span v-if="saving">saving...</span>
            <span v-else>save</span>
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
  width: 420px;
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
.field input {
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
.error { font-size: 12px; color: #ff6b6b; padding: 4px 0; }
.modal-foot {
  display: flex; justify-content: flex-end; gap: 8px;
  padding: 12px 20px;
  border-top: 2px solid var(--border);
}
.btn-cancel, .btn-save {
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
.btn-save {
  background: var(--main);
  color: var(--main-foreground);
  border-color: var(--main);
}
.btn-cancel:disabled, .btn-save:disabled { opacity: 0.5; cursor: not-allowed; }
</style>
