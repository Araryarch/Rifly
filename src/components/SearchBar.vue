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
  <div :class="['search-box', { focused }]">
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
    <kbd v-if="!focused && !query" class="search-kbd">CTRL+K</kbd>
    <button v-if="query" class="search-clear" @click="query = ''; emit('search', '')">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M18 6 6 18M6 6l12 12"/></svg>
    </button>
  </div>
</template>

<style scoped>
.search-box {
  display: flex;
  align-items: center;
  gap: 8px;
  background: var(--secondary-background);
  border: 2px solid var(--border);
  border-radius: var(--radius-base);
  padding: 6px 12px;
  max-width: 280px;
  width: 100%;
  transition: all 0.15s;
}

.search-box.focused {
  outline: 2px solid var(--ring);
  outline-offset: 2px;
}

.search-icon {
  width: 14px;
  height: 14px;
  color: color-mix(in srgb, var(--foreground) 50%, transparent);
  flex-shrink: 0;
}

.search-input {
  flex: 1;
  background: transparent;
  border: none;
  outline: none;
  color: var(--foreground);
  font-family: var(--font);
  font-size: 13px;
  min-width: 0;
}

.search-input::placeholder {
  color: color-mix(in srgb, var(--foreground) 50%, transparent);
}

.search-kbd {
  font-family: var(--font);
  font-size: 10px;
  font-weight: 700;
  color: var(--main-foreground);
  background: var(--main);
  border: 1px solid var(--border);
  border-radius: 4px;
  padding: 2px 6px;
  flex-shrink: 0;
  pointer-events: none;
}

.search-clear {
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  width: 18px;
  height: 18px;
  cursor: pointer;
  color: color-mix(in srgb, var(--foreground) 50%, transparent);
  flex-shrink: 0;
  transition: color 0.1s;
}

.search-clear:hover {
  color: var(--foreground);
}
</style>
