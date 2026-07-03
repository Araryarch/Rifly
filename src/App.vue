<script setup lang="ts">
import { ref, onMounted } from 'vue'
import Sidebar from './components/Sidebar.vue'
import PlayerBar from './components/PlayerBar.vue'
import LibraryView from './views/LibraryView.vue'
import NowPlayingView from './views/NowPlayingView.vue'
import QueueView from './views/QueueView.vue'
import { usePlayerStore } from './stores/player'
import type { ViewName } from './types'

const player = usePlayerStore()
const currentView = ref<ViewName>('library')

onMounted(() => {
  player.init()
})
</script>

<template>
  <div class="app-shell">
    <!-- Sidebar -->
    <div class="app-sidebar">
      <Sidebar v-model:view="currentView" />
    </div>

    <!-- Main Content -->
    <main class="app-main">
      <Transition name="view-fade" mode="out-in">
        <LibraryView v-if="currentView === 'library'" key="library" />
        <NowPlayingView v-else-if="currentView === 'now-playing'" key="now-playing" />
        <QueueView v-else-if="currentView === 'queue'" key="queue" />
      </Transition>
    </main>

    <!-- Player Bar -->
    <div class="app-player">
      <PlayerBar />
    </div>
  </div>
</template>

<style>
@import './styles/global.css';
</style>

<style scoped>
.app-shell {
  height: 100vh;
  display: grid;
  grid-template-columns: var(--sidebar-width) 1fr;
  grid-template-rows: 1fr var(--player-height);
  grid-template-areas:
    "sidebar main"
    "player  player";
  background: var(--color-bg-primary);
  overflow: hidden;
}

.app-sidebar {
  grid-area: sidebar;
}

.app-main {
  grid-area: main;
  overflow: hidden;
  background: var(--color-bg-primary);
  position: relative;
  border-radius: 8px 8px 0 0;
  margin-top: 8px;
  margin-right: 8px;
}

.app-player {
  grid-area: player;
}

/* ── View Transitions ── */
.view-fade-enter-active {
  transition: opacity 0.2s ease, transform 0.2s ease;
}

.view-fade-leave-active {
  transition: opacity 0.15s ease;
}

.view-fade-enter-from {
  opacity: 0;
  transform: translateY(6px);
}

.view-fade-leave-to {
  opacity: 0;
}
</style>
