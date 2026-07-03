<script setup lang="ts">
import type { ViewName } from '../types'

defineProps<{ view: ViewName }>()
const emit = defineEmits<{ 'update:view': [view: ViewName] }>()
</script>

<template>
  <aside class="sidebar">
    <div class="nav-top">
      <button :class="['nav-item', { active: view === 'library' }]" @click="emit('update:view', 'library')" title="Home">
        <svg viewBox="0 0 24 24" fill="currentColor"><path d="M12 3l9 8h-3v8h-4v-6h-4v6H6v-8H3l9-8z"/></svg>
        <span>Home</span>
      </button>
      <button class="nav-item" title="Search">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><circle cx="11" cy="11" r="8"/><path d="M21 21l-4.35-4.35"/></svg>
        <span>Search</span>
      </button>
    </div>

    <div class="nav-library">
      <div class="lib-header">
        <button class="nav-item header-item" title="Your Library">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M4 19V5a2 2 0 0 1 2-2h13.4a.5.5 0 0 1 .49.6l-2 10a.5.5 0 0 1-.49.4h-10M4 19a2 2 0 0 0 2 2h14c1.1 0 2-.9 2-2M4 19h14"/></svg>
          <span>Your Library</span>
        </button>
      </div>

      <div class="lib-items">
        <button class="nav-item">
          <div class="icon-box liked">
            <svg viewBox="0 0 24 24" fill="currentColor"><path d="M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z"/></svg>
          </div>
          <span>Liked Songs</span>
        </button>
        <button :class="['nav-item', { active: view === 'settings' }]" @click="emit('update:view', 'settings')" title="Settings">
          <div class="icon-box settings-icon">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/></svg>
          </div>
          <span>Settings</span>
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
  padding: 12px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.nav-library {
  background: var(--panel-bg);
  border-radius: var(--radius-base);
  flex: 1;
  display: flex;
  flex-direction: column;
  padding: 12px;
}

.lib-header {
  margin-bottom: 16px;
}

.lib-items {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 16px;
  background: transparent;
  border: none;
  color: var(--text-muted);
  padding: 12px 16px;
  border-radius: 6px;
  cursor: pointer;
  font-family: var(--font);
  font-size: 14px;
  font-weight: 700;
  transition: all 0.2s;
  width: 100%;
  text-align: left;
}
.nav-item:hover {
  color: var(--foreground);
}
.nav-item.active {
  color: var(--foreground);
}
.nav-item svg {
  width: 24px;
  height: 24px;
  flex-shrink: 0;
}

.header-item {
  padding: 8px 16px;
}
.header-item svg {
  width: 24px;
  height: 24px;
}

.icon-box {
  width: 24px;
  height: 24px;
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}
.icon-box.liked {
  background: linear-gradient(135deg, #450af5, #c4efd9);
  color: #fff;
}
.icon-box.liked svg {
  width: 12px;
  height: 12px;
}
.icon-box.settings-icon {
  background: var(--secondary-background);
  color: var(--text-muted);
}
.icon-box.settings-icon svg {
  width: 20px;
  height: 20px;
}
.nav-item:hover .settings-icon {
  color: var(--foreground);
}
.nav-item.active .settings-icon {
  color: var(--foreground);
}
</style>
