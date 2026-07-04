import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { ViewName } from '../types'

export const useUiStore = defineStore('ui', () => {
  const searchQuery = ref('')
  const searchTrigger = ref(0)

  // --- View navigation history ---
  const history = ref<ViewName[]>(['library'])
  const historyIndex = ref(0)

  const currentView = computed(() => history.value[historyIndex.value])
  const canGoBack = computed(() => historyIndex.value > 0)
  const canGoForward = computed(() => historyIndex.value < history.value.length - 1)

  function navigate(view: ViewName) {
    if (currentView.value === view) return
    // Truncate any forward history when navigating to a new page
    history.value = history.value.slice(0, historyIndex.value + 1)
    history.value.push(view)
    historyIndex.value = history.value.length - 1
  }

  function goBack() {
    if (canGoBack.value) {
      historyIndex.value--
    }
  }

  function goForward() {
    if (canGoForward.value) {
      historyIndex.value++
    }
  }

  function triggerSearch(query: string) {
    searchQuery.value = query
    searchTrigger.value++
  }

  return {
    searchQuery,
    searchTrigger,
    triggerSearch,
    // Navigation
    currentView,
    canGoBack,
    canGoForward,
    navigate,
    goBack,
    goForward,
  }
})
