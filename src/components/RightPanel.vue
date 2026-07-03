<script setup lang="ts">
import { usePlayerStore } from '../stores/player'

const player = usePlayerStore()
</script>

<template>
  <aside class="right-panel">
    <div class="r-header">
      <div class="r-title">{{ player.currentTrack?.title || 'Code and Chill' }}</div>
      <div class="r-actions">
        <button class="icon-btn"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="1"/><circle cx="19" cy="12" r="1"/><circle cx="5" cy="12" r="1"/></svg></button>
        <button class="icon-btn"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6M15 3h6v6M10 14L21 3"/></svg></button>
      </div>
    </div>

    <div class="r-scroll">
      <div class="r-art">
        <div v-if="player.coverArt" class="r-art-img" :style="{ backgroundImage: `url(${player.coverArt})` }" />
        <div v-else class="r-art-empty">
          <svg viewBox="0 0 24 24" fill="currentColor"><circle cx="7" cy="17" r="3"/><circle cx="17" cy="15" r="3"/><polyline points="10 17 10 5 20 3 20 15"/></svg>
        </div>
      </div>

      <div class="r-info">
        <div class="r-song-title">{{ player.currentTrack?.title || 'Stone Man Rises - Wildcard' }}</div>
        <div class="r-song-artist">{{ player.currentTrack?.artist || 'Codfish' }}</div>
      </div>

      <div class="r-about">
        <div class="about-header">About the artist</div>
        <div class="about-img" />
      </div>
    </div>
  </aside>
</template>

<style scoped>
.right-panel {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.r-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px;
}

.r-title {
  font-weight: 700;
  font-size: 14px;
}

.r-actions {
  display: flex;
  gap: 8px;
}

.icon-btn {
  background: transparent;
  border: none;
  color: var(--text-muted);
  width: 32px;
  height: 32px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
}
.icon-btn:hover { color: var(--foreground); background: rgba(255,255,255,0.1); }
.icon-btn svg { width: 16px; height: 16px; }

.r-scroll {
  flex: 1;
  overflow-y: auto;
  padding: 0 16px 16px;
}

.r-art {
  width: 100%;
  aspect-ratio: 1;
  border-radius: 8px;
  overflow: hidden;
  background: var(--background);
  margin-bottom: 16px;
}

.r-art-img {
  width: 100%;
  height: 100%;
  background-size: cover;
  background-position: center;
}
.r-art-empty {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}
.r-art-empty svg { width: 48px; height: 48px; color: var(--text-muted); }

.r-info {
  margin-bottom: 24px;
}

.r-song-title {
  font-size: 24px;
  font-weight: 800;
  margin-bottom: 4px;
}

.r-song-artist {
  font-size: 14px;
  color: var(--text-muted);
}

.r-about {
  background: #2a2a2a;
  border-radius: 8px;
  height: 250px;
  position: relative;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.about-header {
  position: absolute;
  top: 16px;
  left: 16px;
  font-weight: 700;
  font-size: 14px;
  z-index: 2;
}

.about-img {
  flex: 1;
  background-color: #333;
  /* Add a gradient overlay or image here later */
}
</style>
