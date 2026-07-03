import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useUiStore = defineStore('ui', () => {
  const searchQuery = ref('')
  const searchTrigger = ref(0) // used to trigger search

  function triggerSearch(query: string) {
    searchQuery.value = query
    searchTrigger.value++
  }

  return {
    searchQuery,
    searchTrigger,
    triggerSearch
  }
})
