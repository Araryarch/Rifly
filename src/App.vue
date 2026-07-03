<script setup lang="ts">
import { ref, onMounted } from 'vue'
import Sidebar from './components/Sidebar.vue'
import Topbar from './components/Topbar.vue'
import RightPanel from './components/RightPanel.vue'
import PlayerBar from './components/PlayerBar.vue'
import LibraryView from './views/LibraryView.vue'
import NowPlayingView from './views/NowPlayingView.vue'
import QueueView from './views/QueueView.vue'
import SettingsView from './views/SettingsView.vue'
import { usePlayerStore } from './stores/player'
import type { ViewName } from './types'

const player = usePlayerStore()
const currentView = ref<ViewName>('library')

onMounted(() => {
  player.init()
})
</script>

<template>
  <div class="shell theme-spotify">
    <div class="shell-content">
      <div class="shell-topbar">
        <Topbar />
      </div>

      <div class="shell-sidebar">
        <Sidebar v-model:view="currentView" />
      </div>

      <main class="shell-main">
        <Transition name="fade" mode="out-in">
          <LibraryView v-if="currentView === 'library'" key="library" />
          <NowPlayingView v-else-if="currentView === 'now-playing'" key="now-playing" />
          <QueueView v-else-if="currentView === 'queue'" key="queue" />
          <SettingsView v-else-if="currentView === 'settings'" key="settings" />
        </Transition>
      </main>

      <div class="shell-right">
        <RightPanel />
      </div>
    </div>

    <div class="shell-player">
      <PlayerBar />
    </div>
  </div>
</template>

<style>
@import './styles/global.css';
</style>

<style scoped>
.shell {
  height: 100vh;
  display: flex;
  flex-direction: column;
  background: var(--background);
  overflow: hidden;
}

.shell-content {
  flex: 1;
  display: grid;
  grid-template-columns: var(--sidebar-width) 1fr var(--right-panel-width);
  grid-template-rows: var(--topbar-height) 1fr;
  grid-template-areas:
    "topbar  topbar topbar"
    "sidebar main   right";
  gap: 8px;
  padding: 8px 8px 0 8px;
  min-height: 0;
}

.shell-topbar {
  grid-area: topbar;
}

.shell-sidebar { 
  grid-area: sidebar; 
  background: var(--panel-bg);
  border-radius: var(--radius-base);
  overflow: hidden;
}

.shell-main { 
  grid-area: main; 
  background: var(--panel-bg);
  border-radius: var(--radius-base);
  overflow: hidden; 
}

.shell-right {
  grid-area: right;
  background: var(--panel-bg);
  border-radius: var(--radius-base);
  overflow: hidden;
}

.shell-player { 
  height: var(--player-height);
  flex-shrink: 0;
}

.fade-enter-active { transition: opacity 0.15s ease; }
.fade-leave-active { transition: opacity 0.1s ease; }
.fade-enter-from, .fade-leave-to { opacity: 0; }
</style>
