<script setup lang="ts">
import type { ViewName } from '../types'

defineProps<{ view: ViewName }>()
const emit = defineEmits<{ 'update:view': [view: ViewName] }>()
</script>

<template>
  <aside class="sidebar">
    <div class="nav-top">
      <button :class="['nav-icon', { active: view === 'library' }]" @click="emit('update:view', 'library')" title="Home">
        <svg viewBox="0 0 24 24" fill="currentColor"><path d="M12 3l9 8h-3v8h-4v-6h-4v6H6v-8H3l9-8z"/></svg>
      </button>
      <button class="nav-icon" title="Search">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><circle cx="11" cy="11" r="8"/><path d="M21 21l-4.35-4.35"/></svg>
      </button>
    </div>

    <div class="nav-library">
      <div class="lib-header">
        <button class="nav-icon" title="Your Library">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M4 19V5a2 2 0 0 1 2-2h13.4a.5.5 0 0 1 .49.6l-2 10a.5.5 0 0 1-.49.4h-10M4 19a2 2 0 0 0 2 2h14c1.1 0 2-.9 2-2M4 19h14"/></svg>
        </button>
      </div>

      <div class="lib-items">
        <button class="lib-icon-btn" title="Liked Songs">
          <svg viewBox="0 0 24 24" fill="currentColor" class="text-main"><path d="M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z"/></svg>
        </button>
        <button :class="['lib-icon-btn', { active: view === 'settings' }]" @click="emit('update:view', 'settings')" title="Settings">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/></svg>
        </button>
      </div>
    </div>
  </aside>
</template>

<style scoped>
.sidebar {
  display: flex;
  flex-direction: column;
  height: 100%;
  gap: 8px;
}

.nav-top {
  background: var(--panel-bg);
  border-radius: var(--radius-base);
  padding: 16px 12px;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
}

.nav-library {
  background: var(--panel-bg);
  border-radius: var(--radius-base);
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 16px 12px;
}

.lib-header {
  margin-bottom: 24px;
}

.lib-items {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.nav-icon {
  background: transparent;
  border: none;
  color: var(--text-muted);
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: color 0.2s;
}
.nav-icon:hover { color: var(--foreground); }
.nav-icon.active { color: var(--foreground); }
.nav-icon svg { width: 24px; height: 24px; }

.lib-icon-btn {
  background: transparent;
  border: none;
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 8px;
  cursor: pointer;
  color: var(--text-muted);
}
.lib-icon-btn:hover { background: var(--panel-bg-hover); color: var(--foreground); }
.lib-icon-btn.active { background: var(--panel-bg-hover); color: var(--foreground); }
.lib-icon-btn svg { width: 24px; height: 24px; }
.text-main { color: var(--main); }
</style>
