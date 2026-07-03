<script setup lang="ts">
import { usePlayerStore } from '../stores/player'
import { formatTime } from '../types'
import EqBars from '../components/EqBars.vue'

const player = usePlayerStore()
</script>

<template>
  <div class="q-view">
    <div class="q-head">
      <div class="q-head-left">
        <h1 class="q-title">queue</h1>
        <span class="q-count">{{ player.queue.length }} tracks</span>
      </div>
      <div class="q-head-right">
        <button :class="['btn-chip', { on: player.shuffle }]" @click="player.toggleShuffle()">SHUFFLE</button>
        <button :class="['btn-chip', { on: player.repeat !== 'off' }]" @click="player.cycleRepeat()">
          {{ player.repeat === 'all' ? 'REPEAT ALL' : player.repeat === 'one' ? 'REPEAT 1' : 'REPEAT' }}
        </button>
        <button v-if="player.queue.length > 0" class="btn-clear" @click="player.clearQueue()">clear</button>
      </div>
    </div>

    <div class="q-scroll">
      <div v-if="player.currentTrack && player.queueIndex >= 0" class="q-sec">
        <h3 class="q-sec-title">NOW PLAYING</h3>
        <div class="q-item playing">
          <div class="qi-eq"><EqBars :playing="player.isPlaying" size="sm" /></div>
          <div class="qi-art">
            <div v-if="player.coverArt" class="qi-art-img" :style="{ backgroundImage: `url(${player.coverArt})` }" />
            <div v-else class="qi-art-empty">
              <svg viewBox="0 0 24 24" fill="currentColor"><circle cx="7" cy="17" r="3"/><circle cx="17" cy="15" r="3"/><polyline points="10 17 10 5 20 3 20 15"/></svg>
            </div>
          </div>
          <div class="qi-info">
            <div class="qi-title on">{{ player.currentTrack.title }}</div>
            <div class="qi-sub">{{ player.currentTrack.artist }} -- {{ player.currentTrack.album }}</div>
          </div>
          <div class="qi-dur">{{ formatTime(player.currentTrack.duration) }}</div>
        </div>
      </div>

      <div v-if="player.queue.length > 0" class="q-sec">
        <h3 class="q-sec-title">{{ player.queueIndex >= 0 ? 'NEXT UP' : 'QUEUE' }}</h3>
        <div
          v-for="(track, idx) in player.queue"
          :key="track.path + idx"
          v-show="idx !== player.queueIndex"
          class="q-item"
          @dblclick="player.playIndex(idx)"
        >
          <div class="qi-num">{{ idx + 1 }}</div>
          <div class="qi-info">
            <div class="qi-title">{{ track.title }}</div>
            <div class="qi-sub">{{ track.artist }} -- {{ track.album }}</div>
          </div>
          <div class="qi-dur">{{ formatTime(track.duration) }}</div>
          <button class="btn-rm" @click="player.removeFromQueue(idx)">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M18 6 6 18M6 6l12 12"/></svg>
          </button>
        </div>
      </div>

      <div v-if="player.queue.length === 0" class="empty">
        queue is empty
      </div>
    </div>
  </div>
</template>

<style scoped>
.q-view {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.q-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 24px 24px 16px;
  flex-shrink: 0;
}
.q-head-left {
  display: flex;
  align-items: baseline;
  gap: 12px;
}
.q-title {
  font-size: 28px;
  font-weight: 800;
  text-transform: lowercase;
}
.q-count {
  font-size: 11px;
  color: color-mix(in srgb, var(--foreground) 40%, transparent);
}
.q-head-right {
  display: flex;
  align-items: center;
  gap: 8px;
}
.btn-chip {
  padding: 6px 12px;
  font-family: var(--font);
  font-size: 10px;
  font-weight: 700;
  background: var(--secondary-background);
  color: var(--foreground);
  border: 2px solid var(--border);
  border-radius: var(--radius-base);
  cursor: pointer;
  box-shadow: var(--shadow);
  transition: all 0.15s;
}
.btn-chip:hover {
  box-shadow: none;
  transform: translate(4px, 4px);
}
.btn-chip.on {
  background: var(--main);
  color: var(--main-foreground);
  box-shadow: none;
  transform: translate(4px, 4px);
}
.btn-clear {
  background: transparent;
  border: none;
  color: color-mix(in srgb, var(--foreground) 50%, transparent);
  font-family: var(--font);
  font-size: 11px;
  text-decoration: underline;
  cursor: pointer;
  margin-left: 8px;
}
.btn-clear:hover { color: var(--foreground); }

.q-scroll {
  flex: 1;
  overflow-y: auto;
  padding: 0 16px 24px;
}

.q-sec {
  margin-bottom: 24px;
}
.q-sec-title {
  font-size: 11px;
  font-weight: 700;
  color: color-mix(in srgb, var(--foreground) 40%, transparent);
  padding: 0 8px 8px;
}

.q-item {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 8px;
  border: 2px solid transparent;
  border-radius: var(--radius-base);
  cursor: pointer;
  transition: background 0.1s;
}
.q-item:hover {
  background: var(--secondary-background);
  border-color: var(--border);
}

.q-item.playing {
  background: color-mix(in srgb, var(--main) 15%, transparent);
  border-color: var(--main);
}
.q-item.playing:hover {
  background: color-mix(in srgb, var(--main) 20%, transparent);
}

.qi-eq {
  width: 24px;
  display: flex;
  justify-content: center;
}
.qi-num {
  width: 24px;
  text-align: center;
  font-size: 12px;
  font-variant-numeric: tabular-nums;
  color: color-mix(in srgb, var(--foreground) 50%, transparent);
}

.qi-art {
  width: 40px;
  height: 40px;
  border: 2px solid var(--border);
  border-radius: var(--radius-base);
  background: var(--background);
  overflow: hidden;
  flex-shrink: 0;
}
.qi-art-img { width: 100%; height: 100%; background-size: cover; background-position: center; }
.qi-art-empty { width: 100%; height: 100%; display: flex; align-items: center; justify-content: center; }
.qi-art-empty svg { width: 16px; height: 16px; color: color-mix(in srgb, var(--foreground) 40%, transparent); }

.qi-info {
  flex: 1;
  min-width: 0;
}
.qi-title {
  font-size: 13px;
  font-weight: 600;
  color: var(--foreground);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.qi-title.on { color: var(--main); }
.qi-sub {
  font-size: 11px;
  color: color-mix(in srgb, var(--foreground) 50%, transparent);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.qi-dur {
  font-size: 12px;
  color: color-mix(in srgb, var(--foreground) 50%, transparent);
  font-variant-numeric: tabular-nums;
}

.btn-rm {
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  color: color-mix(in srgb, var(--foreground) 50%, transparent);
  cursor: pointer;
  opacity: 0;
  transition: opacity 0.1s;
}
.btn-rm svg { width: 14px; height: 14px; }
.q-item:hover .btn-rm { opacity: 1; }
.btn-rm:hover { color: oklch(60% 0.2 20); }

.empty {
  text-align: center;
  padding: 64px 0;
  color: color-mix(in srgb, var(--foreground) 50%, transparent);
}
</style>
