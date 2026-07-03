<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'

const query = ref('')
const focused = ref(false)
const inputRef = ref<HTMLInputElement | null>(null)

const emit = defineEmits<{ search: [query: string] }>()

function onInput() {
  emit('search', query.value)
}

function handleKeydown(e: KeyboardEvent) {
  if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
    e.preventDefault()
    inputRef.value?.focus()
  }
  if (e.key === 'Escape') {
    query.value = ''
    emit('search', '')
    inputRef.value?.blur()
  }
}

onMounted(() => window.addEventListener('keydown', handleKeydown))
onUnmounted(() => window.removeEventListener('keydown', handleKeydown))
</script>

<template>
  <div :class="['search', { focused }]">
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="search-icon">
      <circle cx="11" cy="11" r="7" /><path d="m20 20-3.5-3.5" />
    </svg>
    <input
      ref="inputRef"
      v-model="query"
      type="text"
      placeholder="Search"
      class="search-input"
      @input="onInput"
      @focus="focused = true"
      @blur="focused = false"
    />
    <kbd v-if="!focused && !query" class="search-key">ctrl+k</kbd>
    <button v-if="query" class="search-clear" @click="query = ''; emit('search', '')">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M18 6 6 18M6 6l12 12"/></svg>
    </button>
  </div>
</template>

<style scoped>
.search {
  display: flex;
  align-items: center;
  gap: 8px;
  background: var(--color-bg-tertiary);
  border: 2px solid var(--color-border);
  padding: 5px 10px;
  max-width: 260px;
  width: 100%;
  transition: border-color 0.1s;
}

.search:hover {
  border-color: var(--color-border-bold);
}

.search.focused {
  border-color: var(--color-text-primary);
}

.search-icon {
  width: 13px;
  height: 13px;
  color: var(--color-text-tertiary);
  flex-shrink: 0;
}

.search-input {
  flex: 1;
  background: transparent;
  border: none;
  outline: none;
  color: var(--color-text-primary);
  font-family: var(--font);
  font-size: 12px;
  min-width: 0;
}

.search-input::placeholder {
  color: var(--color-text-tertiary);
}

.search-key {
  font-family: var(--font);
  font-size: 9px;
  color: var(--color-text-subdued);
  border: 1px solid var(--color-border);
  padding: 0px 4px;
  flex-shrink: 0;
  pointer-events: none;
  text-transform: lowercase;
}

.search-clear {
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  width: 16px;
  height: 16px;
  cursor: pointer;
  color: var(--color-text-tertiary);
  flex-shrink: 0;
}

.search-clear:hover {
  color: var(--color-text-primary);
}
</style>
