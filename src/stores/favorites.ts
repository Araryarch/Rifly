import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export const useFavoritesStore = defineStore('favorites', () => {
  const paths = ref<Set<string>>(new Set())

  async function load() {
    try {
      const favs: string[] = await invoke('get_favorites')
      paths.value = new Set(favs)
    } catch { paths.value = new Set() }
  }

  function isFavorite(path: string): boolean {
    return paths.value.has(path)
  }

  async function toggle(path: string): Promise<boolean> {
    try {
      const now: boolean = await invoke('toggle_favorite', { path })
      if (now) {
        paths.value.add(path)
      } else {
        paths.value.delete(path)
      }
      return now
    } catch { return false }
  }

  return { paths, load, isFavorite, toggle }
})
